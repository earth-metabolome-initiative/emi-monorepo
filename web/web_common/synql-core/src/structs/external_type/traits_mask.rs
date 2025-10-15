//! Submodule providing a struct defining which traits are supported by some
//! type found in the postgres database schema.

use enumflags2::{BitFlags, bitflags};

#[bitflags]
#[repr(u16)]
#[derive(Copy, Clone, Debug, PartialEq)]
/// Enumeration of the traits which can be supported by some type found
/// in the postgres database schema.
///
/// # Implementation Notes
/// This enum is used as a bitflag, so each variant must be a power of two
/// (i.e., 1, 2, 4, 8, 16, 32, 64, 128). This allows for efficient storage
/// and manipulation of multiple traits using bitwise operations.
pub enum Trait {
    /// The type implements the `Copy` trait.
    Copy,
    /// The type implements the `Clone` trait.
    Clone,
    /// The type implements the `Default` trait.
    Default,
    /// The type implements the `Debug` trait.
    Debug,
    /// The type implements the `Hash` trait.
    Hash,
    /// The type implements the `Ord` trait.
    Ord,
    /// The type implements the `PartialOrd` trait.
    PartialOrd,
    /// The type implements the `Eq` trait.
    Eq,
    /// The type implements the `PartialEq` trait.
    PartialEq,
}

#[derive(Clone, Debug, Default, PartialEq)]
/// Struct defining which traits are supported by some type found
/// in the postgres database schema.
pub(super) struct TraitsMask {
    /// The flags representing the supported traits.
    flags: BitFlags<Trait>,
}

impl TraitsMask {
    /// Returns whether the current type supports the given trait.
    pub(super) fn supports(&self, r#trait: Trait) -> bool {
        self.flags.contains(r#trait)
    }

    /// Sets that the current type supports the given trait.
    pub(super) fn set_supports(&mut self, r#trait: Trait) {
        self.flags.insert(r#trait);
    }
}
