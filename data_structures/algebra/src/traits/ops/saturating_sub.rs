//! Submodule defining the `SaturatingSub` trait.

/// Trait defining the `saturating_sub` operation.
pub trait SaturatingSub<Rhs = Self> {
    /// The type of the result of the `saturating_sub` operation.
    type Output;

    /// Performs the `saturating_sub` operation.
    fn saturating_sub(self, rhs: Rhs) -> Self::Output;
}

/// Macro implementing the `SaturatingSub` trait for all numerical types.
macro_rules! impl_saturating_sub_for_numerical {
	($($t:ty),+) => {
		$(
			impl SaturatingSub for $t {
				type Output = Self;

				#[inline]
				fn saturating_sub(self, rhs: Self) -> Self::Output {
					self.saturating_sub(rhs)
				}
			}
		)+
	};
}

impl_saturating_sub_for_numerical!(u8, u16, u32, u64, u128, usize);
impl_saturating_sub_for_numerical!(i8, i16, i32, i64, i128, isize);
