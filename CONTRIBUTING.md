---
title: Contributing
---

> [!WARNING]
> **We don't accept pull requests from outside contributors.** Pull requests opened by anyone outside the team are closed without review. If you've found a bug or have an idea, open an issue or start a discussion instead, as described below.

Fairspec is an open standard and software developed by the [Datist](https://datist.io) team. Everyone is welcome to help shape it with bug reports, ideas and questions; changes to the specification and the code come from team members.

## Contributors

### Report a bug

Open an issue on [GitHub Issues](https://github.com/datisthq/fairspec/issues) for a bug in the software or an error, ambiguity or inconsistency in the specification. A good report includes:

- the Fairspec version and the part you use: the specification, the Python or TypeScript library, the CLI, the MCP server or the desktop app
- the section of the specification or the smallest input that shows the problem
- what you got and what you expected

### Share an idea or ask a question

Use [GitHub Discussions](https://github.com/datisthq/fairspec/discussions) for anything that isn't a bug:

- [Ideas](https://github.com/datisthq/fairspec/discussions/new?category=ideas): features and improvements you'd like to see
- [Q&A](https://github.com/datisthq/fairspec/discussions/new?category=q-a): questions about using Fairspec
- [Show and tell](https://github.com/datisthq/fairspec/discussions/new?category=show-and-tell): projects built with Fairspec

A proposal for the specification is easier to pick up when it explains the problem it solves, gives concrete use cases, and considers backward compatibility and the effect on existing users. Ideas the team picks up turn into issues.

## Members

The rest of this guide is for Datist team members working on Fairspec.

### Working on the Code

The repository is a monorepo: the standard lives in `standard/`, the implementations in
`python/` and `typescript/`, and the desktop application in `desktop/`. Clone it and install
everything from the root:

```bash
git clone https://github.com/datisthq/fairspec.git
cd fairspec
pnpm install
```

Run `pnpm test` for the full gate, or `pnpm -C <section> <script>` for one section — for
example `pnpm -C python test` or `pnpm -C typescript test:unit`. Each section has its own contributing
notes in its directory, and the conventions coding agents follow — which hold for people too —
are in `AGENTS.md` and the skills in `.agents/skills/`.

Commits follow [Conventional Commits](https://www.conventionalcommits.org/) scoped by section,
such as `fix(python): …` or `docs(standard): …`. Each section is versioned and released on its
own, so use `feat` and `fix` only for changes to what that section ships.
