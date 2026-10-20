---
name: monorepo-layout
description: Where things live in the Fairspec monorepo and how root scripts, sections, workflows and release components fit together. Use when adding a section, a root or section script, a CI job, a release component, or a page to the website.
---

# Monorepo layout

One repo, six sections, one site. The root owns everything shared; a section owns only what
is specific to it.

## Root vs section

| lives at the root                                                           | lives in a section                                |
| --------------------------------------------------------------------------- | ------------------------------------------------- |
| `package.json` scripts, `pnpm-workspace.yaml` + its `catalog:`              | the section's `package.json` and its deps         |
| one `pnpm-lock.yaml`                                                        | `python/uv.lock` (uv owns Python resolution)      |
| `vite.config.ts` fmt + lint for every file                                  | `vite.config.ts` with a `test` block only         |
| `tsconfig.json` (root files only), `lefthook.yaml`, `.gitleaks.toml`        | `tsconfig.json`, nested `.gitignore`              |
| `.github/`, `.release/`, `.agents/`, `.humans/`, `.scripts/`, `.skills/`    | `README.md`, `CONTRIBUTING.md`, `CHANGELOG.md`    |
| the one `AGENTS.md`                                                         | its `<section>-development` skill (in `.agents/`) |
| the site: `livemark.config.ts`, `.livemark/`, `worker.ts`, `wrangler.jsonc` | the Markdown the site includes                    |

Skills come in two kinds. `.agents/skills/` teaches agents to develop this repo; only it is
symlinked into `.claude/skills` and listed in `AGENTS.md`. `.skills/` holds the `fairspec-*`
skills users install to work with Fairspec. Every command, flag and API name in them must match
both CLIs' `--help` and both libraries, so a change to either surface checks them too.

vite-plus reads `fmt` and `lint` from the root config only; a nested block is silently
ignored, so path-specific lint rules go in the root `overrides` with a section prefix
(`desktop/routes/**`).

The root `vite.config.ts` also has a `test` block, whose `include` is only
`.scripts/-test/*.unit.ts`: repo-wide guards that belong to no section, today the docs guard
and the published-profiles guard.
Root `vp test` runs them; root `test` and `test:unit` call it first, and CI runs it in the always-on
`lint` job, so a docs-only PR is still checked. A section's `vp test` uses its own config and
never picks them up. Root `tsconfig.json` includes `.scripts/` so `pnpm check:types` covers them.

## Scripts

- `pnpm -C <section> <script>` runs one section's script from the root; arguments pass
  through. There are no per-section alias scripts at the root. Use `-C`, not
  `-F ./<section>`: a filter that matches nothing exits 0 having run nothing, while `-C` on a
  wrong directory fails
- Script names follow the `AGENTS.md` rule, `verb[:object]`, as in the other Datist repos —
  never `<target>:<verb>`. Keys stay in alphabetical order
- `pnpm squash` (`.scripts/squash-branch.ts`) lands a worktree branch as one commit on local
  `main`; procedure in **`worktree-workflow`**
- Root `build`, `check:types`, `test:unit` and `test` chain sections **explicitly, in dependency order**
  (`typescript` before `desktop`). Never switch them to `pnpm -r`: that runs every
  `typescript/*` package directly _and_ again through the `typescript` group package's
  fan-out, so each runs twice. Its order is not the problem (desktop's `workspace:*` deps put
  it after `library`); excluding the group needs a negated filter in every chain, the Windows
  quoting trap below
- pnpm runs scripts through `cmd.exe` on Windows, where single quotes are literal: quote
  filters and globs with escaped double quotes (`pnpm -F \"./*\" run build`), never `'./*'`,
  or the filter silently matches nothing and CI fails later with unresolved packages
- A section's `test` never lints — root `test` runs `pnpm check:lint` once for the whole tree
- Python is reached only through `python/package.json`, whose scripts call `uv run …` from
  `python/`. There is no taskipy; pytest, ty, ruff and coverage find their config in
  `python/pyproject.toml` because they run from there
- `prepare` runs `pnpm -C python setup` only when `uv` is on the PATH, so Node-only CI jobs work

## Adding a section

1. Create `<section>/package.json` (private, `@fairspec/<section>`) with `test`/`check:types`/`test:unit`
   scripts as they apply; add it to `pnpm-workspace.yaml` `packages`
2. Add the section to the root `build`/`check:types`/`test:unit`/`test` chains as
   `pnpm -C <section> <script>`, in dependency order
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

- `separate-pull-requests: false`: one combined Release PR for every section, on branch
  `release-please--branches--main`. Merging it tags and publishes each section it lists, so
  the standard and the libraries that bundle its profiles always ship together; a
  library-only fix is a Release PR listing only that library
