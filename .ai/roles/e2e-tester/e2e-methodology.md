# E2E Testing Methodology

## Purpose

Detailed two-session workflow for creating E2E tests. Session 1 (Planning) produces an implementation plan document. Session 2 (Execution) implements the plan in two separate commits.

## Prerequisites

- `e2e-tester.md` — role entry point
- `selector-strategy.md` — selector rules
- `test-quality.md` — quality standards
- MCP Playwright tools available

## Workflow Overview

| Session | Phases | Output |
|---------|--------|--------|
| **Session 1: Planning** | Phase 0 → Phase 1 → Phase 2 → Phase 3 | `implementation-plan.md` in the task directory |
| **Session 2: Execution** | Phase 4 → Phase 5 | Two commits: (1) `data-testid` in templates, (2) components + POM + tests |

> **Start a new session (or clear context) between Planning and Execution.** The implementation plan document is the handoff artifact between sessions.

---

## Session 1: Planning

### Phase 0: Understand the Task

1. Read the user's requirements, ticket description, or task brief
2. Identify the feature under test and its business purpose
3. Define success criteria: **"What does WORKING look like for the end user?"**
4. Identify the page(s) involved and how to reach them (auth, navigation path)
5. If requirements are unclear — ask the user before proceeding

**Output:** Clear understanding of what to test and why.

---

### Phase 1: Page Analysis — MANDATORY MCP Playwright

This phase is **not optional**. Every test must be informed by a real page inspection.

#### Steps

1. **Navigate** to the target page using `browser_navigate`
   - If the page requires authentication, log in first
   - If the page requires specific navigation (e.g., My Account → Orders), perform all steps
2. **Take a snapshot** using `browser_snapshot` (accessibility tree)
   - This is the **primary source for selectors** — not guessing, not copying from other tests
3. **Map all interactive elements** — for each element document:
   - Element type and purpose (form field, button, link, table column, action, etc.)
   - Available stable attributes: `data-testid`, `name`, `id`, `data-*`, `aria-*`
   - Locator candidate — **must be language-independent** (see `selector-strategy.md`)
4. **Identify missing `data-testid` attributes:**
   - For every element that lacks a stable, language-independent selector → mark it as **requiring a new `data-testid`**
   - Identify the **exact template file** where the attribute must be added
   - Propose the `data-testid` value following the naming convention: `{feature}-{element}` (e.g., `orders-search`, `shipping-address-save`)
5. **Map page states:**
   - Loading → Loaded → Empty state → Error state → Success state
   - Which states can we test? Which require specific data?
6. **Document navigation path:** Full path from login to the target page

#### Required Output Format (per page/area)

Document each analyzed page area using this structure:

```markdown
#### {N}) {Area Name} (`/url-path`)
- Elements:
  - {element description}: {type, purpose}
  - ...
- Locator candidates (language-independent):
  - `page.getByTestId('{name}')` — {element description}
  - `page.locator('[name="{attr}"]')` — {element description}
  - ...
- Required `data-testid` additions:
  - `{template-file-path}`
    - `data-testid="{name}"` on {element description}
    - ...
```

#### Checklist Before Moving On

- [ ] Every target page visited and snapshot taken
- [ ] All interactive elements listed with language-independent locator candidates
- [ ] Missing `data-testid` attributes identified with exact template file paths and proposed names
- [ ] Page states mapped

---

### Phase 2: Codebase Analysis

#### Check Existing Assets

1. **Page Objects** (test framework page object directory, e.g., `src/playwright/pages/`)
   - Is there already a page object for this page?
   - If yes: read it, list existing locators and methods, identify gaps
   - If no: plan to create one — list the locators and methods it will need
2. **Components** (test framework component directory, e.g., `src/playwright/components/`)
   - What reusable components apply? (login, cookie banner, flash messages, navigation)
   - Do existing components need new methods or locators?
   - Are there shared UI patterns across pages that warrant a new component?
3. **Test Data** (test data directory, e.g., `src/playwright/test-data/`)
   - Is the needed test data available?
   - If data is missing → note what needs to be added
4. **Similar Tests** (test specs directory, e.g., `src/playwright/tests/`)
   - Find tests for similar page types as reference patterns
   - Note common patterns: setup, teardown, assertion style

#### Required Output

List every file that must be created or modified:

```markdown
- **Components:**
  - `{file}` — {create/update}: {what to add/change}
- **Page Objects:**
  - `{file}` — {create/update}: {what to add/change}
- **Test Data:**
  - `{file}` — {what to add}
```

#### Checklist Before Moving On

- [ ] Existing page objects reviewed — gaps identified
- [ ] Reusable components identified — necessary changes listed
- [ ] Test data availability confirmed — missing data noted
- [ ] Reference patterns from similar tests noted

---

### Phase 3: Test Plan & Implementation Plan

#### Write Test Cases

For each test case, use this format — **every test case explicitly lists its `data-testid` dependencies**:

```markdown
### TC{N} – {Area}: {short description}
- Required `data-testid`: {list of data-testid values this test depends on}
- **Given** (precondition)
- **When** (action)
- **Then** (assertion — language-independent: data-testid, URL, attributes, count, field values)
- Cleanup: {how to clean up created data, or "none"}
```

#### Apply the "Break Test" Filter

Every assertion must pass this question:

> **"If this feature broke in production, would this assertion catch it?"**

