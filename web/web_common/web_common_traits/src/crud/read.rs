//! Submodule providing the `Read` trait and its implementations.

use std::{fmt::Debug, hash::Hash};

/// The `Read` trait is used to define the read operation in a CRUD context.
pub trait Read: Sized {
    #[must_use]
    /// Returns a new read operation.
    fn read_all(offset: Option<usize>, limit: Option<usize>) -> ReadAll<Self> {
        ReadAll {
            offset: offset.unwrap_or(0),
            limit: limit.unwrap_or(100),
            _marker: std::marker::PhantomData,
        }
    }
}

impl<R> Read for R {}

/// Struct to represent a read operation with an offset and limit.
pub struct ReadAll<T> {
    /// The offset for the read operation.
    offset: usize,
    /// The limit for the read operation.
    limit: usize,
    _marker: std::marker::PhantomData<T>,
}

#[derive(serde::Deserialize, serde::Serialize)]
struct ReadAllData {
    offset: usize,
    limit: usize,
}

impl<T> From<ReadAll<T>> for ReadAllData {
    fn from(value: ReadAll<T>) -> Self {
        ReadAllData { offset: value.offset, limit: value.limit }
    }
}

impl<T> From<ReadAllData> for ReadAll<T> {
    fn from(value: ReadAllData) -> Self {
        ReadAll { offset: value.offset, limit: value.limit, _marker: std::marker::PhantomData }
    }
}

impl<T> serde::Serialize for ReadAll<T> {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        let data: ReadAllData = (*self).into();
        data.serialize(serializer)
    }
}

impl<'de, T> serde::Deserialize<'de> for ReadAll<T> {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let data = ReadAllData::deserialize(deserializer)?;
        Ok(data.into())
    }
}

impl<T> Hash for ReadAll<T> {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.offset.hash(state);
        self.limit.hash(state);
    }
}

impl<T> Debug for ReadAll<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ReadAll").field("offset", &self.offset).field("limit", &self.limit).finish()
    }
}
impl<T> Clone for ReadAll<T> {
    fn clone(&self) -> Self {
        *self
    }
}
impl<T> Copy for ReadAll<T> {}
impl<T> PartialEq for ReadAll<T> {
    fn eq(&self, other: &Self) -> bool {
        self.offset == other.offset && self.limit == other.limit
    }
}
impl<T> Eq for ReadAll<T> {}
