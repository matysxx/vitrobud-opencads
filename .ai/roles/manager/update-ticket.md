# Update Ticket

## Purpose

Procedure for adding comments and changing statuses on issue tracker tickets.

> Concrete tool names, project keys, and issue tracker details come from `project/context.md`. This file describes the process only.

## Prerequisites

- `project/context.md` — issue tracker type, project key (`{PROJECT_KEY}`), MCP availability
- MCP tools for the issue tracker must be available (see Fallback section if not)

## Trigger

- **On-demand**: user asks to update a ticket (add comment, change status, transition)
- **From flow**: called by `close-task.md` (after PR) or `create-task-from-ticket.md` (starting work)

## Steps

1. **Determine ticket number** — from user message, current branch name, or task directory in `prd/`
2. **Check MCP availability** — verify that MCP tools for the issue tracker are available
3. **Read the ticket** — current status, existing comments
4. **Gather context** — Git branch, PR URL, requirements from `prd/`
5. **Determine scenario** — from trigger or user intent (see Scenario Mapping below)
6. **Compose comment** — following the Comment Structure below
7. **Validate transition** — check available transitions via the issue tracker API (if transition planned)
8. **Present to user for approval**:
   - Show full comment text
   - Show proposed transition (if any)
   - Ask for explicit confirmation
9. **On approval** — post comment and perform transition
10. **On rejection** — skip (do not post or transition)

## Approval Requirement

**The agent must NEVER post a comment or change a ticket status without explicit user approval.**

Before executing, always present:
- The full comment text that will be posted
- The proposed status transition (if any)

If the user modifies the text or transition — apply changes and re-confirm.

## Comment Structure

### Components

| Component | Required | Purpose |
|-----------|----------|---------|
| `Status:` | Yes | What was done |
| `Next Steps:` | If applicable | What comes next |
| `Clarification:` | Optional | Context provided or questions asked |

### Action Marks

Prefix each action line with a mark:

| Mark | Meaning |
|--|---------|
| ✅| Done |
| ⚠️| Done with caveats |
| ⛔| Blocked or not done |


### Actions Vocabulary

| Action | When |
|--------|------|
| Implemented | Feature or change is coded |
| Fixed | Bug fix applied |
| Adjusted / Improved | Minor change or enhancement |
| PR: {url} | Pull request created (include link) |
| Review | Code review needed or completed |
| Approved | PR approved |
| Merged & Deployed to {env} | Code merged and deployed |
| QA {env} by author | Author-side QA on environment |
| QA {env} | QA needed or completed on environment |

### Environments

Use environment names from `project/context.md`. Common conventions:

| Env | Description |
|-----|-------------|
| Dev | Development environment |
| Stage | Staging / pre-production |
| Prod | Production |

## Clarification — Two Modes

### 1. Providing Context

Use when explaining an implementation decision, naming change, or deviation from the original ticket:

```
**Clarification:**
Renamed command from `sync:emails` to `email-template:sync` for consistency with existing commands.
```

### 2. Requesting Clarification

Use when asking the stakeholder a question. Consider transitioning to a feedback/blocked status if the question blocks further work:

```
**⚠️Clarification Needed:**
The acceptance criteria mention "all templates" but the module only handles mail templates, not CMS pages. Please confirm scope.
```

## Status Transitions

### Typical Flow

| From | To | When |
|------|----|------|
| READY | IN PROGRESS | Work starts |
| IN PROGRESS | DEV QA | Implementation done, PR created |
| DEV QA | READY FOR STAGE | Dev QA passed |
| READY FOR STAGE | INTERNAL TESTING | Deployed to Stage |
| INTERNAL TESTING | READY FOR TESTING | Stage QA passed |
| Any | FEEDBACK REQUIRED | Clarification needed from stakeholder |

> **Important:** Always validate available transitions via the issue tracker API before attempting a transition. The actual available transitions depend on the ticket's current status and the project's workflow configuration.

## Scenario Mapping