If the answer is no, the assertion is too weak. Strengthen it or replace it.

#### Minimum Required Scenarios by Page Type

| Page Type | Minimum Scenarios |
|-----------|-------------------|
| **Form** | Submit valid data; submit invalid data; verify saved values persist after reload |
| **Table/List** | Verify data is displayed with correct content; verify sorting/filtering; verify pagination if present |
| **CRUD** | Create item; read/verify item; update item; delete item |
| **Navigation** | Verify all links/buttons lead to correct destinations |
| **Dashboard/Overview** | Verify all widgets show real data (content, not just visibility) |

#### Create the Implementation Plan Document

Save the complete plan as `implementation-plan.md` in the task directory (location defined in `project/context.md`, e.g., `.ai/prd/{TASK}/implementation-plan.md`).

The document **must** contain all of the following sections:

1. **Global rules** — selector strategy reminders, language-independence, project-specific constraints
2. **Page analysis** — structured output from Phase 1 (per area: elements, locators, required `data-testid`)
3. **Files to create/modify** — from Phase 2 (components, POM, test data)
4. **Test cases** — TC1–TCN from this phase
5. **Implementation order** — explicit step-by-step sequence for Phase 4

#### Checklist Before Moving On

- [ ] All test scenarios written with `data-testid` dependencies listed per test case
- [ ] Every assertion passes the "Break Test" filter
- [ ] Minimum scenarios for each page type are covered
- [ ] Implementation plan document saved in the task directory
- [ ] **Plan reviewed and approved by the user**

---

## Session 2: Execution

> Start a new session or clear context. Read the implementation plan document to begin.

### Phase 4: Implementation

**Two separate commits. Strict order.**

#### Commit 1: Add `data-testid` Attributes

1. Read the implementation plan — specifically the "Required `data-testid` additions" sections
2. Add all required `data-testid` attributes to the template files
3. Verify each attribute is correctly placed on the right element
4. **Commit** with a message describing the scope (e.g., `Add data-testid attributes for {feature} E2E tests`)

> **Do NOT write any test code in this commit.** It contains only template changes.

#### Commit 2: Components, Page Objects, and Tests

**Strict order within this commit — do not skip steps or change the sequence.**

##### Step 1: Create/Update Components

- Create or update components in the test framework component directory based on the plan
- Components encapsulate shared UI patterns used across multiple pages

##### Step 2: Create/Update Page Objects

- Create or update page objects in the test framework page object directory based on the plan
- All selectors use the `data-testid` attributes added in Commit 1
- Encapsulate all selectors in the page object — **never put raw selectors in test files**
- Add action methods (click, fill, navigate) and assertion methods (expectLoaded, expectVisible, expectDataCorrect)
- Follow the selector strategy in `selector-strategy.md`

##### Step 3: Add/Update Test Data

- Add missing data to test data files (e.g., `test-data/*.data.ts`)
- Every data file exports typed constants or arrays
- Use environment variables for sensitive or environment-specific data

##### Step 4: Write Test Specs

- Implement test cases from the implementation plan (TC1–TCN)
- Compose tests using page objects, components, and test data
- Use test framework grouping (e.g., `test.describe()`) to group related tests
- Use test framework setup hooks (e.g., `test.beforeEach()`) for shared setup (login, navigation)
- Each test must be independently runnable
- Follow quality standards in `test-quality.md`

##### Step 5: Commit

- **Commit** components, page objects, test data, and test specs together
- Descriptive message (e.g., `Add E2E tests for {feature}`)

#### Rules (both commits)

- **Never put raw selectors in test files** — always go through page objects/components
- **Never hardcode test data in test files** — use test-data files
- **Never rely on state from a previous test** — each test is independent
- **All selectors must be language-independent** — see `selector-strategy.md`

---

### Phase 5: Validation

#### Run the Test

Execute the test using the project's test command (e.g., `npx playwright test ./tests/path/to/test.spec.ts`)

#### Verify

1. **Test passes** — if it fails, diagnose and fix
2. **Mental "Break Test"** — would this test fail if the feature stopped working?
3. **Quality gate** — does the test meet the standards in `test-quality.md`?
4. **Selector quality** — are selectors following the hierarchy in `selector-strategy.md`?
5. **Language independence** — no selector or assertion depends on translated UI text

#### If the Test Only Checks Visibility

It is **insufficient**. Go back to the implementation plan and add meaningful assertions:
- Data content assertions (attribute values, field values, URL patterns)
- State change verification
- Navigation verification
- Error handling verification

---

## Quick Reference: MCP Playwright Tools

| Tool | Use For |
|------|---------|
| `browser_navigate` | Go to a URL |
| `browser_snapshot` | Get accessibility tree (primary selector source) |
| `browser_take_screenshot` | Capture visual state |
| `browser_click` | Click an element |
| `browser_type` | Type text into an input |
| `browser_fill_form` | Fill multiple form fields |
| `browser_select_option` | Select dropdown option |
| `browser_press_key` | Press keyboard key |
| `browser_evaluate` | Run JavaScript on the page |
| `browser_wait_for` | Wait for text/element/time |
| `browser_console_messages` | Check console for errors |
| `browser_network_requests` | Inspect network activity |

## Artifacts

- `implementation-plan.md` — comprehensive test plan document in task directory (Session 1 output)
- Commit 1 — template changes with `data-testid` attributes
- Commit 2 — test components, page objects, test data, and test specs
