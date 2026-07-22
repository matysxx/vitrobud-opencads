# Custom Commits — Ticket-Prefixed Format

## Purpose

Commit message format using ticket number as prefix and bulleted body. Use this file when `project/context.md` specifies `custom-commits` as the commit format.

## Prerequisites

- `project/context.md` — project key (`{PROJECT_KEY}`)

## Before Committing

Always check changes first:

```bash
git status --porcelain    # Quick overview of changed files
git diff --stat           # Summary of changes
```

## Format

```
{PROJECT_KEY}-{number} Short description of the change

- First change or implementation detail
- Second change or implementation detail
- Third change if applicable
```

> Use `{PROJECT_KEY}-X` for tasks without a ticket number (e.g., docs, infra, misc).

## Structure

1. **First line**: Ticket code + short description (max ~72 chars)
   - Always starts with `{PROJECT_KEY}-{number}`
   - Imperative mood: "Add", "Fix", "Update", "Remove", "Implement"

2. **Second line**: Empty (required)

3. **Body**: Bulleted list of changes
   - Use `-` for bullets
   - Each point is concise (one line)
   - Focus on WHAT changed, not HOW
   - 3-7 points typically sufficient

## Examples

### Feature Implementation

```
PROJ-42 Implement user authentication

- Add LoginCommand for CLI authentication
- Implement caching layer for session tokens
- Add user validation middleware
- Create progress indicator for long operations
```

### Bug Fix

```
PROJ-108 Fix token refresh on expiration

- Handle token expiration in middleware
- Add automatic retry with refreshed token
- Log refresh attempts for debugging
```

### Refactoring

```
PROJ-67 Extract processing to dedicated service

- Move processing logic from controller to service
- Add interface for processing strategies
- Update tests for new service structure
```

### Small Change

```
PROJ-15 Add missing validation for email field

- Add email format validation in DTO
- Return proper error message on invalid input
```

## Anti-patterns (Avoid)

```
# BAD: No ticket code
Fix login bug

# BAD: No description after code
PROJ-23

# BAD: Description too vague
PROJ-45 Updates

# BAD: No bullet points
PROJ-67 Refactor service
Changed some code and fixed stuff

# BAD: First line too long
PROJ-89 Add try-catch block in fetchHistory method to handle RateLimitException
```

## When to Commit

- One logical change per commit
- All tests pass
- Code style checks pass
- Commit relates to a single task (or clearly documented if spanning multiple)
