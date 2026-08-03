# OCSSTACK-3 Sync Upstream Revision 2c5b7e76

## Goal

Integrate the four Open CAD Studio commits after the reviewed `v0.9.2`
baseline through revision `2c5b7e76`, while preserving the reusable container
stack and the isolated binary DXF R12 exporter.

## Requirements

- [ ] Merge `upstream/main` at exact revision `2c5b7e76` without rewriting
  upstream history.
- [ ] Preserve `EXPORTDXFR12`, alias `EXPORTR12`, normal save behavior, tests,
  and public documentation.
- [ ] Review the new `acadrust` pin and confirm the existing Rust,
  wasm-bindgen, Trunk, and container builder pins remain compatible.
- [ ] Update the documented reviewed upstream baseline without introducing a
  second source of truth for runtime revision or image tag.
- [ ] Verify the merged tree with static checks, the targeted R12 regression
  tests, a full web image build, and a disposable health check.
- [ ] Publish only anonymized repository content after separate commit/push
  approval, then roll out only the exact GitHub revision after separate SSH
  approval.

## Scope

In scope: upstream font/ViewCube improvements, creation-style fixes, ACIS solid
rendering fixes, dependency lock update, merge documentation, and verification.

Out of scope: LDAP, Nextcloud, reverse proxy, firewall, private configuration,
other stacks, global Podman changes, and functional changes to the R12 exporter.

## Acceptance Criteria

- [ ] Git ancestry contains upstream revision `2c5b7e76`.
- [ ] Synthetic and real merge complete without unresolved conflicts.
- [ ] Both R12 command names and exporter integration remain present.
- [ ] Targeted exporter tests pass 3/3 and the web image builds successfully.
- [ ] Disposable runtime passes `/healthz` before production activation.
- [ ] Local, GitHub, and server revisions are recorded exactly at each boundary.
