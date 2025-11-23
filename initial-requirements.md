Generate a production‑grade Rust workspace that implements an autonomous, multi‑agent Test‑Driven Development machine for code katas. The tool must run locally as a CLI, orchestrate three agents (tester, implementor, refactorer), and follow a strict red‑green‑refactor loop for a configurable number of steps. It must store state in git and allow each agent to read the last commit message, the last git diff, and the entire working tree.

## Objectives

* Create a Rust workspace with clean boundaries, strong types, and testable modules.
* Implement an orchestrator that cycles over agents: tester → implementor → refactorer → implementor → … for N steps.
* Each agent must run tests and compile checks, and must be able to edit the codebase across multiple files and modules.
* Persist progress via conventional commits. Commit messages must include all context needed by the next agent.
* Consume a kata description from a Markdown file. Agents should align their actions to that document.
* Support pluggable LLMs through OpenAI‑compatible APIs for each role, configurable per role.

## Non‑Goals (for the initial version)
* No GUI. CLI only.
* No remote execution or containers. Local process execution is fine.
* No patch‑file handoffs. Handoff happens by reading the repo state and git history.

## Workspace Layout

Create a cargo workspace with these crates:

```
/ Cargo.toml                # workspace table
/ crates/
  tdd-cli/                  # binary, user entrypoint
  tdd-core/                 # domain model, orchestrator, traits, commit policy
  tdd-agents/               # role implementations that call the LLMs
  tdd-exec/                 # test runner, git, fs, process utilities
  tdd-llm/                  # client and adapters for OpenAI‑compatible providers
  tdd-fixtures/             # sample katas and tests for e2e validation (dev-dependency)
```

## Key Dependencies

* `clap` for CLI.
* `serde`, `serde_yaml`, `serde_json` for config and logs.
* `tokio` for async when calling LLMs.
* `reqwest` for HTTP.
* `git2` for git operations.
* `tempfile`, `anyhow`, `thiserror` for ergonomics.
* `walkdir`, `ignore` for repo scanning.
* `which`, `duct` or `tokio::process` for process execution.

## Config File

Implement `tdd.yaml` at repo root. Example:

```yaml
kata_description: "path/to/kata-description.md"          # path to markdown
language: "rust"                     # used by runner to select commands
steps: 20
max_attempts_per_agent: 5
roles:
  tester:
    model: "openai:gpt-4.1-mini"
    temperature: 0.4
  implementor:
    model: "deepseek:coder-v2"
    temperature: 0.2
  refactorer:
    model: "glm:glm-4-air"
    temperature: 0.3
llm:
  base_url: "http://localhost:11434/v1"  # OpenAI‑compatible
  api_key_env: "LLM_API_KEY"
ci:
  test_cmd: ["cargo", "test", "--all"]
  check_cmd: ["cargo", "clippy", "--all", "--", "-D", "warnings"]
  fmt_cmd: ["cargo", "fmt"]
commit:
  author_name: "TDD Machine"
  author_email: "tdd@local"
```

## CLI Surface

```
USAGE:
  tdd-cli init                 # initialize repo and scaffolding
  tdd-cli run --steps N        # run N full TDD steps
  tdd-cli step                 # run a single agent step (debug)
  tdd-cli status               # show current agent, step counter, last commit summary
  tdd-cli doctor               # verify tools, versions, environment
```

## Domain Model (in `tdd-core`)

Define these types and traits:

```rust
pub enum Role { Tester, Implementor, Refactorer }

pub struct StepContext {
    pub role: Role,
    pub step_index: u32,
    pub kata_description: String,
    pub git_last_commit_msg: String,
    pub git_last_diff: String,              // unified diff
    pub repo_snapshot_paths: Vec<String>,   // files in repo
}

pub struct StepResult {
    pub files_changed: Vec<String>,         // absolute or repo‑relative
    pub commit_message: String,             // conventional commit
    pub notes: String,                      // extra info for logs
}

#[async_trait::async_trait]
pub trait Agent {
    fn role(&self) -> Role;
    async fn plan(&self, ctx: &StepContext) -> anyhow::Result<String>;        // reasoning and change plan
    async fn edit(&self, ctx: &StepContext) -> anyhow::Result<StepResult>;    // apply edits to fs
}

pub trait Orchestrator {
    fn current_role(&self) -> Role;
    async fn next(&mut self) -> anyhow::Result<()>; // run a role step, commit, rotate role
}
```

