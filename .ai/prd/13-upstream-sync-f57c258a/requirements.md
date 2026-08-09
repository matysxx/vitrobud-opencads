# OCSSTACK-13 Sync Reviewed Upstream Revision f57c258a

## Requirements

- [ ] Merge upstream `main` at exact revision
  `f57c258ada7a76731f6a0a7894752ee1665ce334`.
- [ ] Preserve the fork-only `EXPORTDXFR12` command, its tests, and ordinary
  save behavior.
- [ ] Keep unfinished OCSSTACK-12 diagnostic work outside this update.
- [ ] Preserve the reusable container stack and public anonymization boundary.
- [ ] Review Rust, WASM, Trunk, and container-builder dependency changes.
- [ ] Verify the merged source and fork-specific tests locally before push.
- [ ] Publish only the reviewed revision to GitHub `main`.
- [ ] Roll out on the private Debian runtime only from that GitHub revision,
  rebuilding the image because application source and dependencies changed.

## Upstream scope

The target includes Open CAD Studio v0.9.4 and 44 later commits. Major changes
include the cadkernel/acadifc stack transition, curve-based geometry, renderer
and scene-pipeline restructuring, unit handling, block palette, LAYTRANS,
expanded locale catalogs, and fixes to snapping, dimensions, hatches, trim,
offset, splines, paper space, and browser file loading.
