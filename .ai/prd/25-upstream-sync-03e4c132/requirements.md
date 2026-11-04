# OCSSTACK-25 — Upstream sync 03e4c132

## Goal
Integrate Open CAD Studio 2026.44.0 at upstream SHA
`03e4c1324021afe183e44dd4149de7bff135fa6a` while preserving the
anonymized rootless Podman stack and all maintained fork changes.

## Requirements
- Preserve merge ancestry, one EXPORTDXFR12 command, its writer and regression
  tests, isolated handlers, WebGL2, cache stamping and fork release guard.
- Keep external analytics out of the private web entry.
- Review dependency/API and CI/release workflow changes; do not publish releases
  from this fork inadvertently.
- Update public documentation without private runtime values or drawings.
- Require exact-SHA packaging, native/R12, web and MCP CI before server rebuild.
- Deploy only via local -> GitHub -> server, with exact SSH command approved first.

## Scope and acceptance
No unrelated stack edits, upstream PR, backup timer activation or direct server
hotfix. Confirm both upstream and prior fork as ancestors; test and privacy audit
the result; verify exact server revision, image, health, autostart and HTTPS.
Manual machine/Fusion importer testing remains separate.
