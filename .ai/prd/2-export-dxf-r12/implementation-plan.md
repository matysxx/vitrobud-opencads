# Implementation Plan: OCSSTACK-2

## Overview

Implement `EXPORTDXFR12` with alias `EXPORTR12` as a separate, fail-safe binary
DXF R12 machine-export workflow. The work is split between generic
version-aware R12 serialization in `acadifc/acadrust` and isolated command/UI
integration in Open CAD Studio. Normal save behavior remains unchanged.

The implementation follows test-driven development and is delivered through
`local -> anonymized GitHub -> server`. No server action occurs until the exact
GitHub revision has passed local/static and disposable build verification.

## Steps

### 1. Freeze anonymized regression evidence

- **What:** Convert the diagnostic findings into generated minimal tests without
  importing user files into Git.
- **Where:** New tests in the temporary/public `acadifc` worktree and
  `tests/export_dxf_r12.rs` in Open CAD Studio.
- **How:** Programmatically create a document containing one line and another
  containing a closed bulged lightweight polyline. Add assertions demonstrating
  that version-inappropriate sections/codes must not appear in AC1009 output.
- **Tests:** Tests initially fail against the pinned writer; no binary fixture
  copied from Downloads.

### 2. Define the R12 compatibility model

- **What:** Add typed normalization results, supported-entity policy, and
  compatibility diagnostics.
- **Where:** Prefer generic types/functions in `acadifc`; add
  `src/io/export_dxf_r12.rs` for OCS policy and orchestration.
- **How:** Define supported native entities, lossless conversions,
  substitutions, and fatal unsupported-entity reports. Work on a cloned
  document and model space only.
- **Tests:** Unit tests for supported-type classification, deterministic reports,
  no live-document mutation, and unsupported entity failure.

### 3. Make acadifc emit genuine AC1009

- **What:** Add complete version-aware R12 behavior to the DXF writer.
- **Where:** `acadifc/src/io/dxf/writer/mod.rs`,
  `acadifc/src/io/dxf/writer/section_writer.rs`, and focused writer tests.
- **How:** For AC1009 binary output:
  - omit `CLASSES` and `OBJECTS`;
  - omit `BLOCK_RECORD`, handles/owners, subclass markers, layout data, and
    post-R12 group codes;
  - write only R12-compatible headers/tables/blocks/entities;
  - validate that no unsupported entity reaches serialization;
  - retain current behavior for AC1012+.
- **Tests:** Golden structural assertions for section order, forbidden codes,
  binary sentinel, AC1009, EOF, and unchanged R14/R2018 regression behavior.

### 4. Implement lossless model-space normalization

- **What:** Create a minimal R12 document from the OCS snapshot.
- **Where:** Generic conversion helpers in `acadifc` where reusable; OCS export
  policy in `src/io/export_dxf_r12.rs`.
- **How:** Copy supported model entities, rebuild minimal layer/linetype/style
  tables, map unsupported linetypes to `CONTINUOUS`, convert LWPOLYLINE to
  POLYLINE/VERTEX/SEQEND, rebuild required handles internal to the temporary
  model, and calculate finite extents.
- **Tests:** Exact geometry/topology/bulge/layer tests; empty drawing behavior;
  missing linetype substitution; unsupported entity failure.

### 5. Add serialization and post-write verification

- **What:** Produce binary bytes atomically and verify the result before final
  delivery.
- **Where:** `src/io/export_dxf_r12.rs` and `acadifc` reader/writer tests.
- **How:** Use `DxfWriter::new_binary().write_to_vec()`, verify sentinel and
  forbidden structures, re-read bytes, compare normalized entity inventory and
  extents, then return bytes plus a typed compatibility report.
- **Tests:** Corruption/failure cases, deterministic inventory, no partial file,
  and binary round-trip.

### 6. Integrate canonical command and alias

- **What:** Add `EXPORTDXFR12` and `EXPORTR12` to command dispatch and
  autocomplete.
