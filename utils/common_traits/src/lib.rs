#![doc = include_str!("../README.md")]

pub mod builder;
pub mod camel_case;
pub mod serde_trait;
pub mod total_ord;
pub mod transmute;
pub mod try_from_async;

/// Re-export of the core traits.
pub mod prelude {
    pub use super::{
        builder::{Builder, BuilderError},
        camel_case::ToCamelCase,
        serde_trait::Serde,
        total_ord::TotalOrd,
        try_from_async::TryFromAsync,
    };
}
