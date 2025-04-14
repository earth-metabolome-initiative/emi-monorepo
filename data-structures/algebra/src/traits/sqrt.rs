//! Submodule providing the squared root trait.

/// Trait providing the square root operation.
pub trait Sqrt {
    #[must_use]
    /// Returns the square root of the provided value.
    fn sqrt(self) -> Self;
}

impl Sqrt for f32 {
    #[inline]
    fn sqrt(self) -> Self {
        self.sqrt()
    }
}

impl Sqrt for f64 {
    #[inline]
    fn sqrt(self) -> Self {
        self.sqrt()
    }
}
