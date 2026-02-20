---
description: Fast single-task execution. Haiku gathers context, then executes precisely. For quick fixes, refactors, or implementations.
argument-hint: <task-description>
---

# OneShot Command - Fast Focused Execution

Execute a single, well-defined task with maximum efficiency.

## Task
$ARGUMENTS

## Execution Strategy

### Phase 1: Rapid Context Gathering with HAIKU (5-10 seconds)

Launch 2 parallel Haiku agents to understand the codebase context:

1. **Structure Agent** (subagent_type: Explore, model: haiku)
   ```
   prompt: "For this task: '$ARGUMENTS' - Identify the 3-5 most relevant files. Return file paths and brief descriptions of what each contains."
   ```

2. **Pattern Agent** (subagent_type: Explore, model: haiku)
   ```
   prompt: "For this task: '$ARGUMENTS' - Find existing patterns, conventions, and similar implementations in this codebase. Return examples to follow."
   ```

### Phase 2: Direct Execution (Main Thread)

With context from Haiku agents:
1. Read the identified files directly (no agent needed)
2. Apply the task following discovered patterns
3. Make the changes immediately
4. Verify with quick validation

## Execution Rules

- **No planning phase** - Go straight to implementation
- **No over-engineering** - Minimal changes only
- **Follow existing patterns** - Match codebase style exactly
- **One responsibility** - Do exactly what was asked, nothing more

## Output

After completion, report:
```markdown
## Done
- [What was changed]
- Files modified: [list]

## Verification
- [How to test the change]
```

## When NOT to use /oneshot

Use /apex instead if:
- Task requires architectural decisions
- Multiple interconnected changes needed
- Unclear requirements needing clarification
- Risk of breaking existing functionality
