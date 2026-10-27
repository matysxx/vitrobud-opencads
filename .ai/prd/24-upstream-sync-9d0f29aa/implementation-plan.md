# Implementation plan — OCSSTACK-24

User authorized planning and immediate execution on 2026-10-27.

1. Merge exact reviewed upstream 9d0f29aa without squashing in the isolated
   worktree based on f5f862ef. Preserve unfinished original checkout.
2. Resolve integration conflicts preserving upstream features and fork R12
   command, isolated handlers, explicit test target and deterministic font tests.
   Review opencaddocument imports and adapt only where compilation requires it.
3. Add a packaging regression test rejecting the external analytics loader in
   web-app.html; remove that loader from the fork entry point. Keep bundle cache
   stamping and root URL. Do not claim all application networking is disabled.
4. Update technical spec dependency pins and affected maintenance/privacy/export
   documentation. No private deployment details in tracked files.
5. Run packaging and shell checks, inspect maintained delta, privacy and ancestry.
   Publish Conventional Commit to origin/main without rewriting history. Require
   exact-SHA Tests (all native OS and R12), Web and MCP success before rollout.
6. Present exact SSH command: require expected clean baseline, back up private
   env, set sole rollout revision, invoke dev-ops/update, verify SHA/image health,
   user autostart and cache stamp. Execute only after approval and green CI.
7. Verify HTTPS startup and update ignored task wiki. Record rollback and manual
   Fusion/machine testing limitation; no backup timer or unrelated stack changes.

Risks: document API migration may require exporter adaptations; preserve fail-safe
semantics and synthetic regression fixtures. CI failures must be diagnosed, not
skipped. New upstream commits after the reviewed SHA belong to a later update.
