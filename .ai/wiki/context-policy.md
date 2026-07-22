# Context Policy — Observer, Reflector, Heartbeat

## Purpose

Define how agents manage task context without letting local memory grow into another full transcript.

## Core Model

Use three lightweight mechanisms:

1. **Observer** — records important task events in `observations.md`
2. **Reflector** — periodically compresses observations into `summary.md` and `reflection.md`
3. **Heartbeat** — records the current execution checkpoint in `heartbeat.md`

The task wiki is not a chat log. It is a compact operational memory.

## Procedure vs Operational Memory

Separate reusable procedure from local task memory:

**Safe for GitHub:**
- `wiki/README.md`
- `wiki/context-policy.md`
- `wiki/*-template.md`
- Generic changes in `flows/`, `roles/`, and `meta/`
- Sanitized project instructions in `project/` only when they contain no host-local or sensitive runtime data

**Local-only:**
- `wiki/tasks/**`
- Task-specific `summary.md`, `observations.md`, `heartbeat.md`, `reflection.md`, `handoff.md`, `decisions.md`, and `artifacts.md`
- Session snapshots and current deployment/task state
- Local status files such as `prd/task-status.local.md`
- Private IPs, customer names, hostnames, service maps, credentials, certificates, logs, command output, and host-local configuration

In short: GitHub stores procedure and templates. Local-only files store session memory, task state, heartbeat, observations, handoffs, and deployment details.

## Read Policy

Agents must read the smallest useful context set:

1. Always read `summary.md` first, if it exists.
2. Read `handoff.md` when continuing work from another role.
3. Read `heartbeat.md` when task execution order, blockers, or parallel work matter.
4. Read `decisions.md` before changing architecture, scope, or public behavior.
5. Read `artifacts.md` to locate source documents instead of duplicating them.
6. Read `reflection.md` when prior observations may affect the current decision.
7. Read `observations.md` only when the compressed context is insufficient.
8. Do not read `archive/` unless explicitly needed.

## Write Policy

Agents should write only durable, useful context:

- Record important events in `observations.md`
- Update `summary.md` after completing a meaningful phase
- Update `handoff.md` before switching roles or stopping work
- Update `heartbeat.md` when status, blocker, owner, or next action changes
- Move stale details to `archive/` instead of expanding the active context

Do not store raw transcripts, full command output, large diffs, secrets, credentials, or sensitive data.

## Mandatory Final Context Dump

Before ending any task, every agent must update the local task wiki. This is mandatory even if the user does not explicitly ask for it.

If a task key is known, update:

- `wiki/tasks/{TASK_KEY}/summary.md`
- `wiki/tasks/{TASK_KEY}/heartbeat.md`
- `wiki/tasks/{TASK_KEY}/handoff.md`
- `wiki/tasks/{TASK_KEY}/observations.md`

When a meaningful phase completed, context grew, or the next agent needs compressed reasoning, also update:

- `wiki/tasks/{TASK_KEY}/reflection.md`

If no task key is known, create or reuse a reasonable local task key, for example:

- `wiki/tasks/{PROJECT_KEY}-context-snapshot/`

The final context dump must include:

- Current status
- What changed
- Files touched or relevant artifacts
- Decisions made
- Blockers and assumptions
- Next recommended action
- Handoff notes for the next agent

Never commit `wiki/tasks/**`. It is local-only operational memory.

## Compression Policy

Run reflection when any of these conditions is true:

- `summary.md` exceeds 150 lines
- `observations.md` exceeds 200 lines
- A role finishes a major phase
- A handoff occurs between roles
- The current context contains obsolete details that could confuse the next agent

Reflection means:

1. Review recent observations and handoff notes.
2. Keep durable facts, decisions, blockers, and next actions.
3. Remove stale details from active files.
4. Move historical detail to `archive/` only when it may be useful later.
5. Rewrite `summary.md` so the next agent can start from it directly.

## Sealing Policy

When observations have been reflected into `summary.md` or `reflection.md`, treat the old details as sealed:

- Do not keep re-reading sealed details by default.
- Do not copy sealed details back into `summary.md` unless they become relevant again.
- Preserve only references or short notes needed for traceability.

## Heartbeat Policy

`heartbeat.md` is the task checkpoint. It should answer:

- What is the current status?
- Who or which role owns the next action?
- What is blocked?
- Which dependencies must finish first?
- What can run in parallel?
- What is the next concrete step?

Keep it short and update it more often than `summary.md`.
