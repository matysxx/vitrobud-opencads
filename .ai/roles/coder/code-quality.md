# Code Quality

## Purpose

Quality assurance rules, error handling, security, and refactoring safety. Concrete QA tools come from `project/tech-spec.md`.

## Prerequisites

- `project/tech-spec.md` — QA tools (linters, static analysis), coding standard configuration

## Quality Priorities

When trade-offs arise, follow this hierarchy (top wins):

1. **Correctness** — code does what it should
2. **Readability** — another developer (or agent) can understand it quickly
3. **Simplicity** — no unnecessary layers, abstractions, or dependencies
4. **Extensibility** — easy to change later, but never at the cost of the above

## Decision Justification

Briefly explain non-obvious implementation choices — why this approach, why not the alternative. One sentence is enough. This applies to commit messages, PR descriptions, and inline comments for surprising logic.

## Before Committing

1. Run QA tools (defined in `project/tech-spec.md`, section QA Tools)
2. Scope QA to changed files only (exclude deleted files) based on `git status --porcelain`
3. Pass the file list to QA tools so they use project standards

## Error Handling

- Use specific exception types
- Log errors with context
- Don't catch and ignore exceptions
- Fail fast on unrecoverable errors

## Security

- Validate all input
- Use parameterized queries (or ORM equivalents)
- Escape output in templates
- Never expose sensitive data in logs

## Refactoring Safety

- Backward compatibility has priority over elegance
- Do not change production code solely to make testing easier
- If a breaking change is unavoidable — stop, explain why, and do not implement automatically
