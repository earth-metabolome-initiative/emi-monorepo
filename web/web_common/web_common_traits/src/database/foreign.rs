//! Traits relative to the presence of foreign keys in a struct.

/// Trait for a struct that has a foreign key.
pub trait Foreign<T> {
    #[cfg(feature = "backend")]
    /// Returns the foreign key.
    fn foreign(
        &self,
        conn: &mut crate::types::DBConn,
    ) -> impl std::future::Future<Output = Result<Option<T>, diesel::result::Error>>;

    #[cfg(not(feature = "backend"))]
    /// Returns the foreign key.
    fn foreign(
        &self,
    ) -> impl std::future::Future<Output = Result<Option<T>, std::convert::Infallible>>;
}
