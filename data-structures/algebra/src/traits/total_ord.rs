//! Submodule providing the `TotalOrd` trait to sort elements.

use core::cmp::Ordering;

/// Trait defining a total order.
pub trait TotalOrd {
    /// Compares two elements.
    fn total_cmp(&self, other: &Self) -> Ordering;
}

impl TotalOrd for f32 {
    fn total_cmp(&self, other: &Self) -> Ordering {
        self.total_cmp(other)
    }
}

impl TotalOrd for f64 {
    fn total_cmp(&self, other: &Self) -> Ordering {
        self.total_cmp(other)
    }
}

impl TotalOrd for i8 {
    fn total_cmp(&self, other: &Self) -> Ordering {
        self.cmp(other)
    }
}

impl TotalOrd for i16 {
    fn total_cmp(&self, other: &Self) -> Ordering {
        self.cmp(other)
    }
}

impl TotalOrd for i32 {
    fn total_cmp(&self, other: &Self) -> Ordering {
        self.cmp(other)
    }
}

impl TotalOrd for i64 {
    fn total_cmp(&self, other: &Self) -> Ordering {
        self.cmp(other)
    }
}

impl TotalOrd for i128 {
    fn total_cmp(&self, other: &Self) -> Ordering {
        self.cmp(other)
    }
}

impl TotalOrd for isize {
    fn total_cmp(&self, other: &Self) -> Ordering {
        self.cmp(other)
    }
}

impl TotalOrd for u8 {
    fn total_cmp(&self, other: &Self) -> Ordering {
        self.cmp(other)
    }
}

impl TotalOrd for u16 {
    fn total_cmp(&self, other: &Self) -> Ordering {
        self.cmp(other)
    }
}

impl TotalOrd for u32 {
    fn total_cmp(&self, other: &Self) -> Ordering {
        self.cmp(other)
    }
}

impl TotalOrd for u64 {
    fn total_cmp(&self, other: &Self) -> Ordering {
        self.cmp(other)
    }
}

impl TotalOrd for u128 {
    fn total_cmp(&self, other: &Self) -> Ordering {
        self.cmp(other)
    }
}

impl TotalOrd for usize {
    fn total_cmp(&self, other: &Self) -> Ordering {
        self.cmp(other)
    }
}
