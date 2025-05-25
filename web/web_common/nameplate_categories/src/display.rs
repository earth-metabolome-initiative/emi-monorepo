//! Submodule providing the implementation of the `Display` trait for the
//! [`NameplateCategory`] enumeration.

impl core::fmt::Display for crate::NameplateCategory {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::Permanent => write!(f, "Permanent"),
            Self::SemiPermanent => write!(f, "SemiPermanent"),
        }
    }
}
