# TDD Agent - Perplexity Sonar Model Testing

## Date: 2025-11-23

## Objective
Test the TDD Agent with Perplexity's `sonar` model to validate multi-agent TDD workflow.

## Configuration
- Model: `sonar`
- API: Perplexity (https://api.perplexity.ai/)
- Kata: Mars Rover

## Findings

### Issue 1: Missing JSON Fields
**Problem:** Initial run failed because the LLM returned incomplete JSON - missing `commit_message` and `notes` fields.

**Response returned:**
```json
{
  "edits": [...]
}
```

**Expected schema:**
```json
{
  "edits": [...],
  "commit_message": "...",
  "notes": "..."
}
```

**Solution:** Strengthened prompts with explicit instructions:
- Added "CRITICAL: You MUST output ONLY valid JSON"
- Explicitly listed all three required fields
- Emphasized that all fields are REQUIRED

**Result:** ✅ Fixed - model now returns all required fields

### Issue 2: TDD Discipline Violation
**Problem:** The Tester role wrote BOTH tests AND implementation, violating the red-green-refactor cycle.

**What happened:**
- Tester role created tests for `move_forward` method
- Tester role ALSO implemented the `move_forward` method
- Tests passed (should fail in Tester role)
- Orchestrator correctly rejected the attempt (tests must fail for Tester)

**Expected behavior:**
- Tester: Write ONLY failing tests
- Implementor: Write minimal code to make tests pass
- Refactorer: Improve structure without changing behavior

**Root cause:** The `sonar` model is optimized for search/research and general helpfulness. It tries to be "helpful" by providing complete solutions rather than strictly following role constraints.

## Conclusion
Perplexity's `sonar` model:
- ✅ Can return properly formatted JSON (with strengthened prompts)
- ❌ Does not follow strict role-based constraints required for TDD
- ❌ Not suitable for this multi-agent TDD workflow

### Sonar-Pro Testing (2025-11-23)
**Model:** `sonar-pro`
**Result:** ❌ Same issue as `sonar`

The `sonar-pro` model also violated TDD discipline:
- Wrote tests for `move_forward` method
- Also implemented the `move_forward` method in the same step
- Tests passed (should fail in Tester role)
- All 5 retry attempts failed for the same reason

**Conclusion:** Both `sonar` and `sonar-pro` models are not suitable for this strict role-based TDD workflow.

## Recommendation
Use LLMs with better instruction-following capabilities:
- OpenAI GPT-4 / GPT-4o
- Anthropic Claude (Sonnet/Opus)
- Google Gemini Pro
- DeepSeek Coder

These models have demonstrated better adherence to system prompts and role constraints in similar multi-agent workflows.
