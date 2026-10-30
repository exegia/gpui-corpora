# Release lifecycle

Start work from `dev`, using a short-lived branch such as `feat/search`,
`fix/index`, `doc/setup`, `ai/ranking`, or `chore/build`. A stacked PR targets
its parent work branch until that parent merges; then retarget it to `dev`.
The policy check validates the base without changing it.

```text
work / stacked PRs
        │ squash
        ▼
       dev ── green CI ── draft staging PR ── squash ──▶ next
                                                        │
                                         staging validation / publish / deploy
                                                        │
                                                draft production PR
                                                        │ squash
                                                        ▼
                                                       main
                                                        │
                                              vX.Y.Z tag + release
                                                        │
                                                production deploy
```

The staging and production drafts use target-based snapshot branches named
`release/dev-to-next` and `release/next-to-main`. Source history remains intact;
the snapshot branch records the validated source commit. Later promotions stay
clean even when the preceding promotion was squash merged.

See [WORKFLOW.md](WORKFLOW.md) for the six workflows, validation commands,
version rules, and deployment configuration. Both promotion steps require
human review; no workflow approves or merges its own PR.
