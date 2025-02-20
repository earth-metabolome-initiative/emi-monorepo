//! Implementations of traits for standard library types.

#[cfg(feature = "std")]
mod hashmap;

#[cfg(any(feature = "std", feature = "alloc"))]
mod vec;

#[cfg(any(feature = "std", feature = "alloc"))]
mod sorted_vec;