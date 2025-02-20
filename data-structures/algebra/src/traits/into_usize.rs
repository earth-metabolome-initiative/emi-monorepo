//! Implements the conversion into `usize` strictly only for the types that 
//! always fit into `usize` without loss of information, with the appropriate
//! compilation flags.

use super::Integer;

/// Trait defining the conversion into `usize`.
pub trait IntoUsize: Integer {
	/// Converts the value into `usize`.
	fn into_usize(self) -> usize;
}

impl IntoUsize for u8 {
	fn into_usize(self) -> usize {
		self.into()
	}
}

impl IntoUsize for u16 {
	fn into_usize(self) -> usize {
		self.into()
	}
}

impl IntoUsize for u32 {
	fn into_usize(self) -> usize {
		self as usize
	}
}

#[cfg(target_pointer_width = "64")]
impl IntoUsize for u64 {
	#[allow(clippy::cast_possible_truncation)]
	fn into_usize(self) -> usize {
		self as usize
	}
}

impl IntoUsize for usize {
	fn into_usize(self) -> usize {
		self
	}
}