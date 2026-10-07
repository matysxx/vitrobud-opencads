# OCSSTACK-22 — Synchronize upstream and complete documentation

## Goal

Integrate upstream `41d93b6f5421477b0566197dd4ee581f7d48db3e` through
local -> anonymized GitHub -> server, preserving maintained fork behavior.

## Requirements and acceptance criteria

- Preserve upstream ancestry and existing deployment scripts, privacy boundaries,
  root application URL, WebGL2 behavior, and weekly-release fork guard.
- Adapt the isolated ASCII R12 exporter to renamed upstream codec dependencies
  without changing output semantics, and run its regression tests explicitly.
- Preserve the separate unfinished local OCSSTACK-12 working tree.
- Refresh technical specification and upstream maintenance documentation.
- Pass full native tests and the WASM build check before server deployment.
- Deploy only the full verified fork revision after showing the SSH command
  and receiving approval; build on Debian with rootless Podman.
- Record verification and deployment state in ignored local task wiki.

## Scope

Upstream integration, necessary compatibility changes, and documentation.
LDAP, Nextcloud implementation, new export capabilities, and upstream PRs are
outside this task. The user's update request authorizes this established scope.

## Open questions

None for local integration. Server operations require the displayed command's
approval under the user's standing policy.
