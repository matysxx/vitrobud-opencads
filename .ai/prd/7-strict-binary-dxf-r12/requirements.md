# OCSSTACK-7 Correct Strict Binary DXF R12 Framing

## Goal

Correct `EXPORTDXFR12` so it emits the pre-R13 binary DXF wire format required
by strict R12 consumers, without changing normal save/export behavior.

## Confirmed defect

The isolated R12 exporter labels output as `AC1009` but delegates framing to
acadifc's general `DxfBinaryWriter`. That writer always emits a two-byte
little-endian group code. Binary DXF R12 requires a one-byte group code for
codes below 255. A strict machine reader therefore loses synchronization at
the first record and reports the later `ENTITIES` section as missing or invalid.
Fusion 360 accepting the same file is evidence of a permissive reader, not of
R12 conformance.

## Requirements

- [ ] Write the standard binary DXF sentinel followed by genuine pre-R13 group
  framing: one byte for codes 0 through 254; `0xFF` plus a little-endian `u16`
  for any higher code.
- [ ] Keep the current `AC1009` section/entity policy and fail-closed supported
  entity set.
- [ ] Do not use acadifc's general binary writer on the R12 compatibility path.
- [ ] Validate generated output with an independent strict R12 parser that
  does not share the faulty writer/reader framing assumption.
- [ ] Verify `AC1009`, section balance, a non-empty valid `ENTITIES` section
  when entities are expected, exact top-level entity count, terminal `EOF`, and
  absence of forbidden post-R12 records.
- [ ] Preserve atomic native file replacement and browser download behavior.
- [ ] Add byte-level generated tests; do not commit user or customer DXF files.
- [ ] Keep `SAVE`, `SAVEAS`, general DXF handling, reverse proxy, runtime and
  other stacks unchanged.
- [ ] Deliver only through `local -> anonymized GitHub -> server`, with separate
  approval before commit/push and before every SSH command.

## Acceptance criteria

- [ ] Immediately after the sentinel, the first record is byte `0x00` followed
  by `SECTION\0`, not `0x00 0x00 SECTION\0`.
- [ ] Generated one-line and bulged-polyline files pass an independent strict
  parser and preserve their expected records and values.
- [ ] A synthetic stream using the former two-byte group-code framing is
  rejected by the strict verifier.
- [ ] Targeted Rust tests and the web image build pass at the exact published
  revision.
- [ ] A newly exported representative drawing is accepted by the target
  machine before the rollout is considered complete.

## Upstream boundary

The immediate correction belongs in this fork's isolated compatibility
exporter because it can be implemented without changing upstream application
behavior. A reusable version-aware pre-R13 binary writer and parser correction
should subsequently be proposed to `OpenAEC-Foundation/acadifc`; adopting it
later must retain these independent byte-level regression tests.
