# Native corpus workspace

Explore opens local Text-Fabric feature directories and compiled CFR files through
Context-Fabric 0.6.0-rc.1, pinned at `3f431862f67defca3717d8d20ec4b9498e607419`.
Python and network services are not involved in opening, reading, or searching.
The engine manages compilation and its CFR cache when opening TF source.

Each workspace tab owns its corpus and selection. Choose a corpus or enter its
absolute path and press Enter. Structure lists the available node types in pages
of 50 nodes. Select a node to read its default text format. Previous/next passage
uses Context-Fabric's same-type navigation. Section references come from the
corpus's configured section features.

Search accepts Context-Fabric templates, including multiple lines. Submit with
the Search button or the input's submit shortcut. Results show at most 100 rows;
select a result to open its containing innermost section. The first matched node
is shown in each result label. Passages display up to 16,000 characters; choose a
smaller section when the preview is truncated.

For a small local example, open `crates/corpora_services/tests/fixtures/native`
from this checkout. The first verse reads “hello world”; `word word=hello` finds
that verse. `word word=absent` returns no matches.

Loading, passage rendering, structure reads, and search execute off the UI
thread. A newer operation cancels and supersedes the preceding operation in that
tab. Closing the tab cancels its pending work. Search uses the engine's interrupt
hook. Initial TF compilation and text rendering have no mid-operation interrupt
API in this pinned engine; cancelled results are discarded after that call
returns. Cancellation does not roll back an engine cache already written.

This slice does not persist open corpora across restarts or resolve catalog IDs
into local paths. The Viewer route uses the tab's local corpus view. It does not
implement corpus downloads, annotations, editing, or conversion.

## Validation

The macOS development app was rebuilt from the isolated `dev`-based checkout.
The local fixture opened through the path input and Enter, and its generated CFR
opened directly. Passage navigation and `word word=hello` search selected the
expected verse. Route changes retained the corpus and result state. Light and
dark appearance, window zoom, rapid panel toggles, and tab creation/closing were
checked. A second close during a tab transition needed another click after the
transition settled; selection and content agreed afterward.

The workspace suite covers reduced motion, panel reversal and width retention,
and stale/cancelled native completions. Manual drag-resizing during panel motion
and a production-sized corpus were not checked. Process CPU was sampled after
interaction (0.4–1.3%); this is a spot check, not a performance benchmark.