- **Where:** `src/app/commands/display.rs`, `src/app/commands/mod.rs`, and
  `src/app/mod.rs`.
- **How:** Both names dispatch one new message; no interactive command state and
  no changes to `EXPORT`, `EXPORTPDF`, `SAVE`, or `SAVEAS`.
- **Tests:** Command registry/dispatch tests verify both names and identical
  behavior.

### 7. Add native and web file delivery

- **What:** Add asynchronous export preparation, file selection, success/failure
  reporting, and browser download support.
- **Where:** `src/app/update/mod.rs`, existing platform file abstractions, locale
  files, and generated locale catalog if required by project tooling.
- **How:** Suggest `<drawing>_R12.dxf`; generate/verify bytes before replacing a
  destination; use the existing web file-handle path; surface conversions and
  unsupported types in the command line.
- **Tests:** Native temporary-file test, WASM compile/test boundary, cancel flow,
  write failure, and no mutation of tab state.

### 8. Document behavior and upstream split

- **What:** Document the command contract, supported geometry, incompatibility
  policy, and contribution topology.
- **Where:** New `docs/export-dxf-r12.md`, README command list, PR notes for
  `acadifc` and Open CAD Studio.
- **How:** Keep examples generic and public. Document binary-only output and the
  distinction between machine export and normal drawing save.
- **Tests:** Documentation review against actual command names and supported
  entity matrix; privacy scan.

### 9. Run local verification without a Mac container

- **What:** Verify source and generated artifacts locally, following the
  workspace convention that container execution occurs only on the Debian
  server.
- **Where:** Local repository and disposable temporary directories.
- **How:** Run formatting, targeted Rust tests, native compile if available,
  WASM compile/check, binary structure inspector, read-back tests, and tracked
  data/privacy scans. Manually test generated files in Fusion 360 and the target
  machine before publication.
- **Tests:** All automated tests plus recorded pass/fail acceptance checklist;
  no private machine files committed.

### 10. Publish through anonymized GitHub

- **What:** Commit reviewed changes and publish the exact tested revision only
  after separate user approval.
- **Where:** Public Open CAD Studio fork and, if needed, a temporary public
  `acadifc` fork/branch.
- **How:** Use focused Conventional Commits. Prefer upstream PRs; if the engine
  PR is pending, pin the public fork to an immutable commit and document the
  temporary delta. Rebase/merge current upstream before final verification if
  it changed during implementation.
- **Tests:** Clean tree, ancestry/remotes, exact dependency revision, GitHub
  revision equality, and privacy scan.

### 11. Rebuild and roll out the stack

- **What:** Build a new OCI image on the Debian server and activate only this
  stack after explicit approval of the exact SSH command.
- **Where:** Private runtime checkout and stack-local rootless Podman/systemd
  resources.
- **How:** Fetch the exact GitHub commit, verify no private-config drift, build a
  new immutable image tag, run health/static asset checks, replace only
  `vitrobud-opencads-web`, and retain the prior image/config for rollback.
- **Tests:** Healthy container, expected application revision/version, worker and
  WASM assets HTTP 200, user systemd enabled/active, manual `EXPORTDXFR12` from
  the HTTPS web runtime, Fusion/machine import, and restart persistence.

### 12. Roll back or close

- **What:** Provide an immediate rollback boundary and close only after machine
  acceptance.
- **Where:** Existing stack-local rollback procedure and task documentation.
- **How:** On build/runtime/export regression, restore the previous exact image
  and private configuration without modifying reverse proxy, firewall, global
  Podman, or other stacks. On success, record only anonymized acceptance facts.
- **Tests:** Verify previous container health after a rehearsed/non-destructive
  rollback check or document why rehearsal was not run.

## Affected Files

