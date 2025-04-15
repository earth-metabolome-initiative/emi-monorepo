//! Submodule defining the `TryFromAsync` trait which provides an asyncronous
//! version of the `TryFromAsync` trait.

/// The `TryFromAsync` trait is an asynchronous version of the `TryFrom` trait.
pub trait TryFromAsync<T>: Sized {
    /// The associated error type.
    type Error;

    /// Attempts to convert `T` into `Self`.
    fn try_from(t: T) -> impl std::future::Future<Output = Result<Self, Self::Error>>;
}
