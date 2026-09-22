# OCSSTACK-21 — Requirements

## Goal

Synchronize the maintained fork with reviewed upstream revision
`652f4d6bbc040dc0b6950086099603dc28fb4185` while preserving the reusable
rootless Podman stack and deliberate fork integrations.

## Requirements

- Merge the exact upstream revision without rewriting upstream history.
- Preserve `EXPORTDXFR12`, its public documentation, and regression tests.
- Preserve a Safari-safe browser renderer policy; review upstream's new GPU
  fallback before resolving renderer-related changes.
- Preserve the application-at-root container build and the single immutable
  `ROLLOUT_REVISION` production version input.
- Preserve the canonical-repository guard on inherited weekly release publishing.
- Keep host-specific configuration, credentials, runtime state, and private
  infrastructure outside the public repository.
- Do not include unfinished local OCSSTACK-12 or Nextcloud planning changes.
- Record version `2026.38.0`, tag distance, and updated dependency pins.
- Validate locally, publish the anonymized result to GitHub, and only then deploy
  that exact revision to Debian after approval of the exact SSH command.

## Acceptance criteria

- The merge has `652f4d6bbc040dc0b6950086099603dc28fb4185` as a parent.
- Fork-only R12 export remains registered, documented, and covered by tests.
- Browser rendering remains compatible with the established Safari workaround.
- The weekly publisher cannot run in the maintained fork.
- Container and operational files pass static validation and privacy audit.
- Local `main`, GitHub `main`, server checkout, and runtime image identify the
  same verified fork revision after rollout.
