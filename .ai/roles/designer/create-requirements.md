# Create Requirements

## Purpose

Create a requirements document for a new task. Produces `prd/{TASK_KEY}/requirements.md`.

## Prerequisites

- Task key or identifier (from ticket system or user)
- User available for questions and approval
- `project/context.md` and `project/tech-spec.md` read

## Instructions

### Step 1: Gather Information

Collect information from all available sources before writing:

- **User briefing** — ask clarifying questions, do not assume
- **Ticket content** — if a ticket exists, extract requirements from it
- **Existing code** — scan affected areas to understand current behavior
- **Related tasks** — check `prd/` for related or similar tasks

### Step 2: Create Task Directory

Create `prd/{TASK_KEY}/` if it does not exist.

`{TASK_KEY}` format:
- Lowercase letters, numbers, and hyphens only
- Include ticket number if available
- Keep it short and descriptive

Examples: `prd/2961-mail-sync-command/`, `prd/ALF-2-similarity/`

### Step 3: Write requirements.md

Create `prd/{TASK_KEY}/requirements.md` using the template below. Fill every section. Mark unknown items as open questions rather than guessing.

### Step 4: Quality Checklist

Verify before presenting to the user:

- [ ] Goal is clear and measurable
- [ ] Requirements are testable (can verify done/not done)
- [ ] Scope is bounded (clear what is NOT included)
- [ ] Dependencies on existing code documented
- [ ] Open questions listed (do not guess — ask)

### Step 5: Get Approval

Present the document to the user. Iterate until approved. Do not proceed to implementation planning without approval.

## Template

```markdown
# {TASK_KEY} Task Title

## Goal

What we want to achieve (1-2 sentences).

## Background

Why this task exists, context, dependencies.

## Requirements

### Functional Requirements

- [ ] FR1: Description
- [ ] FR2: Description
- [ ] FR3: Description

### Non-Functional Requirements

- [ ] NFR1: Performance/security/etc requirement

## Scope

### In Scope

- What IS included

### Out of Scope

- What is NOT included (to avoid scope creep)

## Technical Notes

Architecture decisions, integration points, constraints.

## Acceptance Criteria

- [ ] AC1: Verifiable criterion
- [ ] AC2: Verifiable criterion

## Open Questions

- Question 1?
- Question 2?
```

## Artifacts

- `prd/{TASK_KEY}/requirements.md`
