# OCSSTACK-17 Sync Upstream 6d6defa0

## Requirements

- [ ] Merge exact upstream revision
  `6d6defa0ad6c4a8d1a18069e05a59584a26dfb96` without rewriting upstream
  history.
- [ ] Preserve the fork-only `EXPORTDXFR12` command, ASCII R12 writer,
  localized behavior, and regression tests.
- [ ] Preserve the private web application at `/`, the explicit WebGL2 browser
  renderer, and the external reverse-proxy/TLS boundary.
- [ ] Preserve the anonymized rootless-Podman stack and the delivery path
  `local -> GitHub -> server`.
- [ ] Keep the unfinished OCSSTACK-12 and Nextcloud work outside this update.
- [ ] Update the exact Open CAD Studio, cadcodec, cadkernel, and build baseline.
- [ ] Remove `OPENCADS_TAG` as a second rollout-version input; derive the image
  tag from `ROLLOUT_REVISION`, with `dev` only when no rollout revision exists.
- [ ] Run all available static, privacy, shell, YAML, source-integration, and
  container-route checks before publication.
- [ ] Publish only the verified Conventional Commit to anonymized GitHub
  `main`.
- [ ] Show the exact SSH rollout command and obtain explicit approval before
  rebuilding and recreating only this stack on Debian.

