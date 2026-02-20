---
description: SOLID & Clean Code audit - Deep review of a module or directory for architecture violations
allowed-tools: [Read, Glob, Grep, Task, Bash]
---

# Audit Command

Deep audit of a code module or directory to detect SOLID principle violations, clean code issues, and architectural smells. Proposes actionable refactoring solutions.

The user may provide a target path as argument: `$ARGUMENTS`
If no argument is provided, ask the user which module or directory to audit.

## Instructions

### 1. Scope Discovery

Determine the audit perimeter:
- If a path is provided, audit all files in that directory (recursively)
- If a file is provided, audit that file and its direct dependencies
- List all files in scope with `Glob`

### 2. Read & Map the Code

For each file in scope:
- Read the full content
- Identify: classes, composables, components, services, stores, types, utilities
- Map dependencies between files (imports/exports)
- Note file sizes (flag files > 200 lines)

### 3. SOLID Violations Check

Analyze each file against all 5 SOLID principles:

#### S - Single Responsibility Principle
- Does the file/class/composable do ONE thing?
- Red flags: mixed concerns (API calls + UI logic + data transform in same file), god components, composables that manage unrelated state
- Check: "Can you describe what this file does in one sentence without using 'and'?"

#### O - Open/Closed Principle
- Is the code open for extension but closed for modification?
- Red flags: long switch/if-else chains that grow with each new case, hardcoded values that should be configurable, functions that need modification for each new variant
- Check: "Can I add a new behavior without modifying existing code?"

#### L - Liskov Substitution Principle
- Can subtypes replace their parent without breaking behavior?
- Red flags: type assertions/casts that bypass type safety, conditional logic based on type checking (`instanceof`, discriminated unions misused)
- Check: "Do all implementations honor the contract?"

#### I - Interface Segregation Principle
- Are interfaces focused and minimal?
- Red flags: large Props interfaces with many optional fields, components that receive props they don't use, types with fields only relevant to some consumers
- Check: "Is any consumer forced to depend on things it doesn't use?"

#### D - Dependency Inversion Principle
- Does high-level code depend on abstractions, not concrete implementations?
- Red flags: direct API calls inside components, hardcoded service instantiation, tight coupling to specific libraries without abstraction layer
- Check: "Can I swap the implementation without changing the consumer?"

### 4. Clean Code Smells

Also flag these common issues:

**Naming & Readability**
- Unclear or misleading variable/function names
- Abbreviations that hurt readability
- Boolean variables without `is`/`has`/`should` prefix
- Functions that don't describe their action

**Complexity**
- Functions > 20 lines
- Deeply nested logic (> 3 levels)
- Cyclomatic complexity too high (many branches)
- Callback hell or promise chain spaghetti

**Structure**
- Dead code (unused exports, unreachable branches)
- Magic numbers/strings without constants
- Duplicated logic across files (DRY violation)
- Inconsistent error handling patterns
- Missing TypeScript types (implicit `any`)

**Vue-Specific**
- Business logic in templates (complex expressions, inline functions)
- Oversized components that should be split
- Props drilling instead of provide/inject or store
- Reactive state mutations outside proper patterns
- Missing or incorrect type definitions for props/emits

### 5. Severity Classification

Rate each finding:
- **Critical**: Actively causes bugs or blocks maintainability. Must fix.
- **Major**: Significant design issue that will cause pain as code grows. Should fix.
- **Minor**: Style or convention issue. Nice to fix.

### 6. Propose Solutions

For each finding, provide:
- The **principle violated** (S/O/L/I/D or Clean Code category)
- The **current code** snippet
- The **problem** explained in one sentence
- A **proposed solution** with code sketch
- The **impact**: what improves after the fix (testability, readability, extensibility...)

## Output Format

```
# Audit Report: [module/directory name]

## Summary
- Files audited: X
- Critical issues: X
- Major issues: X
- Minor issues: X
- Overall health: [Good / Needs attention / Needs refactoring]

## Critical Issues

### [Issue title]
- **Principle**: [S/O/L/I/D or Clean Code rule]
- **File**: [path:line]
- **Problem**: [one sentence]
- **Current code**:
  [snippet]
- **Proposed solution**:
  [snippet]
- **Impact**: [what improves]

## Major Issues
[same format]

## Minor Issues
[same format]

## Refactoring Roadmap
Suggested order to address issues:
1. [First fix - why first]
2. [Second fix - why second]
...
```

## Guidelines

- **Be precise**: Point to exact files and line numbers
- **Be pragmatic**: Don't flag things that work fine and are readable
- **Context matters**: A 250-line file with clear sections may be acceptable; a 150-line file with mixed concerns is not
- **Propose, don't impose**: Give the rationale, let the developer decide
- **No gold-plating**: Don't suggest architectural astronautics - keep solutions proportional to the problem
- **Respect existing patterns**: If the codebase has a convention, follow it even if it's not your preference
