# Create Task

## Purpose

Procedure for creating a new task from scratch — branch, task directory, and index entry. No issue tracker ticket exists yet.

> If starting work from an existing issue tracker ticket, use `create-task-from-ticket.md` instead.

## Prerequisites

- `project/context.md` — project key (`{PROJECT_KEY}`)
- `manager.md` — branching strategy and branch naming patterns

## Steps

1. **Determine task number** — check the task index (`prd/prd.md`) for the next available `{PROJECT_KEY}-N`
2. **Create branch** — use the pattern from `manager.md` branching strategy:
   - Feature: `feature/{PROJECT_KEY}-{N}-short-description`
   - Bug fix: `fix/{PROJECT_KEY}-{N}-short-description`
3. **Create task directory** — `prd/{task-number}-{slug}/`
4. **Add entry in task index** (`prd/prd.md`) — task name and directory
5. **Set status** — `In Progress` in `prd/task-status.local.md`
6. **Hand off to Designer** — `designer/create-requirements.md`

## Task Directory Naming

- Use only the numeric part of the task number (e.g., `42`, `108`)
- Generate a slug from the title: lowercase, trim, replace spaces with `-`, keep only `a-z`, `0-9`, `-`
- Result: `prd/{task-number}-{slug}/`
- Main file: `description.md` (or `requirements.md` if created by Designer)

## Task Content

Always write task content in **English**. Each task should contain:

1. **Title** — short, action-oriented (one line)
2. **Description** — essential context and scope, no verbosity
3. **Acceptance Criteria** — bullet list of verifiable outcomes, each starting with a verb

## Checklist

```
[ ] Task number determined
[ ] Branch created
[ ] Task directory created
[ ] Task index entry added in prd/prd.md
[ ] Status set to In Progress in prd/task-status.local.md
[ ] Designer notified to create requirements
```
