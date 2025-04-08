//! Submodule providing a definition of a coordinate.

use core::fmt::Debug;

/// Trait defining coordinates.
pub trait Coordinates: core::fmt::Debug {
    /// Returns the number of dimensions of the coordinate.
    fn dimensions() -> usize;
}

impl Coordinates for () {
    fn dimensions() -> usize {
        0
    }
}

impl<A: Debug> Coordinates for (A,) {
    fn dimensions() -> usize {
        1
    }
}

impl<A: Debug, B: Debug> Coordinates for (A, B) {
    fn dimensions() -> usize {
        2
    }
}

impl<A: Debug, B: Debug, C: Debug> Coordinates for (A, B, C) {
    fn dimensions() -> usize {
        3
    }
}

impl<A: Debug, B: Debug, C: Debug, D: Debug> Coordinates for (A, B, C, D) {
    fn dimensions() -> usize {
        4
    }
}

impl<A: Debug> Coordinates for [A; 0] {
    fn dimensions() -> usize {
        0
    }
}

impl<A: Debug> Coordinates for [A; 1] {
    fn dimensions() -> usize {
        1
    }
}

impl<A: Debug> Coordinates for [A; 2] {
    fn dimensions() -> usize {
        2
    }
}

impl<A: Debug> Coordinates for [A; 3] {
    fn dimensions() -> usize {
        3
    }
}

impl<A: Debug> Coordinates for [A; 4] {
    fn dimensions() -> usize {
        4
    }
}
