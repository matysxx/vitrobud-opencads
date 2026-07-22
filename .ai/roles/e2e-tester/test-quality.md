# Test Quality Standards

## Purpose

Quality gates for E2E tests. Every test must meet these standards before being accepted.

## Prerequisites

- `e2e-tester.md` — role entry point
- `e2e-methodology.md` — methodology context
- `selector-strategy.md` — selector rules

## The Cardinal Rule

> **Every test must answer a business question.**
> "Can the user do X?" — not "Is element Y visible?"

---

## Language-Independent Assertions

> **No assertion may depend on translated UI text.**

Assertions must verify functionality through language-independent means:

| Allowed | Example |
|---------|---------|
| URL pattern | `await expect(page).toHaveURL(/\/account\/orders/)` |
| Element attribute value | `await expect(select).toHaveValue('option-value')` |
| Field value (user-entered data) | `await expect(input).toHaveValue('E2E-Test-Company')` |
| Element count / structure | `await expect(rows).not.toHaveCount(0)` |
| Element visibility by `data-testid` | `await expect(page.getByTestId('orders-table')).toBeVisible()` |
| CSS state / attribute | `await expect(checkbox).toBeChecked()` |
| Data attribute | `await expect(row).toHaveAttribute('data-order-id', '12345')` |

| Forbidden | Why | Fix |
|-----------|-----|-----|
| `expect(el).toHaveText('Orders')` | Translated heading text | Assert by URL, `data-testid`, or structure |
| `expect(el).toContainText('Save')` | Translated button label | Assert by `data-testid` or action result |
| `expect(flash).toHaveText('Successfully saved')` | Translated success message | Assert by flash message `data-testid` + state change (reload and verify data persisted) |

---

## Meaningful Assertions

Each test **must** include at least one of:

| Assertion Type | What It Proves | Example |
|----------------|----------------|---------|
| **Data verification** | Displayed data matches expected values | `await expect(input).toHaveValue('10041')` |
| **State change** | An action changes system state | Fill form → submit → reload → verify values persisted |
| **Navigation** | Action leads to correct destination | `await expect(page).toHaveURL(/\/account\/orders/)` |
| **Error handling** | Invalid input produces correct feedback | Submit invalid data → verify error element visible via `data-testid` |

---

## Anti-Patterns — Tests That MUST NOT Be Accepted

| Pattern | Why It's Bad | Fix |
|---------|-------------|-----|
| Visibility-only: `expect(el).toBeVisible()` alone | Does not test functionality — element could show wrong data | Add content assertion: `expect(input).toHaveValue(...)` |
| Count-only: `expect(items).toHaveCount(N)` | Proves elements exist but not their content | Add content check for at least one item |
| Navigation-only: go to page + screenshot | No assertion on page content or functionality | Add assertions on key page elements and data |
| Smoke-only: check page loads without errors | Too shallow — catches only crashes | Add assertions on the feature the page provides |
| Text-based assertion on translated UI | Breaks when language changes | Use URL, attribute, value, or structure assertions |

---

## Minimum Quality by Page Type

| Page Type | Required Assertions |
|-----------|-------------------|
| **Form** | Fill all fields → submit → verify success feedback → verify data persisted (reload or check values) |
| **Table** | Verify row count > 0 → verify cell content of at least one row (by attribute or value, not text) → verify column count matches expected |
| **CRUD** | Create with specific data → verify created item appears with correct data → modify → verify changes → delete → verify removed |
| **Search** | Enter query → verify results contain expected item → verify result count > 0 → verify result content |
| **Dashboard** | Verify each section has real data (not empty, not placeholder — check by value/attribute/count) |

---

## Visual Regression Testing (VRT)

- VRT (e.g., `toHaveScreenshot`) is a **supplement**, not a replacement for functional assertions
- **Always add functional assertions BEFORE the screenshot assertion**
- Screenshot name convention: `feature-area/descriptive-name.png`

```typescript
// GOOD — functional assertions first, VRT as supplement
await expect(page.getByTestId('orders-table')).toBeVisible();
await expect(page.getByTestId('orders-table').locator('tbody tr')).not.toHaveCount(0);
await expect(page).toHaveScreenshot('my-account/orders-list.png');

// BAD — VRT without functional assertions
await expect(page).toHaveScreenshot('orders.png');
```

---

## Test Data Rules

1. **Never hardcode test data in test files** — use test data files (e.g., `test-data/*.data.ts`)
2. Every data file exports **typed** constants or arrays
3. If the test needs data that doesn't exist:
   - Ask the user to provide the data
   - Add it to the appropriate data file
   - Then write the test
4. Environment-specific data uses environment variables
5. Document data dependencies in test file comments when they are non-obvious
6. For data created during tests, use unique identifiers (e.g., `E2E-ADDR-${Date.now()}`) to avoid conflicts

---

## Test Independence

1. Each test must be **runnable in isolation** — executing a single test must work
2. Use test framework setup hooks (e.g., `test.beforeEach()`) for shared setup (login, navigation to target page)
3. **Never rely on state from a previous test** — no shared mutable state between tests
4. Clean up created data when possible, or use unique identifiers to avoid conflicts
5. Tests must not depend on execution order within a describe block

---

## Test Structure

```typescript
test.describe('Feature Name', () => {
    // Shared setup
    test.beforeEach(async ({ page }) => {
        // Login, navigate to page
    });

    test('should [business-level description]', async ({ page }) => {
        // Arrange — set up preconditions (if any beyond beforeEach)

        // Act — perform the user action

        // Assert — verify the business outcome (language-independent)
    });
});
```

### Naming Convention

- `test.describe`: Feature or page name (e.g., `'Customer Calculation'`, `'Shipping Address'`)
- `test`: Starts with `should` + business-level description (e.g., `'should display order history with correct data'`)
- Do not describe implementation details in test names (e.g., avoid `'should click the button and check the div'`)

---

## Code Organization

1. **Raw selectors never appear in test files** — they belong in page objects or components
2. **Page objects contain:**
   - Locator properties (all selectors for the page — using `data-testid` as priority)
   - Action methods (click, fill, navigate)
   - Assertion methods (expectLoaded, expectVisible, expectDataCorrect)
3. **Test files contain:**
   - Setup (beforeEach)
   - Business-level steps using page object methods
   - High-level assertions
4. **Components contain:**
   - Reusable UI elements shared across pages (login, flash messages, cookie banner, navigation)

---

## Checklist Before Submitting a Test

- [ ] Test answers a business question, not just "does element exist?"
- [ ] At least one data/state/navigation/error assertion present
- [ ] No assertion depends on translated UI text
- [ ] No raw selectors in the test file
- [ ] Test data comes from test data files, not hardcoded
- [ ] Test runs independently (no dependency on other tests)
- [ ] Test passes when executed
- [ ] Selectors follow the priority hierarchy from `selector-strategy.md`
- [ ] VRT screenshots have functional assertions before them

## Artifacts

None — this file contains reference standards only.
