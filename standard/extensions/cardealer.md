---
title: Fairspec Cardealer
label: Cardealer
path: /extensions/cardealer/
order: 2
---

[Fairspec Cardealer](https://fairspec.github.io/fairspec-cardealer/) is a reference Fairspec extension that captures the data model of a car dealership — vehicles, listings, dealers, and transactions. It is published as a versioned profile alongside its specification and validation tests, and serves as a concrete example of what a fully-formed Fairspec extension looks like.

> [!TIP]
> Browse the [Cardealer documentation site](https://fairspec.github.io/fairspec-cardealer/) for the full specification, JSON Schema profile, and example datasets.

## Why look at it

- **End-to-end example.** It demonstrates the full lifecycle of an extension — from JSON Schema profile to documentation to release — built on top of the [Fairspec Extension Template](/extensions/template/).
- **Domain-specific properties.** Shows how to add custom fields (vehicle make, model, mileage, listing price) while remaining structurally compatible with the base [Fairspec Dataset](/specs/dataset/) schema.
- **Composition with base profiles.** Illustrates the `allOf` pattern used to combine the base Fairspec dataset profile with a domain mixin.

Use Cardealer as a starting point when designing your own extension, or as a teaching example when explaining the extension mechanism to new contributors.