Each scenario combines: trigger, comment, and optional transition.

### Scenario 1 — Starting Work

- **Trigger**: `create-task-from-ticket.md` or user starts working on a ticket
- **Transition**: READY → IN PROGRESS
- **Comment**:
  ```
  **Status:**
  * ✅Work started
  ```

### Scenario 2 — Implementation + PR

- **Trigger**: `close-task.md` after PR creation, or user requests
- **Transition**: IN PROGRESS → DEV QA
- **Comment**:
  ```
  Status:
  * ✅Implemented
  * ✅PR: {pr-url}

  Next Steps:
  * Review
  * Merge & Deploy
  * QA Dev by author
  ```

### Scenario 3 — Implementation + PR + Clarification

- **Trigger**: same as Scenario 2, but implementation includes deviations or decisions worth noting
- **Transition**: IN PROGRESS → DEV QA
- **Comment**:
  ```
  **Status:**
  * ✅Implemented
  * ✅PR: {pr-url}

  **Next Steps:**
  * Review
  * Merge & Deploy
  * QA Dev by author

  **Clarification:**
  {explanation of decision or deviation}
  ```

### Scenario 4 — PR Approved

- **Trigger**: user reports PR was approved
- **Transition**: none
- **Comment**:
  ```
  **Status:**
  ✅Approved

  **Next Steps:**
  * Merge & Deploy
  * QA Dev by author
  ```

### Scenario 5 — Merged to Dev & QA Dev by author

- **Trigger**: user reports merge to Dev & QA Dev by author
- **Transition**: none
- **Comment**:
  ```
  **Status:**
  * ✅Merged & Deployed to Dev
  * ✅QA Dev by author

  **Next Steps:**
  * QA Dev
  * Status: READY FOR STAGE
  ```

### Scenario 6 — Dev QA Passed

- **Trigger**: user reports Dev QA passed
- **Transition**: DEV QA → READY FOR STAGE
- **Comment**:
  ```
  **Status:**
  * ✅QA Dev

  **Next Steps:**
  * Merge ({version}) & Deploy to Stage
  ```

### Scenario 7 — Deployed to Stage

- **Trigger**: user reports deployment to Stage
- **Transition**: READY FOR STAGE → INTERNAL TESTING
- **Comment**:
  ```
  **Status:**
  ✅Merged ({version}) & Deployed to Stage

  -- no next steps needed, we manage it via ticket statuses --
  ```

### Scenario 8 — Stage QA Passed

- **Trigger**: user reports Stage QA passed
- **Transition**: INTERNAL TESTING → READY FOR TESTING
- **Comment**:
  ```
  **Status:**
  * ✅QA Stage

  -- no next steps needed, we manage it via ticket statuses --
  ```

### Scenario 9 — Feedback Required

- **Trigger**: blocker found, question for stakeholder
- **Transition**: Any → FEEDBACK REQUIRED
- **Comment**:
  ```
  **⚠️Clarification Needed:**
  {question or blocker description}
  ```

### Scenario 10 — Custom

- **Trigger**: user defines custom comment and/or transition
- **Transition**: as specified by user
- **Comment**: as specified by user

## Fallback — MCP Unavailable

When MCP tools for the issue tracker are not available:

1. Save the intended comment to `prd/task-status.local.md`:
   ```
   ## {PROJECT_KEY}-{N} — Pending Ticket Update
   **Intended comment:**
   {comment text}
   **Intended transition:** {from} → {to} (or "none")
   **Reason not posted:** MCP unavailable
   ```
2. Inform the user that the update was saved locally and needs to be posted manually.

## Checklist

```
[ ] Ticket number determined
[ ] Issue tracker MCP available (or fallback used)
[ ] Current ticket status read
[ ] Context gathered (Git, PR, requirements)
[ ] Scenario determined
[ ] Comment composed
[ ] Transition validated via API (if applicable)
[ ] User approved comment and transition
[ ] Comment posted (or saved to fallback)
[ ] Transition executed (or skipped)
```
