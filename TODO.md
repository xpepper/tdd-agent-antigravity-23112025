# TDD Agent - TODO List

## High Priority

### LLM Provider Support
- [ ] Add support for GitHub Copilot LLM coding models via personal GitHub token
  - Research GitHub Copilot API endpoints
  - Implement authentication with personal access token
  - Add configuration option for GitHub Copilot models

- [ ] Add support for non-OpenAI API formats
  - [ ] Google Gemini API support
    - Different request/response format
    - API key authentication
  - [ ] Anthropic Claude API support
    - Different request/response format
    - API key authentication
  - Consider creating adapter pattern for different LLM providers
  - Update `tdd-llm` crate to support multiple API formats

### Logging & Observability
- [ ] Add comprehensive logging for LLM interactions
  - Log full prompts sent to LLMs
  - Log full responses received from LLMs
  - Log token usage and costs (if available)
  - Add configurable log levels (debug, info, warn, error)
  - Option to save logs to `.tdd/logs/llm-interactions/`

- [ ] Add progress monitoring
  - Real-time progress updates during kata execution
  - Show current step, role, and attempt number
  - Display test results and verification status
  - Option for verbose output mode

### Refactoring Improvements
- [ ] Review and update Refactorer prompt
  - Explicitly allow creating new files (e.g., `src/rover.rs`, `src/heading.rs`)
  - Encourage splitting large `lib.rs` into modules
  - Provide examples of good module organization
  - Add guidance on when to extract to separate files
  - Update JSON schema to support multiple file edits in one step

### Kata Constraints & Requirements
- [ ] Update all role prompts to properly read and follow kata constraints
  - Emphasize reading the entire kata description carefully
  - Explicitly instruct agents to follow any constraints mentioned (e.g., "each function <= 5 LOC")
  - Add examples of common kata constraints:
    - Function length limits
    - No loops/only recursion
    - Specific design patterns required
    - Performance requirements
  - Ensure constraints are checked during verification
  - Add validation that code adheres to stated constraints

## Medium Priority

### Configuration Enhancements
- [ ] Add per-role timeout configuration
- [ ] Add retry strategy configuration (exponential backoff, etc.)
- [ ] Support for local LLM providers (Ollama, LM Studio, etc.)
- [ ] Allow different models for different roles

### CLI Improvements
- [ ] Better `status` command implementation
  - Show current step and role
  - Display last commit summary
  - Show test pass/fail status
  - Display remaining steps

- [ ] Add `resume` command to continue from last step
- [ ] Add `rollback` command to undo last step
- [ ] Add `--verbose` flag for detailed output
- [ ] Add `--dry-run` flag to preview without executing

### Testing & Quality
- [ ] Add integration tests with mock LLM responses
- [ ] Add example successful runs for each kata
- [ ] Add benchmarking for different LLM providers
- [ ] Add validation for kata description format

## Low Priority

### Documentation
- [ ] Add troubleshooting guide
- [ ] Add examples of successful kata runs
- [ ] Document best practices for kata descriptions
- [ ] Add architecture diagrams

### Features
- [ ] Support for languages other than Rust (Python, TypeScript, etc.)
- [ ] Web UI for monitoring progress
- [ ] Export kata run as video/animation
- [ ] Support for custom test runners
- [ ] Parallel execution of independent tests

## Completed
- [x] Initial workspace implementation
- [x] OpenAI-compatible API support
- [x] Git integration
- [x] Three-agent orchestration (Tester, Implementor, Refactorer)
- [x] Conventional commit support
- [x] Retry mechanism for failed attempts
- [x] Sample katas (Bowling, FizzBuzz, String Calculator, Mars Rover, Battleships)
- [x] Strengthened TDD role prompts with RED/GREEN/REFACTOR phases
- [x] Comprehensive Perplexity model testing and documentation
