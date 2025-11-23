# TDD Agent Constitution

## Core Principles

### I. Readable, Intent-Revealing Code
Code MUST be written so that the next developer can
understand intent quickly without guessing. Names MUST be
descriptive, side effects explicit, and control flow
straightforward. Comments are reserved for non-obvious
decisions, not restating what code already expresses.

### II. Consistent Code Quality Standards
The codebase MUST follow a consistent style and quality
baseline: no dead code, no commented-out blocks as long-term
scaffolding, and no unused abstractions. Existing patterns
and conventions in this kata MUST be followed unless there is
a clear, documented reason to improve them.

### III. Test-Driven, High-Coverage Development
New behavior MUST be introduced using Test-Driven
Development: write a failing test, make it pass with the
simplest implementation, then refactor. Unit tests MUST
cover all new logic paths that can be reasonably isolated.
Regression tests SHOULD be added for any fixed bug.

### IV. Small, Focused, Reversible Commits
Commits MUST be small, cohesive, and tell a clear story.
Each commit SHOULD represent a single behavior change or
refactoring step that can be reviewed independently and
reverted without collateral damage. Work-in-progress code
MUST NOT be committed unless it is explicitly guarded and
does not break existing behavior.

### V. Pre-Commit Safety Gate
Before any commit, all automated checks MUST pass: code MUST
compile, the full test suite MUST be green, and static
analysis or linters configured for this project MUST report
no new issues. If a check is flaky or temporarily failing, it
MUST be either fixed immediately or the risk explicitly
documented in the commit message and follow-up task.

## Additional Constraints

This kata favors clarity over cleverness. Introducing new
dependencies, patterns, or architectural layers MUST be
justified by concrete needs (e.g., new behavior, testability,
or safety) rather than anticipated future requirements.

## Development Workflow

Work SHOULD proceed in short, test-driven cycles. Each cycle
follows this pattern:
- Clarify the next small behavior change.
- Write or adjust tests to express that behavior.
- Implement the minimal code to make tests pass.
- Refactor for readability and code quality while keeping
	tests green.
- Run the full safety gate before committing.

Code review, when applicable, MUST check for alignment with
these principles in addition to functional correctness.

## Governance

This constitution defines the non-negotiable engineering
standards for this kata. Where it conflicts with past habits
or ad-hoc practices, this constitution takes precedence.

Amendments MUST:
- Be documented as explicit version bumps in this file using
	semantic versioning.
- State which principles or sections are added, removed, or
	redefined.
- Be communicated to anyone contributing to this kata.

Compliance:
- All new work and code reviews MUST assess adherence to
	readability, code quality, testing discipline, commit
	hygiene, and the pre-commit safety gate.
- Periodic review of this constitution SHOULD occur when the
	kata evolves or new constraints emerge.

