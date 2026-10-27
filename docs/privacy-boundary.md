# Privacy boundary

## Public repository

The private web entry (`web-app.html`) deliberately omits upstream's external
OpenAEC analytics loader. A packaging test guards this fork policy. The upstream
marketing page is not the container entry point. This does not disable other
application network features; do not describe the application as air-gapped.

- upstream source and GPL notices
- container build and anonymous Compose defaults
- operational scripts and reusable documentation
- `.env.dist` templates containing no real infrastructure values

## Private runtime only

- `.env`, `src/.env`, and `compose.override.yaml`
- internal hostnames, addresses, DNS, ports, and server paths
- registry credentials, tokens, certificates, and keys
- backup archives and generated systemd units
- CAD drawings and all user runtime data

Review `git status`, `git diff --cached`, and `git ls-files` before every push.

Local wiki (`.ai/wiki/tasks/`), local task status and `.env.*` backups are excluded
from Git and both build contexts; `.env.dist` templates remain available. Podman
prefers `.containerignore`, so keep it aligned with `.dockerignore` and run
`python3 scripts/test_container_packaging.py`. Ignore rules do not untrack files
or replace a secret/private-data review. Public upstream fixtures are not user
drawings; never add customer CAD files as public regression data.
