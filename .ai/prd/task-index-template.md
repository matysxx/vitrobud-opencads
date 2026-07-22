# Task Index Template

## Purpose

Template for the task index file (`prd/prd.md` or `prd/task-index.md`). This file serves as the central registry of all tasks in the project.

## Prerequisites

- `project/context.md` — contains `PROJECT_KEY` used in task IDs
- `roles/manager/create-task.md` — procedure for creating new tasks

## Template

Copy the template below and customize it for your project:

---

```markdown
# Task Index — {PROJECT_NAME}

Central index of all tasks managed through `.ai/prd/`.

## Before Starting

Always read first:
- `../project/context.md` — Current project context
- `../project/tech-spec.md` — Technical constraints

## Tasks

| ID | Name | Directory |
|----|------|-----------|
| {PROJECT_KEY}-1 | {Task name} | prd/{task-directory}/ |
| {PROJECT_KEY}-2 | {Task name} | prd/{task-directory}/ |

> New tasks are added by the Manager role per `manager/create-task.md`.
> Task statuses are tracked in `prd/task-status.local.md` (local only, not committed).

## Task Statuses

Statuses are tracked in `task-status.local.md` (gitignored, local only). This avoids circular dependencies between commit and status update.

Rules:
- Allowed values: `Planned`, `In Progress`, `Done`.
- Set `In Progress` when work begins (branch created and requirements exist).
- Set `Done` after commit is created and acceptance criteria are checked off.
- If work stops or changes scope, record it in the task's `requirements.md`.

## Task Directory Structure

Each task has its own directory: `{PROJECT_KEY}-{number}-{short-name}/`

Contents:
- `requirements.md` — Main requirements document (mandatory)
- `implementation-plan.md` — Implementation plan (if created by Designer role)
- `investigation-report.md` — Investigation findings (if Debugger role was used)
- Additional files (images, specs, etc.) as needed

## Naming Convention

Task directories: `{PROJECT_KEY}-{number}-{short-name}/`

> Get `{PROJECT_KEY}` from `project/context.md`.

Examples:
- `ALF-1-layout/`
- `DEVINTER-3025-configurable-minimum-surcharge/`
- `SHOP-42-fix-checkout-bug/`
```

---

## Customization Notes

1. **Replace placeholders:**
   - `{PROJECT_NAME}` — your project name (e.g., "DEVINTER", "Alfred", "E-commerce Platform")
   - `{PROJECT_KEY}` — your project key from `project/context.md` (e.g., "ALF", "DEVINTER", "SHOP")
   - `{task-directory}` — actual task directory name following naming convention

2. **Task table:**
   - Add rows for each task as they are created
   - Keep the table sorted by task ID (ascending)
   - Format: `| ID | Name | Directory |`

3. **Optional sections:**
   - If your project doesn't use `task-status.local.md`, remove the "Task Statuses" section
   - If your project has custom task structure, document it in "Task Directory Structure"
   - Add project-specific rules or conventions as needed

4. **Files listing (optional):**
   - Some projects include a detailed "Files in this Directory" section listing all subdirectories
   - This is optional — use it if your project has many tasks and it helps navigation

## Usage

This template is used by:
- `roles/manager/create-task.md` — when creating the initial task index file for a new project
- Project maintainers — when manually organizing the task index

## Artifacts

- `prd/prd.md` or `prd/task-index.md` — task index file in target project
