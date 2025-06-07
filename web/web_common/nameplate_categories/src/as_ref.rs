//! Submodule providing implementations of the `AsRef` trait for the
//! [`NameplateCategory`] enumeration.

impl AsRef<str> for crate::NameplateCategory {
    fn as_ref(&self) -> &str {
        match self {
            Self::Digital => "Digital",
            Self::Permanent => "Permanent",
            Self::SemiPermanent => "SemiPermanent",
        }
    }
}
