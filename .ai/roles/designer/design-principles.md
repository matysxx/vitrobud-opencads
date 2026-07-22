# Design Principles

## Purpose

Abstract design principles for solution architecture. Concrete technologies and frameworks are defined in `project/tech-spec.md`.

## Prerequisites

- `project/tech-spec.md` — for project-specific technology choices

## Instructions

Apply these principles when designing solutions. They are defaults — override with project-specific conventions from `project/tech-spec.md` when they conflict.

### Layered Architecture

Separate concerns into layers. Business logic must not depend on delivery mechanism (HTTP, CLI, queue).

```
Entry Point (Controller / Command / Handler)
    ↓
Service Layer (business logic, orchestration)
    ↓
Data Access Layer (repositories, clients, adapters)
    ↓
Infrastructure (database, filesystem, external APIs)
```

### Dependency Direction

- Depend on abstractions, not implementations
- Use the framework's dependency injection container
- Avoid manual instantiation of services

### Data Flow

- Use DTOs for complex data transfer between layers
- Controllers/commands handle I/O concerns only — delegate logic to services
- External integrations go through adapter/client classes, not called directly from business logic

### Error Handling

- Use domain-specific exceptions, not generic ones
- Catch at boundaries (controllers, commands), not deep inside business logic
- Log failures with context: what was attempted, what went wrong

### Caching Strategy

When the project requires caching:

- Cache at the appropriate layer (HTTP, application, query)
- Use TTL-based invalidation as default
- Specific cache backends defined in `project/tech-spec.md`

## Architecture Decision Records

When making significant design decisions, document them as ADRs in the task directory.

### ADR Template

```markdown
# ADR-NNN: Decision Title

## Status

Proposed | Accepted | Deprecated | Superseded

## Context

What is the issue or problem we're addressing?

## Decision

What change are we making?

## Consequences

Positive and negative effects.

## Alternatives Considered

What other options were evaluated and why they were rejected?
```

Naming convention: `adr-NNN-short-title.md` (e.g., `adr-001-cache-strategy.md`)

## Artifacts

- ADR documents when significant decisions are made
