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

The current reviewed upstream rollout baseline is Open CAD Studio `0.9.2` at
post-tag revision `734afd5611c3d4ea4db6b807123d95f0098ad74b` (2026-08-04).
This includes eight commits after tag `v0.9.2`; therefore the exact Git revision,
not only the application version string, identifies the build. The baseline
pins `OpenAEC-Foundation/acadifc` at `ab388a7de9b38f7b06e963c35b45fb35d9fee97a`.
Upstream now includes the no-default-features ACIS helper fix previously carried
temporarily by this fork.
The container builder must install the exact `wasm-bindgen-cli` version selected
in `Cargo.lock`; for this baseline that version remains `0.2.108`. Its Iced
dependency requires Rust `1.92`, so the verified builder baseline remains the
official `rust:1.92.0-bookworm` image.

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

The upstream web build disables the default `solid3d` feature because its native
dependencies do not cross-compile to WASM. Upstream also documents browser
limitations compared with the native desktop build. The private web runtime
must not be described as feature-equivalent to the native application.

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
