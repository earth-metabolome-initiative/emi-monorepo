//! Submodule providing algorithms for solving the Weighted Assignment Problem.

mod lapjv;
pub use lapjv::{LAPJV, LAPJVError, SparseLAPJV};
