# TDD Agent

An autonomous, multi-agent Test-Driven Development machine for code katas.

## Features

- **Three Agents**: Tester, Implementor, Refactorer.
- **Strict TDD Loop**: Red -> Green -> Refactor.
- **Git Integration**: Persists state in git with conventional commits.
- **LLM Powered**: Uses OpenAI-compatible API (GPT-4, etc.) to generate code.
- **CLI**: Simple command-line interface.

## Installation

Build from source:

```bash
cargo build --release
cp target/release/tdd-cli /usr/local/bin/
```

## Usage

### 1. Initialize a new kata

```bash
mkdir my-kata
cd my-kata
tdd-cli init
```

This creates:
- `tdd.yaml`: Configuration.
- `kata.md`: Description of the kata.
- `Cargo.toml`: Rust project scaffolding.
- `.gitignore`
- Initializes a git repository.

### 2. Configure

Edit `tdd.yaml` to select your LLM model and API settings.
Set your API key in the environment variable specified in `tdd.yaml` (default `OPENAI_API_KEY`).

```bash
export OPENAI_API_KEY=sk-...
```

Edit `kata.md` to describe the problem you want the agents to solve.

### 3. Run

```bash
tdd-cli run --steps 10
```

This will run 10 steps of the TDD cycle.
You can stop it at any time with Ctrl+C.

### 4. Check Status

```bash
tdd-cli status
```

### 5. Debug

Run a single step:

```bash
tdd-cli step
```

## Architecture

- `tdd-cli`: CLI entrypoint.
- `tdd-core`: Domain logic, orchestrator, traits.
- `tdd-agents`: Agent implementations (Tester, Implementor, Refactorer).
- `tdd-exec`: Git and process execution.
- `tdd-llm`: LLM client.

## Configuration

`tdd.yaml` example:

```yaml
kata_description: "kata.md"
language: "rust"
steps: 20
max_attempts_per_agent: 5
roles:
  tester:
    model: "openai:gpt-4o"
    temperature: 0.4
  implementor:
    model: "openai:gpt-4o"
    temperature: 0.2
  refactorer:
    model: "openai:gpt-4o"
    temperature: 0.3
llm:
  base_url: "https://api.openai.com/v1"
  api_key_env: "OPENAI_API_KEY"
ci:
  test_cmd: ["cargo", "test", "--all"]
  check_cmd: ["cargo", "clippy", "--all", "--", "-D", "warnings"]
  fmt_cmd: ["cargo", "fmt"]
commit:
  author_name: "TDD Machine"
  author_email: "tdd@local"
```
