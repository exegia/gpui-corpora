# AGENTS.md

Hard rules for humans and coding agents in this repo. They override model or
personal defaults. Setup, workflow, and PR process live in CONTRIBUTING.md.

## Comments

- Max 1-2 lines. If a comment needs a paragraph, refactor the code instead.
- A comment states a non-obvious constraint: why the code is shaped this way,
  a workaround, an invariant. Never narrate what the next line does.
- Never explain a diff or PR in code ("changed because...", "new:", "removed
  per review"). That belongs in the commit message.

## Prose

- All copy (comments, README, docs, commit text) reads like a competent human
  wrote it: direct, concrete, factual. No marketing filler. No adjectives
  doing the work facts should do.
- Banned tells: "delve", "seamless", "leverage", "It's not just X, it's Y",
  rule-of-three padding, em-dash spam.
- Doc comments (`///`) on public APIs state what callers get and any
  preconditions, in plain sentences.

## De-bloat

- Dead code is deleted, never commented out. Git keeps the history.
- Unused dependencies are removed, not left for later.
- No speculative abstractions. Write the second implementation when the
  second use case exists.

## Rust quality gates

All three must pass before you call the work done. CI runs clippy with
`-D warnings` and rejects anything less.

```sh
cargo fmt --all -- --check
cargo clippy --workspace --all-features
cargo test --workspace --all-features
```

`just clippy` and `just test` run those commands verbatim. `just fmt` is
write-mode formatting (sort-derives, rustfmt, taplo, rumdl), not a check;
`just check` is a separate cargo check pass.

- Match existing patterns: imports, module layout, error handling, naming.
  Read the neighboring files first.
- Conventional Commits (`feat:`, `fix:`, ...), subject under 72 characters.
  The body explains why, not what the diff shows.

## UI glitch check

For UI changes, finish with a short check in the rebuilt app from the edited
checkout. Keep the check focused on the affected controls.

- Switch routes and tabs, create a tab, and close the active tab and its neighbor
  quickly. Selection, page content, and available panels must agree throughout.
- Toggle panels twice quickly and reverse a transition midway. Resize a panel
  during motion, then hide and reopen it. Check for jumps, blank frames, clipped
  controls, and lost widths; check hover and pressed states over glass surfaces.
- Check keyboard activation and reduced motion when affected. Check window
  resizing and both themes when the change touches layout or color.
- Let motion settle and check that idle CPU settles too. Look for repeated
  notifications or tasks; the on-screen render timer alone is not a performance
  measurement.
- Fix reproducible glitches with the smallest change and a focused regression
  test. Advance the test clock for motion checks. Report what ran and any gaps;
  do not add a performance framework or unrelated refactors for this check.

## Review lenses

For anything beyond mechanical edits, review against the relevant body of
practice. These own their details; this file does not duplicate them.

- Async code (tokio runtime, channels, cancellation): rust-async-patterns
- Tests (naming, assertions, harness layout): rust-testing
- General Rust (borrowing vs cloning, error handling, performance):
  rust-best-practices
- UI components and design conventions: gpui-kit, gpui-kit-design-guides
