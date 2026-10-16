---
name: desktop-development
description: Use when working in desktop/ — the Electron application. Covers commands, the renderer/process split and its lint boundary, shadcn elements, the pinned productName, the landing page, and the Lingui i18n loop with its obsolete-entry traps.
---

# Desktop

The `desktop/` section is the Electron application, consuming `@fairspec/library` and
`@fairspec/metadata` from `typescript/`.

## Commands

Run from the repo root; `pnpm desktop <script>` runs a script of this section. Run
`pnpm typescript build` first: the app resolves the workspace packages through their `build/`.

- `pnpm desktop start` — run the app in development
- `pnpm desktop type` — `tsgo --noEmit`
- `pnpm desktop test` — type + unit
- `pnpm desktop unit` — the Vitest tests; `pnpm desktop unit path/to/test.unit.ts` or `pnpm desktop unit -t "test name"` for one
- `pnpm desktop build` / `pnpm desktop compile` — bundle with electron-vite, package with electron-builder into `compile/`
- `pnpm desktop extract` — extract i18n messages
- Lint and format are repo-wide: `pnpm lint`, `pnpm format`

## Structure

- The renderer (React SPA) lives flat at the section root: `routes/`, `components/`,
  `elements/`, `hooks/`, `helpers/`, `services/`, `constants/`, `styles/`, `locales/`
- Electron process code lives under `desktop/processes/main/` and `desktop/processes/preload/`
- `desktop/models/` holds the zod schemas shared by the renderer and the engine — the oRPC wire contract
- The renderer must never import from `#processes/*`, `electron` or `node:*`; the only legal
  crossing is the type-only `Router` import in `desktop/services/engine.ts` (enforced by the
  `no-restricted-imports` override in the root `vite.config.ts`)
- Don't change the shadcn code in `desktop/elements/`
- The landing page is `.livemark/routes/desktop.tsx` at the repo root, published at
  fairspec.org/desktop/. It is independent of the Electron app, English-only and outside the
  i18n workflow
- `desktop/package.json` pins `productName: fairspec`: Electron derives the user-data directory
  from it, so renaming it would orphan every user's settings

## i18n

- Any change that adds or edits a user-facing string must run `pnpm desktop extract` and then
  fill `msgstr` for every new or changed message in all seven non-source catalogues
- `pt` is European Portuguese, never Brazilian
- `lingui extract` reports a Total count that **includes obsolete entries**. A `#~ msgid` whose
  string no longer exists still counts, and grepping `^msgstr ""` will never find it because the
  line is `#~ msgstr ""`. Parse with `@lingui/format-po`'s own `parse()` to see what is genuinely
  untranslated, or run `pnpm desktop extract --clean` to drop obsolete entries
- Long msgids wrap across lines in the PO, so naive text replacement silently misses them. Fill
  those through the same parser and `serialize()` rather than string surgery

## Specs

- Run specs as part of your tasks
- Place unit tests in `<module>.unit.ts` files and don't add useless comments like "Arrange", "Act", "Assert"

## Releases

This section is the `desktop` release-please component: tags `desktop-vX.Y.Z`, its own
Release PR; `release.yaml` builds installers for macOS, Windows and Linux and attaches them to
the GitHub release. Only changes to the shipped application use `feat` or `fix`; the landing
page, tests, READMEs and tooling are `chore` or `docs`. When in doubt, ask whether the change
alters what someone installing the app gets.
