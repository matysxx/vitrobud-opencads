# OCSSTACK-13 Implementation Plan

1. Fetch and record upstream `f57c258a` and GitHub `main`.
2. Integrate upstream in an isolated worktree based on local `main`.
3. Resolve conflicts narrowly, preserving upstream behavior and the isolated
   fork-only R12 exporter.
4. Update the recorded upstream baseline and builder pins from the merged lock
   file; do not introduce host-specific data.
5. Run conflict-marker, privacy, formatting, Rust, web/WASM, exporter, shell,
   and Compose checks in proportion to available local tooling.
6. Commit the reviewed integration with Conventional Commits and fast-forward
   local `main` without touching the dirty OCSSTACK-12 branch.
7. Push the exact local `main` revision to the anonymized GitHub repository.
8. Show the exact SSH rollout command and wait for approval.
9. On Debian, fetch GitHub `main`, verify the expected revision, rebuild the
   image, replace only this stack, and confirm health/version.
