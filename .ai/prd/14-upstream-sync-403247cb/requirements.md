# OCSSTACK-14 Sync Reviewed Upstream Revision 403247cb

## Requirements

- [ ] Merge upstream `main` at exact revision
  `403247cbc3c4348987e9a61ef7aced01b7692f5e`.
- [ ] Preserve the fork-only `EXPORTDXFR12` command, exporter module, tests,
  command autocomplete entry, browser download, and native file path.
- [ ] Preserve the reusable rootless-Podman stack, external TLS boundary,
  systemd-user integration, and public anonymization rules.
- [ ] Keep unfinished OCSSTACK-12 DXF diagnostics and Nextcloud planning out of
  this update.
- [ ] Review the upstream removal of the `solid3d` feature and direct CAD-stack
  dependency transition for the WASM Containerfile build.
- [ ] Pin all builder inputs required by the merged `Cargo.lock`.
- [ ] Verify static integration locally, then publish the exact reviewed commit
  to GitHub `main` before any server operation.
- [ ] Rebuild and replace only this stack on Debian, verify the exact revision,
  container health, and HTTPS application behavior.

## Upstream scope

The target contains Open CAD Studio v0.9.5, v0.9.6, and 29 later commits (176
new commits from the current fork baseline). It retires the old solid3d feature
gate so web builds receive kernel-backed solids, moves to direct CAD stack
dependencies, expands plugin API V4, reworks rendering and block instancing,
adds associative/multi-region hatch behavior and many unit, annotation, snap,
plot, localization, and stability fixes.
