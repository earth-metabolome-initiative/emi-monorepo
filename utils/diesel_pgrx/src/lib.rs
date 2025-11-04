#![doc = include_str!("../README.md")]

/// Trait marker defining a type that can be used with both `diesel` and `pgrx`
pub trait DieselPGRX {}

pub use diesel;
pub use diesel_pgrx_derive::DieselPGRX;
pub use serde_cbor;
