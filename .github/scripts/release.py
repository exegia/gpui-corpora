"""Draft squash-safe promotions and tag the reviewed production commit."""

import argparse
import json
import os
from pathlib import Path
import re
import subprocess
import tempfile


MARKER = re.compile(r"<!-- release-plan: (\{[^\n]+\}) -->")
VERSION = re.compile(r"v(\d+)\.(\d+)\.(\d+)")
CONVENTIONAL = re.compile(r"^(feat|feature|fix|bug|doc|docs|ai|chore|ci|refactor|perf|test|style|build|revert)(\([^\n)]+\))?(!)?: .+", re.M)
WORK_BRANCH = re.compile(r"^(feat|feature|fix|bug|doc|docs|ai|chore|ci|refactor|perf|test|style|build|hotfix|dependabot|copilot|claude)/.+$")


def command(*args, data=None):
    return subprocess.check_output(args, input=data, text=True).strip()


def git(*args):
    return command("git", *args)


def gh(*args, payload=None):
    if payload is not None:
        return command("gh", *args, "--input", "-", data=json.dumps(payload))
    return command("gh", *args)


def api(path, method="GET", payload=None):
    return json.loads(gh("api", "--method", method, f"repos/{os.environ['GH_REPO']}/{path}", payload=payload))


def metadata(body):
    match = MARKER.search(body or "")
    return json.loads(match[1]) if match else None


def bump_level(messages):
    if re.search(r"^BREAKING[ -]CHANGE:", messages, re.M) or any(
        match[3] for match in CONVENTIONAL.finditer(messages)
    ):
        return "major"
    if re.search(r"^(feat|feature)(\([^\n)]+\))?: ", messages, re.M):
        return "minor"
    return "patch"


def next_version(tag, level):
    match = VERSION.fullmatch(tag)
    if not match:
        raise ValueError(f"Invalid stable version: {tag}")
    major, minor, patch = map(int, match.groups())
    if level == "major":
        return f"v{major + 1}.0.0"
    if level == "minor":
        return f"v{major}.{minor + 1}.0"
    return f"v{major}.{minor}.{patch + 1}"


def validate_pr(pr):
    head, base = pr["head"]["ref"], pr["base"]["ref"]
    same_repo = pr["head"].get("repo", {}).get("full_name") == pr["base"]["repo"]["full_name"]
    promotions = {"next": {"release/dev-to-next"}, "main": {"release/next-to-main"}}
    if base in promotions:
        if not same_repo or head not in promotions[base]:
            raise ValueError(f"Only {sorted(promotions[base])} may target {base}")
    elif base != "dev" and not WORK_BRANCH.fullmatch(base):
        raise ValueError(f"Unsupported base: {base}; target dev or a work branch")
    elif not WORK_BRANCH.fullmatch(head):
        raise ValueError(f"Unsupported work branch: {head}")
    if not CONVENTIONAL.fullmatch(pr["title"]):
        raise ValueError("Use a conventional PR title, for example feat: add search")
    if head.startswith("release/") and metadata(pr.get("body")) is None:
        raise ValueError("Generated promotions must include their release plan")


def messages_since(base, head):
    if base:
        subprocess.run(["git", "merge-base", "--is-ancestor", base, head], check=True)
    return git("log", "--format=%B", f"{base}..{head}" if base else head)


def stable_release():
    tags = [tag for tag in git("tag", "--merged", "origin/main").splitlines() if VERSION.fullmatch(tag)]
    if not tags:
        return "v0.0.0", None
    tag = max(tags, key=lambda value: tuple(map(int, VERSION.fullmatch(value).groups())))
    body = json.loads(gh("release", "view", tag, "--json", "body"))["body"]
    plan = metadata(body)
    if not plan or plan.get("source") != "next":
        raise ValueError(f"Release {tag} lacks the next commit baseline; supply its release-plan metadata")
    return tag, plan["source_sha"]


def last_staged_source():
    prs = json.loads(gh("pr", "list", "--base", "next", "--head", "release/dev-to-next",
                        "--state", "merged", "--limit", "1", "--json", "body"))
    if not prs:
        return None
    plan = metadata(prs[0]["body"])
    if not plan:
        raise ValueError("The last staging promotion lacks release-plan metadata")
    return plan["source_sha"]


def promotion_plan(source, source_sha, target_sha):
    tag, baseline = stable_release()
    messages = messages_since(baseline, "origin/next")
    if source == "dev":
        messages += "\n" + messages_since(last_staged_source(), source_sha)
    level = bump_level(messages)
    return {"source": source, "source_sha": source_sha, "target_sha": target_sha,
            "version": next_version(tag, level), "bump": level}


def branch_sha(branch):
    result = subprocess.run(["gh", "api", f"repos/{os.environ['GH_REPO']}/git/ref/heads/{branch}"],
                            text=True, capture_output=True)
    if result.returncode == 0:
        return json.loads(result.stdout)["object"]["sha"]
    if "HTTP 404" in result.stderr:
        return None
    raise RuntimeError(result.stderr)


def marker(plan):
    return f"<!-- release-plan: {json.dumps(plan, sort_keys=True)} -->"


