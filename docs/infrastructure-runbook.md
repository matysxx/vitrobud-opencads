# Infrastructure runbook

This document is the reusable infrastructure contract for a private Open CAD
Studio web runtime. Real hostnames, addresses, ports, filesystem paths, DNS
names, certificates, and firewall source networks remain in private operations
records and untracked runtime configuration.

## Topology

```text
browser -- HTTPS --> external reverse proxy -- HTTP/private network --> rootless Podman web container
```

- The public repository and GitHub `main` are the rollout source of truth.
- The Debian host checks out one verified full Git SHA and builds the matching
  OCI image locally.
- Rootless Podman publishes one backend TCP port through bridge networking.
- A separately managed reverse proxy owns local DNS, TLS, certificates, and any
  browser-specific compatibility policy.
- `systemd --user` owns container autostart. Global Podman and global systemd
  are outside this stack.
- The web runtime is stateless; CAD files stay in the browser/user workflow.

## Network and firewall contract

- Permit the selected backend TCP port according to the private deployment
  policy: preferably only from the reverse-proxy host and administration
  network; a trusted-LAN rule may be used when direct IP access is intentionally
  supported.
- Do not expose container management, Caddy administration, or Podman sockets.
- Keep the firewall rule and its source scope in the host firewall
  configuration, not in this repository.
- Validate the complete firewall configuration before reload, reload through
  the host's normal service workflow, and confirm the effective rule afterwards.
- Confirm both intended paths: direct backend health where policy permits it,
  and the final HTTPS application URL through the reverse proxy.

The project does not manage the host firewall. A deployment is infrastructure-
ready only after the operator confirms that the effective firewall policy is
correct for the selected exposure model.

## Reverse proxy contract

- Terminate TLS outside this stack with a certificate trusted by managed
  clients.
- Proxy to the published backend port over the controlled private network.
- Preserve the effective browser isolation/header policy selected for the
  deployed browser matrix.
- Serve `.wasm` as `application/wasm` and avoid rewriting large static assets.
- Keep proxy configuration and compatibility workarounds external and private.

See [external reverse proxy](external-reverse-proxy.md) for application-specific
requirements.

## Runtime and autostart contract

- Run the stack as an unprivileged service account with rootless Podman.
- Install only the repository-provided user unit with
  `./dev-ops/install-systemd-user-unit`.
- Install the host-side backup timer only when backup scheduling is approved;
  otherwise record its intentional deferral privately. Never replace it with a
  cron process inside the container.
- Enable lingering for the runtime user privately if boot-time user services
  require it.
- Restart or update only this stack; never restart global Podman for a normal
  rollout.
- Keep generated units, `.env`, rollback copies, image state, and runtime files
  untracked.

## Deployment acceptance

Infrastructure can be marked complete when all of the following are true:

- the container is healthy after a host restart;
- the user service is enabled and active;
- the external HTTPS endpoint loads the application;
- the firewall policy has been validated and confirmed effective;
- direct backend access matches the chosen private exposure policy;
- reverse-proxy and certificate material remain outside Git;
- no unrelated stack or global Podman service was changed.

Repository rollout consistency is a separate acceptance check: the server
checkout, `ROLLOUT_REVISION`, image tag, local `main`, and GitHub `main` must all
identify the same verified revision. Infrastructure availability alone does not
prove that revision alignment.

## Migration record

Keep the following values only in a private operations record:

- source and destination host identifiers;
- migration date and operator;
- deployed full Git SHA and image identifier;
- backend address/port and allowed firewall sources;
- reverse-proxy route and certificate identity;
- systemd user-unit state;
- rollback revision and rollback test result.
