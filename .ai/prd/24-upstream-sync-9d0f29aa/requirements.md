# OCSSTACK-24 — Synchronize upstream and preserve private runtime

## Status

Requirements approved on 2026-10-27. User authorized planning followed immediately
by implementation; exact SSH approval remains a separate gate.

## Goal

Integrate reviewed upstream `9d0f29aac0b238871ad450620972a0a952cba64d`
(package `2026.43.0`) from fork baseline
`f5f862ef339dcfeeb9f56a47d985036f4f3d3928`, then validate and deploy through
local -> anonymized GitHub -> server.

## Background

The update contains 360 upstream commits and changes 490 files. The codec alias
now targets `opencaddocument`, affecting document APIs and ordinary SAVE; upstream
also changes periodic spline serialization and adds converters with loss review.
The web entry now loads a third-party analytics script. Existing dirty R12 work
is separate and must remain untouched.

## Requirements

- Preserve merge ancestry and all maintained fork behavior, including the sole
  EXPORTDXFR12 command, ASCII writer, tests and isolated update handlers.
- Adapt exporter APIs only as needed for the new document library; do not silently
  discard unsupported geometry or claim universal machine/Fusion compatibility.
- Preserve rootless Podman, root application URL, WebGL2, worker/cache stamping,
  autostart, privacy exclusions and fork release guard.
- Preserve or supersede font test fixes only with equivalent deterministic coverage.
- Exclude the newly introduced external analytics loader from the private web
  build; keep this packaging choice explicit and regression-tested.
- Update technical specification and affected export, privacy, browser-update,
  maintenance and rollout documentation. Keep operational task memory local.
- Audit published changes for private data; do not publish local drawings or wiki.
- Require exact-revision native OS tests, R12 regressions, Web, MCP and packaging
  checks before server rollout. Do not build containers on the Mac.
- Show exact SSH commands and obtain approval before every server operation.
- Preserve rollback revision; verify rebuilt image, health, user service and HTTPS.

## Scope

Includes synchronization, necessary compatibility changes, tests and documentation.
Excludes unfinished local R12 work, Nextcloud/LDAP integration, upstream PRs,
backup timer activation, proxy/firewall changes and unrelated stacks.

## Acceptance criteria

- Verify both baseline and reviewed upstream are ancestors of the final commit.
- Pass required CI and privacy checks for the exact published revision.
- Verify the server runs that revision and the rebuilt web container is healthy.
- Record evidence, limitations and rollback reference in ignored local wiki.

## Open questions

- Approve the requirements, including excluding the new external analytics loader?
- Actual Fusion and machine acceptance remains a separate manual test.
