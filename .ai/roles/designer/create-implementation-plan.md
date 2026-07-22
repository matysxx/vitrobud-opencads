# Create Implementation Plan

## Purpose

Create an implementation plan from approved requirements. Produces `prd/{TASK_KEY}/implementation-plan.md`.

## Prerequisites

- Approved `prd/{TASK_KEY}/requirements.md`
- `project/tech-spec.md` — technology stack and project structure
- Codebase access to identify affected files

## Instructions

### Step 1: Analyze Requirements

Read the approved requirements document. Identify:

- Components that need to change
- New components that need to be created
- Integration points with existing code
- Test coverage needed

### Step 2: Scan Affected Code

Explore the codebase to understand:

- Current implementation of related features
- File locations and naming conventions
- Existing patterns for similar functionality
- Test structure and conventions

### Step 3: Write Implementation Plan

Create `prd/{TASK_KEY}/implementation-plan.md` using the template below.

Rules:
- Break work into small, sequential steps
- Each step should be independently testable where possible
- Follow TDD order: write tests first, then implementation
- Reference concrete files and paths from the codebase scan

### Step 4: Get Approval

Present the plan to the user. Iterate until approved. Do not start implementation without approval.

## Template

```markdown
# Implementation Plan: {TASK_KEY}

## Overview

Brief summary of what will be implemented and the approach.

## Steps

### 1. Step Title

- **What:** description of the change
- **Where:** affected files/components
- **How:** approach and patterns to use
- **Tests:** what tests to write

### 2. Step Title

...

## Affected Files

| File | Change type | Description |
|------|-------------|-------------|
| `path/to/file.ext` | Modify | What changes |
| `path/to/new-file.ext` | Create | What it does |

## Dependencies

- Step order constraints (what must be done before what)
- External dependencies (packages, services)

## Testing Strategy

- Unit tests for: ...
- Integration tests for: ...
- Manual verification: ...

## Risks

- Risk and mitigation
```

## Artifacts

- `prd/{TASK_KEY}/implementation-plan.md`
