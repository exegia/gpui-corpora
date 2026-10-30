# Branching and releases

Start work from `dev`. Work PRs may target `dev` or another work branch for a
stack. Squash the stack from its bottom PR upward. Production follows
`dev → next → main`; no workflow merges a PR automatically.

| Workflow | Purpose |
| --- | --- |
| `ci.yml` | Rust checks, release tests, and a staging draft after a green `dev` push |
| `pr-base-policy.yml` | Conventional titles and valid work, stacked, and promotion bases |
| `secret-scan.yml` | Credential scanning before merge |
| `next-preview.yml` | Staging validation, registry publishing, preview deploy, and a production draft |
| `release-tag.yml` | Stable SemVer tag and GitHub Release on the production squash commit |
| `production-deploy.yml` | Production deployment, called by the release workflow or retried manually |

## Review and promote

1. Merge work into `dev` with a title such as `feat: add search`, `fix: repair
   search`, `doc: explain setup`, `ai: improve indexing`, or `chore: update CI`.
2. A green CI run drafts `release/dev-to-next → next`. Review it, mark it ready,
   wait for checks, and squash merge it.
3. The staging workflow runs the configured build, registry, and deployment
   commands. On success it drafts `release/next-to-main → main`.
4. Review the production draft, mark it ready, wait for checks, and squash merge
   it. The release workflow tags that exact commit and calls production deploy.

Promotion branches contain the validated source tree on top of their target
branch. This keeps later PR diffs correct after squash merges. Refreshing a
promotion returns it to draft so new content receives checks before review.
`dev`, `next`, and `main` are never reset or force-pushed by these workflows.

## Versioning

Stable versions start from `v0.0.0` when there is no release. A breaking title
(`feat!:` or `fix(api)!:`) or `BREAKING CHANGE:` footer bumps major; `feat:` and
`feature:` bump minor; other accepted types bump patch. The largest pending
bump wins. Production releases record the staging commit in release metadata,
so a later fix-only cycle does not count old features again.

Only production commits receive stable `vX.Y.Z` tags. Re-running a release
reuses its tag and release. Do not remove the `release-plan` metadata from
promotion PR bodies or GitHub Release notes. Existing stable releases need
this baseline metadata before this pipeline can continue from them.

## Validation

```sh
python3 -m unittest discover -s .github/scripts/tests -v
actionlint .github/workflows/*.yml
cargo fmt --all -- --check
cargo clippy --workspace --all-features
cargo test --workspace --all-features
```

The CI aggregate check is `check`; the PR policy check is `guard`. Secret
scanning reports `Detect hardcoded secrets`. The checked-in ruleset templates
require these checks and squash merges. Changing these templates does not
apply settings to the live repository.

## Publishing and deployment

`next-preview.yml` retains the configured container registry build/push and
preview deployment commands. `production-deploy.yml` retains the production
command and environment. Configure the variables and secrets named in those
files. Production deployment is called explicitly because a release created
with `GITHUB_TOKEN` does not start another release-event workflow.

GitHub Actions must be allowed to create pull requests in repository settings.
Generated PRs start as drafts; a maintainer's **Ready for review** action runs
CI, policy, and secret checks. No model token or agent provider is required.
The obsolete Python/PyPI publisher is not part of this Rust app's pipeline.
