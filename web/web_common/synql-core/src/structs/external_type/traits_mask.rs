//! Submodule providing a struct defining which traits are supported by some
//! type found in the postgres database schema.

use enumflags2::{BitFlags, bitflags};
use strum_macros::EnumIter;

#[bitflags]
#[repr(u16)]
#[derive(Copy, Clone, Debug, PartialEq, EnumIter)]
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

impl AsRef<str> for Trait {
    fn as_ref(&self) -> &str {
        match self {
            Trait::Copy => "Copy",
            Trait::Clone => "Clone",
            Trait::Default => "Default",
            Trait::Debug => "Debug",
            Trait::Hash => "Hash",
            Trait::Ord => "Ord",
            Trait::PartialOrd => "PartialOrd",
            Trait::Eq => "Eq",
            Trait::PartialEq => "PartialEq",
        }
    }
}

impl Trait {
    /// Returns the full path of the trait.
    pub fn path(&self) -> syn::Path {
        match self {
            Trait::Copy => syn::parse_str("Copy").unwrap(),
            Trait::Clone => syn::parse_str("Clone").unwrap(),
            Trait::Default => syn::parse_str("Default").unwrap(),
            Trait::Debug => syn::parse_str("::core::fmt::Debug").unwrap(),
            Trait::Hash => syn::parse_str("::core::hash::Hash").unwrap(),
            Trait::Ord => syn::parse_str("Ord").unwrap(),
            Trait::PartialOrd => syn::parse_str("PartialOrd").unwrap(),
            Trait::Eq => syn::parse_str("Eq").unwrap(),
            Trait::PartialEq => syn::parse_str("PartialEq").unwrap(),
        }
    }
}

#[derive(Clone, Debug, Default, PartialEq, Eq, Hash, PartialOrd, Ord)]
/// Struct defining which traits are supported by some type found
/// in the postgres database schema.
pub(super) struct TraitsMask {
    /// The flags representing the supported traits.
    flags: BitFlags<Trait>,
}

unsafe impl Send for TraitsMask {}
unsafe impl Sync for TraitsMask {}

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
