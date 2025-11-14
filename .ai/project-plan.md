### **Persona**

You are an expert senior software engineer specializing in creating robust, ergonomic command-line tools in Rust. You write clean, maintainable, and well-tested code, with a strong focus on excellent error handling and user experience.

### **1. Project Goal & User Stories**

You are to create a command-line interface (CLI) utility named `shelf`. This tool will help developers manage the context provided to the Gemini CLI by declaratively controlling which files are ignored via the .geminiignore file.

**User Stories:**
*   **As a developer working on a large monorepo,** I want to quickly scope my LLM's context to only the `frontend` project, so I get more relevant suggestions and avoid polluting the context with backend or documentation files.
*   **As a technical writer,** I want to activate an `api-docs` profile so my LLM client focuses on Markdown files, OpenAPI specs, and CI/CD configurations, ignoring all application source code.
*   **As a developer switching tasks,** I want to disable all profiles to reset my LLM's context to the repository's default state.

### **2. Core Functionality: Declarative Profiles**

The tool's behavior will be defined by a YAML configuration file, `.shelf.yaml`, located in the project root.

#### **2.1. `.shelf.yaml` Configuration Structure**

The file defines named "profiles" that specify which files to include or exclude.

*   **`global` (Optional):** A top-level key for patterns that should be excluded from the context *regardless of the active profile*.
*   **`<profile_name>`:** A unique name for a context profile (e.g., `frontend-work`, `api-docs`).
    *   **`description` (Optional):** A short, human-readable description of the profile's purpose.
    *   **`includes`:** A list of directories or files to be **included** in the context.
    *   **`excludes`:** A list of directories or files to be **excluded**, which can override patterns in the `includes` list.

##### **Example `.shelf.yaml`**
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

### **3. CLI Specification & Functional Requirements**

#### **3.1. `shelf enable <profile_name>`**

Activates a profile, modifying a `.geminiignore` file.

*   **Behavior:**
    1.  Search for `.shelf.yaml` and `.geminiignore` by traversing from the current directory up to the repository root. If `.geminiignore` doesn't exist, create it.
    2.  Locate or create the `shelf`-managed block, demarcated by `# --- SHELF START ---` and `# --- SHELF END ---`.
    3.  **Completely clear any existing content within this block.**
    4.  Write the following patterns into the block in this specific order to ensure correct precedence:
        a. A comment identifying the active profile: `# Profile: <profile_name>`
        b. A pattern to ignore files in the current directory: `*`
        c. For each pattern in the profile's `includes` list, add anchored negation patterns to re-include the desired files and directories. For a directory like `docs/`, this generates `!/docs` and `!/docs/**`. For a file, it generates an anchored pattern like `!/README.md`.
        d. All patterns from the profile's `excludes` list.
        e. All patterns from the `global.excludes` list.
*   **Success Output:**
    ```shell
    $ shelf enable frontend
    ✔ Activated profile 'frontend'. .geminiignore updated.
    ```

#### **3.2. `shelf disable`**

Deactivates any active profile.

*   **Behavior:** Clears all content from the `shelf`-managed block in `.geminiignore` but leaves the start and end markers intact.
*   **Success Output:**
    ```shell
    $ shelf disable
    ✔ All shelf profiles disabled. .geminiignore updated.
    ```

#### **3.3. `shelf status`**

Shows the currently active profile.

*   **Behavior:** Reads the `.geminiignore` file and parses the `# Profile: <name>` comment from the managed block.
*   **Output (Profile Active):**
    ```shell
    $ shelf status
    Profile 'frontend' is active.
    ```
*   **Output (No Profile Active):**
    ```shell
    $ shelf status
    No shelf profile is active.
    ```

#### **3.4. `shelf list`**

Lists all available profiles.

*   **Behavior:** Parses `.shelf.yaml` and lists the names and descriptions of all defined profiles.
*   **Output:**
    ```shell
    $ shelf list
    Available profiles:
    • backend:       Focus on all backend services.
    • frontend:      Scope to the main web application and its shared UI components.
    • feature-slice: Full-stack context for a feature (orders service and main app).
    • tech-docs:     Context for writing documentation.
    ```

### **4. Non-Functional Requirements**

#### **4.1. File System & `.geminiignore` Management**

*   The tool must only ever write to the area between the `# --- SHELF START ---` and `# --- SHELF END ---` markers. User-defined rules outside this block must remain untouched.
*   The tool must correctly handle standard `.gitignore`-style glob patterns.
*   It is assumed that the LLM client respects `.gitignore` files, so `shelf` does not need to manage common patterns like `node_modules` unless they fall within an `included` directory.

#### **4.2. Error Handling**

The tool must provide clear, helpful error messages.

*   **`.shelf.yaml` not found:**
    ```shell
    $ shelf list
    ✖ Error: .shelf.yaml not found in this directory or any parent directories.
    Please create a .shelf.yaml configuration file to define profiles.
    ```
*   **Profile not found:**
    ```shell
    $ shelf enable invalid-profile
    ✖ Error: Profile 'invalid-profile' not found in .shelf.yaml.
    Run 'shelf list' to see available profiles.
    ```
*   **Invalid YAML syntax:**
    ```shell
    $ shelf list
    ✖ Error: Failed to parse .shelf.yaml.
    Invalid YAML on line 5: could not find expected ':'
    ```

#### **4.3. Technology & Distribution**

*   **Language:** **Rust**.
*   **Dependencies:** The final product must be a **single, self-contained executable** to simplify distribution. Minimize external dependencies.

### **5. Development Workflow**

We will build the `shelf` utility iteratively, focusing on one command at a time. Each feature implementation will follow this structured, test-driven workflow loop:

1.  **Phase 1: Feature Breakdown & Planning**
    *   **Developer Prompt:** "Let's plan the `[command_name]` command."
    *   **Gemini Task:** Break down the requirements for the specific command into smaller, actionable steps. Define the required logic, error handling cases, and expected outcomes. Create a checklist for the feature and save it to `.ai/todo.md`.

2.  **Phase 2: Implementation (Test-Driven)**
    *   **Developer Prompt:** "Write a failing integration test for `[feature]`."
    *   **Gemini Task:**
        *   First, write a test case in the `tests/` directory that executes the command and asserts the expected outcome. This test should fail initially.
        *   Next, write the minimal Rust source code required to make the test pass.
    *   **Pause for Questions:** After writing the code, I will ask: "The tests are passing. Do you have any questions about the implementation before I proceed with refinement?"

3.  **Phase 3: Refinement & Explanation**
    *   **Developer Prompt:** "The code looks good. Please refine it and explain the Rust-specific concepts."
    *   **Gemini Task:**
        *   Run `cargo fmt` and `cargo clippy -- -D warnings` to ensure code quality and style.
        *   Analyze the code for correctness and suggest refactorings.
        *   Provide a detailed explanation of the implementation, focusing on Rust-specific idioms and concepts.

4.  **Phase 4: Documentation & Commit**
    *   **Developer Prompt:** "The `[command_name]` feature is complete. Let's document and commit it."
    *   **Gemini Task:**
        *   Update the main `README.md` with usage instructions for the new command.
        *   Generate a descriptive commit message following the Conventional Commits standard.

5.  **Phase 5: Reflection & Next Steps**
    *   **Gemini Task:** After each feature is committed, I will state: "The `[command_name]` feature is complete and has been committed. This is a good time to pause and reflect. Would you like to make any changes to our plan, or should I proceed with the next feature?"
