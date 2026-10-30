import importlib.util
import json
import os
import re
from pathlib import Path
import subprocess
import tempfile
from types import SimpleNamespace
import unittest
from unittest.mock import patch


SCRIPTS = Path(__file__).resolve().parents[1]
spec = importlib.util.spec_from_file_location("release", SCRIPTS / "release.py")
release = importlib.util.module_from_spec(spec)
spec.loader.exec_module(release)


class WorkflowContractTests(unittest.TestCase):
    def test_retained_actions_have_required_immutable_pins(self):
        for workflow in (SCRIPTS.parent / "workflows").glob("*.yml"):
            for action in re.findall(r"uses: ([^\s#]+)", workflow.read_text()):
                if not action.startswith("./"):
                    with self.subTest(workflow=workflow.name, action=action):
                        self.assertRegex(action, r"@([0-9a-f]{40})$")

    def test_promotion_readiness_runs_every_required_pr_check(self):
        for name in ["ci.yml", "pr-base-policy.yml", "secret-scan.yml"]:
            source = (SCRIPTS.parent / "workflows" / name).read_text()
            self.assertIn("ready_for_review", source, name)

    def test_only_release_and_validation_workflows_remain(self):
        names = {path.name for path in (SCRIPTS.parent / "workflows").iterdir() if path.is_file()}
        self.assertEqual(names, {"ci.yml", "pr-base-policy.yml", "secret-scan.yml",
                                "next-preview.yml", "release-tag.yml", "production-deploy.yml"})


class VersionTests(unittest.TestCase):
    def test_conventional_bumps(self):
        for message, expected in [
            ("feat: search", "minor"), ("feat(ui): search", "minor"),
            ("fix: repair", "patch"), ("doc: explain", "patch"),
            ("ai: improve", "patch"), ("chore: update", "patch"),
            ("fix(api)!: remove endpoint", "major"),
            ("feat: search\n\nBREAKING CHANGE: remove endpoint", "major"),
            ("chore: update\nBREAKING-CHANGE: remove endpoint", "major"),
        ]:
            with self.subTest(message=message):
                self.assertEqual(release.bump_level(message), expected)

    def test_versions_reset_lower_components(self):
        self.assertEqual(release.next_version("v1.2.3", "major"), "v2.0.0")
        self.assertEqual(release.next_version("v1.2.3", "minor"), "v1.3.0")
        self.assertEqual(release.next_version("v1.2.3", "patch"), "v1.2.4")


class PolicyTests(unittest.TestCase):
    def pr(self, head="ai/setting-window-1", base="feat/scaffold-app-1", title="feat: settings"):
        repo = {"full_name": "example/corpora"}
        return {"head": {"ref": head, "repo": repo}, "base": {"ref": base, "repo": repo},
                "title": title, "body": release.marker({"source": "next"})}

    def test_stacked_pr_and_normal_work_prefixes_are_accepted(self):
        for prefix in ["feat", "fix", "doc", "ai", "chore", "feature", "bug", "docs", "ci"]:
            release.validate_pr(self.pr(head=f"{prefix}/work"))
            release.validate_pr(self.pr(head=f"{prefix}/work", base="dev"))

    def test_promotion_heads_are_restricted(self):
        for head, base in [("release/dev-to-next", "next"), ("release/next-to-main", "main")]:
            release.validate_pr(self.pr(head, base))
        for head, base in [("dev", "next"), ("next", "main"), ("ai/work", "main")]:
            with self.assertRaises(ValueError):
                release.validate_pr(self.pr(head, base))
        pr = self.pr("release/next-to-main", "main")
        pr["head"]["repo"] = {"full_name": "someone/fork"}
        with self.assertRaises(ValueError):
            release.validate_pr(pr)

    def test_invalid_titles_and_bases_fail_without_retargeting(self):
        for pr in [self.pr(title="update"), self.pr(base="unsupported")]:
            before = json.dumps(pr)
            with self.assertRaises(ValueError):
                release.validate_pr(pr)
            self.assertEqual(json.dumps(pr), before)