| File | Change type | Description |
|------|-------------|-------------|
| `src/io/export_dxf_r12.rs` | Create | Clone, normalize, serialize, verify, and report R12 export |
| `src/io/mod.rs` | Modify | Export the isolated R12 module/API |
| `src/app/commands/display.rs` | Modify | Dispatch canonical command and alias |
| `src/app/commands/mod.rs` | Modify | Register command autocomplete names |
| `src/app/mod.rs` | Modify | Add typed export messages/results |
| `src/app/update/mod.rs` | Modify | File dialog, async execution, web/native delivery |
| `tests/export_dxf_r12.rs` | Create | Application-level regression and round-trip tests |
| `docs/export-dxf-r12.md` | Create | User and compatibility documentation |
| `README.md` | Modify | Add the new export command |
| `locales/**` | Modify | Add user-visible command messages where required |
| `Cargo.toml` / `Cargo.lock` | Conditional modify | Pin accepted or temporary public acadifc revision |
| `acadifc/src/io/dxf/writer/*` | Upstream change | Genuine version-aware AC1009 serialization |
| `acadifc/tests/**` | Upstream change | R12 binary and higher-version non-regression tests |

## Dependencies

- Steps 1–5 precede UI integration so the command cannot expose an invalid raw
  writer path.
- The Open CAD Studio integration depends on an accepted or immutably pinned
  `acadifc` revision containing the R12 writer contract.
- Manual Fusion 360 and target-machine tests require user access and occur
  before GitHub publication/rollout approval.
- Every GitHub push, commit, and server SSH operation remains a separate
  approval boundary under repository policy.

## Testing Strategy

- **Unit:** entity policy, LWPOLYLINE conversion, bounds, layer/linetype mapping,
  typed reports, no mutation.
- **Writer integration:** binary sentinel, AC1009, permitted sections/codes,
  entity inventory, EOF, read-back.
- **Regression:** generated minimal reproducer for the discovered R14 class-code
  incompatibility; unchanged AC1014/AC1032 tests.
- **Application:** command dispatch/alias, autocomplete, cancel/error/success,
  native write and web byte download.
- **Manual:** Fusion 360 import, target-machine import, representative straight
  and bulged polylines, HTTPS web runtime.
- **Container:** exact-revision build, static assets, health, systemd restart,
  stack-local rollback.
- **Privacy:** no user DXF fixtures, private paths, hostnames, addresses, or
  runtime data in tracked files.

## Rollout Sequence

```text
local feature branch
  -> tests and generated DXF acceptance
  -> user approval to commit
  -> anonymized public GitHub exact revision
  -> server audit command shown and approved
  -> exact-revision OCI rebuild
  -> stack-local health and export acceptance
  -> retain previous image until acceptance closes
```

## Rollback

- Keep the currently healthy image and private `.env` rollback snapshot.
- If the build fails, leave the current container untouched.
- If activation or export acceptance fails, restore only the previous
  `vitrobud-opencads-web` image/config and restart only its user service.
- Do not change reverse proxy, nftables, global Podman, or unrelated stacks.

## Risks

- **False R12 labeling:** The existing writer exposes AC1009 but is not a full
  down-converter. Mitigation: writer-first tests and forbidden-structure checks.
- **Silent geometry loss:** Unsupported entities could disappear. Mitigation:
  fail closed with typed inventory; no implicit approximation.
- **Bulge/closure regression:** Polyline conversion can alter arcs. Mitigation:
  exact vertex/bulge/topology round-trip tests and machine samples.
- **Web memory pressure:** Clone plus output buffer increases memory. Mitigation:
  bounded pipeline, actionable allocation errors, and representative size test.
- **Fast upstream changes:** OCS and acadifc evolve quickly. Mitigation: isolated
  modules, focused commits, upstream-first engine PR, immutable dependency pin,
  and final upstream sync before rollout.
- **Multiple existing DXF defects:** R12 export does not repair normal R14 SAVE.
  Mitigation: keep the command separate and track general writer normalization
  as upstream work rather than claiming all DXF output fixed.
