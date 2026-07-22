# Bugfix Flow

## Purpose

Workflow for investigating and fixing bugs: from bug report through diagnosis, investigation, fix implementation, to commit. Used when user reports unexpected behavior, errors, or defects.

## Prerequisites

- `project/context.md` — project identity, workflow tools, project key
- `project/tech-spec.md` — technology stack, project structure
- `project/environments.md` — environment access (PROD, Stage, Dev, Local)
- `roles/debugger/` — investigation procedures
- `roles/coder/` — implementation procedures
- `roles/manager/` — commit and close procedures

## When to Use

- User reports a bug, error, or unexpected behavior
- User asks to investigate a problem
- User describes something that "doesn't work" or "works incorrectly"
- Diagnosis is needed before or instead of implementation

## Workflow

### Step 1: Read Project Context

Read these files before proceeding:
1. `project/context.md` — business context, workflow tools, project key
2. `project/tech-spec.md` — technology stack, frameworks, versions
3. `project/environments.md` — environment details, access methods

### Step 2: Understand the Problem

1. Gather information from user's description:
   - What is the expected behavior?
   - What is the actual behavior?
   - How to reproduce the issue?
   - Which environment(s) are affected?
   - Are there error messages, logs, or screenshots?
2. If information is incomplete — ask the user clarifying questions
3. Determine if the bug is tied to an existing task:
   - If yes — note the task number
   - If no — this may require creating a new task (see Step 7)

**Output:** Clear understanding of the problem

### Step 3: Check Task Context (if applicable)

If the bug is tied to an existing task:
1. Read `wiki/tasks/{TASK}/summary.md` if it exists — use it as the concise current state
2. Read `wiki/tasks/{TASK}/heartbeat.md` if it exists — use it for blockers and current owner
3. Read `prd/{TASK}/requirements.md` — understand original requirements
4. If `prd/{TASK}/implementation-plan.md` exists — review planned behavior
5. Check if the bug is a regression or a gap in the original implementation

**Output:** Context about expected vs actual behavior

### Step 4: Investigate

**Debugger role:**
1. Follow investigation procedure from `roles/debugger/debugger.md`
2. Use the 6-phase debugging process:
   - Reproduce the issue
   - Gather evidence (logs, stack traces, network requests)
   - Form hypotheses
   - Test hypotheses
   - Identify root cause
   - Propose solution
3. Document findings as you go
4. Update observations, heartbeat, investigation status, evidence links, and handoff notes

**Output:** Root cause identified, solution proposed

### Step 5: Create Investigation Report

1. Create `investigation-report.md` in the task directory (if task exists) or in a temporary location
2. Document:
   - Problem description
   - Reproduction steps
   - Root cause analysis
   - Proposed solution
   - Alternative solutions considered (if any)
3. Present findings to user
4. Get user approval on the proposed solution

**Output:** `{TASK}/investigation-report.md` — approved investigation report

### Step 6: Implement Fix (if needed)

**Coder role:**
1. If fix is needed — implement the solution following `roles/coder/coder.md`
2. Follow coding standards from `roles/coder/coding-standards.md`
3. Follow code quality rules from `roles/coder/code-quality.md`
4. Follow testing rules from `roles/coder/testing-rules.md`
5. Add tests to prevent regression (if applicable)
6. Update observations, heartbeat, changed files, validation status, and Manager handoff

**Output:** Fix implemented, code ready for commit

### Step 7: Commit and Close

**Manager role:**
1. If no task exists yet and fix was implemented:
   - Create task retroactively following `roles/manager/create-task.md`
   - Register task in `prd/prd.md`
   - Move investigation report to task directory
2. Review changes (`git status`, `git diff`)
3. Create commit — follow commit format from `roles/manager/conventional-commits.md` or `roles/manager/custom-commits.md` (as specified in `project/context.md`)
4. Close task — follow `roles/manager/close-task.md`
5. If PR needed — follow `roles/manager/pr-description.md`
6. If ticket update needed — follow `roles/manager/update-ticket.md`
7. Reflect observations when needed, then update final status, commit/PR links, heartbeat, and any follow-up risks

**Output:** Changes committed, task closed, documentation complete

## Key Decision Points

| Decision Point | Question | Action |
|----------------|----------|--------|
| Task exists? | Is there an existing task for this bug? | If yes → use existing task. If no → may need to create task in Step 7 |
| Root cause found? | Did investigation identify the root cause? | If no → document findings, discuss with user, may need more information |
| Fix needed? | Does the bug require a code change? | If yes → proceed to Step 6. If no (e.g., config issue, user error) → document and close |
| Regression test | Should we add a test to prevent this bug from reoccuring? | If yes → add test in Step 6 following `roles/coder/testing-rules.md` |

## Role Transitions

```
User Report (bug/unexpected behavior)
    ↓
Understand problem
    ↓
[Optional: Read task context]
    ↓
Debugger (investigate)
    ↓
Create investigation report → User Approval
    ↓
[If fix needed] Coder (implement fix)
    ↓
[Optional: E2E Tester (regression tests)]
    ↓
Manager (commit/close)
```

## Special Cases

### Investigation Only (No Fix)

If the investigation reveals:
- Configuration issue (not a code bug)
- User error (feature working as designed)
- External dependency issue (not fixable in this codebase)

**Then:**
1. Complete Step 5 (investigation report)
2. Skip Step 6 (no implementation)
3. Document findings and close

### Critical Production Bug

If the bug is in production and requires urgent fix:
1. Follow expedited workflow from `project/context.md` if defined
2. Create hotfix branch if specified in workflow
3. Minimize investigation time — fix first, deep analysis later
4. Follow all commit and testing procedures even under time pressure

### Bug During Task Implementation

If bug is discovered while working on another task:
1. Decide with user: fix now or create separate task?
2. If fix now → complete fix in current task context
3. If separate task → create new task, switch branches, follow this flow

## Artifacts

- `{TASK}/investigation-report.md` — investigation findings and root cause analysis
- `wiki/tasks/{TASK}/` — local-only investigation summary and handoff notes
- Git commit(s) — fix implementation committed (if applicable)
- Task entry in `prd/prd.md` (if task created)
- Pull request (optional) — if specified in `project/context.md`
- Ticket update (optional) — if issue tracker configured in `project/context.md`
- Regression test (optional) — if test added to prevent recurrence
