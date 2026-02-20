---
description: Deep debugging with parallel multi-agent analysis. Uses Haiku for fast exploration, Opus for root-cause synthesis.
argument-hint: [error-message or file:line]
---

# Debug Command - Multi-Agent Root Cause Analysis

You are an expert debugger. Your mission is to identify and fix the issue described below.

## Target Issue
$ARGUMENTS

## Execution Strategy

**CRITICAL**: Use the Task tool to spawn multiple agents IN PARALLEL for maximum efficiency. Launch all analysis agents simultaneously in a SINGLE message with multiple Task tool calls.

### Phase 1: Parallel Investigation with HAIKU (Fast Exploration)

Spawn these 4 agents simultaneously using the Task tool with **model: "haiku"**:

1. **Error Context Agent** (subagent_type: Explore, model: haiku)
   ```
   prompt: "Find the file/function related to: $ARGUMENTS. Read 100 lines of surrounding context. Report the exact failure point with file:line references."
   ```

2. **Stack Trace Agent** (subagent_type: Explore, model: haiku)
   ```
   prompt: "Trace the call hierarchy that could lead to: $ARGUMENTS. Map the data flow and identify where values might become invalid."
   ```

3. **Similar Patterns Agent** (subagent_type: Explore, model: haiku)
   ```
   prompt: "Search the codebase for similar error handling or patterns related to: $ARGUMENTS. Find existing workarounds or utilities."
   ```

4. **Test/Log Agent** (subagent_type: Explore, model: haiku)
   ```
   prompt: "Find test files and log statements related to: $ARGUMENTS. Check git history for previous similar issues."
   ```

### Phase 2: Synthesis with OPUS (Deep Analysis)

After Haiku agents complete, YOU (running on the main thread) will:
1. Correlate all findings from the 4 agents
2. Apply deep reasoning to identify TRUE root cause (not symptoms)
3. Design minimal, surgical fix
4. Assess risks and edge cases

## Output Format

```markdown
## Root Cause
[One sentence explaining WHY this happens]

## Evidence
- [Finding 1 from agents]
- [Finding 2 from agents]

## Proposed Fix
[Code changes with file:line references]

## Risk Assessment
[Potential side effects]
```

## Constraints
- DO NOT modify code without explicit user approval
- Focus on smallest possible fix
- Haiku explores, Opus thinks
