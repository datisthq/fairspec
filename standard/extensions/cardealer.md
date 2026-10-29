---
title: Fairspec Cardealer
label: Cardealer
path: /extensions/cardealer/
order: 2
---

[Fairspec Cardealer](https://datisthq.github.io/cardealer/) is a reference Fairspec extension that captures the data model of a car dealership — the dealer, its facilities, and its car listings — as one Fairspec dataset with typed tables. It is published as a versioned profile and table schemas alongside its specification, conformance tests, and TypeScript, Python, and Rust libraries, and serves as a concrete example of what a fully-formed Fairspec extension looks like.

> [!TIP]
> Browse the [Cardealer documentation site](https://datisthq.github.io/cardealer/) for the full specification, profile, table schemas, and example datasets.

## Why look at it

- **End-to-end example.** It demonstrates the full lifecycle of an extension — from JSON Schema profile to documentation to release.
- **Domain-specific properties.** Shows how to add domain tables (`dealer`, `facility`, `car`) with custom columns such as brand, model, mileage, and price while remaining structurally compatible with the base [Fairspec Dataset](/specs/dataset/) schema.
- **Libraries on the Fairspec base.** Its TypeScript, Python, and Rust libraries extend the Fairspec metadata packages rather than re-modelling Fairspec fields.
- **Composition with base profiles.** Illustrates the `allOf` pattern used to combine the base Fairspec dataset profile with a domain mixin.

Use Cardealer as a starting point when designing your own extension, or as a teaching example when explaining the extension mechanism to new contributors.
