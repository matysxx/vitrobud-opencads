# Create Task from Issue Tracker Ticket

## Purpose

Procedure for starting work on an existing issue tracker ticket — fetch ticket data, create local task structure, branch, and transition the ticket.

> If creating a task from scratch (no ticket exists), use `create-task.md` instead.

## Prerequisites

- `project/context.md` — project key (`{PROJECT_KEY}`), issue tracker type, MCP availability
- `manager.md` — branching strategy and branch naming patterns

## Steps

1. **Resolve ticket number** — extract the numeric part from the user's input (e.g., "start work on 2230" → `{PROJECT_KEY}-2230`)
2. **Fetch ticket data** — read the ticket from the issue tracker via MCP:
   - Summary / title
   - Description
   - Acceptance criteria / test cases
   - Current status
   - If MCP is unavailable, ask the user to provide the ticket details manually
3. **Create branch** — use the pattern from `manager.md` branching strategy:
   - Feature: `feature/{PROJECT_KEY}-{N}-short-description`
   - Bug fix: `fix/{PROJECT_KEY}-{N}-short-description`
   - Derive `short-description` from the ticket summary (slugified)
4. **Create task directory** — `prd/{ticket-number}-{slug}/`
5. **Populate task content** — write `description.md` with data from the ticket:
   - Title from ticket summary
   - Description from ticket body (concise, in English)
   - Acceptance criteria from ticket (or ask user if missing)
6. **Add entry in task index** (`prd/prd.md`) — task name and directory
7. **Set status** — `In Progress` in `prd/task-status.local.md`
8. **Transition ticket** — propose transitioning the ticket (e.g., READY → IN PROGRESS) per `update-ticket.md`:
   - Present the proposed transition to the user
   - Ask for explicit approval — do not auto-transition
   - If the user declines or MCP is unavailable, skip
9. **Hand off to Designer** — `designer/create-requirements.md`

## Task Directory Naming

- Use only the numeric part of the ticket number (e.g., `2230`, `3001`)
- Generate a slug from the ticket summary: lowercase, trim, replace spaces with `-`, keep only `a-z`, `0-9`, `-`
- Result: `prd/{ticket-number}-{slug}/`

## Fallback — MCP Unavailable

If MCP tools for the issue tracker are not available:

1. Inform the user that automatic ticket lookup is unavailable
2. Ask the user to provide: title, description, acceptance criteria
3. Continue from step 3 (create branch) using the manually provided data
4. Skip step 8 (ticket transition) — note in `prd/task-status.local.md` that transition is pending

## Checklist

```
[ ] Ticket number resolved
[ ] Ticket data fetched from issue tracker (or provided manually)
[ ] Branch created
[ ] Task directory created with description.md
[ ] Task index entry added in prd/prd.md
[ ] Status set to In Progress in prd/task-status.local.md
[ ] Ticket transitioned (if approved by user)
[ ] Designer notified to create requirements
```
