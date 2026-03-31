# Patchnote Command

Generate a patchnote/changelog for the current version.

## Instructions

### 1. Get the current version

Read the version from `package.json` (root).

### 2. Find the previous version tag

Run `git tag --sort=-v:refname` to list tags. Find the most recent tag before the current version (e.g., if current is `0.4.3`, look for `v0.4.2`).

### 3. Gather commits

Run `git log <previous_tag>..HEAD --oneline` to get all commits since the last release.

If no previous tag exists, use `git log --oneline -20` to get the last 20 commits.

### 4. Generate the patchnote

Analyze all commits and group them by category. Read the actual diffs if commit messages are unclear (`git show <hash> --stat`).

## Output Format

```markdown
# CrAIte v{version}

## New Features
- Description of new features

## Improvements
- Description of improvements/enhancements

## Bug Fixes
- Description of bug fixes

## Internal
- Dependencies, CI, tooling, refactoring changes
```

## Rules

- Write in **French**
- Omit empty sections (if no bug fixes, don't include the section)
- Keep descriptions user-facing and concise — don't mention internal file names
- Group related commits into a single bullet point when they address the same feature
- Use clear, non-technical language when possible
- Output the patchnote inside a single markdown code block (```markdown ... ```) so the user can easily copy it
