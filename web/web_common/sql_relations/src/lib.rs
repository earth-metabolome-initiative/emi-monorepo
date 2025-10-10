#![doc = include_str!("../README.md")]

mod impls;
pub mod traits;

/// A prelude to import the most commonly used traits and types.
pub mod prelude {
    pub use sql_traits::prelude::*;

    pub use crate::traits::*;
}
