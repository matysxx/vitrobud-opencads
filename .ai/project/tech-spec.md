# Technical Specification

## Verified upstream baseline

- Project: `HakanSeven12/OpenCADStudio`
- License: GPL-3.0-only
- Implementation: Rust with `iced` and `wgpu`
- Browser target: WebAssembly built by Trunk from `web-app.html`
- Native target: desktop binary with optional headless automation server
- Upstream container image: none published in GitHub Packages
- Upstream container/Compose definition: none
- Upstream web deployment: static GitHub Pages artifact

The current locally integrated upstream candidate is Open CAD Studio `0.9.8` at
post-tag revision `6d6defa0ad6c4a8d1a18069e05a59584a26dfb96` (2026-09-01).
This includes 124 additional commits after tag `v0.9.8`; therefore the exact Git
revision, not only the application version string, identifies the build. The
application pins the CAD codec directly at `cadcodec` revision
`a0f7d444f1607bc4b2c881060cbe7ea1014253cb` and the geometry kernel at
`cadkernel` revision `48c634995fa60d3928c37bae49b12b421c56c886`.
The container builder must install the exact `wasm-bindgen-cli` version selected
in `Cargo.lock`; for this baseline that version remains `0.2.108`. The verified
builder baseline remains the official `rust:1.92.0-bookworm` image.

## Recommended runtime model

- Maintain a source fork because no official OCI image exists and the web build
  must be compiled from source.
- Build a custom OCI image in two stages: pinned Rust/Trunk build stage and a
  small unprivileged static-file server stage.
- Use `ROLLOUT_REVISION` as the single immutable source for both the detached
  Git checkout and OCI image tag. Compose uses `dev` only when no rollout
  revision is configured; there is no separate production image-tag variable.
- Build the upstream `web-app.html` target directly at `/`. Upstream GitHub
  Pages additionally publishes a marketing landing page and moves the app to
  `/app/`; the private runtime intentionally keeps the established root URL so
  reverse-proxy routes and bookmarks remain stable.
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
Upstream now explicitly selects the WebGL2 renderer for the browser build,
avoiding the unstable WebGPU path previously observed in Safari and affected
Chromium adapters.

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
