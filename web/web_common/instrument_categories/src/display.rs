//!  Submodule provding the implementation of the `Display` trait for the
//! [`InstrumentCategory`] enum.

impl core::fmt::Display for crate::InstrumentCategory {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        let name = self.name();
        <str as core::fmt::Display>::fmt(name, f)
    }
}
