# Discovery — Auto-Detection and Question Catalogue

## Purpose

Gather information needed to build project configuration files (`project/context.md`, `project/tech-spec.md`, `project/environments.md`). Combines automatic detection with user questions.

## Prerequisites

- Access to the project root filesystem
- Git repository initialized
- User available for answering questions

## Instructions

### How It Works

```
1. Run auto-detection (scan project files)
2. Build a draft of answers from detected information
3. Present draft to user for confirmation/correction
4. Ask remaining questions that couldn't be auto-detected
5. Generate project files from templates and final answers
```

**Principle:** Auto-detect what you can, ask only what you must.

---

### Part 1: Auto-Detection Rules

Scan the project root for these files and extract information. If a file doesn't exist, skip it — the information will be gathered from user questions.

#### Language and Framework

| Scan file | What to extract |
|-----------|----------------|
| `composer.json` | PHP version (`require.php`), framework (symfony, laravel, etc.), packages |
| `package.json` | Node.js version (`engines`), framework (react, vue, next, angular), dev tools |
| `go.mod` | Go version, module name, dependencies |
| `Cargo.toml` | Rust edition, dependencies |
| `Gemfile` | Ruby version, framework (rails, sinatra), gems |
| `requirements.txt` / `pyproject.toml` / `Pipfile` | Python version, framework (django, flask, fastapi) |
| `build.gradle` / `pom.xml` | Java/Kotlin version, framework (spring, quarkus) |
| `.tool-versions` / `.node-version` / `.python-version` | Runtime versions |

#### Frontend

| Scan file | What to extract |
|-----------|----------------|
| `package.json` | CSS framework (tailwind, bootstrap), bundler (webpack, vite, esbuild) |
| `tailwind.config.*` | Tailwind version and config |
| `vite.config.*` / `webpack.config.*` | Build tool details |
| `tsconfig.json` | TypeScript usage and config |

#### Infrastructure

| Scan file | What to extract |
|-----------|----------------|
| `docker-compose.yml` / `docker-compose.yaml` | Services (database, cache, queue, etc.) |
| `Dockerfile` | Base image, runtime |
| `.env.example` / `.env.dist` | Service dependencies (DB host, Redis URL, etc.) |

#### QA Tools

| Scan file | What to extract |
|-----------|----------------|
| `composer.json` (`require-dev`) | PHP QA tools (phpunit, phpstan, phpcs, etc.) |
| `package.json` (`devDependencies`) | JS QA tools (jest, vitest, eslint, prettier, etc.) |
| `phpunit.xml` / `phpunit.xml.dist` | Test configuration |
| `phpstan.neon` / `phpstan.neon.dist` | Static analysis config |
| `.eslintrc*` / `eslint.config.*` | Linter config |
| `jest.config.*` / `vitest.config.*` | Test framework config |
| `Makefile` / `justfile` | Custom QA commands |
| `tools/` / `scripts/` | Project-specific QA scripts |

#### Version Control

| Scan | What to extract |
|------|----------------|
| `git remote -v` | Platform (github.com, gitlab.com, bitbucket.org) |
| `git branch --show-current` | Current branch |
| Default branch (`git symbolic-ref refs/remotes/origin/HEAD`) | Main branch name |
| `.github/` directory | GitHub Actions CI/CD |
| `.gitlab-ci.yml` | GitLab CI/CD |

#### Project Description

| Scan file | What to extract |
|-----------|----------------|
| `README.md` | Project name, description (first paragraph) |
| `composer.json` → `description` | Package description |
| `package.json` → `description` | Package description |

#### External Tool Access (MCP)

Check if the agent has MCP servers configured that provide access to external tools. This determines whether the user needs to provide connection details manually.

| Check | What it means |
|-------|---------------|
| Issue tracker MCP available (Jira, Linear, etc.) | Agent can discover project keys, read/update tickets — skip manual URL/key questions |
| Documentation MCP available (Confluence, Notion, etc.) | Agent can access docs directly — note in context.md |
| Communication MCP available (Slack, etc.) | Agent can access channels — note in context.md |

If MCP access is detected, auto-populate the relevant answers and present them for confirmation. If no MCP is detected, fall back to manual questions.

---

### Part 2: Question Catalogue

After auto-detection, ask the user about anything that couldn't be determined. Present auto-detected values as defaults — the user confirms or corrects.

#### Category A: Project Identity

These questions build `project/context.md`:

| # | Question | Auto-detect source | Example answer |
|---|----------|-------------------|----------------|
| A1 | What is the project name? | README.md, package.json `name` | "AI Lab Flow" |
| A2 | Brief project description (1-2 sentences)? | README.md first paragraph | "AI experimentation platform" |
| A3 | What is the project key for task numbering? | **Cannot auto-detect** — always ask | "ALF" |
| A4 | What is the main branch name? | git symbolic-ref | "main" |

