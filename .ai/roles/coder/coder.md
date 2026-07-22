# Coder — Coding and Testing

## Purpose

Entry point for the coder role. Guides implementation work — coding standards, quality rules, and testing practices.

## Prerequisites

- `project/tech-spec.md` — technology stack, directory structure, QA tools, test runner
- Task's `requirements.md` — what to implement

## Before Starting

1. Read the task's `requirements.md`
2. Read the task wiki summary (`wiki/tasks/{PROJECT_KEY}-{N}/summary.md`) if it exists
3. Read the latest handoff (`wiki/tasks/{PROJECT_KEY}-{N}/handoff.md`) if it exists
4. Read the heartbeat (`wiki/tasks/{PROJECT_KEY}-{N}/heartbeat.md`) if it exists
5. Read `project/tech-spec.md` for the technology stack
6. Check existing code in the area of changes — follow established patterns

## General Rules

- Do not invent architecture — follow existing patterns in the codebase
- Implement everything that can be implemented locally; if something requires external setup, explicitly flag it and suggest steps
- Respect the defined module or package scope for changes — do not reach into unrelated modules
- Any potentially breaking change must be explicitly highlighted before implementation

## Procedures / Rules

| File | Description |
|------|-------------|
| `coding-standards.md` | Naming conventions, file organization, style |
| `code-quality.md` | QA, error handling, security, refactoring safety |
| `testing-rules.md` | Rules for writing tests (unit, integration, E2E) |

> For comprehensive E2E testing methodology, see `e2e-tester/e2e-tester.md` (if the E2E tester role is configured).

## After Finishing

Before ending work, update `wiki/tasks/{PROJECT_KEY}-{N}/`. This is mandatory even if the user does not explicitly ask for it.

Include:
- What was implemented and which files changed
- Key implementation decisions and tradeoffs
- Tests added or run, with concise results
- Known gaps, blockers, or follow-up tasks
- The next recommended role or handoff target

Use the context policy in `wiki/context-policy.md`:
- Append important events to `observations.md`
- Update `heartbeat.md` with status, blockers, and next action
- Update `handoff.md` before switching roles or stopping
- Reflect observations into `summary.md` when a meaningful implementation phase is complete

Keep the wiki local-only, concise, and free of secrets, raw logs, full diffs, or unnecessary chat history.

## Files in this Directory

| File | Description |
|------|-------------|
| `coder.md` | This index file |
| `coding-standards.md` | Coding conventions and file organization |
| `code-quality.md` | Quality assurance, error handling, security |
| `testing-rules.md` | Testing rules and patterns |
