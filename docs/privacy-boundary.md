# Privacy boundary

## Public repository

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
