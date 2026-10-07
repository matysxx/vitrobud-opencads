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

The integrated upstream baseline is Open CAD Studio `2026.40.1` at revision
`41d93b6f5421477b0566197dd4ee581f7d48db3e` (2026-10-07). The exact Git
revision identifies the build independently of the release version string.
The application now imports its renamed libraries as `codec`, `kernel`,
`kernel-constraints`, and `graph`. The lockfile pins `opencadcodec` at
`063c10671fe7833d562f772159771318c7a0ebb9`, `opencadkernel` at
`3f3781a227008aec6847c894dc843245b539c682`, and `opencadgraph` at
`ca3e6a20d70210d1422feec536e0c2422bfadfd0`.
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
- Serve the static WASM application with the existing COOP/COEP header policy
  coordinated with the external proxy. The current browser build does not
  require SharedArrayBuffer or these headers for its single-threaded execution.
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

## Verification gates

- GitHub Tests runs `cargo test --workspace --locked`, including the explicitly
  registered fork test target `export_dxf_r12` (upstream disables test discovery).
- GitHub Web build check validates the WASM target; the final release bundle
  and worker are built on the Debian host before the web container is replaced.
- Keep the R12 handlers outside the central update function's stack frame.
- Inspect browser startup through the external HTTPS endpoint after rollout.
