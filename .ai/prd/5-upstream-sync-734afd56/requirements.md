# OCSSTACK-5 Sync Upstream Revision 734afd56

## Goal

Integrate the four upstream commits after `2c5b7e76` through exact revision
`734afd5611c3d4ea4db6b807123d95f0098ad74b`, before verification and release of
the canonical-command cleanup.

## Requirements

- [ ] Preserve upstream ancestry through exact revision `734afd56`.
- [ ] Preserve the canonical `EXPORTDXFR12` command and keep `EXPORTR12` absent
  from runtime code and user documentation.
- [ ] Preserve R12 fail-closed behavior and regression coverage.
- [ ] Confirm upstream now contains the no-default-features ACIS web build fix.
- [ ] Verify AREA, supporter/UI, plot-selector, and workflow changes introduce no
  container/toolchain change or private repository data.
- [ ] Deliver through local -> anonymized GitHub -> server with separate commit,
  PR/merge, SSH verification, and rollout approvals.

## Acceptance Criteria

- [ ] Merge completes without unresolved conflicts.
- [ ] `734afd56` is an ancestor of the integration revision.
- [ ] No `EXPORTR12` remains in runtime/user docs.
- [ ] R12 tests pass 3/3, full Trunk/WASM and worker builds pass, and disposable
  `/healthz` succeeds before production rollout.
