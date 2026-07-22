# Coding Standards

## Purpose

Naming conventions, file organization, and coding style. Concrete framework and language details come from `project/tech-spec.md`.

## Prerequisites

- `project/tech-spec.md` — language, framework, directory structure, coding standard tooling

## Naming Conventions

| Type | Convention | Example |
|------|------------|---------|
| Class / Type | PascalCase | `OrderHistoryService` |
| Method / Function | camelCase | `fetchOrderHistory()` |
| Variable | camelCase | `channelId` |
| Constant | UPPER_SNAKE | `MAX_RETRY_COUNT` |
| Service | Suffix with purpose | `*Service`, `*Handler`, `*Factory` |

> If the project's language uses a different convention (e.g., snake_case for Python), follow `project/tech-spec.md`.

## File Organization

Organize code into layers with clear responsibilities:

| Layer | Responsibility |
|-------|---------------|
| Command | CLI entry points only |
| Controller | HTTP handling only, delegate to services |
| Entity / Model | Data models, minimal logic |
| Service | Business logic lives here |
| Repository | Data access queries only |
| DTO | Data structures, no logic |
| Integration | External API clients |
| Utilities | Stateless helper functions |

> Concrete directory structure and layer names are defined in `project/tech-spec.md`. Not all layers apply to every project — use what exists in the codebase.

## Coding Style

- Follow the coding standard defined in `project/tech-spec.md`
- Use strict typing; declare parameter and return types
- Avoid magic methods where explicit alternatives exist
- Follow framework naming and structure conventions
