---
name: change-review
description: Review a pull request, or the local branch before it merges, for correctness, security and repo conventions — inline PR comments, or a ranked list locally. Invoked by the review workflow or a person; never fires on its own.
disable-model-invocation: true
---

# Review a change

Read the diff, judge it, post findings as inline comments. Be useful, not thorough-looking:
a review that flags twenty things teaches nothing, and the twenty-first — the real one — is lost
in it.

## What to look for

**Correctness**

- Logic that is wrong for an input the change makes reachable, not merely unusual.
- Error handling that swallows a failure or reports success on a partial write.
- Concurrency: two requests, two sessions, a retry, a replay.
- Performance where the change makes it structurally worse (a query in a loop, an unbounded
  scan), not where it is merely not optimal.

**Security — OWASP-shaped, but reported only when reachable**

- Injection (SQL and otherwise), XSS, SSRF, XXE, unsafe deserialisation.
- Broken authentication or access control: a route that skips the permission it needs, a scope
  resolved from user input, an id trusted because it came from the client.
- Sensitive data exposure: secrets in code or logs, PII widening, a credential recoverable from
  the database.
- Insecure cryptography, race conditions and TOCTOU, missing logging on a security-relevant
  action.

**This repo's own invariants** — these are where a generic reviewer is blind, and they matter
more than the generic checklist:

- a commit or PR title typed `feat`/`fix` for something that does not change what its section
  ships (the site, an example, tests, tooling), or a normative change to the standard's prose
  typed `docs` — release-please versions on the type, and a burnt version is never reusable;
- a commit marked breaking (`!`, `BREAKING CHANGE:`), or a hand-edited `.release/manifest.json`;
- a package edge that reverses the `metadata`-first DAG in `python/` or `typescript/`;
- a public API missing from its package's `index.ts` / `__init__.py`, or an `export *` there;
- a TypeScript test asserting on a raw path string or a bare `\n` — it fails on Windows CI;
- a renamed recorded test (vitest-polly, pytest-recording) with its old recording left behind;
- a desktop renderer file importing `#processes/*`, `electron` or `node:*`, a new user-facing
  string with unfilled catalogues, or a changed `productName`;
- a root script switched to `pnpm -r`, or a filter quoted with single quotes (literal on Windows);
- a `release-as` pin left in `.release/config.json` after its release merged;
- a doc, skill or `AGENTS.md` claim the diff made untrue (**`human-documentation`**).

The relevant skill explains each of these — `typescript-development`, `python-development`,
`desktop-development`, `monorepo-layout`, `human-documentation`.

## How to report

Rate each finding **CRITICAL / HIGH / MEDIUM / LOW**, and say what input or sequence produces
the failure. A finding that cannot name the case that breaks is a hunch: either verify it or
leave it out.

Post specific issues as inline comments on the lines they concern. Keep the summary short.
Silence on the parts that are fine is not laziness — it is what makes the flagged parts legible.

If the change is good, say so briefly and stop. Do not manufacture findings to justify the run.

## Reviewing locally

With no pull request — a person running `/change-review` in a checkout, or `issue-resolution`
before it hands back — the change is everything this branch adds over `main`:
`git diff main...HEAD` plus the uncommitted and untracked files. Judge it by the same rules,
then print the findings in the terminal, worst first, each with its `file:line`. Post nothing,
and edit nothing unless the caller asks for the fixes.
