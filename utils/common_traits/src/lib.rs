#![doc = include_str!("../README.md")]

pub mod basic;
pub mod builder;
pub mod camel_case;
pub mod serde_trait;
pub mod transmute;
pub mod try_from_async;

/// Re-export of the core traits.
pub mod prelude {
    pub use super::{
        basic::Basic,
        builder::{Builder, BuilderError},
        camel_case::ToCamelCase,
        serde_trait::Serde,
        try_from_async::TryFromAsync,
    };
}
