# OCSSTACK-1 Scaffold Maintained Fork and Rootless Podman Runtime

## Goal

Create a safe, maintainable fork of Open CAD Studio and package its browser
edition as an anonymized, reusable stack for rootless Podman on Debian.

## Background

The upstream project is a Rust desktop application with a reduced-feature
WebAssembly build deployed as static files. It has no official OCI image and no
Compose definition. The intended private runtime therefore requires a custom
image built from the fork, while preserving a clean upstream relationship.

## Functional requirements

- [ ] FR1: Base the repository history on `HakanSeven12/OpenCADStudio` and record
  `upstream` and `origin` remotes without rewriting upstream history.
- [ ] FR2: Build the upstream web target reproducibly into a custom OCI image.
- [ ] FR3: Serve the WASM files as an unprivileged process with COOP/COEP and
  baseline security headers.
- [ ] FR4: Provide `compose.yaml`, `compose.override.example.yaml`, `.env.dist`,
  and `src/.env.dist` compatible with rootless Podman.
- [ ] FR5: Use bridge networking and a configurable published port; keep TLS and
  reverse proxy outside the stack.
- [ ] FR6: Provide `setup`, `start`, `update`, `shutdown`, `reset-runtime`,
  `backup`, `cleanup`, `install-systemd-user-backup`, and `compose` scripts.
- [ ] FR7: Use host-side `systemd --user` for autostart and daily backup at
  03:30, with 30-day retention.
- [ ] FR8: Keep any persistent host data under `dev-ops/storage/*` and exclude it
  from Git.
- [ ] FR9: Document deployment, parallel first start, migration checklist,
  cutover/rollback, backups/retention, privacy boundaries, and upstream sync.
- [ ] FR10: Ensure server rollout checks out exactly the locally verified and
  GitHub-published revision.

## Non-functional requirements

- [ ] NFR1: Commit no secrets, private hostnames, private IP addresses,
  certificates, CAD documents, or runtime state.
- [ ] NFR2: Run the container read-only with dropped capabilities and
  `no-new-privileges`, subject to validation against the selected server image.
- [ ] NFR3: Pin build/runtime dependencies sufficiently to make updates explicit
  and reviewable; avoid a second version source of truth.
- [ ] NFR4: Preserve GPL-3.0-only notices and provide corresponding source through
  the public fork.
- [ ] NFR5: Clearly document that the web build is not feature-equivalent to the
  native desktop application, especially for 3D functionality.
- [ ] NFR6: Make scripts fail fast, quote paths, and avoid affecting unrelated
  Podman stacks.

## Scope

### In scope

- Public maintained fork and container build files
- Static web runtime for Open CAD Studio
- Rootless Podman Compose operations
- User-level systemd autostart and backup scheduling
- Anonymized operational documentation and rollout workflow

### Out of scope

- Native GUI streaming through VNC/RDP/WebRTC
- Collaborative editing, locking, or automatic conflict merging
- Browser-side storage of LDAP or Nextcloud passwords
- LDAP authentication and Nextcloud WebDAV integration (deferred to phase 2
  after the basic web runtime is proven)
- Reverse proxy and certificate management
- Production rollout, DNS changes, and GitHub publication before explicit
  approval
- Any change to unrelated stacks or global Podman services

## Technical notes

- Proposed public image name: configurable as `OPENCADS_IMAGE`; no private
  registry or account is hard-coded.
- Proposed container name template: configurable and derived from the public
  stack identifier; private project/pod identifiers stay outside tracked files.
- The root `.env` owns image tag/digest, bind address, port, project name,
  upstream release/source revision, backup schedule, and retention.
- `src/.env` is retained by convention but may contain no required values for
  the initial static runtime.
- Backup is intentionally small because the initial application is stateless;
  it archives only explicitly allowlisted private operational configuration and
  future `dev-ops/storage` content, never repository source or arbitrary CAD
  files.

## Acceptance criteria

- [ ] AC1: All required repository files and scripts exist and are documented.
- [ ] AC2: `bash -n` passes for every shell script.
- [ ] AC3: Compose renders successfully with sanitized test environment values.
- [ ] AC4: The image builds from an explicitly selected upstream source revision.
- [ ] AC5: The running container is unprivileged, has the expected headers, and
  exposes only the configured port.
- [ ] AC6: Secret/private-data scans find no prohibited operational values.
- [ ] AC7: Backup retention and generated user units are testable without cron
  in the container or global systemd changes.
- [ ] AC8: Documentation gives an exact `local -> GitHub -> server` rollout with
  an approval boundary before each SSH command.
- [ ] AC9: A stack-level `systemd --user` unit manages autostart independently
  from the backup timer, and neither uses generated `container-*.service` files.

## Open questions

None blocking for the basic runtime. External hostname, port, registry
coordinates, and production digests remain private deployment inputs. LDAP and
Nextcloud identity decisions are deferred to phase 2.

## References

- Upstream: https://github.com/HakanSeven12/OpenCADStudio
- Upstream web/native comparison:
  https://github.com/HakanSeven12/OpenCADStudio/blob/main/docs/native-vs-web.md
- Upstream web workflow:
  https://github.com/HakanSeven12/OpenCADStudio/blob/main/.github/workflows/pages.yml
