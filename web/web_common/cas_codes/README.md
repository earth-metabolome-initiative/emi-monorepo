# CAS Codes

[![Crates.io](https://img.shields.io/crates/v/cas_codes)](https://crates.io/crates/cas_codes)
[![Documentation](https://docs.rs/cas_codes/badge.svg)](https://docs.rs/cas_codes)
[![PGRX Build](https://github.com/earth-metabolome-initiative/emi-monorepo/actions/workflows/pgrx-build-cas_codes.yml/badge.svg)](https://github.com/earth-metabolome-initiative/emi-monorepo/actions/workflows/pgrx-build-cas_codes.yml)
[![Clippy](https://github.com/earth-metabolome-initiative/emi-monorepo/actions/workflows/cargo-clippy-cas_codes.yml/badge.svg)](https://github.com/earth-metabolome-initiative/emi-monorepo/actions/workflows/cargo-clippy-cas_codes.yml)
[![Test](https://github.com/earth-metabolome-initiative/emi-monorepo/actions/workflows/cargo-test-cas_codes.yml/badge.svg)](https://github.com/earth-metabolome-initiative/emi-monorepo/actions/workflows/cargo-test-cas_codes.yml)

A Rust library for parsing and validating [Chemical Abstracts Service (CAS) Registry Numbers](https://en.wikipedia.org/wiki/CAS_Registry_Number).

## Features

- Parse and validate CAS numbers with checksum verification
- Zero-cost abstractions (stored as `u32`)
- Optional Serde, `PostgreSQL`, and Diesel integration

## Installation

```toml
[dependencies]
cas_codes = "0.1.0"

# With optional features
cas_codes = { version = "0.1.0", features = ["serde", "pgrx"] }
```

## Usage

```rust
use cas_codes::CAS;

let cas = CAS::try_from("7732-18-5")?; // Water
assert_eq!(cas.first(), 7732);
assert_eq!(cas.second(), 18);
assert_eq!(cas.check_digit(), 5);
assert_eq!(cas.to_string(), "7732-18-5");

# Ok::<(), cas_codes::errors::Error>(())
```

## `PostgreSQL` Extension

The crate can be built as a `PostgreSQL` extension using PGRX:

```toml
[dependencies]
cas_codes = { version = "0.1.0", features = ["pgrx", "pg17"] }
```

**Note**: Currently uses `PostgreSQL`'s varlena type since PGRX doesn't yet support custom fixed-size types. Future PGRX versions will enable more efficient fixed-size storage.

### Using the Extension

**Step 1:** Build the extension:

```bash
USER_ID=$(id -u) GROUP_ID=$(id -g) docker compose up
```

**Step 2:** Copy extension files to PostgreSQL:

```bash
# Copy the shared library
sudo cp extension/usr/lib/postgresql/17/lib/cas_codes.so /usr/lib/postgresql/17/lib/

# Copy the control file  
sudo cp extension/usr/share/postgresql/17/extension/cas_codes.control /usr/share/postgresql/17/extension/

# Copy the SQL file
sudo cp extension/usr/share/postgresql/17/extension/cas_codes--0.1.0.sql /usr/share/postgresql/17/extension/
```

**Step 3:** Enable in PostgreSQL:

```sql
CREATE EXTENSION IF NOT EXISTS cas_codes;
```

**Step 4:** Use the CAS type:

```sql
-- Create a table with CAS numbers
CREATE TABLE reagents (
    id SERIAL PRIMARY KEY,
    name TEXT NOT NULL,
    cas_number CAS NOT NULL UNIQUE,
    purity REAL CHECK (purity > 0 AND purity <= 100)
);
```

### Diesel ORM Integration

For use with Diesel ORM:

```toml
[dependencies]
cas_codes = { version = "0.1.0", features = ["postgres"] }
```

Example Diesel table definition:

```rust
#[cfg(feature = "diesel")]
diesel::table! {
    reagents(id) {
        id -> diesel::sql_types::Integer,
        name -> diesel::sql_types::Text,
        purity -> diesel::sql_types::Float,
        cas_code -> ::cas_codes::diesel_impls::CAS,
    }
}
```

## License

MIT Licensed.
