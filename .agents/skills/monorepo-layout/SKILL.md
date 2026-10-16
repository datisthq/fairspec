---
name: monorepo-layout
description: Where things live in the Fairspec monorepo and how root scripts, sections, workflows and release components fit together. Use when adding a section, a root or section script, a CI job, a release component, or a page to the website.
---

# Monorepo layout

One repo, five sections, one site. The root owns everything shared; a section owns only what
is specific to it.

## Root vs section

| lives at the root                                                           | lives in a section                                |
| --------------------------------------------------------------------------- | ------------------------------------------------- |
| `package.json` scripts, `pnpm-workspace.yaml` + its `catalog:`              | the section's `package.json` and its deps         |
| one `pnpm-lock.yaml`                                                        | `python/uv.lock` (uv owns Python resolution)      |
| `vite.config.ts` fmt + lint for every file                                  | `vite.config.ts` with a `test` block only         |
| `tsconfig.json` (root files only), `lefthook.yaml`, `.gitleaks.toml`        | `tsconfig.json`, nested `.gitignore`              |
| `.github/`, `.release/`, `.agents/`, `.humans/`, `.scripts/`                | `README.md`, `CONTRIBUTING.md`, `CHANGELOG.md`    |
| the one `AGENTS.md`                                                         | its `<section>-development` skill (in `.agents/`) |
| the site: `livemark.config.ts`, `.livemark/`, `worker.ts`, `wrangler.jsonc` | the Markdown the site includes                    |

vite-plus reads `fmt` and `lint` from the root config only; a nested block is silently
ignored, so path-specific lint rules go in the root `overrides` with a section prefix
(`desktop/routes/**`).

The root `vite.config.ts` also has a `test` block, whose `include` is only
`.scripts/-test/*.unit.ts`: repo-wide guards that belong to no section, today the docs guard.
Root `vp test` runs them; root `test` and `unit` call it first, and CI runs it in the always-on
`lint` job, so a docs-only PR is still checked. A section's `vp test` uses its own config and
never picks them up. Root `tsconfig.json` includes `.scripts/` so `pnpm type` covers them.

## Scripts

- `pnpm <section> <script>` is `pnpm -F ./<section> run`; arguments pass through
- Root `build`, `type`, `unit` and `test` chain sections **explicitly, in dependency order**
  (`typescript` before `desktop`). Never switch them to `pnpm -r`: that runs every
  `typescript/*` package directly _and_ through the `typescript` group package, building twice,
  and gives no order between the group and desktop
- pnpm runs scripts through `cmd.exe` on Windows, where single quotes are literal: quote
  filters and globs with escaped double quotes (`pnpm -F \"./*\" run build`), never `'./*'`,
  or the filter silently matches nothing and CI fails later with unresolved packages
- A section's `test` never lints — root `test` runs `pnpm lint` once for the whole tree
- Python is reached only through `python/package.json`, whose scripts call `uv run …` from
  `python/`. There is no taskipy; pytest, ty, ruff and coverage find their config in
  `python/pyproject.toml` because they run from there
- `prepare` runs `pnpm python setup` only when `uv` is on the PATH, so Node-only CI jobs work

## Adding a section

1. Create `<section>/package.json` (private, `@fairspec/<section>`) with `test`/`type`/`unit`
   scripts as they apply; add it to `pnpm-workspace.yaml` `packages`
2. Add a root script `"<section>": "pnpm -F ./<section> run"` and add the section to the
   root `build`/`type`/`unit`/`test` chains in dependency order
3. Add a `changes` filter and a gated job in `.github/workflows/test.yaml`, and list the job
   in the final `test` job's `needs`
4. If it ships, add a package to `.release/config.json` (component = section name, tags
   `<section>-vX.Y.Z`) and an entry to `.release/manifest.json`, plus a publish job in
   `release.yaml` gated on `steps.release.outputs['<section>--release_created']`
5. Add a row to the section tables in the root `AGENTS.md` and `README.md`, add the section to
   `SECTIONS` in `.scripts/-test/docs.unit.ts`, and write its conventions into a
   `.agents/skills/<section>-development/` skill listed in the `AGENTS.md` skills table — never a
   section `AGENTS.md`, which the docs guard rejects

## Releases

- `separate-pull-requests: true`: one Release PR per section, on branch
  `release-please--branches--main--components--<section>`
- `extra-files` paths are relative to the section directory
- `release-as` pins one component's next version (typescript is pinned to `0.13.1` because
  npm already holds a partial `0.13.0`); delete the key once that release is merged, or every
  later Release PR keeps proposing the same version
- The standard's Release PR is the one `release.yaml` regenerates profiles on
  (`pnpm standard generate` writes `.livemark/public/profiles/{latest,<version>}`)
- npm and PyPI trusted publishing is bound to repo `datisthq/fairspec`, workflow
  `release.yaml` and environment `release` — renaming any of them breaks publishing

## Website

- One site on fairspec.org. Each language is an article section under its prefix
  (`/python/`, `/typescript/`); library pages live under `/<lang>/library/`, CLI pages under
  `/<lang>/terminal/`. A doc's URL is its `path:` frontmatter, not its file location
- No article sits at a section prefix: livemark nests every article under the one at its
  parent path, so a README at `/python/` would swallow the whole sidebar. The language READMEs
  live at `/<lang>/getting-started/`, and `worker.ts` 301s the bare `/<lang>/` there
- Landing pages are routes in `.livemark/routes/` (`index.tsx`, `cli-tool.tsx`,
  `mcp-server.tsx`, `desktop.tsx`), each a `type: "custom"` section in `livemark.config.ts`
  in that nav order. A landing that covers both languages (`cli-tool.tsx`) puts a
  Python/TypeScript switch over the install-method tabs and shows only commands that run
  unchanged on both CLIs — check each against both `--help` outputs, since the language
  docs can lag the code (both CLIs take `-d/--dataset`, `-r/--resource`)
- A new route blanks on hydration under `pnpm site:start` until a hard reload: livemark's
  target lives in `node_modules`, so Vite serves `routeTree.gen.ts?v=<hash>` as `immutable`
  and the browser keeps the tree without the route. Not a bug in the route; builds are fine
- Changelogs are articles, not `type: "changelog"` sections: each article section includes its
  `<section>/CHANGELOG.md` and patches it to `<prefix>changelog/` with `order: -1`, so it is
  the last item of that section's sidebar. One page per changelog, no per-version pages
- Cloudflare Workers Builds deploys the site. `wrangler.jsonc` `build.command` runs
  `pnpm site:build` before every `wrangler versions upload`, so the deploy never depends on the
  dashboard's build command; root `pnpm build` builds the packages, not the site
- No section sets `siteLink`; the site title always links to `/`
- The retired subdomains (`python.`, `typescript.`, `application.`) are custom domains of the
  same Worker; `worker.ts` 301s them to the mapped path. When a doc's path changes, add the old
  path to `mapPath` if the old URL was ever public
