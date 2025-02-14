//! Submodolue providing the `Basic` trait, which is a trait deriving several basic traits
//! for a struct such as `Debug`, `Clone`, `Serialize`, and `Deserialize`.

/// Trait providing several basic traits for a struct such as `Debug`, `Clone`, `Serialize`, and `Deserialize`.
pub trait Basic: std::fmt::Debug + Clone + serde::Serialize + serde::de::DeserializeOwned {}
