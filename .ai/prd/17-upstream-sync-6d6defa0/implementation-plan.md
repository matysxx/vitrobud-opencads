# OCSSTACK-17 Implementation Plan

1. Review upstream changes from `8c61d89b` through exact target `6d6defa0`,
   focusing on dependencies, web build, save/export paths, command registry,
   renderer selection, and cross-document clipboard behavior.
2. Merge upstream in the isolated worktree, retain upstream architecture, and
   reapply only the maintained container and DXF R12 deltas where conflicts
   occur.
3. Make `ROLLOUT_REVISION` the single immutable input for both checkout and OCI
   image tag, retaining `dev` only for non-rollout local configuration.
4. Update project specification and relevant runbooks for version 0.9.8,
   dependency pins, and the single-source image-tag rule.
5. Run available local validation and inspect the final fork delta for privacy,
   route stability, R12 integration, and maintainability.
6. Commit, fast-forward local `main`, and push the verified revision to
   anonymized GitHub `main`.
7. Present one exact SSH command. After approval, update only the private
   rollout revision, rebuild/recreate only this web stack, and verify revision,
   image tag, health, and the application root.

