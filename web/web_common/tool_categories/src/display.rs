//! Submodule providing the implementation of the `Display` trait for the
//! [`ToolCategory`] enumeration.

impl core::fmt::Display for crate::ToolCategory {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "{}", self.as_ref())
    }
}
