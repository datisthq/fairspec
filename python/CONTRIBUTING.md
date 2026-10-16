# Contributing

Thank you for your interest in contributing to Fairspec Python! This document provides guidelines and instructions for contributing to this project.

## Project Overview

Project is a monorepo with the following packages:

- `fairspec-metadata`: Core metadata models and descriptors
- `fairspec-dataset`: File, folder and stream I/O plus dataset source plugins
- `fairspec-table`: Polars-backed table engine plus file format plugins
- `fairspec-library`: All the above functionality behind one plugin registry
- `fairspec-terminal`: Terminal interface
- `fairspec`: Meta-package that re-exports the underlying functionality

## Development Environment

### Prerequisites

- **Python**: v3.12 or higher
- **uv**: v0.7.0 or higher
- **Node**: v24.11.0 or higher, with **pnpm** v11 (the repo scripts run through pnpm)

### Setup

1. Clone the repository

   ```bash
   git clone https://github.com/datisthq/fairspec.git
   cd fairspec
   ```

2. Install dependencies
   ```bash
   pnpm install
   ```

## Development Workflow

### Code Style and Quality

We use Ruff for linting and formatting, and ty for type checking:

- **Lint**: Check for code issues

  ```bash
  pnpm -C python lint
  ```

- **Format**: Auto-fix formatting issues

  ```bash
  pnpm -C python format
  ```

- **Type Check**: Verify Python types
  ```bash
  pnpm -C python type
  ```

### Testing

Tests are collocated with the code in `_test` directories and use pytest:

- **Run All Tests**: (includes linting and type checking)

  ```bash
  pnpm -C python test
  ```

- **Run Tests Only**: (without linting/type checking)

  ```bash
  pnpm -C python unit
  ```

- **Run a Specific Test**:
  ```bash
  pnpm -C python unit metadata/fairspec_metadata/actions/descriptor/_test/load_unit.py
  ```

## Code Style Guidelines

- Use Python with strict type checking
- Target Python 3.12+
- Tests should be placed in `*_unit.py` files inside a `_test` directory next to the code,
  together with any `fixtures` they need
- Use 4-space indentation, UTF-8 encoding, and LF line endings
- Use PascalCase for classes, snake_case for functions, methods, and variables

See the root `AGENTS.md` and the `python-development` skill in `.agents/skills/` for the full
set of conventions.

## Making Changes to the Meta-Package

When adding new functionality:

1. Add it to the appropriate package first
2. Ensure it's properly exported from that package
3. No additional work is needed for the meta-package as it automatically re-exports everything

## Submitting Changes

1. Create a feature branch (`git checkout -b feature/your-feature`)
2. Make your changes with appropriate tests
3. Ensure the code passes all checks: `pnpm -C python test`
4. Commit your changes with a descriptive message
5. Submit a pull request

## License

By contributing to Fairspec Python, you agree that your contributions will be licensed under the project's license.

Thank you for your contribution!
