//! Submodule providing the implementation of the `Display` trait for the
//! [`ToolCategory`] enumeration.

impl core::fmt::Display for crate::ToolCategory {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        let name = self.name();
        <str as core::fmt::Display>::fmt(name, f)
    }
}
