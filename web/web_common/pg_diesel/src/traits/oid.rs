//! Submodule defining a trait for a struct which has an OID field.

/// This trait is used in the `oid_auto_cached` macro to derive the cache key from the `oid` field.
pub trait HasOid {
    /// Returns the OID of the struct.
    fn oid(&self) -> u32;
}
