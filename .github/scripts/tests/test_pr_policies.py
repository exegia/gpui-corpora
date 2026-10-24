import json
import os
from pathlib import Path
import subprocess
import tempfile
import textwrap
import unittest


WORKFLOWS = Path(__file__).resolve().parents[2] / "workflows"


def policy_script(name):
    source = (WORKFLOWS / name).read_text()
    return textwrap.dedent(source.split("        run: |\n", 1)[1])


class PolicyTests(unittest.TestCase):
    def setUp(self):
        self.directory = tempfile.TemporaryDirectory()
        self.addCleanup(self.directory.cleanup)
        self.root = Path(self.directory.name)
        self.log = self.root / "calls.jsonl"
        gh = self.root / "gh"
        gh.write_text(textwrap.dedent('''\
            #!/usr/bin/env python3
            import json
            import os
            from pathlib import Path
            import sys

            args = sys.argv[1:]
            if os.environ.get("GH_REPO") != "exegia/gpui-corpora":
                sys.exit("repository context missing")
            with open(os.environ["GH_FAKE_LOG"], "a") as log:
                log.write(json.dumps(args) + "\\n")
            if os.environ.get("GH_FAKE_FAIL"):
                sys.exit("GitHub API unavailable")
            state = Path(os.environ["GH_FAKE_LOG"]).with_suffix(".issue")
            if args[:2] == ["api", "--paginate"]:
                if state.exists():
                    print(state.read_text())
            elif args[0] == "api" and args[1].endswith("/milestones"):
                print(os.environ.get("GH_FAKE_MILESTONE", "7"))
            elif args[:2] == ["issue", "create"]:
                url = "https://github.com/exegia/gpui-corpora/issues/999"
                state.write_text(url)
                print(url)
            elif args[:2] not in (["pr", "comment"], ["pr", "edit"], ["pr", "close"]):
                sys.exit("unexpected GitHub command: " + repr(args))
            '''))
        gh.chmod(0o755)
        self.env = {
            **os.environ,
            "PATH": str(self.root) + os.pathsep + os.environ["PATH"],
            "GH_FAKE_LOG": str(self.log),
            "GH_REPO": "exegia/gpui-corpora",
            "GITHUB_REPOSITORY": "exegia/gpui-corpora",
            "PR_NUMBER": "18",
            "PR_URL": "https://github.com/exegia/gpui-corpora/pull/18",
            "PR_TITLE": "feat: setting-window-1",
            "PR_AUTHOR": "contributor",
            "MILESTONE": "⚙️ EPIC: Workflow",
            "HEAD_REF": "ai/setting-window-1",
            "BASE_REF": "feat/scaffold-app-1",
        }

    def run_policy(self, name, **env):
        result = subprocess.run(
            ["bash", "-c", policy_script(name)],
            cwd=self.root,
            env={**self.env, **env},
            capture_output=True,
            text=True,
        )
        self.assertEqual(result.returncode, 0, result.stdout + result.stderr)
        return result

    def calls(self):
        if not self.log.exists():
            return []
        return [json.loads(line) for line in self.log.read_text().splitlines()]

    def test_stacked_bases_are_preserved_without_git_or_api_writes(self):
        for prefix in ["feature", "feat", "bug", "fix", "doc", "docs", "chore", "ai", "hotfix", "dependabot", "copilot", "claude"]:
            with self.subTest(prefix=prefix):
                self.run_policy("pr-base-policy.yml", BASE_REF=f"{prefix}/parent")
        self.assertEqual(self.calls(), [])

    def test_integration_and_release_promotions_stay_allowed(self):
        for head, base in [("ai/setting-window-1", "dev"), ("dev", "next"), ("next", "main")]:
            with self.subTest(head=head, base=base):
                self.run_policy("pr-base-policy.yml", HEAD_REF=head, BASE_REF=base)
        self.assertEqual(self.calls(), [])

    def test_main_rejects_a_non_promotion_without_a_checkout(self):
        self.run_policy("pr-base-policy.yml", BASE_REF="main")
        self.assertEqual([call[:2] for call in self.calls()], [["pr", "comment"], ["pr", "close"]])

    def test_unsupported_bases_still_redirect_to_dev(self):
        for base in ["next", "unsupported"]:
            with self.subTest(base=base):
                self.run_policy("pr-base-policy.yml", BASE_REF=base)
                self.assertEqual(self.calls()[-1], ["pr", "edit", "18", "--base", "dev"])

    def test_manual_aliases_and_agent_heads_need_no_tracking_issue(self):
        for prefix in ["feature", "feat", "bug", "fix", "doc", "docs", "chore", "ai", "hotfix", "dependabot", "copilot", "claude"]:
            with self.subTest(prefix=prefix):
                self.run_policy("pr-branch-enforcement.yml", HEAD_REF=f"{prefix}/work")
        self.assertEqual(self.calls(), [])

    def test_tracking_issue_uses_repo_and_milestone_title_and_is_not_duplicated(self):
        self.run_policy("pr-branch-enforcement.yml", HEAD_REF="experiment")
        creation = next(call for call in self.calls() if call[:2] == ["issue", "create"])
        self.assertEqual(creation[creation.index("--milestone") + 1], self.env["MILESTONE"])
        self.assertIn("<!-- branch-tag-enforcement:pr-18 -->", creation[creation.index("--body") + 1])
        self.assertEqual(creation[creation.index("--title") + 1], "[FEATURE] feat: setting-window-1")
        before = len(self.calls())
        self.run_policy("pr-branch-enforcement.yml", HEAD_REF="experiment")
        self.assertEqual([call[:2] for call in self.calls()[before:]], [["api", "--paginate"]])

    def test_tracking_issue_does_not_require_an_existing_milestone(self):
        self.run_policy("pr-branch-enforcement.yml", HEAD_REF="experiment", GH_FAKE_MILESTONE="")
        creation = next(call for call in self.calls() if call[:2] == ["issue", "create"])
        self.assertNotIn("--milestone", creation)

    def test_api_errors_do_not_create_another_issue(self):
        result = subprocess.run(
            ["bash", "-c", policy_script("pr-branch-enforcement.yml")],
            cwd=self.root,
            env={**self.env, "HEAD_REF": "experiment", "GH_FAKE_FAIL": "1"},
            capture_output=True,
            text=True,
        )
        self.assertNotEqual(result.returncode, 0)
        self.assertEqual([call[:2] for call in self.calls()], [["api", "--paginate"]])

    def test_workflows_bind_repository_and_recheck_pr_updates(self):
        for name in ["pr-base-policy.yml", "pr-branch-enforcement.yml"]:
            with self.subTest(workflow=name):
                source = (WORKFLOWS / name).read_text()
                self.assertIn("GH_REPO: ${{ github.repository }}", source)
                self.assertIn("types: [opened, reopened, synchronize, edited]", source)
                self.assertIn("cancel-in-progress: false", source)


if __name__ == "__main__":
    unittest.main()
