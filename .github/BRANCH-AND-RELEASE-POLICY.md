# Branch and release policy

## Branches and pull requests

| Branch | Role | Allowed target |
| --- | --- | --- |
| `feat/*`, `fix/*`, `doc/*`, `ai/*`, `chore/*` and conventional aliases | Work started from `dev` | `dev` or a short-lived parent work branch |
| `release/dev-to-next` | Validated snapshot of `dev` | `next` |
| `release/next-to-main` | Validated snapshot of `next` | `main` |
| `dev` | Integration | Promoted through the generated staging draft |
| `next` | Staging | Promoted through the generated production draft |
| `main` | Production | Stable release tags and production deployment |

PR titles use lowercase conventional types: `feat`, `feature`, `fix`, `bug`,
`doc`, `docs`, `ai`, `chore`, `ci`, `refactor`, `perf`, `test`, `style`, `build`,
or `revert`. An optional scope and breaking `!` are accepted. Branch aliases
and automation prefixes are listed in `.github/scripts/release.py`.

The policy check never retargets or closes a PR. A stacked PR keeps its chosen
base, including `feat/scaffold-app-1`. Direct work PRs into `next` or `main`
fail validation; those branches accept only their generated promotion draft.

## Merge and release rules

Squash merge work and promotion PRs. The PR title becomes the squash subject;
include breaking footers in the squash message when applicable. Promotion
branches use exact source trees and retain target ancestry without force pushes.

Stable SemVer is computed from pending staged changes: breaking change → major,
feature → minor, other accepted types → patch. The first baseline is `v0.0.0`.
The production tag points at the reviewed squash commit, and the GitHub Release
records the source staging SHA for the next cycle. Tags and published releases
are reused on retries.

`main`, `next`, and `dev` should be protected against deletion and force pushes.
The checked-in rulesets allow squash merges and require `guard`, `check`, and
`Detect hardcoded secrets`. Live rulesets and repository merge settings are
administrative configuration; editing their templates does not apply them.

## Automation

There are no agentic workflows, generated workflow locks, provider credentials,
issue-to-branch bots, or automatic PR merges. CI creates a staging draft only
after validation; staging creates a production draft only after preview success.
The maintainer marks drafts ready, reviews checks, and merges them.

[WORKFLOW.md](WORKFLOW.md) lists the workflow files and configuration.
[LIFECYCLE.md](LIFECYCLE.md) shows the complete release path.
