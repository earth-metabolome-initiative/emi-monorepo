//! Implementations of traits for standard library types.

#[cfg(feature = "std")]
mod hashmap;

#[cfg(any(feature = "std", feature = "alloc"))]
mod vec;

#[cfg(any(feature = "std", feature = "alloc"))]
mod sorted_vec;

mod array;
mod implicit_numeric_vocabularies;
mod sorted_array;
mod tuple;

#[cfg(any(feature = "std", feature = "alloc"))]
mod ranged_csr;

#[cfg(any(feature = "std", feature = "alloc"))]
mod csr2d;
#[cfg(any(feature = "std", feature = "alloc"))]
mod generic_implicit_valued_matrix2d;
#[cfg(any(feature = "std", feature = "alloc"))]
mod squared_csr2d;
#[cfg(any(feature = "std", feature = "alloc"))]
mod symmetric_csr;
#[cfg(any(feature = "std", feature = "alloc"))]
mod upper_triangular_csr;
#[cfg(any(feature = "std", feature = "alloc"))]
mod valued_csr2d;
