---
title: Fairspec Extension Template
label: Template
path: /extensions/template/
order: 1
---

The [Fairspec Extension Template](https://fairspec.github.io/fairspec-extension/) is a ready-made repository template for building and publishing your own Fairspec extension. It bootstraps the JSON Schema profile, validation tests, and a documentation site so you can focus on the domain-specific properties of your extension.

> [!TIP]
> Click **Use this template** on the [GitHub repository](https://github.com/datisthq/fairspec-extension) to start a new extension in seconds.

## What's included

- A pre-wired JSON Schema profile that extends the base [Fairspec Dataset](/specs/dataset/) schema via `allOf`.
- A test harness that validates example descriptors against the profile on every commit.
- A pre-configured documentation site so you can publish your specification alongside the schema.
- GitHub Actions workflows for CI and publishing the profile on release.

## When to use it

Reach for the template when you need to formalize a new extension — for example, a domain-specific profile for spectroscopy, genomics, or imaging — and want to ship it as a versioned, citable schema rather than ad-hoc JSON.

For more about the extension mechanism itself, see the [Extensions overview](/extensions/).
