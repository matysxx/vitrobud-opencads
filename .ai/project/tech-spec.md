# Technical Specification

## Verified upstream baseline

- Project: `HakanSeven12/OpenCADStudio`
- License: GPL-3.0-only
- Implementation: Rust with `iced` and `wgpu`
- Browser target: WebAssembly built by Trunk from `index.html`
- Native target: desktop binary with optional headless automation server
- Upstream container image: none published in GitHub Packages
- Upstream container/Compose definition: none
- Upstream web deployment: static GitHub Pages artifact

The current locally integrated upstream candidate is Open CAD Studio `0.9.6` at
post-tag revision `403247cbc3c4348987e9a61ef7aced01b7692f5e` (2026-08-17).
This includes 29 additional commits after tag `v0.9.6`; therefore the exact Git
revision, not only the application version string, identifies the build. The
application pins the CAD codec directly at `cadcodec` revision
`931c4ab0c590b755e280bed318a35f41c57b139f` and the geometry kernel at
`cadkernel` revision `efc77f5d18375467c3cc2c256a22759bb5f9cb54`.
The container builder must install the exact `wasm-bindgen-cli` version selected
in `Cargo.lock`; for this baseline that version remains `0.2.108`. The verified
builder baseline remains the official `rust:1.92.0-bookworm` image.

## Recommended runtime model

- Maintain a source fork because no official OCI image exists and the web build
  must be compiled from source.
- Build a custom OCI image in two stages: pinned Rust/Trunk build stage and a
  small unprivileged static-file server stage.
- Serve the static WASM application with explicit COOP/COEP headers required
  for SharedArrayBuffer-capable browser execution.
- Use rootless Podman and bridge networking with one explicitly published HTTP
  port. Host networking is unnecessary because the application has no service
  discovery, broadcast, or host-device requirement.
- Keep TLS termination and certificates in the external reverse proxy. The
  proxy-to-backend hop is HTTP unless the private network threat model later
  requires explicit backend TLS.
- Do not bind-mount application data: the web edition operates in the browser
  and does not provide server-side CAD storage. Keep optional host-side runtime
  state under `dev-ops/storage/*` only if a concrete need is introduced.

## Important web limitations

Upstream v0.9.5 retired the native-only `solid3d` feature gate and moved solid
geometry to the pure-Rust kernel, so the web build now includes kernel-backed
solid modeling. Browser file access, printing, native plugins, external
processes, and some platform integrations remain different from desktop; keep
the private web runtime documentation aligned with `docs/native-vs-web.md`.

## Repository target structure

```text
.ai/
.github/workflows/
compose.yaml
compose.override.example.yaml
Containerfile
.env.dist
src/.env.dist
container/
dev-ops/
docs/
README.md
```

## Operations

- Rootless Podman on Debian
- Autostart with `systemd --user`
- Host-side backup timer at 03:30 with 30-day retention
- No cron container
- Generated host units such as `container-*.service` remain untracked
- Shell validation with `bash -n`; Compose validation with
  `podman compose config`; image checks include static headers and health/readiness

The reusable infrastructure, firewall, reverse-proxy, autostart, migration, and
acceptance boundary is defined in `docs/infrastructure-runbook.md`. Concrete
deployment identifiers and acceptance records remain private and untracked.
