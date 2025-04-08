//! Submodule providing the trait for exponentiation.

/// Trait for exponentiation.
pub trait Pow<Exponent> {
    #[must_use]
    /// Compute the exponentiation.
    fn pow(self, exponent: Exponent) -> Self;
}

impl Pow<i8> for f64 {
    fn pow(self, exponent: i8) -> Self {
        self.powi(i32::from(exponent))
    }
}

impl Pow<u8> for f64 {
    fn pow(self, exponent: u8) -> Self {
        self.powi(i32::from(exponent))
    }
}

impl Pow<i16> for f64 {
    fn pow(self, exponent: i16) -> Self {
        self.powi(i32::from(exponent))
    }
}

impl Pow<u16> for f64 {
    fn pow(self, exponent: u16) -> Self {
        self.powi(i32::from(exponent))
    }
}

impl Pow<i32> for f64 {
    fn pow(self, exponent: i32) -> Self {
        self.powi(exponent)
    }
}

impl Pow<f32> for f64 {
    fn pow(self, exponent: f32) -> Self {
        self.powf(f64::from(exponent))
    }
}

impl Pow<f64> for f64 {
    fn pow(self, exponent: f64) -> Self {
        self.powf(exponent)
    }
}

impl Pow<i32> for f32 {
    fn pow(self, exponent: i32) -> Self {
        self.powi(exponent)
    }
}

impl Pow<i8> for f32 {
    fn pow(self, exponent: i8) -> Self {
        self.powi(i32::from(exponent))
    }
}

impl Pow<u8> for f32 {
    fn pow(self, exponent: u8) -> Self {
        self.powi(i32::from(exponent))
    }
}

impl Pow<i16> for f32 {
    fn pow(self, exponent: i16) -> Self {
        self.powi(i32::from(exponent))
    }
}

impl Pow<u16> for f32 {
    fn pow(self, exponent: u16) -> Self {
        self.powi(i32::from(exponent))
    }
}

impl Pow<f32> for f32 {
    fn pow(self, exponent: f32) -> Self {
        self.powf(exponent)
    }
}

impl Pow<u32> for i8 {
    fn pow(self, exponent: u32) -> Self {
        self.pow(exponent)
    }
}

impl Pow<u32> for i16 {
    fn pow(self, exponent: u32) -> Self {
        self.pow(exponent)
    }
}

impl Pow<u32> for i32 {
    fn pow(self, exponent: u32) -> Self {
        self.pow(exponent)
    }
}

impl Pow<u32> for i64 {
    fn pow(self, exponent: u32) -> Self {
        self.pow(exponent)
    }
}

impl Pow<u32> for u8 {
    fn pow(self, exponent: u32) -> Self {
        self.pow(exponent)
    }
}

impl Pow<u32> for u16 {
    fn pow(self, exponent: u32) -> Self {
        self.pow(exponent)
    }
}

impl Pow<u32> for u32 {
    fn pow(self, exponent: u32) -> Self {
        self.pow(exponent)
    }
}

impl Pow<u32> for u64 {
    fn pow(self, exponent: u32) -> Self {
        self.pow(exponent)
    }
}
