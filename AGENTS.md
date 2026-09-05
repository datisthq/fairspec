# Agents

This file provides guidance to coding agents when working with code in this repository.

## General

- Prioritize using LSP capabilities if possible
- When resolving a TODO, follow its instructions literally
- Run type checking as part of your tasks
- Don't run linting as part of your tasks
- Start plan from a summary a new/updated API summary (models/signatures) and most challenging points, continue plan using your default settings

## Commits

- **IMPORTANT**: Only changes to the actual specifications use `feat` or `fix`. That
  means the normative artifacts: `specs/`, `profiles/`, `extensions/`
- Everything else MUST use `chore` or `docs` — the website and its landing pages,
  `examples/`, `overview/`, `README.md`, tooling, CI, dependencies, configuration,
  this file. An example is a demonstration, not a normative artifact: correcting one
  changes nothing about what validates
- This is not a style preference. `feat` and `fix` are read by semantic-release: they
  cut a new version, publish a new profile directory under
  `.livemark/public/profiles/<version>/`, and tell consumers the standard changed.
  Using them for a landing page tweak announces a specification change that did not
  happen, and burns a version number that can never be reused
- When in doubt, ask whether the change alters what a conforming descriptor or
  implementation `MUST` do. If it does not, it is `chore` or `docs`
- **A normative change does not require a JSON diff.** Much of the standard is prose
  that JSON Schema cannot express — how a hash is computed, how multipart parts
  combine, what a dialect means. Tightening or correcting one of those is `feat` or
  `fix` even when every file under `profiles/` stays byte-identical, because an
  implementer pinned to the previous version has no other way to learn the
  requirement changed
- Within `specs/`, `profiles/` and `extensions/`, editorial edits that leave every
  requirement intact — typos, dead links, formatting, rewording that changes no
  `MUST` — are still `docs`

## Commands

- Run `pnpm install` to install all dependencies
- Run `pnpm lint` to lint the code using vite-plus
- Run `pnpm format` to auto-fix formatting issues with vite-plus
- Run `pnpm type` to check TypeScript types
- Run `pnpm -F <name> run type` to check types for a specific package

## Formats

- Use 2-space indentation, UTF-8 encoding, and LF line endings
- Use PascalCase for classes and interfaces, and camelCase for methods and variables
- Place high-level public items first in a file and low-level private items last
- Use ES modules with full import paths including the ".ts(x)" file extension

## Types

- Use strict TypeScript with null checks but don't add explicit return types to functions
- Never use TypeScript `any`, type casting `as`, or `!` without permission

## Docs

- Add Typedoc comments only for public APIs and don't add them for files or use @params directives
- Don't write `//` comments in the code