## Orchestrator Rules

* Start with `Tester` when the repo is empty. `Tester` must initialize git and basic scaffold for the chosen language.
* Every step:

  1. Build `StepContext` from the working tree, last commit message, last diff, and the kata description.
  2. Call `Agent::plan` and persist the plan to `.tdd/plan/step-N-role.md` for traceability.
  3. Call `Agent::edit` that writes files to disk.
  4. Run `fmt`, `check`, then `test` via `tdd-exec`.
  5. If tests fail for Implementor or Refactorer, allow up to `max_attempts_per_agent` retries. Between retries the agent can undo with `git checkout .` or adjust edits.
  6. On success, create a conventional commit using `git2` with a detailed message.
  7. Rotate the role according to the red‑green‑refactor loop.

## Agent Behavior Contracts

All agents must follow the TDD cycle and respect the kata Markdown. They must not change public behavior during refactor.

### Tester

* If repo is uninitialized, create `Cargo.toml`, src layout, test module, and enable `rustfmt` and `clippy`.
* Write the smallest failing test that advances behavior in line with the kata.
* Verify the test fails before handing off.
* Commit `test:` with a message that explains the intent and the smallest behavior slice.

### Implementor

* Read last commit message and diff. Implement the minimal change to pass all tests.
* Prefer simplest design that works. Defer structure to refactorer.
* On failure, iterate up to `max_attempts_per_agent`. May reset the working tree between attempts.
* Commit `feat:` or `fix:` depending on the change.

### Refactorer

* Improve structure without changing observable behavior. May split files, extract modules, rename for clarity, improve API shapes, add types.
* Ensure tests keep passing.
* Commit `refactor:` with a detailed rationale.

## Conventional Commit Policy

Use the following format, with a rich body so the next agent has context.

```
<type>(scope): short summary

Context:
- Role: <Tester|Implementor|Refactorer>
- Step: <N>
- Kata goal: <one sentence from kata.md>

Rationale:
- Why this change right now
- For tests: what behavior is introduced and why
- For implementor: minimal implementation strategy
- For refactorer: structural improvements and intended flexibility

Diff summary:
- List key files touched and intent per file

Verification:
- Test results summary
```

## LLM Adapter (`tdd-llm`)

* Provide a generic `LlmClient` with `chat(messages: Vec<Message>) -> String`.
* Allow per‑role model, temperature, and provider selection via config.
* Support any OpenAI‑compatible base URL and API key.

## Process Runner and Git (`tdd-exec`)

* Utilities to run commands with timeouts, capture stdout and stderr, and map to a `Result<TestOutcome>`.
* `run_fmt`, `run_check`, `run_tests` with language‑aware defaults. Focus on Rust in v1.
* Git helpers: init repo, stage changes, read last commit message, compute diff, commit with author, rollback.

## File Editing Strategy

* Agents produce an edit plan as structured JSON:

```json
{
  "edits": [
    {"path": "src/lib.rs", "action": "upsert", "content": "..."},
    {"path": "src/game.rs", "action": "upsert", "content": "..."},
    {"path": "src/mod.rs", "action": "upsert", "content": "..."}
  ]
}
```

* The `tdd-agents` crate turns this plan into file system changes. Avoid patch files.

## Prompt Templates inside the Code

Embed a system prompt per role that is fed to the LLM along with `StepContext` and selected file snippets. Ensure the model obeys the file editing protocol.

### System prompt for Tester

```
You are the Tester in a TDD cycle for a Rust kata. Your responsibilities:
- Read the kata.md and propose the smallest meaningful test that advances behavior.
- Write or update tests only. Do not implement production code.
- Tests must compile and be focused on one behavior slice.
- Provide a JSON edit plan with files and full contents, not a diff.
- After writing the test, ensure it fails when run against current code.
- Produce a conventional commit message with the `test:` type.
```

