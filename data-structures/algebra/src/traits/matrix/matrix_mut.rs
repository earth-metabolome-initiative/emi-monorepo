//! Submodule defining the `MatrixMut` trait.

use super::Matrix;

/// Trait defining a mutable matrix.
pub trait MatrixMut: Matrix + Default {
    /// Type of the entry of the matrix.
    /// In a matrix with values, this is generally a tuple of the coordinates and the value,
    /// while in a matrix without values, this is generally the coordinates.
    type Entry;

    /// The type of error that can be returned when adding an entry.
    type Error: core::error::Error;

    /// Sets the value at the given entry.
    ///
    /// # Arguments
    ///
    /// * `entry`: The entry to set.
    ///
    /// # Errors
    ///
    /// Returns an error if the entry cannot be added. Possible reasons include:
    /// - The entries are not provided in the expected order.
    /// - The entry is out of bounds.
    /// - The entry is already defined.
    ///
    fn add(&mut self, entry: Self::Entry) -> Result<(), Self::Error>;
}
