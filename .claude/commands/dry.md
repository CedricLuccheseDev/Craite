---
description: DRY check - Review local changes for simplification and best practices
allowed-tools: [Bash, Read, Edit]
---

# DRY Command

Analyze local git changes and suggest improvements following senior-level best practices: DRY (Don't Repeat Yourself), concise code, and avoiding verbosity.

## Instructions

When this command is invoked, perform these steps:

### 1. Gather Local Changes
Run these git commands to see what's been modified:
- `git diff` - Show unstaged changes
- `git diff --cached` - Show staged changes
- `git status --short` - Get a quick list of modified files

### 2. Analyze Each Changed File
For each modified file:
- Read the current implementation
- Look for patterns like:
  - **Verbose code**: Unnecessary lines, early returns that can be simplified
  - **Non-DRY**: Repeated logic that could be extracted
  - **Complex conditionals**: Logic that could be more concise
  - **Inconsistent patterns**: Code that doesn't match surrounding style
  - **Over-engineering**: Unnecessary abstractions or complexity
  - **Verbose naming**: Variables like `request` that could be `r` when context is clear
  - **Multi-line simple logic**: Single expressions split across multiple lines

### 3. Suggest Improvements
For each improvement opportunity:
- Show the **before** code
- Show the **after** code
- Explain **why** it's better (more concise, more readable, follows DRY, etc.)
- Apply the changes if they're straightforward improvements

### 4. Focus Areas
Prioritize reviewing:
- Computed properties and functions
- Conditional logic (if/else chains, ternaries)
- Variable declarations and naming
- Repeated patterns
- Template logic in Vue components

### 5. Guidelines
- **Be pragmatic**: Only suggest changes that genuinely improve the code
- **Match the style**: Follow patterns already established in the file
- **Keep it readable**: Concise doesn't mean cryptic
- **Senior mindset**: Think "what would make this easier to maintain?"
- **No premature optimization**: Focus on clarity and maintainability, not performance

## Output Format

Provide a clear summary:

**Files Reviewed:**
- List of files analyzed

**Improvements Made:**
- Description of each change with before/after code snippets
- Rationale for each improvement

**Recommendations (if not auto-applied):**
- Suggestions that need user decision
- Trade-offs to consider
