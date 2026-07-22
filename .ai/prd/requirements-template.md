# Requirements Template

## Purpose

Template for task requirements documents (`{TASK}/requirements.md`). This document defines what needs to be built, why, and how success is measured.

## Prerequisites

- `roles/designer/create-requirements.md` — procedure for creating requirements
- Task directory must exist (created by Manager role)

## Template

Copy the template below and customize it for your task:

---

```markdown
# {PROJECT_KEY}-{N} {Task Name}

## Goal

{1-2 sentence summary: What does this task accomplish and why?}

## Background

{Context that helps understand the problem:}
- What is the current situation?
- Why is this change needed?
- What triggered this task? (user request, bug report, technical debt, etc.)
- What constraints or dependencies exist?

## Requirements

### Functional Requirements

{What the system must do. Use checkboxes for trackability.}

- [ ] FR1: {Requirement description}
- [ ] FR2: {Requirement description}
- [ ] FR3: {Requirement description}

### Non-Functional Requirements

{Quality attributes: performance, security, maintainability, etc.}

- [ ] NFR1: {Requirement description}
- [ ] NFR2: {Requirement description}

## Scope

### In Scope

{What is included in this task:}
- {Item}
- {Item}

### Out of Scope

{What is explicitly NOT included (to avoid scope creep):}
- {Item} (reason: {why it's out of scope})
- {Item} (reason: {separate ticket/future work})

## Technical Notes

{Implementation hints, architecture decisions, technology choices:}
- {Technical constraint or decision}
- {File/component that needs changes}
- {API or interface to use}
- {Pattern or convention to follow}

## Acceptance Criteria

{How do we know the task is complete? These are testable conditions.}

- [ ] AC1: {Criterion description}
- [ ] AC2: {Criterion description}
- [ ] AC3: {Criterion description}

## Open Questions

{Unresolved issues that need clarification before or during implementation:}

- Q1: {Question}
  - Answer: {Answer or "TBD"}
- Q2: {Question}
  - Answer: {Answer or "TBD"}

{If no open questions: "None — requirements are clear."}

## References

{Links to relevant resources:}
- Related tickets: {TICKET-ID}
- Design documents: {link or file path}
- API documentation: {link}
- Prior art / similar implementations: {link or file path}

{If no references: Remove this section.}
```

---

## Template Variations

### Variation 1: Product Feature (Detailed)

Use this variation for new features with complex business logic:

```markdown
# {PROJECT_KEY}-{N} {Feature Name}

## 1. Overview

### 1.1 Purpose
{What is this feature and why does it exist?}

### 1.2 Target Users
- {User persona 1}
- {User persona 2}

### 1.3 Non-Goals
{What this feature explicitly does NOT do}

## 2. Functional Requirements

### 2.1 {Area 1}
{Detailed requirements for this area}

### 2.2 {Area 2}
{Detailed requirements for this area}

## 3. Non-Functional Requirements

### 3.1 Performance
{Performance targets and constraints}

### 3.2 Security
{Security requirements}

### 3.3 Usability
{UX requirements}

## 4. User Stories

### US1: {Story title}
**As a** {user role}
**I want** {goal}
**So that** {benefit}

**Acceptance Criteria:**
- [ ] {Criterion}
- [ ] {Criterion}

## 5. Technical Design

{High-level architecture, components, data flow}

## 6. Acceptance Criteria

- [ ] {Overall criterion}

## 7. Open Questions

{Unresolved issues}
```

### Variation 2: Bug Fix (Focused)

Use this variation for bug fixes or small technical tasks:

```markdown
# {PROJECT_KEY}-{N} {Bug Title}

## Problem

{What is broken or not working as expected?}

## Expected Behavior

{What should happen?}

## Actual Behavior

{What currently happens?}

## Reproduction Steps

1. {Step 1}
2. {Step 2}
3. Observe: {unexpected behavior}

## Root Cause

{If known from investigation — link to investigation-report.md if it exists}

## Proposed Solution

{How will this be fixed?}

## Scope

### In Scope
- {What will be fixed}

### Out of Scope
- {Related issues that are separate tickets}

## Acceptance Criteria

- [ ] AC1: Bug no longer occurs when following reproduction steps
- [ ] AC2: No regression in related functionality
- [ ] AC3: Test added to prevent regression (if applicable)

## Technical Notes

{Implementation details, files to change, etc.}
```

### Variation 3: Refactoring / Technical Debt

Use this variation for refactoring, code quality improvements, or technical debt:

```markdown
# {PROJECT_KEY}-{N} {Refactoring Title}

## Goal

{What code quality improvement or architectural change is this achieving?}

## Motivation

{Why is this refactoring needed?}
- Technical debt accumulated from: {reason}
- Pain points: {what makes current code problematic}
- Benefits of refactoring: {what improves}

## Current State

{Description of current architecture/code structure}

## Desired State

{Description of target architecture/code structure}

## Migration Strategy

{How to transition from current to desired state without breaking production:}
1. {Step 1}
2. {Step 2}

## Scope

### In Scope
- {Component/module/file to refactor}

### Out of Scope
- {Component left as-is for now}

## Acceptance Criteria

- [ ] AC1: All existing tests still pass
- [ ] AC2: No functional behavior changes
- [ ] AC3: Code quality metrics improved (e.g., complexity, duplication)
- [ ] AC4: Documentation updated to reflect new structure

## Risk Assessment

{What could go wrong and how to mitigate:}
- Risk: {description} — Mitigation: {strategy}
```

## Customization Guidelines

1. **Choose the right variation:**
   - Use **base template** for most tasks
   - Use **Variation 1 (Product Feature)** for complex new features with detailed specs
   - Use **Variation 2 (Bug Fix)** for defects and small fixes
   - Use **Variation 3 (Refactoring)** for technical improvements without functional changes

2. **Replace placeholders:**
   - `{PROJECT_KEY}` — from `project/context.md`
   - `{N}` — task number
   - `{Task Name}` — descriptive task name
   - All `{...}` placeholders with actual content

3. **Use checkboxes for trackability:**
   - `- [ ]` for requirements and acceptance criteria
   - Check them off (`- [x]`) as they are completed
   - This provides visual progress tracking

4. **Keep it concise but complete:**
   - Remove sections that don't apply (e.g., "References" if none)
   - Don't over-specify — leave implementation details to the implementation plan
   - Focus on WHAT and WHY, not HOW (HOW goes in implementation-plan.md)

5. **Link to other documents:**
   - If investigation was done: reference `investigation-report.md`
   - If design decisions were made: reference design documents or ADRs
   - If related to other tasks: reference their IDs

## Quality Checklist

Before finalizing requirements, verify:

- [ ] Goal is clear and concise
- [ ] Background provides sufficient context
- [ ] All requirements are testable
- [ ] Scope is clearly defined (in/out of scope)
- [ ] Acceptance criteria are specific and measurable
- [ ] Open questions are either answered or marked as TBD
- [ ] Technical notes provide implementation guidance (if available)

## Usage

This template is used by:
- `roles/designer/create-requirements.md` — when creating requirements for a new task
- Developers — when reviewing task requirements before implementation
- Reviewers — when validating task completion against acceptance criteria

## Artifacts

- `prd/{TASK}/requirements.md` — requirements document in target project task directory
