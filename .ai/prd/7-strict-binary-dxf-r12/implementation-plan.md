# Implementation Plan: OCSSTACK-7

## 1. Isolate strict R12 framing

Replace `DxfBinaryWriter` only inside `src/io/export_dxf_r12.rs` with a small
writer for the exact value types used by this exporter. Emit pre-R13 group-code
framing, little-endian numeric payloads and null-terminated strings.

## 2. Add an independent verifier

Parse the produced stream from raw bytes using the strict R12 framing rules.
Check record boundaries, value widths, section transitions, `AC1009`, entity
inventory and `EOF`. Do not call `acadrust::DxfReader` from verification.

## 3. Replace self-referential tests

Update `tests/export_dxf_r12.rs` with an independent generated-stream parser.
Assert exact first-record bytes, line geometry/extents, legacy polyline records
and bulge data. Add a negative regression for the former two-byte framing.

## 4. Audit and verify locally

Run formatting/tests if Rust tooling is available, plus `git diff --check`,
scope and privacy audits. The Mac remains source-first and does not run the
application container.

## 5. Publish and roll out only after approvals

After separate commit/push approval, publish anonymized changes to GitHub.
Build and test the exact GitHub revision on Debian only after displaying the
exact SSH command and receiving approval. Replace only this stack after build,
health and generated-file checks; retain the previous image for rollback.

## Rollback

Before activation retain the currently healthy exact image and private runtime
configuration. If build, health, strict verification or machine acceptance
fails, keep or restore only the previous Open CAD Studio container; do not
touch reverse proxy, firewall, global Podman or other stacks.
