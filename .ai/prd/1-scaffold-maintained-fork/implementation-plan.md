# Implementation Plan: OCSSTACK-1

## Overview

Create a maintained source fork and add a reproducible rootless-Podman web
stack. The first runtime stays stateless and uses the upstream browser file
picker/download flow. LDAP and Nextcloud are deferred until this baseline is
proven.

## Steps

### 1. Establish the maintained fork

- **What:** Preserve upstream history and configure `upstream` and `origin`.
- **Where:** Git metadata and `docs/upstream-maintenance.md`.
- **How:** Fork without rewriting upstream; record and verify the baseline SHA.
- **Tests:** Verify remotes, ancestry, license, clean tree, and exact baseline.

### 2. Add repository and privacy scaffolding

- **What:** Add required AI context, templates, ignore rules, README, and docs.
- **Where:** `.ai/`, `.gitignore`, `.env.dist`, `src/.env.dist`, `README.md`.
- **How:** Adapt workspace conventions without private values or obsolete
  cron-container/internal-proxy patterns.
- **Tests:** Scan tracked files for secrets and private infrastructure data.

### 3. Build the web image

- **What:** Compile WASM and serve it from an unprivileged OCI runtime.
- **Where:** `Containerfile` and `container/`.
- **How:** Pin build inputs, use multiple stages, configure COOP/COEP and
  security headers, and expose only an internal HTTP port.
- **Tests:** Build, inspect user/labels, run read-only, verify health and headers.

### 4. Add rootless Podman operations

- **What:** Implement Compose and all required stack-local scripts.
- **Where:** `compose*.yaml` and `dev-ops/`.
- **How:** Use bridge networking, configurable bind/port, hardening, exact
  revision checks, and operations scoped to named stack resources.
- **Tests:** `bash -n`, Compose rendering, and local lifecycle smoke tests.

### 5. Add systemd user units and host-side backup

- **What:** Add autostart plus daily 03:30 backup with 30-day retention.
- **Where:** `dev-ops/systemd/`, installers, `backup`, and `cleanup`.
- **How:** Render host paths during installation and keep generated units/data
  untracked; archive only explicit private operational storage.
- **Tests:** Unit verification and retention tests in temporary storage.

### 6. Complete runbooks and rollout controls

- **What:** Document local verification, GitHub, parallel start, migration,
  cutover/rollback, backup, LDAP, Nextcloud, and upstream sync.
- **Where:** `docs/` and `README.md`.
- **How:** Keep examples anonymous and require exact SSH command approval.
- **Tests:** Review commands for scope, revision pinning, and privacy.

## Dependencies

- GitHub authentication for remote fork creation.
- Rust, `wasm32-unknown-unknown`, and Trunk for the web build.
- Separate explicit approval for production rollout.

## Testing strategy

- Static shell, Compose, and privacy checks.
- Reproducible OCI build and upstream ancestry validation.
- Manual browser open/save, restart, backup, and rollback checks.

## Risks

- **Fast upstream:** pin SHAs and sync in isolated review commits.
- **Web feature gap:** document missing 3D, hatch, and font capabilities.
- **Large files:** the web build loads selected files into browser memory; use
  disposable representative drawings during the baseline test.
- **Future integrations:** implement LDAP and Nextcloud as a separately reviewed
  phase only after the basic web runtime passes.
