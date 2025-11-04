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
| [`diesel_pgrx`](https://github.com/earth-metabolome-initiative/emi-monorepo/tree/main/utils/diesel_pgrx) | Diesel ORM integration for PGRX PostgreSQL extensions | [![Test](https://github.com/earth-metabolome-initiative/emi-monorepo/actions/workflows/cargo-test-diesel-pgrx.yml/badge.svg)](https://github.com/earth-metabolome-initiative/emi-monorepo/actions/workflows/cargo-test-diesel-pgrx.yml) | [![Crates.io](https://img.shields.io/crates/v/diesel_pgrx.svg)](https://crates.io/crates/diesel_pgrx) |
| [`diesel_pgrx_derive`](https://github.com/earth-metabolome-initiative/emi-monorepo/tree/main/utils/diesel_pgrx_derive) | Derive macros for diesel_pgrx | [![Test](https://github.com/earth-metabolome-initiative/emi-monorepo/actions/workflows/cargo-test-diesel-pgrx.yml/badge.svg)](https://github.com/earth-metabolome-initiative/emi-monorepo/actions/workflows/cargo-test-diesel-pgrx.yml) | [![Crates.io](https://img.shields.io/crates/v/diesel_pgrx_derive.svg)](https://crates.io/crates/diesel_pgrx_derive) |

### Web-related crates

| Crate | Description | Status | Crates.io |
|-------|-------------|--------|-----------|
| [`media_types`](https://github.com/earth-metabolome-initiative/emi-monorepo/tree/main/web/web_common/media_types) | [Media types](https://en.wikipedia.org/wiki/Media_type) parser | [![Test](https://github.com/earth-metabolome-initiative/emi-monorepo/actions/workflows/cargo-test-media_types.yml/badge.svg)](https://github.com/earth-metabolome-initiative/emi-monorepo/actions/workflows/cargo-test-media_types.yml) | [Tracking issue](https://github.com/earth-metabolome-initiative/emi-monorepo/issues/84) |

### Chemistry-related crates

| Crate | Description | Status | Crates.io |
|-------|-------------|--------|-----------|
| [`elements_rs`](https://github.com/earth-metabolome-initiative/emi-monorepo/tree/main/web/web_common/elements_rs) | [Chemical elements](https://en.wikipedia.org/wiki/Chemical_element) and their [isotopes](https://en.wikipedia.org/wiki/Isotope) | [![Test](https://github.com/earth-metabolome-initiative/emi-monorepo/actions/workflows/cargo-test-elements_rs.yml/badge.svg)](https://github.com/earth-metabolome-initiative/emi-monorepo/actions/workflows/cargo-test-elements_rs.yml) | [![Crates.io](https://img.shields.io/crates/v/elements_rs.svg)](https://crates.io/crates/elements_rs) |
| [`molecular_formulas`](https://github.com/earth-metabolome-initiative/emi-monorepo/tree/main/web/web_common/molecular_formulas) | [Molecular formula](https://en.wikipedia.org/wiki/Molecular_formula) parser | [![Test](https://github.com/earth-metabolome-initiative/emi-monorepo/actions/workflows/cargo-test-molecular_formulas.yml/badge.svg)](https://github.com/earth-metabolome-initiative/emi-monorepo/actions/workflows/cargo-test-molecular_formulas.yml) | [Tracking issue](https://github.com/earth-metabolome-initiative/emi-monorepo/issues/81) |
| [`cas_codes`](https://github.com/earth-metabolome-initiative/emi-monorepo/tree/main/web/web_common/cas_codes) | [CAS codes](https://en.wikipedia.org/wiki/CAS_Registry_Number) for chemical compounds | [![Test](https://github.com/earth-metabolome-initiative/emi-monorepo/actions/workflows/cargo-test-cas_codes.yml/badge.svg)](https://github.com/earth-metabolome-initiative/emi-monorepo/actions/workflows/cargo-test-cas_codes.yml) | [Tracking issue](https://github.com/earth-metabolome-initiative/emi-monorepo/issues/80) |

### Data structures

| Crate | Description | Status | Crates.io |
|-------|-------------|--------|-----------|
| [`multi_ranged`](https://github.com/earth-metabolome-initiative/emi-monorepo/tree/main/data_structures/multi_ranged) | Efficient data structures for representing ranges of discrete values | [![Test](https://github.com/earth-metabolome-initiative/emi-monorepo/actions/workflows/cargo-test-multi_ranged.yml/badge.svg)](https://github.com/earth-metabolome-initiative/emi-monorepo/actions/workflows/cargo-test-multi_ranged.yml) | [![Crates.io](https://img.shields.io/crates/v/multi_ranged.svg)](https://crates.io/crates/multi_ranged) |

## How to contribute

If you'd like to contribute to the project, please read the [CONTRIBUTING.md](CONTRIBUTING.md) file. We are happy to have you here! This is a short guide on how to contribute to the project. If you have any questions, please feel free to open an issue on the GitHub repository and we will be happy to help you.
