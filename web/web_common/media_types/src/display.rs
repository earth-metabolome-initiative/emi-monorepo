//! Submodule providing the implementation of the `Display` trait for the
//! [`RootMediaType`] enumeration.

impl core::fmt::Display for crate::MediaType {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "{}", self.root)?;
        write!(f, "/{}", self.subtype)?;

        if !self.params.is_empty() {
            write!(f, ";")?;
            for (key, value) in &self.params {
                write!(f, " {key}={value}")?;
            }
        }

        Ok(())
    }
}
