#![doc = include_str!("../README.md")]

pub mod basic;
pub mod builder;
pub mod camel_case;

/// Re-export of the core traits.
pub mod prelude {
    pub use common_traits_derive::basic;

    pub use super::{
        basic::Basic,
        builder::{Builder, BuilderError},
        camel_case::ToCamelCase,
    };
}