- `extra-files` paths are relative to the section directory
- `release-as` pins one component's next version (typescript is pinned to `0.13.1` because
  npm already holds a partial `0.13.0`); delete the key once that release is merged, or every
  later Release PR keeps proposing the same version
- The libraries bundle `standard/profiles/` (refs resolved to `latest`) in
  `typescript/metadata/profiles/` and `python/metadata/fairspec_metadata/profiles/`, and pin
  `FAIRSPEC_VERSION` in their `settings` to `standard/package.json`'s version.
  `pnpm sync:profiles` (`.scripts/@sync.ts`) writes both; the guard fails while either drifts
- `release.yaml`'s `profiles` job runs on the Release PR: `pnpm generate:profiles`
  (`.scripts/@generate.ts`) writes `.livemark/public/profiles/{latest,<version>}`, then
  `pnpm sync:profiles` bumps the libraries' `FAIRSPEC_VERSION`. When the standard is not in
  the release its version already has checksums, so the generator skips and nothing changes
- The Release PR runs no CI (its pushes use `GITHUB_TOKEN`), so the published-profiles guard
  (`.scripts/-test/profiles.unit.ts`) gates it after the merge instead: the folder for
  `standard/package.json`'s version must exist and `latest/` must equal it. Merging before
  the profiles commit lands fails the post-merge `test` job, and release-please never tags
- A published `<version>/` folder never changes. `.livemark/public/profiles/checksums.json`
  records every file's sha256, the guard fails on any mismatch, and the generator skips a
  version that already has an entry, so it writes only a new version's. Changing an existing
  entry is the one way through, so review it as such
- `release.yaml` runs one at a time (`concurrency`, `cancel-in-progress: false`). Never let it
  cancel: release-please tags before the publish jobs run, and a later run only publishes
  what it tagged itself, so a run cancelled in between leaves a tag that never ships
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
  `mcp-server.tsx`, `agent-skills.tsx`, `desktop.tsx`), each a `type: "custom"` section in
  `livemark.config.ts` in that nav order. A landing that covers both languages (`cli-tool.tsx`)
  puts a Python/TypeScript switch over the install-method tabs and shows only commands that run
  unchanged on both CLIs — check each against both `--help` outputs, since the language
  docs can lag the code (both CLIs take `-d/--dataset`, `-r/--resource`)
- Landings follow livemark.dev's own landing (livemark's `livemark-landing` skill). A custom
  route renders bare, so each one draws its chrome: `<Banner />`, `<Header />`, `<main>`,
  `<BackToTop />`. Their shared pieces (`Hero`, `LandingSection`, `WindowCard`, `Reveal`, `tk`,
  `Datist`…) live in `.livemark/components/Landing.tsx`, imported relatively: a new file under
  `.livemark/components/` must not take a livemark component's name (`Section.tsx` would
  replace livemark's), and `livemark/components/<new>` resolves in the build but not for `tsgo`
- `.livemark/.target/` is livemark's generated mount, gitignored and excluded from types and
  fmt. `.livemark/styles/general.css` is an escape of livemark's: when a livemark upgrade adds
  tokens (the build fails on an unknown utility such as `bg-positive/15`), re-copy it from
  `node_modules/livemark/source/styles/general.css` and keep only the bordered `.prose table`
  rules at its end
- The `fairspec-*` skills in `.skills/` are published by the site (`skills:` in
  `livemark.config.ts`) under `/.well-known/agent-skills/`, so `npx skills add https://fairspec.org`
  installs them; `/agent-skills/` is their landing
- Changelogs are articles, not `type: "changelog"` sections: each article section includes its
  `<section>/CHANGELOG.md` and patches it to `<prefix>changelog/` with `order: -1`, so it is
  the last item of that section's sidebar. One page per changelog, no per-version pages
- Cloudflare Workers Builds deploys the site. `wrangler.jsonc` `build.command` runs
  `pnpm build:site` before every `wrangler versions upload`, so the deploy never depends on the
  dashboard's build command; root `pnpm build` builds the packages, not the site
- No section sets `siteLink`; the site title always links to `/`
- The retired subdomains (`python.`, `typescript.`, `application.`) are custom domains of the
  same Worker; `worker.ts` 301s them to the mapped path. When a doc's path changes, add the old
  path to `mapPath` (subdomain URLs) or `MOVED` (fairspec.org URLs) if the old URL was ever
  public; any `<prefix>changelog/<version>/` 301s to its one changelog page
- Google Search Console (domain property `fairspec.org`) is the list of URLs that were ever
  public: before removing or renaming pages, replay its indexed and not-indexed URLs through
  `worker.ts` against the `.livemark/build/client` build and fix any that end on a 404
