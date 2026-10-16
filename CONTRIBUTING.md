---
title: Contributing
---

Thank you for your interest in contributing to Fairspec! As an open standard, contributions from the community help improve the specification and documentation.

## Ways to Contribute

- **Propose Changes**: Suggest improvements to the specification
- **Improve Documentation**: Help clarify the documentation and add examples
- **Share Feedback**: Report issues or inconsistencies you find
- **Join Discussions**: Participate in design discussions about the standard

## Using GitHub Issues

Open an [issue on GitHub](https://github.com/datisthq/fairspec/issues) to:

- Report errors or ambiguities in the specification
- Suggest improvements or new features
- Ask questions about the standard
- Share use cases that aren't well-covered

When creating an issue, please:

- Use a clear, descriptive title
- Provide context and examples when relevant
- Reference specific sections of the specification if applicable

## Using GitHub Discussions

Use [GitHub Discussions](https://github.com/datisthq/fairspec/discussions) for:

- General questions about using Fairspec
- Sharing implementation experiences
- Discussing ideas before proposing formal changes
- Community support and best practices

## Submitting Pull Requests

To propose changes to the specification or documentation:

1. **Fork** the repository on GitHub
2. **Create a branch** for your changes
3. **Make your changes** to the markdown files
4. **Submit a pull request** with a clear description

For significant changes to the specification, please open an issue or discussion first to gather feedback before investing time in a pull request.

### Specification Changes

Proposals for changes to the specification should consider:

- **Backward compatibility**: Avoid breaking existing implementations
- **Clear rationale**: Explain the problem being solved
- **Use cases**: Provide concrete examples
- **Impact**: Consider effects on existing users

## Working on the Code

The repository is a monorepo: the standard lives in `standard/`, the implementations in
`python/` and `typescript/`, and the desktop application in `desktop/`. Clone it and install
everything from the root:

```bash
git clone https://github.com/datisthq/fairspec.git
cd fairspec
pnpm install
```

Run `pnpm test` for the full gate, or `pnpm <section> <script>` for one section — for
example `pnpm python test` or `pnpm typescript unit`. Each section has its own contributing
notes in its directory.

Commits follow [Conventional Commits](https://www.conventionalcommits.org/) scoped by section,
such as `fix(python): …` or `docs(standard): …`. Each section is versioned and released on its
own, so use `feat` and `fix` only for changes to what that section ships.

## Getting Help

- **Issues**: For bug reports and feature requests
- **Discussions**: For questions and general discussions
- **Documentation**: Check the existing docs first

---

Thank you for helping improve Fairspec!
