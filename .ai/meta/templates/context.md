# Project Context

This document gives business context and project identifiers so agents can reason about scope, ownership, and where information lives.

## Business Overview

{PROJECT_DESCRIPTION}

Core themes:
- {THEME_1}
- {THEME_2}
- {THEME_3}

## Project Identifiers

- **Project key:** {PROJECT_KEY}
- **Main branch:** {MAIN_BRANCH}
- **Repository:** {VCS_PLATFORM} — {REPO_PATH}

## Knowledge Sources

Work management and documentation systems are part of the project definition and should be used as primary sources of truth for requirements and decisions.

Current systems in use:
- {TASK_MANAGEMENT_TOOL} for issues (project key: {PROJECT_KEY})
- {DOCUMENTATION_TOOL} for documentation
- {VCS_PLATFORM} for code and pull requests

> If a tool is available via MCP, the agent can access it directly. Otherwise, follow manual workflows.

## Task Management

Tasks are tracked in two places:

- **Task index:** `prd/prd.md` — list of all tasks (committed)
- **Task statuses:** `prd/task-status.local.md` — current status per task (local only, gitignored)

### Task Statuses

| Status | Meaning |
|--------|---------|
| Planned | Task defined, requirements may exist, work not started |
| In Progress | Branch and requirements exist, implementation ongoing |
| Done | Implementation complete, acceptance criteria fulfilled |

## Collaboration Expectations

- Treat requirements and acceptance criteria from the issue tracker and documentation space as authoritative.
- Use `prd/` artifacts when a task is tied to a specific requirement or ticket.
- If a task is ambiguous, ask for clarification before making assumptions that could change scope.