class PromotionTests(unittest.TestCase):
    def setUp(self):
        directory = tempfile.TemporaryDirectory()
        self.addCleanup(directory.cleanup)
        self.previous = Path.cwd()
        self.root = Path(directory.name)
        os.chdir(self.root)
        self.addCleanup(os.chdir, self.previous)
        self.git("init", "-q", "-b", "main")
        self.git("config", "user.name", "Release test")
        self.git("config", "user.email", "test@example.invalid")
        self.git("init", "--bare", "-q", "remote.git")
        self.git("remote", "add", "origin", str(self.root / "remote.git"))
        Path("app.txt").write_text("initial\n")
        self.git("add", "app.txt")
        self.git("commit", "-q", "-m", "chore: initial")
        self.initial = self.git("rev-parse", "HEAD")
        for lane in ["main", "next", "dev"]:
            self.git("update-ref", f"refs/remotes/origin/{lane}", self.initial)
        self.prs = {}
        self.merged = []
        self.releases = {}
        self.commits = []
        self.calls = []
        self.ready_calls = []
        self.next_pr = 1
        self.real_run = subprocess.run
        self.addCleanup(patch.stopall)
        patch.dict(os.environ, {"GH_REPO": "example/corpora", "VALIDATION_URL": "https://example.invalid/run",
                                "GITHUB_OUTPUT": str(self.root / "output")}).start()
        patch.object(release, "gh", side_effect=self.gh).start()
        patch.object(release, "api", side_effect=self.api).start()
        patch.object(release, "branch_sha", side_effect=self.branch_sha).start()
        patch.object(release.subprocess, "run", side_effect=self.fake_run).start()

    def git(self, *args, data=None):
        return subprocess.check_output(["git", *args], input=data, text=True).strip()

    def fake_run(self, args, **kwargs):
        if args[:3] == ["gh", "release", "view"]:
            exists = args[3] in self.releases
            return SimpleNamespace(returncode=0 if exists else 1, stderr="" if exists else "release not found")
        return self.real_run(args, **kwargs)

    def branch_sha(self, branch):
        result = self.real_run(["git", "rev-parse", "--verify", f"refs/heads/{branch}"],
                               capture_output=True, text=True)
        return result.stdout.strip() if result.returncode == 0 else None

    def api(self, path, method="GET", payload=None):
        self.calls.append((method, path, payload))
        if path == "git/commits":
            args = ["commit-tree", payload["tree"]]
            for parent in payload["parents"]:
                args.extend(["-p", parent])
            sha = self.git(*args, data=payload["message"])
            self.commits.append(payload)
            return {"sha": sha}
        ref = payload["ref"] if path == "git/refs" else path.removeprefix("git/")
        self.git("update-ref", ref, payload["sha"])
        if method == "PATCH":
            self.assertIs(payload["force"], False)
        return {}

    def gh(self, *args, payload=None):
        if args[:2] == ("pr", "list"):
            base = args[args.index("--base") + 1]
            if args[args.index("--state") + 1] == "merged":
                return json.dumps([{"body": self.merged[-1]["body"]}] if self.merged else [])
            return json.dumps([self.prs[base]] if base in self.prs else [])
        if args[:2] == ("pr", "ready"):
            self.assertIn("--undo", args)
            self.ready_calls.append(args)
            for pr in self.prs.values():
                if str(pr["number"]) == args[2]:
                    pr["isDraft"] = True
            return ""
        if args[:2] in [("pr", "create"), ("pr", "edit")]:
            body = Path(args[args.index("--body-file") + 1]).read_text()
            plan = release.metadata(body)
            base = "next" if plan["source"] == "dev" else "main"
            if args[1] == "create":
                self.assertIn("--draft", args)
            is_draft = True if args[1] == "create" else self.prs[base]["isDraft"]
            self.prs[base] = {"number": self.next_pr, "body": body, "isDraft": is_draft, "isCrossRepository": False,
                              "title": args[args.index("--title") + 1]}
            self.next_pr += 1
            return ""
        if args[:2] == ("release", "view"):
            return json.dumps({"body": self.releases[args[2]]})
        if args[:2] == ("release", "create"):
            self.releases[args[2]] = Path(args[args.index("--notes-file") + 1]).read_text()
            return ""
        self.fail(f"Unexpected gh command: {args}")

    def commit(self, message, contents):
        parent = self.git("rev-parse", "origin/dev")
        self.git("checkout", "-q", "--detach", parent)
        Path("app.txt").write_text(contents)
        self.git("add", "app.txt")
        self.git("commit", "-q", "-m", message)
        sha = self.git("rev-parse", "HEAD")
        self.git("update-ref", "refs/remotes/origin/dev", sha)
        return sha

    def promote(self, source):
        os.environ["VALIDATED_SHA"] = self.git("rev-parse", f"origin/{source}")
        release.promote(source)

    def squash(self, target):
        pr = self.prs.pop(target)
        plan = release.metadata(pr["body"])
        head = self.branch_sha(f"release/{plan['source']}-to-{target}")
        merge_sha = self.git("commit-tree", self.git("rev-parse", f"{head}^{{tree}}"),
                             "-p", self.git("rev-parse", f"origin/{target}"), data=pr["title"])
        self.git("update-ref", f"refs/remotes/origin/{target}", merge_sha)
        if target == "next":
            self.merged.append(pr)
        return {**pr, "merged": True, "merge_commit_sha": merge_sha,
                "base": {"ref": target}, "head": {"ref": f"release/{plan['source']}-to-{target}", "sha": head}}

    def test_promotion_has_exact_source_tree_and_target_ancestry(self):
        source = self.commit("feat: add search", "search\n")
        self.promote("dev")
        branch = self.branch_sha("release/dev-to-next")
        self.assertEqual(self.git("rev-parse", f"{branch}^{{tree}}"), self.git("rev-parse", f"{source}^{{tree}}"))
        self.assertEqual(self.git("merge-base", branch, "origin/next"), self.initial)
        self.assertIn("v0.1.0", self.prs["next"]["title"])

    def test_reruns_do_not_create_duplicate_commits(self):
        self.commit("feat: add search", "search\n")
        self.promote("dev")
        before = len(self.commits)
        self.promote("dev")
        self.assertEqual(len(self.commits), before)

    def test_refresh_is_fast_forward_without_changing_source(self):
        self.commit("feat: add search", "search\n")
        self.promote("dev")
        previous = self.branch_sha("release/dev-to-next")
        self.prs["next"]["isDraft"] = False
        source = self.commit("fix: repair search", "repaired\n")
        self.promote("dev")
        self.assertIn(previous, self.commits[-1]["parents"])
        self.assertTrue(self.prs["next"]["isDraft"])
        self.assertEqual(len(self.ready_calls), 1)
        self.assertEqual(self.git("rev-parse", "origin/dev"), source)

    def test_stale_validation_and_identical_trees_do_not_promote(self):
        self.promote("dev")
        self.commit("feat: add search", "search\n")
        os.environ["VALIDATED_SHA"] = self.initial
        release.promote("dev")
        self.assertEqual(self.calls, [])

    def test_feature_release_then_fix_release_does_not_repeat_minor_bump(self):
        self.commit("feat: add search", "search\n")
        for expected in ["v0.1.0", "v0.1.1"]:
            self.promote("dev")
            self.assertIn(expected, self.prs["next"]["title"])
            self.squash("next")
            self.promote("next")
            pr = self.squash("main")
            release.release(pr)
            self.assertEqual(self.git("rev-parse", f"{expected}^{{commit}}"), pr["merge_commit_sha"])
            self.assertIn(expected, self.releases)
            before = self.git("tag")
            release.release(pr)
            self.assertEqual(self.git("tag"), before)
            if expected == "v0.1.0":
                self.commit("fix: repair search", "repaired\n")

    def test_release_rejects_content_outside_the_reviewed_snapshot(self):
        self.commit("feat: add search", "search\n")
        self.promote("dev")
        self.squash("next")
        self.promote("next")
        pr = self.squash("main")
        pr["merge_commit_sha"] = self.initial
        with self.assertRaises(ValueError):
            release.release(pr)
        self.assertEqual(self.git("tag"), "")


if __name__ == "__main__":
    unittest.main()
