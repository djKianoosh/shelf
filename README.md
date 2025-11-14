# Shelf CLI

`shelf` is a command-line interface (CLI) utility designed to help developers manage the context provided to Large Language Model (LLM) clients. It allows you to declaratively control which files are ignored, ensuring your LLM focuses on the most relevant parts of your codebase.

## Project Goal

To provide a robust, ergonomic CLI tool in Rust that enables developers to quickly scope their LLM's context to specific parts of a large monorepo or project, improving relevance and reducing noise.

## Core Functionality: Declarative Profiles

The tool's behavior is defined by a YAML configuration file, `.shelf.yaml`, located in the project root. This file defines named "profiles" that specify which files to include or exclude from the LLM's context.

### `.shelf.yaml` Configuration Structure

*   **`global` (Optional):** A top-level key for patterns that should be excluded from the context *regardless of the active profile*.
*   **`<profile_name>`:** A unique name for a context profile (e.g., `frontend`, `backend`).
    *   **`description` (Optional):** A short, human-readable description of the profile's purpose.
    *   **`includes`:** A list of directories or files to be **included** in the context.
    *   **`excludes`:** A list of directories or files to be **excluded**, which can override patterns in the `includes` list.

#### Example `.shelf.yaml`

```yaml
# Global exclusions apply to all profiles.
global:
  excludes:
    - '**/tmp/'
    - '*.log'
    - 'legacy/'
    - '**/node_modules/'
    - '**/.DS_Store'
    - '**/target/'
    - '**/*.pyc'
    - '**/__pycache__/'

# Profile for backend development on all services
backend:
  description: "Focus on all backend services."
  includes:
    - 'services/'
    - 'packages/api-client/'
    - '.gitlab-ci.yml'
  excludes:
    - 'services/*/tests/frontend/'

# Profile for working on just the main frontend application
frontend:
  description: "Scope to the main web application and its shared UI components."
  includes:
    - 'web/main-app/'
    - 'packages/ui-components/'
  excludes:
    - 'web/main-app/dist/'

# Profile for a full-stack developer working on a specific feature
feature-slice:
  description: "Full-stack context for a feature (orders service and main app)."
  includes:
    - 'services/order-service/'
    - 'web/main-app/'
    - 'packages/'
  excludes:
    - 'services/order-service/tmp/'

# Profile for technical writers
tech-docs:
  description: "Context for writing documentation."
  includes:
    - 'docs/'
    - 'database/schema.sql'
    - 'README.md'
```

## CLI Specification

### `shelf list`

Lists all available profiles defined in `.shelf.yaml`.

**Usage:**

```bash
shelf list
```

**Output:**

```
Available profiles:
• backend      : Focus on all backend services.
• feature-slice: Full-stack context for a feature (orders service and main app).
• frontend     : Scope to the main web application and its shared UI components.
• tech-docs    : Context for writing documentation.
```

### `shelf status`

Shows the currently active profile and any user-defined patterns in `.geminiignore`.

**Usage:**

```bash
shelf status
```

**Output (Profile Active with User Patterns):**

```
Profile 'frontend' is active.

User-defined patterns:
• .env
• *.swo
```

**Output (No Profile Active, with User Patterns):**

```
No shelf profile is active.

User-defined patterns:
• .env
• *.swo
```

**Output (No Profile Active, No User Patterns):**

```
No shelf profile is active.
```

**Output (No `.geminiignore` file):**

```
No .geminiignore file found.
```

### `shelf enable <profile_name>`

Activates a profile, modifying the `.geminiignore` file to reflect the profile's include and exclude rules.

**Usage:**

```bash
shelf enable frontend
```

**Output:**

```
✔ Activated profile 'frontend'. .geminiignore updated.
```

### `shelf disable`

Deactivates any active profile by clearing the shelf-managed block in the `.geminiignore` file.

**Usage:**

```bash
shelf disable
```

**Output:**

```
✔ All shelf profiles disabled. .geminiignore updated.
```

## Installation (Coming Soon)

Instructions on how to install `shelf` will be provided here.

## Development (Coming Soon)

Information for contributors will be provided here.
