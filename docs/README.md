# Maintained container fork documentation

This directory contains both inherited Open CAD Studio application documents
and the reusable operational contract for this maintained container fork.
Public procedures contain no private hostnames, addresses, credentials,
certificates, CAD files, or host-specific runtime values.

## Runtime and deployment

- [Container quick start](container-quickstart.md) — prepare private `.env`
  files, install the pinned Compose provider, start the rootless stack, and
  verify health and browser headers.
- [Deployment plan](deployment-plan.md) — required `local → GitHub → server`
  flow and immutable rollout revision.
- [Infrastructure runbook](infrastructure-runbook.md) — Debian, rootless
  Podman, bridge networking, external reverse proxy, `systemd --user`, firewall
  boundary, acceptance, and private migration record.
- [Parallel first start](parallel-start-plan.md) — bring up a candidate runtime
  without replacing the existing service.
- [Private runtime migration checklist](private-runtime-migration-checklist.md)
  — migration and acceptance checklist.
- [Cutover and rollback](cutover-and-rollback.md) — controlled traffic switch
  and revision-based rollback.
- [External reverse proxy](external-reverse-proxy.md) — HTTPS termination,
  backend headers, WASM handling, and Safari compatibility boundary.

## Operations and security

- [Backup and retention](backup-and-retention.md) — host-side backup of private
  operational configuration, optional user timer, and 30-day default retention.
- [Privacy boundary](privacy-boundary.md) — what may be public and what must
  remain only in the private runtime.
- [Upstream maintenance](upstream-maintenance.md) — auditable merge procedure
  for `HakanSeven12/OpenCADStudio` while preserving the small fork delta.

## Fork-specific application behavior

- [DXF R12 ASCII machine export](export-dxf-r12.md) — contract, limitations,
  validation, and consumer acceptance for `EXPORTDXFR12`.
- [Native versus web](native-vs-web.md) — inherited upstream comparison of
  browser and desktop capabilities.

## Source of truth

The public repository and GitHub `main` contain reusable source, procedures,
and templates. Private `.env` files, runtime state, generated systemd units,
backup archives, reverse-proxy configuration, certificates, and CAD drawings
must never be committed. Local agent task memory belongs only in the ignored
`.ai/wiki/tasks/` tree.
