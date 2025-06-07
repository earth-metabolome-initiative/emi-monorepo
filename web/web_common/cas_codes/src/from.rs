//! Submodule providing implementations of the `From` trait for the `CAS` number
//! struct.

impl From<crate::CAS> for u32 {
    /// Converts a `CAS` number to a `u32`.
    fn from(cas: crate::CAS) -> Self {
        cas.0
    }
}
