# Implementation Plan: OCSSTACK-23

## Status and approach

Requirements and this plan approved by the user on 2026-10-23. Work only in
the isolated task worktree based on published fork `74f9db31`; do not include
unfinished changes from the original checkout. Preserve upstream ancestry.

## 1. Establish preservation and privacy checks

- Inventory the maintained delta against upstream `41d93b6f`, including R12,
  previous small fixes, root URL, WebGL2 and weekly-release guard.
- Use existing `tests/export_dxf_r12.rs` assertions as the preservation contract:
  ASCII AC1009, CRLF, finite extents, ENTITIES/EOF, retained bulges, unchanged
  source document and explicit refusal of unsupported entities.
- Before changing ignore rules, check representative private-file names and
  reusable templates with `git check-ignore --no-index`. Add a lightweight
  repeatable privacy regression check for the resulting rules.
- Validate Podman's `.containerignore`, not only `.dockerignore`; compare both
  contexts. Do not create real secrets for test fixtures.

## 2. Merge the reviewed immutable upstream revision

- Merge `79b6a1f97d0455e43f2e553afce2a19031e567a7` without squashing or force-push.
- Review conflicts in `Cargo.toml`, `src/app/update/mod.rs`, `src/io/mod.rs`,
  commands and documentation individually; do not resolve whole files blindly.
- Retain isolated non-inlined R12 handlers and boxed results, visibility of the
  file-replacement helper, explicit Cargo test target, canonical command only,
  and all upstream SAVE/plugin changes.
- Keep the `codec` alias while adapting any necessary APIs to the now-named
  `acadrust` package. Avoid modifying ordinary SAVE semantics for Fusion.
- Confirm both baseline and reviewed upstream are ancestors of the candidate.

## 3. Harden the container boundary and adapt PWA packaging

- Extend `.gitignore`, `.containerignore` and `.dockerignore` to exclude local
  task wiki/status, env backups, private runtime and generated host state while
  retaining required `.env.dist` templates. Preserve public upstream fixtures;
  do not add private CAD files.
- Keep `Containerfile` building `web-app.html` at `/` with a complete worker
  bundle. Verify builder compatibility and match wasm-bindgen-cli to Cargo.lock
  (`0.2.108` in the reviewed upstream); change tool versions only if necessary.
- Reuse the upstream Pages bundle-hash algorithm in a small container-packaging
  helper to stamp generated `dist/sw.js` after Trunk completes. Without this,
  the private container build would ship the unchanged placeholder `v1`.
- Test the helper on synthetic temporary bundles first: unchanged inputs yield
  the same version, changed bundle inputs yield a new version, absent worker
  fails clearly. Do not edit source sw.js to store a separate release version.
- Preserve relative worker/manifest URLs and natural activation after all old
  tabs close. No forced reload, skipWaiting, data deletion or proxy changes.

## 4. Refresh reusable documentation

- Update `.ai/project/tech-spec.md` from the reviewed lockfile/build behavior.
- Update `docs/upstream-maintenance.md`, `docs/deployment-plan.md`,
  `docs/cutover-and-rollback.md`, `docs/privacy-boundary.md`,
  `docs/export-dxf-r12.md` and other affected index/README references as needed.
- Explain root-path PWA scope, browser-local recovery/storage, worker SAVE vs
  separate R12 export, and secure-context browser capability differences.
- Explain the safe version-switch procedure: save/export drawings first, close
  every application tab/PWA window, then reopen and verify the new version.
  Do not prescribe clearing all site data as routine cache maintenance.
- Keep exact deployment inputs and execution records out of public docs.

## 5. Verify locally and through GitHub

- Review diff/conflict markers, shell syntax (`bash -n`/`sh -n` as appropriate),
  privacy checks and synthetic cache-stamping tests. No Mac container runtime.
- Run the explicit R12 target and workspace tests using available supported
  tooling; if local native prerequisites are absent, record the limitation and
  require successful GitHub tests rather than claiming a local pass.
- Publish reviewed Conventional Commits to the fork rollout branch `main`,
  without rewriting upstream history or sending an upstream PR.
- Require Tests OS-matrix, Web build check (app and worker), and applicable MCP
  conformance to pass for the exact final SHA. Informational lint remains
  informational; do not disable failing required tests to make CI green.
- Record full rollout/rollback revisions locally. Recheck origin/main before
  publication and stop on unexpected concurrent changes.

## 6. Approved server rebuild and acceptance

- Present the complete SSH command for explicit approval. Verify the runtime's
  actual revision, clean tracked checkout, user service and space prerequisites
  without revealing secrets. Do not assume its state from the previous task.
- If supported by the existing flow, include preflight and rollout in one
  clearly displayed command; otherwise request approval for each exact command.
- Preserve private .env, set its sole ROLLOUT_REVISION to the verified published
  full SHA, and run `dev-ops/update` on Debian rootless Podman.
- Verify checkout and image tag, HTTP health with retries, running service and
  user autostart, then HTTPS startup through the existing external proxy.
- Check the built worker is stamped and the root-path manifest/worker load.
- Confirm the explicit R12 command is present and perform synthetic geometry
  export/open smoke checks where browser tooling permits. Machine/Fusion manual
  acceptance remains user verification, not inferred from HTTP/CI success.
- On failure, stop and report concrete evidence. Do not perform an unapproved
  rollback; present its exact command against the recorded previous revision.
- Complete ignored task wiki with actual results and any unverified criteria.

## Risks and stop conditions

- Large upstream change: use targeted preservation checks and exact-SHA gates.
- Codec/entity changes: fix only necessary integration and cover changed cases
  with synthetic regressions before adjusting exporter behavior.
- Stale or mixed PWA assets: content-derived cache version plus close/reopen
  acceptance; never discard unsaved drawings to force activation.
- Private data leakage: audit tracked files and Podman context before publishing
  or building. No broad cleanup of local/server runtime state.
- Stop before server writes without exact command approval, or on failed required
  tests, unexpected origin changes, insufficient space or dirty server checkout.

## CI follow-up authorized with test and rollout request

The first run passed Linux/R12, WASM, MCP and packaging but exposed an inherited
Arial-dependent italic test on Windows/macOS. Correct the fallback fixture to an
embedded stroke font and separately assert that system-font face selection agrees
with the required slant. Do not alter renderer behavior or skip OS tests. Publish
the follow-up and require exact-SHA CI again before approved SSH rollout.

## Exclusions

No backup timer activation, LDAP/Nextcloud work, upstream PR, unrelated container
changes, global Podman restart, or inclusion of unfinished OCSSTACK-12 work.
