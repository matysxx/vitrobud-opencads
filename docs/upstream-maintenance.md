# Upstream maintenance

Remotes:

- `origin` — maintained deployment fork
- `upstream` — `HakanSeven12/OpenCADStudio`

## Local integration

Read `.ai/guidelines.md`, project context/specification, and the last task's local
wiki. Create requirements and an implementation plan before merging. If the
main checkout contains unfinished work, use an isolated worktree at `main`.

Review procedure (replace placeholders with reviewed immutable revisions):

```bash
git fetch upstream main
git log --oneline --decorate main..upstream/main
git merge --no-ff <reviewed-upstream-sha>
```

Review upstream web/native behavior, Cargo changes, Trunk configuration, and
release workflows before merging. Re-run stack validation and build testing.
Never force-push rewritten upstream history to `main`. Preserve upstream merge
ancestry rather than squashing the synchronization.

## Maintained fork checks

- Keep the container build at the root application URL `/` using `web-app.html`.
- Preserve WebGL2 and the separate `EXPORTDXFR12` command, writer, and tests.
- Keep R12 handlers isolated from the large central update stack frame.
- Verify renamed codec APIs against the R12 output regression tests.
- Register fork test targets explicitly when upstream disables auto-discovery.
- Preserve the weekly-release repository guard and rootless runtime scripts.
- Match `wasm-bindgen-cli` to `Cargo.lock`; review builder compatibility.
- Update the technical specification when dependencies or web behavior change.
- Confirm private configuration, drawings, local wiki, and runtime state are
  excluded from Git and the container build context.

## GitHub and server gates

Publish the reviewed fork commit to GitHub `main` and require Tests and Web build
check to pass for that exact full SHA. Use that SHA as `ROLLOUT_REVISION` in the
private root `.env`; it drives checkout and image tag without a separate version
setting. Show the complete SSH command and obtain approval before each remote
operation under this workspace's operating policy.

The server runs `./dev-ops/update`, rebuilding the application and worker before
recreating only the web service. Verify checkout, image tag, health, the enabled
user unit, and browser startup through HTTPS. Record the rollback SHA and final
result locally; keep the server configuration and task wiki out of public Git.
