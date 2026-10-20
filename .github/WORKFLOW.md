# Branching and CI

`dev` is the default integration branch. Changes target `dev`, promotion goes
from `dev` to `next`, and production releases go from `next` to `main`.
See [BRANCH-AND-RELEASE-POLICY.md](BRANCH-AND-RELEASE-POLICY.md) and
[LIFECYCLE.md](LIFECYCLE.md) for the policy and workflow stages.

## Rust checks

`ci.yml` checks formatting, strict Clippy, and tests on macOS with Rust 1.98.1
and the declared minimum version, Rust 1.95.0. It runs on pull requests and
pushes to `dev`, `next`, and `main`.

```sh
cargo fmt --all -- --check
cargo clippy --workspace --all-features
cargo test --workspace --all-features
```

`secret-scan.yml` scans for credentials. Dependabot checks Cargo and GitHub
Actions dependencies. `validation.yml` validates the agentic workflow sources
and release-policy assets. When those Markdown sources change, regenerate
their lock files with the pinned compiler:

```sh
gh aw compile --actionlint
```

## Promotion and releases

The agentic `dev-to-next.md` workflow prepares a promotion PR after Validation
passes. `next-preview.yml` runs configured preview commands, and
`next-to-main-release-pr.yml` opens the production PR after Preview CI passes.
`release-tag.yml` tags a merged `next` → `main` PR; `production-deploy.yml`
runs configured deployment commands after a release is published.

The inherited Python wheel/PyPI and `release/v*` Make pipeline is not used by
this Rust application. Application packaging remains in `scripts/` and
`justfile`. No Python package or trusted PyPI publisher is required.

## Copilot configuration

The agentic workflows authenticate using the workflow's `GITHUB_TOKEN` and
`copilot-requests: write`. An `exegia` organization owner must enable
**Allow use of Copilot CLI billed to the organization** under
Settings → Copilot → Policies → Copilot CLI. This bills usage to the
organization; see [GitHub's setup instructions](https://docs.github.com/en/copilot/how-tos/copilot-cli/use-copilot-cli-in-actions).

Until that organization policy is enabled, agent jobs fail with a provider
HTTP 403 even when Rust CI and workflow validation pass. Adding GitHub App
credentials does not fix this Copilot policy error.

The separate Claude workflows need `CLAUDE_CODE_OAUTH_TOKEN`. Preview and
production commands are optional repository variables documented in their
workflow files; deployment runs only after a command is configured.
