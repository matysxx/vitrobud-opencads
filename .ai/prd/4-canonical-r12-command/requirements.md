# OCSSTACK-4 Keep One Canonical DXF R12 Command

## Goal

Remove the redundant `EXPORTR12` alias and expose only the more descriptive
canonical command `EXPORTDXFR12`.

## Requirements

- [ ] `EXPORTDXFR12` remains registered, discoverable, and functional.
- [ ] `EXPORTR12` is removed from autocomplete and command dispatch.
- [ ] Public user documentation describes only `EXPORTDXFR12`.
- [ ] Normal save/export commands and the R12 serializer remain unchanged.
- [ ] Unsupported entities continue to stop export before a file is created;
  no entity may be silently discarded as part of this task.
- [ ] No private drawings, infrastructure values, or screenshots are tracked.

## Acceptance Criteria

- [ ] Searching public runtime code and user docs finds no `EXPORTR12` alias.
- [ ] `EXPORTDXFR12` still dispatches `Message::DxfR12Export`.
- [ ] Existing R12 regression tests remain unchanged and pass before rollout.
