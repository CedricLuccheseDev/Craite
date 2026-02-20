---
description: Create a GitHub issue from a problem description with context analysis
argument-hint: <problem description>
---

# Issue Command - Create GitHub Issue

Create a GitHub issue from the problem described below, with automatic context gathering.

## Problem
$ARGUMENTS

## Instructions

### Phase 1: Context Gathering

Run these commands to gather context:

1. `git branch --show-current` - Get current branch
2. `git status --short` - List modified files
3. `git remote get-url origin` - Verify GitHub repo

If the problem mentions specific files or errors, use Grep/Read to find relevant code sections.

### Phase 2: Issue Content Generation

**Title**: Concise summary (max 80 chars), imperative mood
- Bug: "Fix [what's broken]"
- Feature: "Add [what's needed]"
- Improvement: "Improve [what to enhance]"

**Body structure**:

```markdown
## Description
[Clear explanation of the problem in 2-3 sentences]

## Context
- **Branch**: `[branch-name]`
- **Modified files**: [list if relevant]
- **Related code**: [file:line references if applicable]

## Steps to reproduce (if bug)
1. [Step 1]
2. [Step 2]

## Expected behavior
[What should happen]

## Actual behavior
[What currently happens]
```

### Phase 3: User Confirmation

Display the generated issue content in a clear format:

```
=== ISSUE PREVIEW ===

Title: [generated title]

Body:
[generated body]

=====================
```

Then use AskUserQuestion to confirm:
- Question: "Voici l'issue a creer. Confirmer la creation ?"
- Options: "Creer l'issue", "Modifier le contenu", "Annuler"

If "Modifier", ask what to change and regenerate.
If "Annuler", stop without creating.

### Phase 4: Create Issue

Execute:
```bash
gh issue create --title "TITLE" --body "$(cat <<'EOF'
BODY_CONTENT
EOF
)"
```

Report the issue URL to the user.

## Rules

- Always gather context before generating content
- Keep title under 80 characters
- Use English for issue content (unless user writes in French)
- Never create without user confirmation
- If `gh` CLI not authenticated, guide user to run `gh auth login`
