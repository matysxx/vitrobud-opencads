# Design-to-Code Flow

## Purpose

Complete workflow for new features or tasks: from initial request through requirements gathering, design, implementation, to commit. Used when no task, branch, or requirements exist yet.

## Prerequisites

- `project/context.md` — project identity, workflow tools, project key
- `project/tech-spec.md` — technology stack, project structure
- `roles/manager/` — task creation procedures
- `roles/designer/` — requirements and design procedures
- `roles/coder/` — implementation procedures

## When to Use

- User wants to start a new feature or task
- No task number exists yet
- No branch exists yet
- No requirements document exists yet

## Workflow

### Step 1: Read Project Context

Read these files before proceeding:
1. `project/context.md` — business context, workflow tools, project key
2. `project/tech-spec.md` — technology stack, frameworks, versions

### Step 2: Create Task Structure

**Manager role:**
1. Determine the task number — check `prd/prd.md` for the next free `{PROJECT_KEY}-N`
2. Determine the task name and description with the user (if not provided)
3. Create branch and task directory — follow `roles/manager/create-task.md` OR `roles/manager/create-task-from-ticket.md` if creating from an issue tracker ticket
4. Create local task wiki directory: `wiki/tasks/{PROJECT_KEY}-{N}/`
5. Initialize `wiki/tasks/{PROJECT_KEY}-{N}/summary.md` from `wiki/task-summary-template.md`
6. Initialize `observations.md`, `reflection.md`, and `heartbeat.md` from the wiki templates

**Output:** Branch created, task directory created, task registered in `prd/prd.md`, local task wiki initialized

### Step 3: Create Requirements

**Designer role:**
1. Gather information about the task from user, ticket, or brief
2. Create requirements document — follow `roles/designer/create-requirements.md`
3. Present requirements to user for approval
4. Wait for user approval before proceeding
5. Update observations, heartbeat, approved scope, decisions, and artifact links

**Output:** `{TASK}/requirements.md` — approved requirements document

### Step 4: Create Implementation Plan

**Designer role:**
1. Analyze requirements
2. Review codebase for relevant files, patterns, dependencies
3. Create implementation plan — follow `roles/designer/create-implementation-plan.md`
4. Present implementation plan to user for approval
5. Wait for user approval before proceeding
6. Reflect observations into the summary, then update implementation decisions and Coder handoff

**Output:** `{TASK}/implementation-plan.md` — approved implementation plan

### Step 5: Implement

**Coder role:**
1. Read the implementation plan
2. Read the task wiki summary and latest handoff
3. Implement the solution — follow `roles/coder/coder.md`
4. Follow coding standards from `roles/coder/coding-standards.md`
5. Follow code quality rules from `roles/coder/code-quality.md`
6. Follow testing rules from `roles/coder/testing-rules.md`
7. Update observations, heartbeat, changed files, tests, open questions, and Manager handoff

**Output:** Implementation complete, code ready for commit

### Step 6: Commit and Close

**Manager role:**
1. Review changes (`git status`, `git diff`)
2. Create commit — follow commit format from `roles/manager/conventional-commits.md` or `roles/manager/custom-commits.md` (as specified in `project/context.md`)
3. Close task — follow `roles/manager/close-task.md`
4. If PR needed — follow `roles/manager/pr-description.md`
5. If ticket update needed — follow `roles/manager/update-ticket.md`

**Output:** Changes committed, task closed, PR created (if applicable), ticket updated (if applicable)

## Key Decision Points

| Decision Point | Question | Action |
|----------------|----------|--------|
| Task creation | Is there an existing ticket in the issue tracker? | If yes → use `create-task-from-ticket.md`. If no → use `create-task.md` |
| Requirements approval | Are requirements complete and approved? | If no → return to Step 3 and revise |
| Implementation plan approval | Is the plan complete and approved? | If no → return to Step 4 and revise |
| Testing | Does the project have E2E testing configured? | If yes and testing needed → follow `roles/e2e-tester/e2e-tester.md` before Step 6 |

## Role Transitions

```
User Request
    ↓
Manager (create task)
    ↓
Designer (requirements) → User Approval
    ↓
Designer (implementation plan) → User Approval
    ↓
Coder (implement)
    ↓
[Optional: E2E Tester (E2E tests)]
    ↓
Manager (commit/close)
```

## Artifacts

- `prd/prd.md` — task entry added
- `prd/{TASK}/requirements.md` — requirements document
- `prd/{TASK}/implementation-plan.md` — implementation plan document
- `wiki/tasks/{TASK}/` — local-only task context summary and handoff notes
- Git branch — feature/task branch created
- Git commit(s) — implementation committed
- Pull request (optional) — if specified in `project/context.md`
- Ticket update (optional) — if issue tracker configured in `project/context.md`
