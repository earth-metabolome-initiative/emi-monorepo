#![doc = include_str!("../README.md")]

pub mod basic;

/// Re-export of the core traits.
pub mod prelude {
    pub use super::basic::Basic;
    #[cfg(feature = "derive")]
    pub use common_traits_derive::basic;
}