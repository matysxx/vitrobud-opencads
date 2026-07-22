# Cutover and rollback

## Cutover

1. Confirm the parallel endpoint and disposable drawing tests pass.
2. Record the current reverse-proxy backend and rollout SHA.
3. Switch the external reverse proxy to the new backend.
4. Verify health and a browser session through the final HTTPS endpoint.
5. Observe container logs without restarting global Podman.

## Rollback

1. Point the external reverse proxy back to the recorded prior backend.
2. Stop only this stack with `./dev-ops/shutdown` if necessary.
3. Set `ROLLOUT_REVISION` to the recorded prior verified SHA.
4. Run `./dev-ops/update` and repeat health checks.

No CAD files are stored by this runtime, so application rollback does not move
or restore user drawings.
