# Testing Rules

## Purpose

Rules for writing unit, integration, and E2E tests. Concrete testing frameworks and tools come from `project/tech-spec.md`.

## Prerequisites

- `project/tech-spec.md` — test runner, testing frameworks, test directory structure

## General Principles

- When feasible, write tests before implementation (test-first / TDD): define the expected behavior as a failing test, implement the minimum to pass, then refactor. This is not mandatory for every change, but strongly preferred for non-trivial logic and bug fixes where the failing test captures the exact problem.
- Use the AAA pattern (Arrange-Act-Assert) for all test types
- No logic in tests — no if/else/loops; each test is a straight-line AAA flow
- One assertion concept per test
- Descriptive test names: `test_methodName_condition_expectedResult` (adapt format to project convention)
- Use parameterized tests / data providers for multiple scenarios instead of duplicating test methods
- Increase meaningful coverage; do not inflate with trivial or brittle tests
- Focus on critical logic and edge cases
- Do not test trivial getters/setters, framework internals, or generated code

## Unit Tests

- Test services and utilities in isolation
- Mock external dependencies
- Avoid network, filesystem, or database access

## Integration Tests

- Test component interactions
- Use test database
- Clean up test data after each test

## E2E Tests

- Test complete user flows
- Keep tests focused on user-visible behavior
- Follow existing page object / component patterns
- Prefer stable selectors and deterministic flows
- Browser-based for UI, API client for endpoints

> For comprehensive E2E testing methodology (selectors, quality gates, patterns), see `e2e-tester/e2e-tester.md` if the E2E tester role is configured.
