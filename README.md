# Monorepo for the Earth Metabolome Initiative

[![Cargo Format](https://github.com/earth-metabolome-initiative/emi-monorepo/actions/workflows/global-cargo-fmt.yml/badge.svg)](https://github.com/earth-metabolome-initiative/emi-monorepo/actions/workflows/global-cargo-fmt.yml)
[![Cargo Check](https://github.com/earth-metabolome-initiative/emi-monorepo/actions/workflows/global-check.yml/badge.svg)](https://github.com/earth-metabolome-initiative/emi-monorepo/actions/workflows/global-check.yml)
[![TOML Format](https://github.com/earth-metabolome-initiative/emi-monorepo/actions/workflows/global-toml-fmt.yml/badge.svg)](https://github.com/earth-metabolome-initiative/emi-monorepo/actions/workflows/global-toml-fmt.yml)

Monorepo for the Earth Metabolome Initiative.

## Crates

As part of the Earth Metabolome Initiative, we developed several crates to help us with our work.
Here follows a table with the crates, and the badges illustrating their status.

### Database-related crates

| Crate | Description | Status | Crates.io |
|-------|-------------|--------|-----------|
| [`diesel_pgrx`](https://github.com/earth-metabolome-initiative/emi-monorepo/tree/main/utils/diesel_pgrx) | [Diesel](https://docs.rs/diesel/latest/diesel/) integration for [PGRX](https://github.com/pgcentralfoundation/pgrx) | [![Test](https://github.com/earth-metabolome-initiative/emi-monorepo/actions/workflows/cargo-test-diesel-pgrx.yml/badge.svg)](https://github.com/earth-metabolome-initiative/emi-monorepo/actions/workflows/cargo-test-diesel-pgrx.yml) | [Tracking issue](https://github.com/earth-metabolome-initiative/emi-monorepo/issues/78) |

### Web-related crates

| Crate | Description | Status | Crates.io |
|-------|-------------|--------|-----------|
| [`media_types`](https://github.com/earth-metabolome-initiative/emi-monorepo/tree/main/web/web_common/media_types) | [Media types](https://en.wikipedia.org/wiki/Media_type) parser | [![Test](https://github.com/earth-metabolome-initiative/emi-monorepo/actions/workflows/cargo-test-media_types.yml/badge.svg)](https://github.com/earth-metabolome-initiative/emi-monorepo/actions/workflows/cargo-test-media_types.yml) | [Tracking issue](https://github.com/earth-metabolome-initiative/emi-monorepo/issues/84) |

### Chemistry-related crates

| Crate | Description | Status | Crates.io |
|-------|-------------|--------|-----------|
| [`molecular_formula`](https://github.com/earth-metabolome-initiative/emi-monorepo/tree/main/web/web_common/molecular_formula) | [Molecular formula](https://en.wikipedia.org/wiki/Molecular_formula) parser | [![Test](https://github.com/earth-metabolome-initiative/emi-monorepo/actions/workflows/cargo-test-molecular_formula.yml/badge.svg)](https://github.com/earth-metabolome-initiative/emi-monorepo/actions/workflows/cargo-test-molecular_formula.yml) | [Tracking issue](https://github.com/earth-metabolome-initiative/emi-monorepo/issues/81) |
| [`cas_codes`](https://github.com/earth-metabolome-initiative/emi-monorepo/tree/main/web/web_common/cas_codes) | [CAS codes](https://en.wikipedia.org/wiki/CAS_Registry_Number) for chemical compounds | [![Test](https://github.com/earth-metabolome-initiative/emi-monorepo/actions/workflows/cargo-test-cas_codes.yml/badge.svg)](https://github.com/earth-metabolome-initiative/emi-monorepo/actions/workflows/cargo-test-cas_codes.yml) | [Tracking issue](https://github.com/earth-metabolome-initiative/emi-monorepo/issues/80) |

### Data structures

## How to contribute

If you'd like to contribute to the project, please read the [CONTRIBUTING.md](CONTRIBUTING.md) file. We are happy to have you here! This is a short guide on how to contribute to the project. If you have any questions, please feel free to open an issue on the GitHub repository and we will be happy to help you.
