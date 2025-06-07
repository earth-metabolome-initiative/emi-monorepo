//! Submodule providing the implementation of the `Display` trait for the
//! [`NameplateCategory`] enumeration.

impl core::fmt::Display for crate::NameplateCategory {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "{}", self.as_ref())
    }
}
