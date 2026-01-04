# Diesel PGRX

[![Clippy](https://github.com/earth-metabolome-initiative/emi-monorepo/actions/workflows/cargo-clippy-diesel-pgrx.yml/badge.svg)](https://github.com/earth-metabolome-initiative/emi-monorepo/actions/workflows/cargo-clippy-diesel-pgrx.yml)
[![Test](https://github.com/earth-metabolome-initiative/emi-monorepo/actions/workflows/cargo-test-diesel-pgrx.yml/badge.svg)](https://github.com/earth-metabolome-initiative/emi-monorepo/actions/workflows/cargo-test-diesel-pgrx.yml)

Integration layer enabling [Diesel ORM](https://diesel.rs/) to work with custom Rust types defined in [PGRX](https://github.com/pgcentralfoundation/pgrx) [PostgreSQL extensions](https://www.postgresql.org/docs/current/extend.html). The crate provides a derive macro that implements Diesel's serialization traits ([`ToSql`](https://docs.rs/diesel/latest/diesel/serialize/trait.ToSql.html) and [`FromSql`](https://docs.rs/diesel/latest/diesel/deserialize/trait.FromSql.html)) using PGRX's [binary protocol](https://www.postgresql.org/docs/current/protocol.html), which internally uses [CBOR serialization](https://en.wikipedia.org/wiki/CBOR) via [`serde_cbor`](https://docs.rs/serde_cbor/latest/serde_cbor/). This allows types decorated with PGRX's [`PostgresType`](https://docs.rs/pgrx/latest/pgrx/datum/trait.PostgresType.html) and `#[pg_binary_protocol]` to be seamlessly queried through Diesel. Additional backend support includes SQLite using the same CBOR serialization format.

## Usage

Add this to your `Cargo.toml`:

```toml
[dependencies]
diesel_pgrx = "0.1"
diesel = { version = "2.2", features = ["postgres"] }
serde = { version = "1.0", features = ["derive"] }
```

After building your PGRX extension, install it in PostgreSQL:

```sql
CREATE EXTENSION your_extension;
```

Derive `DieselPGRX` for types that implement `serde::Serialize` and `serde::Deserialize`. The type must also use PGRX's `PostgresType` and `pg_binary_protocol` attributes:

```rust
#[derive(
    Debug,
    serde::Serialize,
    serde::Deserialize,
    diesel_pgrx::DieselPGRX,
    diesel::FromSqlRow,
    diesel::AsExpression
)]
#[cfg_attr(feature = "pgrx", derive(pgrx::PostgresType))]
#[cfg_attr(feature = "pgrx", pg_binary_protocol)]
#[diesel(sql_type = diesel_impls::MyCustomType)]
pub struct MyCustomType {
    pub field: i32,
}
```

## Additional PGRX Derives

PGRX provides optional derives for comparison and hashing operations. When using these, implement the corresponding Rust traits:

```rust
#[derive(serde::Serialize, serde::Deserialize, diesel_pgrx::DieselPGRX)]
#[cfg_attr(
    feature = "pgrx",
    derive(pgrx::PostgresType, pgrx::PostgresEq, pgrx::PostgresOrd, pgrx::PostgresHash)
)]
#[cfg_attr(feature = "pgrx", pg_binary_protocol)]
#[derive(
    Debug,
    Copy,
    Clone,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    Hash,
    diesel::FromSqlRow,
    diesel::AsExpression,
)]
#[diesel(sql_type = diesel_impls::MyCustomType)]
pub struct MyCustomType {
    pub field: i32,
}
```

## Feature Flags

- `postgres` (not default): Enables PostgreSQL backend support with binary protocol serialization
- `sqlite` (not default): Enables SQLite backend support using CBOR-encoded BINARY type
- `pgrx`: Used for documentation tests only

## Note on serde_cbor

The `serde_cbor` crate is archived but remains the CBOR implementation used by PGRX. No known issues exist, and it will be used until PGRX adopts an alternative.
