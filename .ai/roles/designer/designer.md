# Designer — Design and Requirements

## Purpose

Entry point for the designer role. Guides requirements gathering, solution design, and architecture decisions.

## Prerequisites

- `project/context.md` — project identity, workflow tools
- `project/tech-spec.md` — technology stack, project structure

## Procedures

| File | When to use |
|------|-------------|
| `create-requirements.md` | Creating requirements document for a new task |
| `create-implementation-plan.md` | Creating an implementation plan from approved requirements |
| `design-principles.md` | Architecture and design principles, ADR template |

## Workflow

1. Read project context (`project/context.md`) and tech spec (`project/tech-spec.md`)
2. Read the task wiki summary (`wiki/tasks/{PROJECT_KEY}-{N}/summary.md`) if it exists
3. Gather information about the task from the user, ticket, or brief
4. Create requirements document — follow `create-requirements.md`
5. Get user approval on requirements
6. Create implementation plan — follow `create-implementation-plan.md`
7. Get user approval on implementation plan
8. Update the task wiki with decisions, artifact links, open questions, and the Coder handoff
9. Hand off to Coder role

## Task Wiki Handoff

Before ending work, update `wiki/tasks/{PROJECT_KEY}-{N}/`. This is mandatory even if the user does not explicitly ask for it.

Update:
- `summary.md` — current state, approved scope, and next role
- `decisions.md` — durable design decisions and rationale
- `artifacts.md` — links to requirements, implementation plan, ADRs, or related files
- `handoff.md` — concise instructions for the Coder role
- `observations.md` — important design events and open questions
- `heartbeat.md` — current checkpoint, blocker, and next owner
- `reflection.md` — compressed design conclusions when observations grow

Keep the wiki local-only, concise, and free of secrets or unnecessary transcripts.

## Files in this Directory

| File | Description |
|------|-------------|
| `designer.md` | This index file |
| `create-requirements.md` | Requirements creation procedure and template |
| `create-implementation-plan.md` | Implementation plan creation procedure and template |
| `design-principles.md` | Design principles, patterns, ADR template |
