# Diesel PGRX

[![Clippy](https://github.com/earth-metabolome-initiative/emi-monorepo/actions/workflows/cargo-clippy-diesel-pgrx.yml/badge.svg)](https://github.com/earth-metabolome-initiative/emi-monorepo/actions/workflows/cargo-clippy-diesel-pgrx.yml)
[![Test](https://github.com/earth-metabolome-initiative/emi-monorepo/actions/workflows/cargo-test-diesel-pgrx.yml/badge.svg)](https://github.com/earth-metabolome-initiative/emi-monorepo/actions/workflows/cargo-test-diesel-pgrx.yml)

A crate providing the `DieselPGRX` trait to derive all that is needed to use rust-defined types decorated with [`pgrx`](https://github.com/pgcentralfoundation/pgrx)'s [PostgresType](https://docs.rs/pgrx/latest/pgrx/datum/trait.PostgresType.html) and `#[pg_binary_protocol]` in [`diesel`](https://github.com/diesel-rs/diesel) queries using Postgres's binary protocol.

This is achieved via a derive macro to derive the [`diesel`](https://github.com/diesel-rs/diesel) [`ToSql`](https://docs.rs/diesel/latest/diesel/serialize/trait.ToSql.html) and [`FromSql`](https://docs.rs/diesel/latest/diesel/deserialize/trait.FromSql.html) traits so that the struct is compatible with the [`pgrx`](https://github.com/pgcentralfoundation/pgrx) serialization, which uses [`CBOR`](https://en.wikipedia.org/wiki/CBOR) using [`serde_cbor`](https://docs.rs/serde_cbor/latest/serde_cbor/).

## Usage

To use this crate, add the following to your `Cargo.toml`:

TODO: ADD TOML!

Remember that when you are using any `pgrx` extension, you need to install the extension in your Postgres database. First, you need to copy the `your_extension.so`, `your_extension.control`, and `your_extension.sql` files to the Postgres extension directory. Next, you need to run the following command in your Postgres database:

```sql
CREATE EXTENSION your_extension;
```

## Basic example

Most commonly, you will want to use the `DieselPGRX` derive macro to derive the [`ToSql`](https://docs.rs/diesel/latest/diesel/serialize/trait.ToSql.html) and [`FromSql`](https://docs.rs/diesel/latest/diesel/deserialize/trait.FromSql.html) traits for your struct. Since the type is serialized using [`serde_cbor`](https://docs.rs/serde_cbor/latest/serde_cbor/), it is important to derive the [`serde::Serialize`](https://docs.rs/serde/latest/serde/trait.Serialize.html) and [`serde::Deserialize`](https://docs.rs/serde/latest/serde/trait.Deserialize.html) traits as well. This is done by adding the `#[derive(serde::Serialize, serde::Deserialize)]` attributes to your struct.
Pay particular attention to the two [`pgrx`](https://github.com/pgcentralfoundation/pgrx) attributes: the first adds the derive macro for [PostgresType](https://docs.rs/pgrx/latest/pgrx/datum/trait.PostgresType.html), and the second enables the binary protocol for the type, which is necessary for the `diesel` integration to work.

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

### Additional `pgrx` goodies

The [`pgrx`](https://github.com/pgcentralfoundation/pgrx) crate provides several additional **optional** derive macros which you may want to use, so here is a more complete example which includes all of the ones we frequently use. These include [`PostgresEq`](https://docs.rs/pgrx/latest/pgrx/derive.PostgresEq.html), [`PostgresOrd`](https://docs.rs/pgrx/latest/pgrx/derive.PostgresOrd.html), and [`PostgresHash`](https://docs.rs/pgrx/latest/pgrx/derive.PostgresHash.html). These are used to implement the `=` and `<` operators for the type, as well as the `hash` function. Do note that these traits require you to also implement the `PartialEq`, `Eq`, `PartialOrd`, `Ord`, and `Hash` traits for your struct. This is done by adding the `#[derive(PartialEq, Eq, PartialOrd, Ord, Hash)]` attributes to your struct.

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

## Supported diesel backends

Other than `postgres`, the derive macro provides the `ToSql` and `FromSql` implementations for `sqlite` using the `BINARY` type, always using the very same [`serde_cbor`](https://docs.rs/serde_cbor/latest/serde_cbor/) serialization. Feel free to open [a PR](https://github.com/earth-metabolome-initiative/emi-monorepo/pulls) if you want to add support for other backends.

Specifically, the `postgres` backend is gated by the feature flag `postgres`. Similarly, the `sqlite` backend is gated by the feature flag `sqlite`, which is NOT enabled by default. You can enable it by adding the following to your `Cargo.toml`:

TODO: ADD TOML!

## On the `serde_cbor` dependency

At the time of writing, the `serde_cbor` crate is now an archived repository and as such it will no longer receive updates. While there are alternatives, it is currently the [`CBOR`](https://en.wikipedia.org/wiki/CBOR) serialization software adopted by `pgrx` and as such we will continue to use it until `pgrx` decides to change it. There are no known issues with the `serde_cbor` crate, so at this time the lack of updates is not a concern.