def promote(source):
    target = "next" if source == "dev" else "main"
    source_sha = git("rev-parse", f"origin/{source}")
    if source_sha != os.environ["VALIDATED_SHA"]:
        print("A newer source commit is awaiting validation; skipping this stale run")
        return
    target_sha = git("rev-parse", f"origin/{target}")
    if git("rev-parse", f"{source_sha}^{{tree}}") == git("rev-parse", f"{target_sha}^{{tree}}"):
        print("Source and target already have the same content")
        return
    branch = f"release/{source}-to-{target}"
    prs = json.loads(gh("pr", "list", "--base", target, "--head", branch, "--state", "open", "--json", "number,body,isDraft,isCrossRepository"))
    prs = [pr for pr in prs if not pr["isCrossRepository"]]
    plan = promotion_plan(source, source_sha, target_sha)
    if prs and metadata(prs[0]["body"]) == plan:
        print(f"PR #{prs[0]['number']} already contains this validated snapshot")
        return
    title_type = {"major": "feat(release)!", "minor": "feat(release)", "patch": "fix(release)"}[plan["bump"]]
    title = f"{title_type}: {'stage' if source == 'dev' else 'release'} {plan['version']}"
    body = (f"Promote `{source}` at `{source_sha}` into `{target}` by squash merge.\n\n"
            f"Candidate: **{plan['version']}** ({plan['bump']}).\n\n"
            f"Validation: {os.environ['VALIDATION_URL']}\n\n{marker(plan)}")
    if prs and not prs[0]["isDraft"]:
        gh("pr", "ready", str(prs[0]["number"]), "--undo")
    previous = branch_sha(branch)
    parents = list(dict.fromkeys([target_sha] + ([previous] if previous else [])))
    # A target-based snapshot keeps later PR diffs clean after squash merges.
    commit = api("git/commits", "POST", {"message": title, "tree": git("rev-parse", f"{source_sha}^{{tree}}"), "parents": parents})
    if previous:
        api(f"git/refs/heads/{branch}", "PATCH", {"sha": commit["sha"], "force": False})
    else:
        api("git/refs", "POST", {"ref": f"refs/heads/{branch}", "sha": commit["sha"]})
    with tempfile.TemporaryDirectory() as directory:
        path = Path(directory) / "body.md"
        path.write_text(body)
        if prs:
            gh("pr", "edit", str(prs[0]["number"]), "--title", title, "--body-file", str(path))
        else:
            gh("pr", "create", "--draft", "--base", target, "--head", branch,
               "--title", title, "--body-file", str(path))


def release(pr):
    if not pr["merged"] or pr["base"]["ref"] != "main" or pr["head"]["ref"] != "release/next-to-main":
        raise ValueError("Production releases require a merged staging promotion")
    plan = metadata(pr.get("body"))
    if not plan or plan["source"] != "next":
        raise ValueError("Production PR lacks a staging release plan")
    subprocess.run(["git", "merge-base", "--is-ancestor", plan["source_sha"], "origin/next"], check=True)
    merge_sha = pr["merge_commit_sha"]
    source_tree = git("rev-parse", f"{plan['source_sha']}^{{tree}}")
    if any(git("rev-parse", f"{sha}^{{tree}}") != source_tree for sha in [merge_sha, pr["head"]["sha"]]):
        raise ValueError("Production content differs from the reviewed staging snapshot")
    tags = [tag for tag in git("tag", "--points-at", merge_sha).splitlines() if VERSION.fullmatch(tag)]
    if tags:
        tag = tags[0]
    else:
        latest, baseline = stable_release()
        tag = next_version(latest, bump_level(messages_since(baseline, plan["source_sha"])))
        git("tag", "-a", tag, "-m", f"Release {tag}", merge_sha)
        git("push", "origin", tag)
    plan["version"] = tag
    existing = subprocess.run(["gh", "release", "view", tag], capture_output=True, text=True)
    if existing.returncode:
        if "not found" not in existing.stderr.lower() and "HTTP 404" not in existing.stderr:
            raise RuntimeError(existing.stderr)
        with tempfile.TemporaryDirectory() as directory:
            path = Path(directory) / "notes.md"
            path.write_text(f"Production release from PR #{pr['number']}.\n\n{marker(plan)}")
            gh("release", "create", tag, "--target", merge_sha, "--title", tag,
               "--notes-file", str(path), "--generate-notes")
    with open(os.environ["GITHUB_OUTPUT"], "a") as output:
        output.write(f"tag={tag}\n")


def main():
    parser = argparse.ArgumentParser()
    parser.add_argument("action", choices=["policy", "promote-dev", "promote-next", "release"])
    args = parser.parse_args()
    if args.action == "policy":
        validate_pr(json.loads(Path(os.environ["GITHUB_EVENT_PATH"]).read_text())["pull_request"])
    elif args.action == "release":
        release(json.loads(Path(os.environ["GITHUB_EVENT_PATH"]).read_text())["pull_request"])
    else:
        promote("dev" if args.action == "promote-dev" else "next")


if __name__ == "__main__":
    main()
