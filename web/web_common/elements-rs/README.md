# Elements

[![PGRX Build](https://github.com/earth-metabolome-initiative/emi-monorepo/actions/workflows/pgrx-build-elements.yml/badge.svg)](https://github.com/earth-metabolome-initiative/emi-monorepo/actions/workflows/pgrx-build-elements.yml)
[![Clippy](https://github.com/earth-metabolome-initiative/emi-monorepo/actions/workflows/cargo-clippy-elements.yml/badge.svg)](https://github.com/earth-metabolome-initiative/emi-monorepo/actions/workflows/cargo-clippy-elements.yml)
[![Test](https://github.com/earth-metabolome-initiative/emi-monorepo/actions/workflows/cargo-test-elements.yml/badge.svg)](https://github.com/earth-metabolome-initiative/emi-monorepo/actions/workflows/cargo-test-elements.yml)

A comprehensive Rust crate providing type-safe enumerations and rich metadata for all [chemical elements](https://en.wikipedia.org/wiki/Chemical_element) of the [periodic table](https://en.wikipedia.org/wiki/Periodic_table) and their [isotopes](https://en.wikipedia.org/wiki/Isotope). The crate includes all 118 elements from Hydrogen to Oganesson with detailed information for each [isotope](https://en.wikipedia.org/wiki/Isotope) including [mass numbers](https://en.wikipedia.org/wiki/Mass_number), [relative atomic masses](https://en.wikipedia.org/wiki/Relative_atomic_mass), [isotopic composition](https://en.wikipedia.org/wiki/Natural_abundance), and identification of the most abundant isotope. Chemical properties are fully supported: [standard atomic weights](https://en.wikipedia.org/wiki/Standard_atomic_weight), [oxidation states](https://en.wikipedia.org/wiki/Oxidation_state) with validation, [valence electrons](https://en.wikipedia.org/wiki/Valence_electron), bond numbers, [electron configurations](https://en.wikipedia.org/wiki/Electron_configuration) with [atomic orbitals](https://en.wikipedia.org/wiki/Atomic_orbital), and [principal quantum numbers](https://en.wikipedia.org/wiki/Principal_quantum_number). Optional features provide database integration through Diesel (PostgreSQL and SQLite), PostgreSQL extension support via PGRX, and serialization through serde.

## Usage

Add this to your `Cargo.toml`:

```toml
[dependencies]
elements = "0.1"
```

### Basic Example

```rust
use elements::{Element, BondsNumber, ValenceElectrons};

// Get an element
let carbon = Element::C;

// Access properties
println!("Name: {}", carbon.name());
println!("Atomic weight: {}", carbon.standard_atomic_weight());
println!("Valence electrons: {}", carbon.valence_electrons());

// Check oxidation states
assert!(carbon.is_valid_oxidation_state(4));
assert!(carbon.is_valid_oxidation_state(-4));

// Get bond information
let (min_bonds, max_bonds) = carbon.number_of_bonds();
assert_eq!((min_bonds, max_bonds), (4, 4));
```

### Working with Isotopes

```rust
use elements::isotopes::{CarbonIsotope, Isotope, MassNumber, RelativeAtomicMass};

// Get a specific isotope
let carbon_12 = CarbonIsotope::C12;
println!("Mass number: {}", carbon_12.mass_number());
println!("Relative atomic mass: {}", carbon_12.relative_atomic_mass());

// Work with the generic Isotope enum
let isotope = Isotope::C(carbon_12);
```

### Parsing from Strings

```rust
use elements::Element;
use std::str::FromStr;

// Parse element symbols
let oxygen = Element::from_str("O").unwrap();
let nitrogen = Element::from_str("N").unwrap();
```

## Feature Flags

- `serde` (default): Enables `Serialize` and `Deserialize` implementations for `Element` and `Isotope` types
- `diesel`: Adds Diesel ORM trait implementations for database queries
- `sqlite`: Enables SQLite-specific type mappings (requires `diesel`)
- `postgres`: Enables PostgreSQL-specific type mappings (requires `diesel`)
- `diesel_pgrx`: Integrates Diesel with PGRX for PostgreSQL extension development (requires `diesel`)
- `pgrx`: Enables PostgreSQL extension API through PGRX
- `pg13`, `pg14`, `pg15`, `pg16`, `pg17`: Select specific PostgreSQL version (requires `pgrx`, mutually exclusive)

## Compiling the PGRX Extension

After cloning the repository, you can compile the PGRX extension in the `./extension` directory by running:

```bash
USER_ID=$(id -u) GROUP_ID=$(id -g) docker compose up
```

Note: The `USER_ID` and `GROUP_ID` environment variables ensure proper file permissions when mounting volumes, avoiding root-owned files on the host system.

## License

This project is licensed under the same terms as the parent EMI monorepo.
