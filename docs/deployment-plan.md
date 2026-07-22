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
