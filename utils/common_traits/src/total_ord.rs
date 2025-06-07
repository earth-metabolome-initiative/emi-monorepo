//! Submodule providing the `TotalOrd` trait to sort elements.

use core::cmp::Ordering;

/// Trait defining a total order.
pub trait TotalOrd {
    /// Compares two elements.
    fn total_cmp(&self, other: &Self) -> Ordering;
}

impl TotalOrd for f32 {
    #[inline]
    fn total_cmp(&self, other: &Self) -> Ordering {
        self.total_cmp(other)
    }
}

impl TotalOrd for f64 {
    #[inline]
    fn total_cmp(&self, other: &Self) -> Ordering {
        self.total_cmp(other)
    }
}

impl TotalOrd for i8 {
    #[inline]
    fn total_cmp(&self, other: &Self) -> Ordering {
        self.cmp(other)
    }
}

impl TotalOrd for i16 {
    #[inline]
    fn total_cmp(&self, other: &Self) -> Ordering {
        self.cmp(other)
    }
}

impl TotalOrd for i32 {
    #[inline]
    fn total_cmp(&self, other: &Self) -> Ordering {
        self.cmp(other)
    }
}

impl TotalOrd for i64 {
    #[inline]
    fn total_cmp(&self, other: &Self) -> Ordering {
        self.cmp(other)
    }
}

impl TotalOrd for i128 {
    #[inline]
    fn total_cmp(&self, other: &Self) -> Ordering {
        self.cmp(other)
    }
}

impl TotalOrd for isize {
    #[inline]
    fn total_cmp(&self, other: &Self) -> Ordering {
        self.cmp(other)
    }
}

impl TotalOrd for u8 {
    #[inline]
    fn total_cmp(&self, other: &Self) -> Ordering {
        self.cmp(other)
    }
}

impl TotalOrd for u16 {
    #[inline]
    fn total_cmp(&self, other: &Self) -> Ordering {
        self.cmp(other)
    }
}

impl TotalOrd for u32 {
    #[inline]
    fn total_cmp(&self, other: &Self) -> Ordering {
        self.cmp(other)
    }
}

impl TotalOrd for u64 {
    #[inline]
    fn total_cmp(&self, other: &Self) -> Ordering {
        self.cmp(other)
    }
}

impl TotalOrd for u128 {
    #[inline]
    fn total_cmp(&self, other: &Self) -> Ordering {
        self.cmp(other)
    }
}

impl TotalOrd for usize {
    #[inline]
    fn total_cmp(&self, other: &Self) -> Ordering {
        self.cmp(other)
    }
}
