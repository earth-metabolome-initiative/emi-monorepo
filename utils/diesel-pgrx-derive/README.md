# Diesel-pgrx derive

Rust crate providing a procedural macro to derive the diesel `ToSql` and `FromSql` traits so that the struct is compatible with the `pgrx` serialization, which uses [`CBOR`](https://en.wikipedia.org/wiki/CBOR) using [`serde_cbor`](https://docs.rs/serde_cbor/latest/serde_cbor/).
