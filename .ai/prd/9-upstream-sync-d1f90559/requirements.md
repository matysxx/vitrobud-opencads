# OCSSTACK-9 Sync Upstream Revision d1f90559

## Goal

Extend the reviewed upstream integration to exact revision
`d1f905590126495c734c63984f2345ffbbd2ec70` without changing the strict R12
export contract or publishing changes to an upstream repository.

## Requirements and acceptance

- Preserve auditable merge ancestry through `d1f90559`.
- Retain both `EXPORTDXFR12` and upstream `PRINTALL`.
- Confirm the four new commits do not alter the isolated R12 writer or acadifc
  dependency pin.
- Keep the public repository anonymized and the local Nextcloud plan separate.
- Publish only to `matysxx/vitrobud-opencads`; do not create an upstream PR.
- Re-run exact-revision tests and image build before server rollout.
