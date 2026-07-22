# Task Wiki — Local Agent Context

## Purpose

The task wiki is a local-only context exchange layer for AI agents working on the same task. It stores concise summaries, decisions, handoffs, observations, reflections, heartbeat checkpoints, and artifact links so the next agent can resume work without loading the full chat history.

## Privacy Rule

Task wiki entries are operational memory and must stay local. Do not commit task-specific wiki files.

Never store:
- Secrets, tokens, credentials, private keys, or session data
- Raw customer data, personal data, or production data dumps
- Long chat transcripts, raw logs, full diffs, or generated noise
- Information that is not needed for the current task handoff

## Directory Convention

Use one directory per task:

```text
wiki/
├── README.md
├── context-policy.md
├── task-summary-template.md
├── observations-template.md
├── reflection-template.md
├── heartbeat-template.md
└── tasks/
    └── {PROJECT_KEY}-{N}/
        ├── summary.md
        ├── decisions.md
        ├── handoff.md
        ├── artifacts.md
        ├── observations.md
        ├── reflection.md
        ├── heartbeat.md
        └── archive/
```

Examples:
- `wiki/tasks/SHOP-123/summary.md`
- `wiki/tasks/API-42/handoff.md`

## File Roles

- `summary.md` — current task state; primary file agents read first
- `decisions.md` — durable decisions and rationale
- `handoff.md` — latest role handoff; keep this short and current
- `artifacts.md` — links to requirements, implementation plans, reports, PRs, files, or commands
- `observations.md` — concise append-only log of important task events
- `reflection.md` — compressed conclusions derived from observations
- `heartbeat.md` — current checkpoint, blockers, dependencies, and next action
- `archive/` — sealed historical details not read by default

## Context Mechanisms

Use the policy in `context-policy.md`:

- **Observer:** append important events to `observations.md`
- **Reflector:** compress observations into `summary.md` and `reflection.md`
- **Heartbeat:** keep `heartbeat.md` updated with current status and next action

## Writing Rules

- Keep summaries concise; target 100–150 lines maximum for `summary.md`
- Keep `handoff.md` under 30 lines when possible
- Reflect or archive observations when `observations.md` exceeds 200 lines
- Prefer bullets over prose
- Link to artifacts instead of copying their full contents
- Record decisions, blockers, next steps, and affected files
- Move obsolete details to an `archive/` subdirectory if they must be kept locally
- If context grows too large, compress it into a shorter current-state summary

## Agent Workflow

Before starting work:
1. Identify the current task ID.
2. Read `wiki/tasks/{PROJECT_KEY}-{N}/summary.md` if it exists.
3. Read `handoff.md` and `heartbeat.md` if they exist.
4. Read `decisions.md`, `artifacts.md`, `reflection.md`, and `observations.md` only when relevant.

After finishing work:
1. Append important events to `observations.md`.
2. Update `heartbeat.md` with status and next action.
3. Update `handoff.md` before switching roles or stopping work.
4. Reflect observations into `summary.md` when context has grown or a phase completed.
5. Keep only information needed by the next agent in active files.

## Mandatory Final Context Dump

Before ending any task, every agent must update the local task wiki. This is mandatory even if the user does not explicitly ask for it.

If a task key is known, update:
- `wiki/tasks/{TASK_KEY}/summary.md`
- `wiki/tasks/{TASK_KEY}/heartbeat.md`
- `wiki/tasks/{TASK_KEY}/handoff.md`
- `wiki/tasks/{TASK_KEY}/observations.md`

Also update `wiki/tasks/{TASK_KEY}/reflection.md` when a meaningful phase completed, context grew, or the next agent needs compressed reasoning.

If no task key is known, create or reuse a reasonable local task key, for example `wiki/tasks/{PROJECT_KEY}-context-snapshot/`.

The dump must capture current status, what changed, files touched, decisions made, blockers, assumptions, next recommended action, and handoff notes. Never commit `wiki/tasks/**`.
