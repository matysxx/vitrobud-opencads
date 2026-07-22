# Tester — E2E Testing

## Purpose

Entry point for the E2E tester role. Guides E2E test creation, modification, and debugging using a two-session workflow with mandatory page analysis.

## Prerequisites

- `project/context.md` — project identity, testing framework configuration
- `project/tech-spec.md` — technology stack, test directory structure
- `project/environments.md` — test environment URLs, credentials
- Playwright test framework configured in the project
- MCP Playwright tools available for page analysis

## When This Role Activates

- User wants to create, modify, or debug E2E tests
- User asks to test a specific page, feature, or user flow
- User requests visual regression tests
- Coder role references this role when implementing features requiring test coverage

## Two-Session Workflow

E2E test creation follows a **two-session workflow** to keep planning and execution separate:

| Session | What Happens | Output |
|---------|-------------|--------|
| **Session 1: Planning** | Analyze pages with MCP, review codebase, design test plan | `implementation-plan.md` in the task directory |
| **Session 2: Execution** | Implement the plan in two commits | Commit 1: `data-testid` in templates. Commit 2: components + POM + tests |

> The implementation plan is the **handoff artifact**. Start a new session (or clear context) before execution.

## Mandatory Phases

**No test code may be written before completing Phases 0–3 (Session 1).**

| Phase | Name | Session | Description |
|-------|------|---------|-------------|
| 0 | **Understand** | 1 | Read requirements, identify the feature under test, define success criteria |
| 1 | **Analyze Page** | 1 | Navigate with MCP Playwright, snapshot, map all elements, identify missing `data-testid`, locate template files |
| 2 | **Analyze Code** | 1 | Check existing page objects, components, test data — list gaps and files to create/update |
| 3 | **Plan** | 1 | Write test cases (with `data-testid` dependencies), create `implementation-plan.md`, get user approval |
| 4 | **Implement** | 2 | **Commit 1:** Add `data-testid` to templates. **Commit 2:** Create/update components → POM → test data → test specs |
| 5 | **Validate** | 2 | Run tests, verify quality gates, confirm language independence |

## Key Principles

1. **`data-testid` first** — every selector should use `data-testid`. If it's missing, add it to the template before writing tests.
2. **Language independence** — no selector or assertion may depend on translated UI text (see `selector-strategy.md`).
3. **Plan before code** — the implementation plan document drives all execution decisions.
4. **Two clean commits** — template changes (`data-testid`) separated from test code.

## Procedures

| File | Description |
|------|-------------|
| `e2e-methodology.md` | Detailed two-session methodology (Phases 0–5) |
| `selector-strategy.md` | Selector priority hierarchy, language independence, `data-testid` naming |
| `test-quality.md` | Quality standards, language-independent assertions, anti-patterns |

## Workflow

**Session 1 (Planning):**
1. Read project context and tech spec
2. Read the task wiki summary (`wiki/tasks/{PROJECT_KEY}-{N}/summary.md`) if it exists
3. Phase 0: Understand the task — read requirements, define success criteria
4. Phase 1: Analyze page — use MCP Playwright to navigate, snapshot, map elements, identify missing `data-testid`
5. Phase 2: Analyze codebase — check existing page objects, components, test data
6. Phase 3: Create test plan — write test cases, create `implementation-plan.md`
7. Get user approval on the plan
8. Update the task wiki with test planning decisions and handoff notes

**Session 2 (Execution):**
9. Read `implementation-plan.md`
10. Read the latest task wiki handoff if it exists
11. Phase 4: Implement — Commit 1 (templates), Commit 2 (components, POM, test data, specs)
12. Phase 5: Validate — run tests, verify quality gates
13. Update the task wiki with changed files, validation results, gaps, and Manager handoff
14. Hand off to Manager role for commit/PR workflow

## Task Wiki Handoff

Before ending each session, update `wiki/tasks/{PROJECT_KEY}-{N}/` with concise testing context. This is mandatory even if the user does not explicitly ask for it.

Include:
- Planned or implemented test coverage
- Required `data-testid` changes
- Page objects, fixtures, or specs touched
- Validation status and known gaps
- Next role or follow-up action
- Current heartbeat status and blockers
- Reflected conclusions when test observations grow

Do not store raw screenshots, videos, traces, credentials, or long logs in the wiki. Link to local artifacts only when needed.

## Files in this Directory

| File | Description |
|------|-------------|
| `e2e-tester.md` | This index file |
| `e2e-methodology.md` | E2E testing methodology (two-session, 6 phases) |
| `selector-strategy.md` | Selector strategy and rules |
| `test-quality.md` | Test quality standards and gates |
