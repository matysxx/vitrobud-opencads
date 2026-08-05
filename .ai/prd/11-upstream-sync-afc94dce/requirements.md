# OCSSTACK-11 Sync Reviewed Upstream Revision afc94dce

## Requirements

- [ ] Merge upstream `main` at exact revision
  `afc94dce1350cfaa4ac2d6983328721c1bcdcb6e`.
- [ ] Preserve the isolated `EXPORTDXFR12` command and normal save behavior.
- [ ] Preserve the reusable container stack and anonymization boundary.
- [ ] Review dependency and container-builder changes before rollout.
- [ ] Verify both upstream functionality and fork-specific tests on the exact
  branch revision before merging into rollout branch `main`.

## Upstream scope

The update adds Open CAD Studio v0.9.3 plus QSELECT workflow expansion, UCS
integration, view/projection fixes, selection improvements and UI refinements.
The reviewed upstream diff does not modify the fork's isolated R12 exporter or
its integration test file.
