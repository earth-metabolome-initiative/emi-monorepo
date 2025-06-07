#![doc = include_str!("../README.md")]

pub mod constants;
pub mod finite;
pub mod integer;
pub mod into_usize;
pub mod number;
pub mod ops;
pub mod positive_integer;
pub mod pow;
pub mod sqrt;
pub mod try_from_usize;

/// A prelude module that re-exports commonly used traits and types
pub mod prelude {
    pub use crate::{
        constants::{Bounded, Ten, Two},
        finite::Finite,
        integer::Integer,
        into_usize::IntoUsize,
        number::{Number, PositiveNumber},
        ops::SaturatingSub,
        positive_integer::PositiveInteger,
        pow::Pow,
        sqrt::Sqrt,
        try_from_usize::TryFromUsize,
    };
}
