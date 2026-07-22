# AI Agent Guidelines

This repository is the public, reusable container stack and maintained fork for
Open CAD Studio.

## Required context

Before changing the stack, read:

1. `.ai/project/context.md`
2. `.ai/project/tech-spec.md`
3. the active task in `.ai/prd/`

When continuing an existing task, use its local task wiki as the operational
memory. Read `summary.md` first, followed by `heartbeat.md` and `handoff.md`
before starting work. Read the remaining task files only when needed, following
`.ai/wiki/context-policy.md`.

## Operating rules

- Use the delivery path `local -> GitHub -> server`.
- Use `main` as the rollout branch.
- Show the exact SSH command and wait for explicit approval before every remote
  operation.
- Do not edit production directly or affect unrelated stacks.
- Keep the public repository anonymized and reusable.
- Never commit secrets, private hostnames, private addresses, certificates,
  runtime state, CAD files, or host-specific configuration.
- Keep image/version and host-side settings in root `.env`; keep only container
  runtime secrets in `src/.env`.
- Use Conventional Commits.

## Local task wiki

- Treat `.ai/wiki/` policy and templates as reusable repository content.
- Treat `.ai/wiki/tasks/**` as local-only operational memory; never commit it.
- During work, append durable events to `observations.md` and update
  `heartbeat.md` whenever status, blockers, ownership, dependencies, or the next
  action changes.
- Before pausing, handing off, or ending work, update `handoff.md` with the
  current state and next action.
- Before every final response, perform the mandatory local context dump even if
  the user did not request or remind you: update `summary.md`, `heartbeat.md`,
  `handoff.md`, and `observations.md`; also update `reflection.md` after a
  meaningful phase or when context needs compression.
- If no task key is available, create or reuse
  `.ai/wiki/tasks/OCSSTACK-context-snapshot/`.
- Do not store secrets, private infrastructure data, raw logs, full diffs, or
  chat transcripts in the task wiki.
