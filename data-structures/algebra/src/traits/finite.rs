//! Submodule providing the `Finite` trait and its implementations.

/// Trait for finite values.
pub trait Finite {
    /// Returns `true` if the element is finite, `false` otherwise.
    ///
    /// # Implementation details
    ///
    /// A value is considered finite if it is not `NaN` or `Inf`.
    fn is_finite(&self) -> bool;
}

impl Finite for f32 {
    #[inline]
    fn is_finite(&self) -> bool {
        !self.is_nan() && !self.is_infinite()
    }
}

impl Finite for f64 {
    #[inline]
    fn is_finite(&self) -> bool {
        !self.is_nan() && !self.is_infinite()
    }
}

/// Macro to implement the `Finite` trait for all integer types.
macro_rules! impl_finite_for_integers {
	($($t:ty),+) => {
		$(
			impl Finite for $t {
				#[inline]
                fn is_finite(&self) -> bool {
					true
				}
			}
		)+
	};
}

impl_finite_for_integers!(i8, i16, i32, i64, i128, isize);
impl_finite_for_integers!(u8, u16, u32, u64, u128, usize);
