# Repository instructions

Read `AGENTS.md` and `CONTRIBUTING.md` before changing code. Application entry
points live in `src/`; shared Rust crates live in `crates/`.

Run formatting, Clippy, and workspace tests from the repository root. Release
workflow tests run with `python3 -m unittest discover -s .github/scripts/tests -v`.
Use `actionlint .github/workflows/*.yml` for Actions changes.

Start work from `dev`. Work PRs target `dev` or a parent work branch. Promotion
uses reviewed draft PRs from `dev` through `next` to `main`. See
`.github/WORKFLOW.md` for release and deployment configuration.

Never commit credentials. Keep real values in local environment files or GitHub
secrets, and preserve the secret-scanning check.
