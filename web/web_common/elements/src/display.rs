//!  Submodule provding the implementation of the `Display` trait for the
//! [`Element`] enum.

impl core::fmt::Display for crate::Element {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        let element: &str = self.as_ref();

        write!(f, "{element}")
    }
}
