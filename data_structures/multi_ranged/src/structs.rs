//! Submodule defining structs which can be used to implement the
//! multi-ranged trait.

pub mod birange;
pub mod multi_range;
pub mod simple_range;
pub use birange::BiRange;
pub use multi_range::MultiRange;
pub use simple_range::SimpleRange;
