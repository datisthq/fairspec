# Fairspec

Fairspec is an open standard for exchanging FAIR data, together with the software that
implements it. Read the documentation at [fairspec.org](https://fairspec.org).

This repository holds every part of the project:

| section                      | what                                                     |
| ---------------------------- | -------------------------------------------------------- |
| [`standard/`](standard/)     | the specification, its JSON Schema profiles and examples |
| [`python/`](python/)         | the Python framework and CLI, published to PyPI          |
| [`typescript/`](typescript/) | the TypeScript framework, CLI and MCP server, on npm     |
| [`desktop/`](desktop/)       | the desktop application for macOS, Windows and Linux     |
| [`catalog/`](catalog/)       | reserved for the catalog application                     |

## Development

Requires Node `^24.11`, pnpm `^11` and, for the Python section, [uv](https://docs.astral.sh/uv/).

```bash
git clone https://github.com/datisthq/fairspec.git
cd fairspec
pnpm install
pnpm test
```

Every command runs from the root: `pnpm <verb>` covers all sections, and
`pnpm -C <section> <script>` runs one of them, for example `pnpm -C python test` or
`pnpm -C desktop start`. See [`CONTRIBUTING.md`](CONTRIBUTING.md) for details. Coding agents
read [`AGENTS.md`](AGENTS.md) and the skills in `.agents/skills/`.

## License

MIT — see [`LICENSE.md`](LICENSE.md).
