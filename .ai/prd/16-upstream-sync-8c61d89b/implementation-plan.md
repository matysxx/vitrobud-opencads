# OCSSTACK-16 Implementation Plan

1. Review all upstream changes since `403247cb`, with special attention to web
   packaging, Cargo pins, save/export paths, renderer selection, and command
   registration.
2. Merge exact upstream `8c61d89b` in this isolated worktree and resolve
   conflicts by retaining upstream architecture while reapplying only the
   maintained container and DXF R12 deltas.
3. Adapt the OCI build to compile the new `web-app.html` target directly at
   `/`, preserving the private runtime and reverse-proxy route contract.
4. Update project context, technical specification, README/runbooks, and
   regression coverage where the verified behavior changed.
5. Run local validation and inspect the final fork-only delta for privacy and
   upstream maintainability.
6. Commit, fast-forward local `main`, and push the exact verified revision to
   anonymized GitHub `main`.
7. Present one exact SSH rollout command. After approval, update the private
   rollout revision, rebuild only this image, recreate only this stack, and
   verify health and the served application.
