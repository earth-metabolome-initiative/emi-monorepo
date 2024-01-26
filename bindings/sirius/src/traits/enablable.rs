/// Trait for enablable objects
pub trait Enablable {
    /// Returns true if the object is an enabler
    fn is_enabler(&self) -> bool;

    /// Returns the enabler variant of the object
    fn enabler() -> Self;
}