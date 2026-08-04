# OCSSTACK-8 Sync Upstream Revision b0806679

## Goal

Integrate reviewed upstream Open CAD Studio revision
`b0806679c9ff2934ca6ce586306cc361f2925bfc` while retaining the isolated,
anonymized stack and DXF R12 compatibility command.

## Requirements

- [ ] Preserve auditable upstream ancestry; do not copy a source snapshot.
- [ ] Resolve shared command-registry changes by retaining both upstream
  `PRINTALL` and the fork's canonical `EXPORTDXFR12` command.
- [ ] Preserve all stack-only files and public privacy boundaries.
- [ ] Revalidate the strict R12 exporter after integration.
- [ ] Keep version sources unchanged unless upstream changes them; identify the
  exact revision separately from the unchanged `0.9.2` application version.
- [ ] Commit/push and server rollout require their normal separate approvals.

## Acceptance criteria

- [ ] Merge ancestry includes upstream `b0806679`.
- [ ] No unresolved conflicts or conflict markers remain.
- [ ] `EXPORTDXFR12` and upstream `PRINTALL` are both registered.
- [ ] Targeted R12 tests and the full web image build pass at the exact GitHub
  revision before rollout.
