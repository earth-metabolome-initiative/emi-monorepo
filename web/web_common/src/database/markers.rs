//! Submodule providing struct markers that may be used to indicate that a struct is deletable or editable,
//! primarily useful to indicate that a struct is deletable or editable in the database, i.e. that the user
//! has the right to delete or edit the struct.
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
/// Marker indicating that a struct is editable.
pub struct Editable<T>(pub T);

impl<T> Editable<T> {
    pub fn into_inner(self) -> T {
        self.0
    }
}

impl<T> From<T> for Editable<T> {
    fn from(t: T) -> Self {
        Self(t)
    }
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
/// Marker indicating that a struct is deletable.
pub struct Deletable<T>(pub T);

impl<T> Deletable<T> {
    pub fn into_inner(self) -> T {
        self.0
    }
}

impl<T> From<T> for Deletable<T> {
    fn from(t: T) -> Self {
        Self(t)
    }
}
