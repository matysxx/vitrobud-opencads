# Selector Strategy

## Purpose

Rules for choosing element selectors in E2E tests. Ensures language independence and selector stability.

## Prerequisites

- `e2e-tester.md` — role entry point
- `e2e-methodology.md` — methodology context

## Cardinal Rule: Language Independence

> **No selector or assertion may depend on translated UI text.**

The following locator patterns are **forbidden** when the text value comes from translations:

| Forbidden Pattern | Why |
|-------------------|-----|
| `getByRole('button', { name: 'Save' })` | `name` is a translated label |
| `getByText('Orders')` | Translated visible text |
| `getByLabel('First Name')` | Translated label text |
| `getByPlaceholder('Search ...')` | Translated placeholder |
| `.filter({ hasText: 'Export' })` | Translated filter text |

**Allowed alternatives:**

| Pattern | Why It's Stable |
|---------|-----------------|
| `getByTestId('orders-search')` | `data-testid` — never translated |
| `locator('[name="fieldName"]')` | HTML `name` attribute — not translated |
| `locator('[data-action="save"]')` | Data attribute — not translated |
| `locator('#elementId')` | Stable, non-generated ID — not translated |
| `getByRole('button')` without `name` | Role only — no text dependency (must be unambiguous in scope) |

**If no language-independent selector exists → add `data-testid` to the template.**

---

## Priority Hierarchy

Use the **highest available** option. Move down only when the higher option is not possible.

| Priority | Selector | When to Use |
|----------|----------|-------------|
| 1 | `page.getByTestId('id')` | Most stable. Use when `data-testid` exists or can be added |
| 2 | `page.locator('[name="fieldName"]')` | Form inputs by `name` attribute — stable, language-independent |
| 3 | `page.locator('[data-attribute="value"]')` | Custom data attributes (e.g., `data-id`, `data-action`) |
| 4 | `page.locator('#elementId')` | Stable, non-generated HTML `id` attributes |
| 5 | `page.getByRole('role')` | Semantic role **without `name`** — only when the element is unique in its scope. Document why. |
| 6 | `page.locator('.semantic-class')` | CSS classes that are part of the component API (e.g., `.alert-success`, `.offcanvas-cart`) |
| 7 | Complex CSS selectors | **LAST RESORT** — document WHY no better option exists |

---

## Adding `data-testid` Attributes

When an element lacks a stable, language-independent selector:

1. **Identify the template file** — find the template file that renders the element
2. **Choose a descriptive name** following the naming convention below
3. **Add the attribute** to the correct HTML element in the template
4. **Use it** in the page object: `page.getByTestId('orders-search')`

### Naming Convention for `data-testid`

| Pattern | Example | Use When |
|---------|---------|----------|
| `{feature}-{element}` | `orders-search` | Unique element on the page |
| `{feature}-{section}-{element}` | `account-email-save` | Element within a named section |
| `{feature}-{action}` | `shipping-address-create` | Action button/link |
| `{feature}-{element}-{qualifier}` | `orders-date-from` | Multiple similar elements needing distinction |
| `{feature}-{element}-{id}` | `services-row-{id}-delete` | Row-level actions with dynamic IDs |

### Rules

- Names are **lowercase, hyphen-separated**
- Names describe **what the element is**, not how it looks
- The `{feature}` prefix groups all `data-testid` values for one page/area
- Dynamic IDs (e.g., row IDs) use template interpolation: `data-testid="services-row-{{ id }}-delete"`

---

## Anti-Patterns — NEVER Use

| Pattern | Why It Breaks |
|---------|---------------|
| Text-based selectors (`getByText`, `getByLabel`, `getByRole` with `name`) | Breaks when language changes — see Cardinal Rule above |
| `nth-of-type()`, `nth-child()` | Breaks when DOM order changes (items added/removed/reordered) |
| Deep descendant chains (`.a > .b > .c > .d`) | Extremely fragile — any template restructuring breaks it |
| Layout/styling classes (`.col-md-6`, `.mt-3`, `.d-flex`, `.row`) | Styling changes break tests without any functional change |
| Generated class names or IDs | Framework may regenerate them at any time |
| XPath | Harder to read and maintain; Playwright discourages it |
| IDs that look auto-generated (e.g., `#field-1234`) | Not stable across builds or environments |

---

## Locator Composition

### Prefer Chaining Over Complex CSS

```typescript
// GOOD — scoped lookup using data-testid
page.getByTestId('shipping-address-table').locator('tbody tr');

// BAD — fragile CSS chain
page.locator('.shipping-table > .inner-wrapper > table > tbody > tr');
```

### Use `filter()` for Conditional Matching (Language-Independent)

```typescript
// GOOD — filter by data attribute or structure
page.getByTestId('orders-table').locator('tr').filter({
    has: page.locator('[data-order-id="123"]')
});

// BAD — filter by translated text
page.getByRole('row').filter({ hasText: 'Order #123' });
```

### Use `nth()` for Indexed Access (When Necessary)

```typescript
// ACCEPTABLE — when you need a specific item from a list
page.getByTestId('orders-table').locator('tbody tr').nth(0);

// BAD — CSS pseudo-selector
page.locator('li:first-child');
```

### Scope Lookups to a Container

```typescript
// GOOD — scoped to a test-id container
const form = page.getByTestId('calculation-form');
const roundingSelect = form.locator('select[name="rounding"]');

// BAD — global selector that may match multiple elements
page.locator('select[name="rounding"]');
```

---

## Selector Source: Always from `browser_snapshot`

- **Never guess selectors** from template source code alone
- **Always verify** selectors against the live accessibility tree from `browser_snapshot`
- The snapshot shows the actual rendered DOM, including dynamically added elements
- If the snapshot shows a `data-testid` — use `getByTestId` (priority 1)
- If the snapshot shows a `name` attribute — use `locator('[name="..."]')` (priority 2)
- If no stable selector exists — plan to add `data-testid` (see "Adding `data-testid` Attributes" above)

---

## Naming Conventions for Locator Properties

In page objects, name locator properties descriptively:

```typescript
// GOOD — describes what the element IS, uses data-testid
this.saveButton = page.getByTestId('calculation-save');
this.roundingSelect = page.getByTestId('calculation-rounding');
this.searchInput = page.getByTestId('orders-search');
this.addressTable = page.getByTestId('shipping-address-table');

// ACCEPTABLE — stable name attribute when data-testid is not yet available
this.roundingSelect = page.locator('select[name="rounding"]');

// BAD — translated text or implementation details
this.saveButton = page.getByRole('button', { name: 'Save' });
this.btn1 = page.locator('.btn-primary');
this.select = page.locator('select');
```

## Artifacts

None — this file contains reference rules only.
