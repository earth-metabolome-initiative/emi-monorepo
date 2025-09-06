//! Submodule defining the `TryInsert` trait, which allows to convert an
//! insertable builder into an insertable object after having processed the
//! necessary parent builders, if any.

use crate::{
    database::{IdOrBuilder, InsertError},
    prelude::SetPrimaryKey,
};

/// Trait defining the properties that any generic associated with a type
/// implementing `TryInsert` must have.
pub trait TryInsertGeneric<C>: SetPrimaryKey {
    /// Attributes enumeration for the insertable object.
    type Attributes;

    /// Returns whether the generic is complete and can mint a primary key.
    fn is_complete(&self) -> bool;

    /// Consumes the generic, potentially inserting it into the database,
    /// and returns the primary key.
    ///
    /// # Arguments
    ///
    /// * `user_id` - The ID of the user performing the insert operation.
    /// * `conn` - A mutable reference to the database connection.
    ///
    /// # Errors
    ///
    /// * `InsertError` - If the insert operation fails, it returns an error
    ///   containing the attributes of the insertable object.
    fn mint_primary_key(
        self,
        user_id: i32,
        conn: &mut C,
    ) -> Result<Self::PrimaryKey, InsertError<Self::Attributes>>;
}

/// When the extended table in a DAG structure is not another builder
/// which would implement `TryInsertGeneric` but an `Option<Primary Key Type>`,
/// the trait makes reference to the following implementation.
impl<C, T> TryInsertGeneric<C> for Option<T>
where
    T: SetPrimaryKey<PrimaryKey = T>,
{
    type Attributes = ();

    fn is_complete(&self) -> bool {
        self.is_some()
    }

    fn mint_primary_key(
        self,
        _user_id: i32,
        _conn: &mut C,
    ) -> Result<Self::PrimaryKey, InsertError<Self::Attributes>> {
        self.ok_or(InsertError::BuilderError(
            common_traits::prelude::BuilderError::IncompleteBuild(()),
        ))
    }
}

impl<Id, Builder> SetPrimaryKey for IdOrBuilder<Id, Builder>
where
    Id: SetPrimaryKey<PrimaryKey = Id>,
    Builder: SetPrimaryKey<PrimaryKey = Id>,
{
    type PrimaryKey = Id;

    fn set_primary_key(self, primary_key: Self::PrimaryKey) -> Self {
        match self {
            Self::Id(maybe_id) => {
                Self::Id(<Option<Id> as SetPrimaryKey>::set_primary_key(maybe_id, primary_key))
            }
            Self::Builder(builder) => Self::Builder(builder.set_primary_key(primary_key)),
        }
    }
}

impl<C, Id, Builder> TryInsertGeneric<C> for IdOrBuilder<Id, Builder>
where
    Id: SetPrimaryKey<PrimaryKey = Id>,
    Builder: TryInsertGeneric<C, PrimaryKey = Id>,
{
    type Attributes = Builder::Attributes;

    fn is_complete(&self) -> bool {
        match self {
            Self::Id(maybe_id) => <Option<Id> as TryInsertGeneric<C>>::is_complete(maybe_id),
            Self::Builder(builder) => builder.is_complete(),
        }
    }

    fn mint_primary_key(
        self,
        user_id: i32,
        conn: &mut C,
    ) -> Result<Self::PrimaryKey, InsertError<Self::Attributes>> {
        match self {
            Self::Id(id) => {
                if let Some(id) = id {
                    Ok(id)
                } else {
                    unreachable!("Checked completeness before calling mint_primary_key")
                }
            }
            Self::Builder(builder) => builder.mint_primary_key(user_id, conn),
        }
    }
}