### System prompt for Implementor

```
You are the Implementor in a TDD cycle for a Rust kata. Your responsibilities:
- Read the last commit message, the last diff, and the full tree.
- Implement the smallest change that makes all tests pass.
- Keep the design simple. You may add files, structs, modules.
- Provide a JSON edit plan with files and full contents.
- Produce a conventional commit message with `feat:` or `fix:`.
```

### System prompt for Refactorer

```
You are the Refactorer in a TDD cycle for a Rust kata. Your responsibilities:
- Improve structure and readability without changing behavior.
- You may reorganize modules, extract types, rename for clarity.
- Do not modify test assertions, only restructure code under test.
- Provide a JSON edit plan with files and full contents.
- Produce a `refactor:` commit message.
```

## Example Rust APIs to Scaffold

Implement these in `tdd-core` and `tdd-exec`:

```rust
pub struct RepoState {
    pub last_commit_message: String,
    pub last_diff: String,
    pub files: Vec<String>,
}

pub struct RunnerOutcome { pub ok: bool, pub stdout: String, pub stderr: String }

pub trait Runner {
    fn fmt(&self) -> anyhow::Result<RunnerOutcome>;
    fn check(&self) -> anyhow::Result<RunnerOutcome>;
    fn test(&self) -> anyhow::Result<RunnerOutcome>;
}

pub trait Vcs {
    fn init_if_needed(&self) -> anyhow::Result<()>;
    fn read_state(&self) -> anyhow::Result<RepoState>;
    fn stage_all(&self) -> anyhow::Result<()>;
    fn commit(&self, message: &str) -> anyhow::Result<String>; // returns commit id
}
```

## Initialization Behavior

* `tdd-cli init` should create a new cargo library with a `tests/` folder, `.gitignore`, `rust-toolchain.toml`, and preconfigured `clippy` and `fmt`.
* It should create `kata.md` placeholder and `tdd.yaml` with defaults.

## Status and Logging

* Persist JSON logs per step in `.tdd/logs/step-N-role.json` including plan, runner outputs, and commit id.
* Provide `tdd-cli status` to print the latest step summary and failing diagnostics if any.

## Acceptance Criteria

* Running `tdd-cli init` on an empty folder initializes a working Rust kata scaffold and a git repo.
* Running `tdd-cli run --steps 3` produces alternating commits from tester, implementor, refactorer, with passing tests after implementor and refactorer steps.
* Each commit follows the specified conventional commit format and includes context sections.
* Agents can create multiple files and modules. The tool compiles and runs tests at every step.
* Config supports different models per role and a custom OpenAI‑compatible base URL.

## Deliverables

* Fully compilable workspace with crates listed above.
* Unit tests for `tdd-core` and `tdd-exec` utilities.
* Example kata fixture such as String Calculator or Bowling, under `tdd-fixtures`, to validate the loop.
* A `README.md` describing usage, config, and architecture.

## Step‑by‑Step Tasks

1. Create the cargo workspace and crate skeletons with `Cargo.toml` files.
2. Implement `tdd-exec` for process runner and git.
3. Implement `tdd-llm` with a minimal OpenAI‑compatible client and per‑role routing.
4. Implement `tdd-core` domain types, traits, orchestrator, and commit policy.
5. Implement `tdd-agents` with the three role templates and JSON edit plan mechanism.
6. Implement `tdd-cli` with `init`, `run`, `step`, `status`, `doctor` commands.
7. Add `tdd-fixtures` with a sample kata and an e2e test that runs a few steps using a mocked LLM client.
8. Write the `README.md` with instructions.

## Quality Bar

* The code must compile with stable Rust and pass `cargo clippy -D warnings`.
* All public functions should have doc comments.
* Provide meaningful error messages and map process failures to structured errors.
* Keep modules small and cohesive. Favor traits and dependency injection for testability.

## Optional Future Enhancements

* Add per‑language runners, starting with Node and Python.
* Add a sandbox mode that runs agents in a temporary worktree and only commits on success.
* Add a risk budget that controls how many files a refactor is allowed to move in a single step.
* Add a guard that blocks refactor commits if test coverage drops.
