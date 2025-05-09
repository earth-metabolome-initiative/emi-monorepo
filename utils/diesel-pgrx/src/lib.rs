#![doc = include_str!("../README.md")]

/// Trait marked defining a type that can be used with both `diesel` and `pgrx`
pub trait DieselPGRX {}

#[cfg(feature = "macro")]
pub use diesel;
#[cfg(feature = "macro")]
#[doc(hidden)]
pub use diesel_pgrx_derive::DieselPGRX;
#[cfg(feature = "macro")]
pub use serde_json;
