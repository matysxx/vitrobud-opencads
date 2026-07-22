# Conventional Commits

## Purpose

Commit message format following the [Conventional Commits](https://www.conventionalcommits.org/) standard. Use this file when `project/context.md` specifies `conventional-commits` as the commit format.

## Prerequisites

- `project/context.md` — project key (optional, for ticket references), commit format confirmation

## Before Committing

Always check changes first:

```bash
git status --porcelain    # Quick overview of changed files
git diff --stat           # Summary of changes
```

## Format

```
<type>(<scope>): <short description>

<body>

<footer>
```

### Type (required)

| Type | When to use |
|------|-------------|
| `feat` | New feature or user-visible functionality |
| `fix` | Bug fix |
| `refactor` | Code change that neither fixes a bug nor adds a feature |
| `docs` | Documentation only |
| `test` | Adding or updating tests |
| `chore` | Build, tooling, CI, dependencies |
| `style` | Formatting, whitespace, semicolons (no logic change) |
| `perf` | Performance improvement |
| `ci` | CI/CD configuration |
| `revert` | Reverting a previous commit |

### Scope (optional)

A noun describing the area of the codebase affected. Use names consistent with project structure (modules, packages, components).

### Short Description (required)

- Imperative mood: "add", "fix", "update", "remove"
- No capitalization of first letter
- No period at the end
- Max ~72 characters for the full first line

### Body (recommended)

- Separated from subject by a blank line
- Explain WHAT changed and WHY (not HOW)
- Use bulleted list (`-`) for multiple points
- Wrap at 72 characters

### Footer (optional)

- **Breaking changes:** start with `BREAKING CHANGE:` followed by description
- **Ticket references:** `Refs: {PROJECT_KEY}-{number}` or `Closes: {PROJECT_KEY}-{number}`

## Examples

### Feature

```
feat(auth): add token refresh on expiration

- Handle token expiration in authentication middleware
- Add automatic retry with refreshed token
- Store refresh token securely in session

Refs: PROJ-42
```

### Bug Fix

```
fix(cart): prevent duplicate items on rapid clicks

- Add debounce to add-to-cart handler
- Return existing item reference if already in cart

Closes: PROJ-108
```

### Refactoring

```
refactor(orders): extract processing to dedicated service

- Move processing logic from controller to service
- Add interface for processing strategies
- Update tests for new service structure

Refs: PROJ-67
```

### Breaking Change

```
feat(api): replace session auth with JWT

- Remove session-based authentication endpoints
- Add JWT token generation and validation
- Update all protected routes to use Bearer token

BREAKING CHANGE: All API consumers must switch to Bearer token
authentication. Session cookies are no longer accepted.

Refs: PROJ-201
```

### Small Change

```
fix(validation): add missing email format check

- Add email format validation in registration DTO
- Return proper error message on invalid input

Refs: PROJ-15
```

## Anti-patterns (Avoid)

```
# BAD: No type
Add login feature

# BAD: Past tense
feat: added new button

# BAD: Too vague
fix: updates

# BAD: Type not from the list
feature(auth): add login

# BAD: No description, only ticket
fix: PROJ-23

# BAD: First line too long
feat(integration): add try-catch block in fetchHistory method to handle RateLimitException from external API
```

## When to Commit

- One logical change per commit
- All tests pass
- Code style checks pass
- Commit relates to a single task (or clearly documented if spanning multiple)
