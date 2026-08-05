# OCSSTACK-10 Switch Machine Export to ASCII DXF R12

## Evidence

The strict binary AC1009 output is structurally valid and accepted by Fusion
360, but the target legacy machine rejects it. An ASCII conversion containing
the exact same 75 DXF groups, written as AC1009 with CRLF line endings, passed
the machine test. The serialization, not the geometry or section inventory, is
therefore the confirmed compatibility boundary.

## Requirements

- [ ] Keep `EXPORTDXFR12` as the only machine-export command.
- [ ] Preserve the isolated, non-mutating export path and standard save logic.
- [ ] Write ASCII DXF R12 (`AC1009`) using group/value line pairs and CRLF only.
- [ ] Preserve the existing supported geometry, normalization and fail-closed
  unsupported-entity policy.
- [ ] Validate the generated ASCII stream before download or file replacement.
- [ ] Require HEADER, TABLES, BLOCKS and ENTITIES, finite numeric values,
  matching entity inventory and a final EOF.
- [ ] Keep customer and machine-test drawings out of the public repository.
- [ ] Update public documentation from binary to ASCII without rewriting
  historical task records.
- [ ] Do not add an upstream pull request as part of this task.

## Acceptance criteria

- [ ] Independent tests parse generated ASCII group/value pairs.
- [ ] Tests prove CRLF-only output, AC1009, ENTITIES and terminal EOF.
- [ ] Line and bulged-polyline regressions preserve values and source state.
- [ ] Unsupported entities still stop the export with an explicit error.
- [ ] A reviewed branch build passes automated tests before rollout.
- [ ] The target legacy machine remains the final acceptance boundary.
