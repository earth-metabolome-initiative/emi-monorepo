//! Submodule for the `SortedVec` struct and its primary methods.

use std::ops::Index;

#[cfg(all(not(feature = "std"), feature = "alloc"))]
use alloc::vec::Vec;
use common_traits::transmute::TransmuteFrom;

use crate::error::Error;

#[derive(Debug, Default)]
/// Struct defining a sorted vector and its primary methods.
pub struct SortedVec<V> {
    vec: Vec<V>,
}

impl<V> TryFrom<Vec<V>> for SortedVec<V>
where
    V: Ord,
{
    type Error = Error;

    fn try_from(vec: Vec<V>) -> Result<Self, Self::Error> {
        if vec.is_sorted() {
            Ok(Self { vec })
        } else {
            Err(Error::NotSorted)
        }
    }
}

impl<V: Ord> TransmuteFrom<Vec<V>> for SortedVec<V> {
    unsafe fn transmute_from(source: Vec<V>) -> Self {
        debug_assert!(source.is_sorted(), "The source vector is not sorted.");
        Self { vec: source }
    }
}

impl<V, Idx> Index<Idx> for SortedVec<V>
where
    Vec<V>: Index<Idx>,
{
    type Output = <Vec<V> as Index<Idx>>::Output;

    fn index(&self, index: Idx) -> &Self::Output {
        &self.vec[index]
    }
}

impl<V> SortedVec<V> {
	#[must_use]
	/// Returns the entry at the provided index.
	pub fn get(&self, index: usize) -> Option<&V> {
		self.vec.get(index)
	}

	#[must_use]
	/// Returns the length of the vector.
	pub fn len(&self) -> usize {
		self.vec.len()
	}

	#[must_use]
	/// Returns whether the vector is empty.
	pub fn is_empty(&self) -> bool {
		self.vec.is_empty()
	}

	/// Returns an iterator over the vector.
	pub fn iter(&self) -> std::slice::Iter<V> {
		self.into_iter()
	}
}

impl<V: Ord> SortedVec<V> {
	#[must_use]
	/// Returns whether the vector is sorted.
	pub fn is_sorted(&self) -> bool {
		true
	}

	/// Returns the index of the provided value.
	/// 
	/// # Arguments
	/// 
	/// * `value`: The value to search for.
	/// 
	/// # Returns
	/// 
	/// * `Ok(index)` if the value is found at the provided index.
	/// 
	/// # Errors
	/// 
	/// * `Err(index)` if the value is not found.
	pub fn binary_search(&self, value: &V) -> Result<usize, usize> {
		self.vec.binary_search(value)
	}
}

impl<'a, V> IntoIterator for &'a SortedVec<V> {
	type Item = &'a V;
	type IntoIter = std::slice::Iter<'a, V>;

	fn into_iter(self) -> Self::IntoIter {
		self.vec.iter()
	}
}