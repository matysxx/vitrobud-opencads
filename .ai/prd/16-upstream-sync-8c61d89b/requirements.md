# OCSSTACK-16 Sync Upstream 8c61d89b

## Requirements

- [ ] Merge the exact reviewed upstream revision
  `8c61d89b1e447925d9fab73c242194a61fe7f1ec` without rewriting upstream
  history.
- [ ] Preserve the anonymized reusable rootless-Podman stack and the delivery
  path `local -> GitHub -> server`.
- [ ] Preserve the fork-only `EXPORTDXFR12` command, strict ASCII R12 writer,
  localized messages, and regression tests.
- [ ] Adapt the container build to the new `web-app.html` target while keeping
  the private runtime application at its established `/` URL; do not introduce
  the upstream marketing landing page into this stack.
- [ ] Keep TLS, reverse proxy, private hostnames, addresses, runtime values,
  credentials, certificates, and CAD files outside the public repository.
- [ ] Update the recorded upstream baseline and exact dependency/build pins.
- [ ] Run all locally available static, Rust, web-build, Compose, and privacy
  checks; record any validation deferred to the Debian runtime.
- [ ] Publish only a verified Conventional Commit to GitHub `main`.
- [ ] Before server rollout, show the exact SSH command and wait for explicit
  approval; rebuild and restart only this stack.
