# OCSSTACK-20 — Requirements

## Goal

Prevent the upstream weekly release publisher from running in this maintained
deployment fork while preserving the workflow file for low-conflict upstream
synchronization.

## Requirements

- Scheduled and manually dispatched release jobs must run only in the canonical
  `HakanSeven12/OpenCADStudio` repository.
- The guard must be explicit, easy to audit, and independent of secrets or
  repository-specific configuration.
- Do not alter upstream release scripts, application code, container runtime, or
  server configuration.
- Preserve the existing schedule and release workflow logic so future upstream
  merges remain straightforward.
- Validate YAML syntax and the exact repository guard locally.
- Publish through `local -> GitHub`; no server rollout is required because the
  runtime image and application are unchanged.

## Acceptance criteria

- The `prepare` job is skipped outside `HakanSeven12/OpenCADStudio`.
- Dependent `web` and `native` jobs cannot run when `prepare` is skipped.
- The workflow remains valid YAML.
- No private data or runtime changes are introduced.
