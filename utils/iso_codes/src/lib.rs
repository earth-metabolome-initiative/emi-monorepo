#![doc = include_str!("../README.md")]

#[cfg(feature = "pgrx")]
::pgrx::pg_module_magic!();

pub mod country_codes;
pub mod errors;
pub use country_codes::CountryCode;