#### Category B: Workflow Tools

These questions build the "Workflow Tools" section of `project/context.md`:

| # | Question | Auto-detect source | Options |
|---|----------|-------------------|---------|
| B1 | Version control platform? | git remote URL | GitHub / GitLab / Bitbucket / Other |
| B2 | Task management tool? | MCP detection or ask | Jira / Linear / GitHub Issues / Files in .ai/prd/ / Other |
| B3 | If Jira/Linear: how is access configured? | MCP detection | **MCP available** → confirm auto-detected project key / **No MCP, will configure later** → note in context.md / **Manual** → ask for project URL or key |
| B4 | Communication tools? | MCP detection or ask | Slack / Teams / Discord / None |

#### Category C: Technology Stack

These questions build `project/tech-spec.md`:

| # | Question | Auto-detect source | Example answer |
|---|----------|-------------------|----------------|
| C1 | Primary backend language and version? | composer.json, package.json, go.mod, etc. | "PHP 8.4" |
| C2 | Backend framework and version? | Same dependency files | "Symfony 8.0" |
| C3 | Frontend framework? | package.json | "Stimulus + Turbo (Hotwire)" |
| C4 | CSS framework? | package.json, tailwind config | "Tailwind CSS 4.x" |
| C5 | Database engine? | docker-compose, .env.example | "MySQL 8.3" |
| C6 | ORM / data access? | dependency files | "Doctrine ORM 3.6" |
| C7 | Cache system? | docker-compose, .env.example | "Redis" |
| C8 | Queue system? | docker-compose, dependency files | "RabbitMQ" / "None" |
| C9 | QA tools (tests, linter, static analysis)? | dependency files, config files | "PHPUnit, PHPStan, PHPCS" |
| C10 | Project directory structure? | `ls src/` or equivalent | Auto-detected from filesystem |

#### Category D: Environments

These questions build `project/environments.md`:

| # | Question | Auto-detect source | Example answer |
|---|----------|-------------------|----------------|
| D1 | What environments exist? | **Cannot auto-detect** — always ask | "PROD, Stage, Dev, Local" |
| D2 | How to access each environment? | **Cannot auto-detect** — always ask | "PROD: ssh user@ip-prod" |
| D3 | Application path on each server? | **Cannot auto-detect** — always ask | "/var/www/app" |
| D4 | Log locations per environment? | Common conventions (var/log/) | "/var/www/app/var/log/" |
| D5 | Database access method per environment? | docker-compose, .env.example | "SSH tunnel + credentials from .env" |
| D6 | Other services per environment (cache, queue)? | docker-compose | "Redis localhost:6379" |

#### Category E: Conventions (always ask)

These cannot be auto-detected and always require user input:

| # | Question | Purpose |
|---|----------|---------|
| E1 | Any specific coding conventions beyond the standard? | Extend coding-standards.md if needed |
| E2 | Any specific testing conventions? | Extend testing-rules.md if needed |
| E3 | Any additional integrations or services? | Add to context.md |

---

### Part 3: Presentation Format

After auto-detection, present findings to the user as:

```
## Auto-Detected Configuration

### Project
- Name: [detected or "?"]
- Description: [detected or "?"]
- Platform: [GitHub/GitLab/...] (detected from git remote)
- Main branch: [detected]

### Technology Stack
- Language: [detected]
- Framework: [detected]
- Frontend: [detected or "none detected"]
- Database: [detected or "?"]
- Cache: [detected or "?"]
- QA Tools: [detected]

### External Tools (MCP)
- Issue tracker: [detected via MCP / not detected]
- Documentation: [detected via MCP / not detected]
- Communication: [detected via MCP / not detected]

### Still Needed (please answer)
- Project key for task numbering: ?
- Task management tool: [auto-detected via MCP or "?"]
- Communication tools: [auto-detected via MCP or "?"]
- [any other unresolved questions]
```

The user confirms/corrects auto-detected values and provides missing answers. Then proceed to generate files.

---

### Part 4: File Generation

After all answers are collected, generate files using templates from `meta/templates/`:

1. **`project/context.md`** — use `meta/templates/context.md`, fill in with answers from categories A, B, E
2. **`project/tech-spec.md`** — use `meta/templates/tech-spec.md`, fill in with answers from categories C, E
3. **`project/environments.md`** — use `meta/templates/environments.md`, fill in with answers from category D
4. **`prd/prd.md`** — empty task index with the correct `{PROJECT_KEY}`
5. **`prd/task-status.local.md`** — empty status tracking file

After generation, run the audit (`meta/audit.md`) to verify completeness.

## Artifacts

- `project/context.md` — project identity, workflow tools, collaboration expectations
- `project/tech-spec.md` — technology stack, QA tools, project structure
- `project/environments.md` — environment matrix, access methods, service addresses
- `prd/prd.md` — empty task index
- `prd/task-status.local.md` — empty status tracking file
