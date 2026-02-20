---
description: Full-power orchestrated workflow. Massive parallel Haiku exploration, Opus planning, then coordinated implementation. For complex features.
argument-hint: <feature-or-task-description>
---

# Apex Command - Maximum Power Orchestration

Execute complex, multi-faceted tasks with full agent orchestration.

## Mission
$ARGUMENTS

## Execution Strategy (3 Phases)

---

### PHASE 1: Massive Parallel Reconnaissance (HAIKU x6)

Launch ALL 6 agents simultaneously in ONE message with **model: "haiku"**:

1. **Architecture Agent** (subagent_type: Explore, model: haiku)
   ```
   prompt: "Map the complete architecture for: '$ARGUMENTS'. Identify entry points, data flow, dependencies, and integration points. Return a structured overview."
   ```

2. **Patterns Agent** (subagent_type: Explore, model: haiku)
   ```
   prompt: "Find ALL existing patterns related to: '$ARGUMENTS'. Include: naming conventions, file structure, error handling, testing patterns, API patterns."
   ```

3. **Dependencies Agent** (subagent_type: Explore, model: haiku)
   ```
   prompt: "Analyze dependencies for: '$ARGUMENTS'. Check package.json, imports, shared utilities, database schemas, API contracts."
   ```

4. **Testing Agent** (subagent_type: Explore, model: haiku)
   ```
   prompt: "Map the testing strategy for: '$ARGUMENTS'. Find test files, fixtures, mocks, test utilities, coverage patterns."
   ```

5. **Security Agent** (subagent_type: Explore, model: haiku)
   ```
   prompt: "Security scan for: '$ARGUMENTS'. Check auth patterns, validation, sanitization, secrets handling, OWASP concerns."
   ```

6. **History Agent** (subagent_type: Explore, model: haiku)
   ```
   prompt: "Git history analysis for areas related to: '$ARGUMENTS'. Find recent changes, contributors, related PRs, potential conflicts."
   ```

---

### PHASE 2: Strategic Planning (OPUS - Main Thread)

With reconnaissance complete, YOU will:

1. **Synthesize** all 6 agent reports into unified understanding
2. **Design** implementation strategy with clear steps
3. **Identify** risks, edge cases, breaking changes
4. **Plan** file changes in dependency order
5. **Present** plan to user using TodoWrite tool

**Use TodoWrite** to create actionable task list before proceeding.

**Use AskUserQuestion** if:
- Multiple valid architectural approaches exist
- Trade-offs need user decision
- Scope clarification needed

---

### PHASE 3: Coordinated Implementation (OPUS)

Execute the plan:

1. **For each task in todo list:**
   - Mark as in_progress
   - Implement with full context awareness
   - Verify against patterns from Phase 1
   - Mark as completed

2. **Parallel Background Agents** (when beneficial):
   - Spawn background agents for independent subtasks
   - Use `run_in_background: true` for parallel work
   - Coordinate results with TaskOutput

3. **Continuous Validation:**
   - Type check after major changes
   - Run relevant tests
   - Verify no regressions

---

## Output Format

```markdown
## Summary
[What was accomplished]

## Changes Made
| File | Change Type | Description |
|------|-------------|-------------|
| ... | added/modified/deleted | ... |

## Testing
- [Tests added/modified]
- [Verification steps]

## Follow-up
- [Any remaining items]
- [Recommendations]
```

---

## Quality Gates

Before marking complete:
- [ ] All todos completed
- [ ] Code follows existing patterns
- [ ] No TypeScript/lint errors
- [ ] Tests pass
- [ ] No security concerns introduced

---

## When to Use /apex

- New features requiring multiple files
- Complex refactoring
- Architecture changes
- Cross-cutting concerns
- When /oneshot feels insufficient
