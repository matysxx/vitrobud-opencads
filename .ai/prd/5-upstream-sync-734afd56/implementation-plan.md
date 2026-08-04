# Implementation Plan: OCSSTACK-5

1. Fetch and audit exact upstream commits through `734afd56`, dependency/version
   changes, file overlap, and synthetic merge outcome.
2. Merge exact upstream revision into the existing OCSSTACK-4 integration branch
   with `--no-ff --no-commit` for review.
3. Update the reviewed upstream baseline and audit command registration,
   fail-closed exporter, adopted web fix, workflow privacy, and whitespace.
4. After separate approval, create an auditable upstream merge commit and push
   the updated integration branch.
5. Verify the exact GitHub revision on Debian with R12 tests, full web/worker OCI
   build, and disposable health check.
6. Merge the verified branch to public `main` and perform a separately approved
   rollback-safe rollout of only this stack.

## Rollback

Production remains on `7ef9faa` until the new exact revision passes every
boundary. Retain the current image and private `.env` for stack-local rollback.
