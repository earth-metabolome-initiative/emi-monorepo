//! Serde trait with dispatch according to the enabled features.

#[cfg(feature = "serde")]
/// Trait for types that can be serialized and deserialized.
pub trait Serde: serde::Serialize + serde::de::DeserializeOwned {}

#[cfg(feature = "serde")]
impl<T> Serde for T where T: serde::Serialize + serde::de::DeserializeOwned {}

#[cfg(not(feature = "serde"))]
/// Trait for types that can be serialized and deserialized.
pub trait Serde {}

#[cfg(not(feature = "serde"))]
impl<T> Serde for T {}
