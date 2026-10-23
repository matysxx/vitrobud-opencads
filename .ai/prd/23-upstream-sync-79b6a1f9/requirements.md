# OCSSTACK-23 — Synchronize upstream and refresh container documentation

## Goal

Integrate reviewed upstream `79b6a1f97d0455e43f2e553afce2a19031e567a7`
(package version `2026.42.0`, 2026-10-23), retaining maintained fork behavior
and the local -> anonymized GitHub -> server delivery path.

## Background

The last verified deployed fork is `74f9db31ab64fd106b34d4116ce1e4f885c97d25`,
based on upstream `41d93b6f`. Upstream has advanced 1225 commits. It now includes
shared SAVE serialization through `ocs_save`, worker-routed operations, PWA
offline caching, and expanded cross-platform CI. The codec still uses the Rust
alias `codec`, but its package is now `acadrust`.

## Requirements

- Preserve upstream ancestry with a reviewed merge, without history rewriting.
- Preserve `EXPORTDXFR12`, ASCII R12 output semantics, isolated update handlers,
  explicit regression test registration, and the absence of the removed alias.
- Review codec API changes and shared SAVE/worker paths; do not silently route
  the separate R12 exporter through ordinary SAVE or claim new Fusion support.
- Preserve all maintained fork fixes, root application URL, WebGL2 rendering,
  rootless Podman runtime, user autostart, and weekly-release fork guard.
- Keep the original dirty checkout and unfinished R12/Nextcloud task work intact;
  integrate only published fork changes in the isolated task worktree.
- Exclude private wiki, local task status, env backups, runtime state and private
  drawings from Git and Podman/Docker contexts, retaining required dist templates.
- Review new PWA/service-worker scope and update behavior at `/`; document how
  to verify the deployed version without losing unsaved browser drawings.
- Refresh the technical specification and relevant deployment, rollback,
  privacy, upstream-maintenance and exporter documentation.
- Verify native tests (including the fork R12 target), WASM checks, operational
  shell scripts, privacy exclusions, and release guards before server rollout.
- Publish only reviewed anonymized changes. Server deployment must use the exact
  verified full GitHub SHA, rebuild on Debian, and affect only this stack.
- Obtain approval for each exact SSH command. Verify revision, image, HTTP health,
  user autostart and HTTPS startup; record the outcome in ignored local wiki.

## Scope

Upstream merge, necessary compatibility fixes, privacy exclusions, regression
verification and documentation are included. No upstream PR, new export format,
LDAP/Nextcloud implementation, backup-timer activation, unrelated stack change,
or Mac container execution is included.

## Acceptance criteria

- Confirm that the merged revision contains both the reviewed upstream ancestor
  and the maintained fork baseline.
- Pass the explicit R12 regression target and required CI checks at the final SHA.
- Confirm that local wiki and env backups are ignored while reusable policies
  and `.env.dist` templates are not ignored.
- Complete an approved exact-revision server rebuild/rollout and HTTPS acceptance.
- Preserve unfinished local work and record truthful verification limits.

## Approval and open questions

Requirements approved by the user on 2026-10-23. Prepare and present the
implementation plan for approval before code changes.
No technical choice is currently blocked; API conflicts and builder requirements
must be established by review/testing, not assumed compatible.
