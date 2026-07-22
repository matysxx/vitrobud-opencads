# AI Agent Guidelines

This file is the single entry point for AI agents working on this project. Read it first, then follow references based on the task type.

## Before Any Task

Read these files to understand the project:

1. `project/context.md` — project identity, workflow tools, task management
2. `project/tech-spec.md` — technology stack, QA tools, project structure

If the task ID is known, read local task wiki context before role-specific files:

3. `wiki/tasks/{TASK_KEY}/summary.md` — concise current task state, if it exists
4. `wiki/tasks/{TASK_KEY}/handoff.md` — latest handoff notes, if they exist
5. `wiki/tasks/{TASK_KEY}/heartbeat.md` — current checkpoint, blockers, and next action, if it exists

## Role Selection

Determine the task type and read the corresponding role index:

| Task type | Role | Read |
|-----------|------|------|
| New feature or task | Full workflow | Follow "New Task" below |
| Continue existing task | Coder | `roles/coder/coder.md` |
| Bug investigation | Debugger | `roles/debugger/debugger.md` |
| Write or update E2E tests | E2E Tester | `roles/e2e-tester/e2e-tester.md` |
| Create commit | Manager | `roles/manager/commit-message.md` |
| Create PR description | Manager | `roles/manager/pr-description.md` |
| Set up or audit AI instructions | Meta | `meta/meta.md` |
| General conversation | — | No additional files needed |

## New Task Workflow

When starting a new task from scratch:

1. **Design phase** — Read `roles/designer/designer.md`
   - Gather requirements → create `prd/{TASK_KEY}/requirements.md`
   - Follow design principles from `roles/designer/design-principles.md`
2. **Planning phase** — Create implementation plan in `prd/{TASK_KEY}/implementation-plan.md`
3. **Implementation phase** — Read `roles/coder/coder.md`
   - Follow coding standards, testing rules, code quality guidelines
4. **Commit phase** — Read `roles/manager/commit-message.md`
   - Create task entry via `roles/manager/create-task.md`
   - Close task via `roles/manager/close-task.md`

## Task Continuation

When continuing work on an existing task:

1. Read local task wiki context: `wiki/tasks/{TASK_KEY}/summary.md`, if it exists
2. Read local heartbeat context: `wiki/tasks/{TASK_KEY}/heartbeat.md`, if it exists
3. Read the task requirements: `prd/{TASK_KEY}/requirements.md`
4. Read `roles/coder/coder.md` and referenced files
5. If the task has an implementation plan, follow it
6. After completion, append observations, update heartbeat, and write a concise handoff
7. Reflect observations into `summary.md` when a phase completes or context grows too large
8. Follow commit and close-task procedures

## Mandatory Final Context Dump

Before ending any task, update the local task wiki. This is mandatory even if the user does not explicitly ask for it.

If a task key is known, update:

- `wiki/tasks/{TASK_KEY}/summary.md`
- `wiki/tasks/{TASK_KEY}/heartbeat.md`
- `wiki/tasks/{TASK_KEY}/handoff.md`
- `wiki/tasks/{TASK_KEY}/observations.md`

When a meaningful phase completed, context grew, or the next agent needs compressed reasoning, also update:

- `wiki/tasks/{TASK_KEY}/reflection.md`

If no task key is known, create or reuse a reasonable local task key, for example `wiki/tasks/{PROJECT_KEY}-context-snapshot/`.

The dump must include current status, what changed, files touched, decisions made, blockers, assumptions, next recommended action, and handoff notes for the next agent.

## Environment Access

When debugging or verifying deployments, read:
- `project/environments.md` — access details, logs, services
- `roles/debugger/debugger.md` — debugging methodology

## Important Rules

- **Do not modify portable files** in `roles/` or `meta/` — they are shared across projects.
- **Project-specific content** goes in `project/` and `prd/` only.
- **Task wiki entries** in `wiki/tasks/` are local operational memory and must not be committed.
- **GitHub stores procedure and templates**; local-only files store session memory, heartbeat, observations, handoffs, status, and deployment details.
- **Always ask before** making assumptions that could change scope.
- **Follow the tech-spec** — use the project's tools, not your defaults.
