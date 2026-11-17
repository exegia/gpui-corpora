# Test the current API in Corpora

This QA branch starts from local `dev` at `2e96fc5`. The remote had no `dev`
branch when this work began. It does not include the newer sidecar branch.

## Start with public checks

Run `bash scripts/api-qa.sh`, open **Home → Open API QA**, and select
**Check public API**. The target is `https://api.exegia.co/v1`.
The result must show health 200, valid capability fields, and an anonymous
corpus-list denial (401). The legacy `hub_writable` flag does not govern
new corpus creation.

The launcher sets `CORPORA_QA_MODE` and uses separate QA preferences, data,
cache, and instance identity. It does not reset your normal Corpora state.
No service-role or Hugging Face key belongs in this app.

## Run the disposable corpus test

1. Ask the QA administrator for two fresh dedicated user sessions in the API's
   Supabase project. Store the JSON below outside the repository with mode 600.
2. Set `CORPORA_QA_CREDENTIALS` to that file and `CORPORA_QA_RECEIPT` to a new
   local filename. Restart with `bash scripts/api-qa.sh`.
3. Select **Run disposable corpus test**. It creates real synthetic data,
   publishes it through the API, makes it private, deletes it, and signs out
   the owner. Allow 1–3 minutes under normal service conditions.
4. Read the visible result and sanitized receipt. An existing receipt prevents
   a second run so an uncertain operation cannot be hidden by a new UUID.
5. Have the administrator verify Hub absence and retire the temporary accounts.
   Close the app and remove the credential file. Retain the sanitized receipt.

```json
{
  "user_token": "REPLACE_ME",
  "foreign_token": "REPLACE_ME",
  "supabase_url": "https://ivaecofevxactmmupvyp.supabase.co",
  "publishable_key": "REPLACE_ME"
}
```

The screen verifies creation, real initial import, exact import retry, the five
owner detail views, anonymous/foreign private denial, public reads, invalid-token
denial, privacy reversal, foreign deletion denial, deletion and exact retry,
post-deletion reads, UUID reuse rejection, and session revocation.

If any step fails, keep the recorded corpus/import/deletion IDs and inspect
the relevant API receipt. Do not delete unknown data or retry with new IDs.
Product deletion retains audit history and a tombstone; no database history
is erased by this runner.

## Fixture and verification

`crates/corpora_services/src/services/api_qa/tiny.corpus` is a 2,350-byte synthetic
archive with shipped CFR and the text **In the gap and out.** Its incomplete
passage coverage deliberately checks that the reader retains the middle word.
Node 3 has annotation `uncovered`. SHA256:
`8ff99caaf9a9e25297e7dba4ae3e431f5d5065203ed407f63a5477d0f5aefb2d`.

Run `cargo fmt --all -- --check`, `cargo clippy --workspace --all-features -- -D warnings`,
and `cargo test --workspace --all-features`. Then click both checks in the
rebuilt app; compilation alone is not native QA evidence.

The report excludes tokens and raw upstream errors. It does not prove every
endpoint, file format, latency target, or failure-recovery scenario.
See https://docs.exegia.co/core/qa-testing for the shared cleanup procedure.
