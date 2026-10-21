---
name: worktree-workflow
description: Use when starting, running or finishing work in a worktree branch — bootstrap, landing as one squashed commit on local main with `pnpm squash`, cleanup. Covers what `pnpm squash` does that `git merge --squash` does not.
---

# Working in a worktree

The root checkout is shared by several sessions, **including its index**. Branch work happens
in a worktree inside the repo, never by switching the root's branch:

- `.worktrees/<name>` for one you add with `git worktree add`; it is gitignored
- `.claude/worktrees/<name>` for one Claude Code's `EnterWorktree` creates

## Bootstrap

```sh
git worktree add .worktrees/<name> -b <name>
cd .worktrees/<name>
pnpm install
```

`pnpm install` also links the skills and, when `uv` is on the PATH, sets up the Python
environment. uv picks the newest interpreter it finds, and one without wheels for the native
dependencies fails to build them: check `python/.venv/bin/python -V`, and if it is not the
root's version, `rm -rf python/.venv && uv sync --all-packages -p 3.12` in `python/`. Run `pnpm -C typescript build` before typing or testing `typescript/` or `desktop/`:
the packages type-check against each other's `build/`, and a fresh worktree has none.

## Landing

"Land" means one squashed commit on **local** `main`. It never means push; pushing `main`
happens only when someone asks for it.

```sh
# in the worktree, with the change-verification slice green
git merge main          # reconcile
pnpm squash <branch> -m "<type(scope): subject>"
```

After the merge, `main...HEAD` holds only this branch's change. Re-run the slice the merge
reaches before squashing (**`change-verification`**). The subject is what release-please reads.
List further `fix(…)`/`feat(…)` subjects in the body, each passed as another `-m`.

`pnpm squash` (`.scripts/squash-branch.ts`):

- builds the commit with plumbing
- advances `main` by compare-and-swap
- refreshes only the landed paths in the root

It refuses:

- a branch that is missing `main`
- a subject marked breaking, or one that is not a Conventional Commit
- a landed path that someone has dirty in the root

Never run `git merge --squash` in the root: its commit takes everything any session has staged.
Never fast-forward or merge-commit a branch into `main` either, because both carry its WIP commits.

## Cleanup

Once a worktree **this session created** has landed:

```sh
git worktree remove .worktrees/<name>
git branch -D <name>
```

Use `-D`, not `-d`. The branch was squashed, so git never sees it as merged.

Never remove another session's worktree or branch.
