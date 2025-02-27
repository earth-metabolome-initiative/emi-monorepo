//! Trait implementing the conversion from `usize`, including the occasional
//! error, depending on the compilation flags (whether target is 64 or 32 bits).

/// Trait defining the conversion from `usize`.
pub trait TryFromUsize: Sized {
    /// The error type.
    type Error;

    /// Converts the value from `usize`.
    ///
    /// # Errors
    ///
    /// * Returns an error if the conversion is not possible.
    fn try_from_usize(value: usize) -> Result<Self, Self::Error>;
}

impl TryFromUsize for usize {
    type Error = core::convert::Infallible;

    fn try_from_usize(value: usize) -> Result<Self, Self::Error> {
        Ok(value)
    }
}

impl TryFromUsize for u128 {
    type Error = core::convert::Infallible;

    fn try_from_usize(value: usize) -> Result<Self, Self::Error> {
        // Since `u128` is always larger than `usize`, the conversion is always safe.
        Ok(value as u128)
    }
}

impl TryFromUsize for u64 {
    type Error = core::convert::Infallible;

    fn try_from_usize(value: usize) -> Result<Self, Self::Error> {
        // Since `u64` is either exactly `usize` when the target is 64 bits, or
        // larger than `usize` when the target is 32 bits, the conversion is always
        // safe.
        Ok(value as u64)
    }
}

#[cfg(target_pointer_width = "32")]
impl TryFromUsize for u32 {
    type Error = core::convert::Infallible;

    fn try_from_usize(value: usize) -> Result<Self, Self::Error> {
        // Since `u32` is exactly `usize` when the target is 32 bits, the conversion is
        // always safe.
        Ok(value as u32)
    }
}

#[cfg(not(target_pointer_width = "32"))]
impl TryFromUsize for u32 {
    type Error = core::num::TryFromIntError;

    fn try_from_usize(value: usize) -> Result<Self, Self::Error> {
        u32::try_from(value)
    }
}

impl TryFromUsize for u16 {
    type Error = core::num::TryFromIntError;

    fn try_from_usize(value: usize) -> Result<Self, Self::Error> {
        u16::try_from(value)
    }
}

impl TryFromUsize for u8 {
    type Error = core::num::TryFromIntError;

    fn try_from_usize(value: usize) -> Result<Self, Self::Error> {
        u8::try_from(value)
    }
}

impl TryFromUsize for isize {
    type Error = core::num::TryFromIntError;

    fn try_from_usize(value: usize) -> Result<Self, Self::Error> {
        isize::try_from(value)
    }
}

impl TryFromUsize for i128 {
    type Error = core::convert::Infallible;

    fn try_from_usize(value: usize) -> Result<Self, Self::Error> {
        // Since `i128` is always larger than `usize`, the conversion is always safe.
        Ok(value as i128)
    }
}

impl TryFromUsize for i64 {
    type Error = core::num::TryFromIntError;

    fn try_from_usize(value: usize) -> Result<Self, Self::Error> {
        i64::try_from(value)
    }
}

impl TryFromUsize for i32 {
    type Error = core::num::TryFromIntError;

    fn try_from_usize(value: usize) -> Result<Self, Self::Error> {
        i32::try_from(value)
    }
}

impl TryFromUsize for i16 {
    type Error = core::num::TryFromIntError;

    fn try_from_usize(value: usize) -> Result<Self, Self::Error> {
        i16::try_from(value)
    }
}

impl TryFromUsize for i8 {
    type Error = core::num::TryFromIntError;

    fn try_from_usize(value: usize) -> Result<Self, Self::Error> {
        i8::try_from(value)
    }
}
