//! Submodolue providing the `Basic` trait, which is a trait deriving several
//! basic traits for a struct such as `Debug`, `Clone`, `Serialize`, and
//! `Deserialize`.

/// Trait providing several basic traits for a struct such as `Debug`, `Clone`,
/// `Serialize`, and `Deserialize`.
pub trait Basic: core::fmt::Debug + Clone {}

impl<T> Basic for T where T: core::fmt::Debug + Clone {}
