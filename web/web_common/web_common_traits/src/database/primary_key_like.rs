//! Submodule defining the `PrimaryKeyLike` trait and some of its
//! implementations, which define a common interface for types which have the
//! same primary key or can be trivially converted to such a type.

use std::rc::Rc;

/// Trait for types which have the same primary key or can be trivially
/// converted to such a type.
pub trait PrimaryKeyLike {
    /// The type of the primary key.
    type PrimaryKey;

    /// Returns the primary key of the type.
    fn primary_key(&self) -> Self::PrimaryKey;
}

/// Trait for types which may have the same primary key or may be trivially
/// converted to such a type.
pub trait MaybePrimaryKeyLike {
    /// The type of the primary key.
    type PrimaryKey;

    /// Returns the primary key of the type.
    fn maybe_primary_key(&self) -> Option<Self::PrimaryKey>;
}

impl<T> MaybePrimaryKeyLike for T
where
    T: PrimaryKeyLike,
{
    type PrimaryKey = T::PrimaryKey;

    fn maybe_primary_key(&self) -> Option<Self::PrimaryKey> {
        Some(self.primary_key())
    }
}

impl<T, PK> MaybePrimaryKeyLike for Option<T>
where
    T: PrimaryKeyLike<PrimaryKey = PK>,
{
    type PrimaryKey = PK;

    fn maybe_primary_key(&self) -> Option<Self::PrimaryKey> {
        self.as_ref().map(PrimaryKeyLike::primary_key)
    }
}

/// Macro to implement `PrimaryKeyLike` for types which
/// are primary keys themselves, such as `i32`.
macro_rules! impl_primary_key_like_for_pk_copy {
	($($t:ty),*) => {
		$(
			impl PrimaryKeyLike for $t {
				type PrimaryKey = Self;

				fn primary_key(&self) -> Self::PrimaryKey {
					*self
				}
			}
		)*
	};
}

impl_primary_key_like_for_pk_copy!(i16, i32, i64, rosetta_uuid::Uuid, iso_codes::CountryCode);

macro_rules! impl_primary_key_like_for_pk_clone {
	($($t:ty),*) => {
		$(
			impl PrimaryKeyLike for $t {
				type PrimaryKey = Self;

				fn primary_key(&self) -> Self::PrimaryKey {
					self.clone()
				}
			}
		)*
	};
}

impl_primary_key_like_for_pk_clone!(String);

impl<T, PK> PrimaryKeyLike for &T
where
    T: PrimaryKeyLike<PrimaryKey = PK>,
{
    type PrimaryKey = PK;

    fn primary_key(&self) -> Self::PrimaryKey {
        (*self).primary_key()
    }
}

impl<T, PK> PrimaryKeyLike for Rc<T>
where
    T: PrimaryKeyLike<PrimaryKey = PK>,
{
    type PrimaryKey = PK;

    fn primary_key(&self) -> Self::PrimaryKey {
        (**self).primary_key()
    }
}

impl<T, PK> PrimaryKeyLike for Box<T>
where
    T: PrimaryKeyLike<PrimaryKey = PK>,
{
    type PrimaryKey = PK;

    fn primary_key(&self) -> Self::PrimaryKey {
        (**self).primary_key()
    }
}
