# Deployment plan

## 1. Local

1. Fetch `upstream/main` and review the selected upstream SHA.
2. Prepare stack changes on the maintained fork.
3. Run shell, Compose, Git, and privacy validation.
4. Commit with Conventional Commits.
5. Record the exact commit SHA intended for rollout.

## 2. GitHub

1. Push the verified commit to `origin/main`.
2. Confirm GitHub `main` resolves to the same SHA.
3. Confirm tracked files contain no `.env`, credentials, private hostnames,
   addresses, certificates, server paths, runtime state, or CAD files.

## 3. Server

Every remote operation requires the exact SSH command to be shown and approved
before execution. Clone or fetch only from `origin`, set private `.env` values,
set `ROLLOUT_REVISION` to the verified full SHA, then build and start through
the repository scripts. Do not edit tracked files on the server.

## 4. Infrastructure acceptance

1. Confirm rootless Podman and the stack's `systemd --user` unit survive a host
   restart without restarting global Podman.
2. Validate the host firewall and confirm the effective backend-port rule uses
   the intended reverse-proxy/admin or trusted-LAN exposure model.
3. Verify direct `/healthz` access where policy permits it and verify the final
   application through the external HTTPS endpoint.
4. Confirm reverse-proxy configuration, certificates, host addresses, and
   migration records remain outside Git.
5. Record the deployed full SHA and image identity privately, and confirm they
   match local and GitHub `main`.

Use the [infrastructure runbook](infrastructure-runbook.md) as the acceptance
contract and the [private runtime migration checklist](private-runtime-migration-checklist.md)
as the execution checklist.
