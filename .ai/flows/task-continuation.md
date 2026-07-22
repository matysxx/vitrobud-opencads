# Task Continuation Flow

## Purpose

Workflow for continuing work on an existing task: from locating the task through implementation to commit. Used when task, branch, and requirements already exist.

## Prerequisites

- `project/context.md` — project identity, workflow tools, project key
- `project/tech-spec.md` — technology stack, project structure
- `roles/coder/` — implementation procedures
- `roles/manager/` — commit and close procedures
- Task must already exist in `prd/prd.md`
- Task must have `requirements.md` in its directory

## When to Use

- Task exists in `prd/prd.md`
- Branch exists
- Requirements document exists
- User wants to continue implementation or add to existing work

## Workflow

### Step 1: Read Project Context

Read these files before proceeding:
1. `project/context.md` — business context, workflow tools, project key
2. `project/tech-spec.md` — technology stack, frameworks, versions

### Step 2: Locate the Task

1. Find the task in `prd/prd.md` — either by task number (e.g., `{PROJECT_KEY}-11`) or by searching for keywords
2. If task number is provided by user — use it directly
3. If task is ambiguous — ask the user which task to work on

**Output:** Task identified

### Step 3: Read Task Context

1. Read `wiki/tasks/{TASK}/summary.md` if it exists — use it as the concise current state
2. Read `wiki/tasks/{TASK}/handoff.md` if it exists — use it for next-step context
3. Read `wiki/tasks/{TASK}/heartbeat.md` if it exists — use it for blockers, dependencies, and next owner
4. Read `prd/{TASK}/requirements.md` — understand what needs to be built
5. If `prd/{TASK}/implementation-plan.md` exists — read it for implementation guidance
6. If other task documents exist (e.g., `investigation-report.md`, `test-plan.md`) — read them for additional context

**Output:** Current task state understood without loading unrelated history

### Step 4: Switch to Task Branch

1. Check current branch: `git status`
2. If already on the correct branch — proceed
3. If on a different branch:
   - Verify no uncommitted changes (or stash them)
   - Switch to task branch: `git checkout {branch-name}`
4. Pull latest changes if needed: `git pull`

**Output:** Working on correct branch

### Step 5: Implement

**Coder role:**
1. Implement the solution — follow `roles/coder/coder.md`
2. Follow coding standards from `roles/coder/coding-standards.md`
3. Follow code quality rules from `roles/coder/code-quality.md`
4. Follow testing rules from `roles/coder/testing-rules.md`
5. If additional context needed — discuss with user before proceeding
6. Update observations, heartbeat, implementation progress, and any handoff notes

**Output:** Implementation complete, code ready for commit

### Step 6: Commit and Close

**Manager role:**
1. Review changes (`git status`, `git diff`)
2. Create commit — follow commit format from `roles/manager/conventional-commits.md` or `roles/manager/custom-commits.md` (as specified in `project/context.md`)
3. If task is complete — close task following `roles/manager/close-task.md`
4. If task continues — commit but do not close; update task status if local task tracking is used
5. If PR needed — follow `roles/manager/pr-description.md`
6. If ticket update needed — follow `roles/manager/update-ticket.md`
7. Reflect observations when needed, then update commit/PR links, status, heartbeat, and final or next-role handoff

**Output:** Changes committed, task status updated

## Key Decision Points

| Decision Point | Question | Action |
|----------------|----------|--------|
| Branch location | Are we on the correct branch? | If no → switch branch in Step 4 |
| Implementation clarity | Is the requirement clear? | If no → discuss with user before Step 5 |
| Task completion | Is the task fully complete? | If yes → close task. If no → commit and leave open |
| Testing | Does the project have E2E testing configured and is testing needed? | If yes → follow `roles/e2e-tester/e2e-tester.md` before Step 6 |

## Role Transitions

```
User Request (continue task X)
    ↓
Locate task in prd.md
    ↓
Read task wiki + requirements/implementation-plan
    ↓
Switch branch
    ↓
Coder (implement)
    ↓
[Optional: E2E Tester (E2E tests)]
    ↓
Manager (commit/close or commit/continue)
```

## Differences from Design-to-Code Flow

| Aspect | Design-to-Code | Task Continuation |
|--------|----------------|-------------------|
| Task creation | Yes — Manager creates task | No — task already exists |
| Requirements | Yes — Designer creates requirements | No — requirements already exist, just read them |
| Implementation plan | Yes — Designer creates plan | Maybe — read if exists |
| Branch creation | Yes — Manager creates branch | No — branch already exists, switch to it |

## Artifacts

- Git commit(s) — implementation committed
- Task status updated (in `prd/prd.md` or local task tracking)
- Task wiki updated in `wiki/tasks/{TASK}/` (local-only)
- Pull request (optional) — if task complete and specified in `project/context.md`
- Ticket update (optional) — if issue tracker configured in `project/context.md`
