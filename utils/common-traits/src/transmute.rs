//! Submodule defining the Transmute trait, which allows for unsafe transmutation between types.

/// Trait defining the transmutation of a type into another type.
pub trait TransmuteFrom<Source> {
    /// Transmutes the given value from the source type into the target type
    /// 
    /// # Safety
    /// 
    /// This function is unsafe because it performs a transmutation between types
    /// without any checks. It is up to the caller to ensure that the transmutation
    /// is safe.
    unsafe fn transmute_from(source: Source) -> Self;
}
