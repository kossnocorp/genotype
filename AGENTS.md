# AI Agent Instructions

## Architecture Hiccup Records

When you try to make a change and it breaks something in unexpected ways, handle it using the following algorithm:

1. Write down the assumptions you had before making the change. This includes what you wanted to achieve, what change you made and why, what you expected to happen, and what actually happened. Don't try to analyze the issue at this point. Just write down the facts as you see them.
2. Create a Markdown with the description of the issue in `.agents/artifacts/architecture-hiccups/<id>-<slug>.md` where `<id>` is an incremented ID (e.g., `004`) and `<slug>` is a brief description of the issue (e.g., `hard-to-use-references-in-nodes.md`). Keep everything in the "Description" section.
3. Continue with the task at hand, but keep the issue in mind.
4. After you finish the task, come back to the issue and analyze it. Using the knowledge you obtain while working around the issue, try to find the root cause and possible solutions. Write down your analysis in the same Markdown file, adding a new section "Analysis".

The resulting Markdown should follow this template:

```md
# <issue-title>

<issue-brief>

## Description

<issue-description>

## Analysis

<issue-analysis>
```