# Copilot instructions for `gpui-corpora`

## Validation commands

Run commands from the repository root.

| Task | Command |
| --- | --- |
| Run the macOS development app | `just mac` |
| Check Rust formatting | `cargo fmt --all -- --check` |
| Lint the workspace | `cargo clippy --workspace --all-features` |
| Test the workspace | `cargo test --workspace --all-features` |
| Compile agentic workflows | `gh aw compile` |
| Compile + lint generated workflows | `gh aw compile --actionlint` |
| Show workflow status | `gh aw status` |
| Install the secret-scanning hook | `pre-commit install` |
| Scan working tree for secrets | `gitleaks dir .` |

Read `AGENTS.md` and `CONTRIBUTING.md` before changing code. Python packaging and the old Make-based release pipeline are not used by this Rust application.

## High-level architecture

This repository contains the Corpora desktop application, built with Rust, GPUI, and Ely. Application entry points live in `src/`; workspace crates live in `crates/`. The inherited automation surfaces are:

- `.github/workflows/*.md`: source files for GitHub Agentic Workflows
- `.github/workflows/*.lock.yml`: compiled workflow artifacts
- plain workflows such as `ci.yml`, `validation.yml`, `secret-scan.yml`, `pr-base-policy.yml`, and `release-tag.yml`
- starter docs: `README.md`, `AGENTS.md`, `CLAUDE.md`
- repo policy docs: `.github/LIFECYCLE.md`, `.github/BRANCH-AND-RELEASE-POLICY.md`

## Repository-specific conventions

### Workflow conventions

- Treat workflow markdown as the source of truth; recompile after frontmatter changes.
- Keep agentic jobs read-only and use safe outputs for mutations.
- Prefer generic validation that works before downstream code exists.

### Branching conventions

- `dev` is the default branch; lifecycle branches target it.
- Promotion happens from `dev` to `next`.
- Release happens from `next` to `main`.
- `main` is read-only except for explicitly configured owner and GitHub Actions bypass actors.

### Integration conventions

- `copilot-setup-steps.yml` is the bootstrap hook for GitHub Copilot coding agents.
- Copilot agent jobs require the organization's **Allow use of Copilot CLI billed to the organization** policy. See `WORKFLOW.md`; HTTP 403 from this policy is not a token-wiring error.
- `claude.yml` and `claude-code-review.yml` depend on `CLAUDE_CODE_OAUTH_TOKEN`.
- Do not commit local-only agent state such as `.claude/`, `.agents/`, or workflow logs.

### Secret conventions

- Never commit credentials, tokens, or API keys — including in tests, fixtures, and examples.
- Real values belong in `.env`, which is gitignored; commit `.env.example` with placeholders.
- `gitleaks` runs as a pre-commit hook and as the `Secret Scan` CI workflow. Do not bypass the
  hook with `--no-verify`; if it fires, remove and rotate the secret.
