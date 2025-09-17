//! Submodule defining the `TryInsert` trait, which allows to convert an
//! insertable builder into an insertable object after having processed the
//! necessary parent builders, if any.

use common_traits::builder::{Attributed, IsCompleteBuilder};

use crate::{
    database::{IdOrBuilder, InsertError},
    prelude::SetPrimaryKey,
};

/// Trait defining the properties that any generic associated with a type
/// implementing `TryInsert` must have.
pub trait TryInsertGeneric<C>: SetPrimaryKey + IsCompleteBuilder + Attributed {
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
    ) -> Result<Self::PrimaryKey, InsertError<Self::Attribute>>;
}

/// When the extended table in a DAG structure is not another builder
/// which would implement `TryInsertGeneric` but an `Option<Primary Key Type>`,
/// the trait makes reference to the following implementation.
impl<C, T> TryInsertGeneric<C> for Option<T>
where
    T: SetPrimaryKey<PrimaryKey = T>,
{
    fn mint_primary_key(
        self,
        _user_id: i32,
        _conn: &mut C,
    ) -> Result<Self::PrimaryKey, InsertError<Self::Attribute>> {
        self.ok_or(InsertError::BuilderError(
            common_traits::prelude::BuilderError::IncompleteBuild(Default::default()),
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
            Self::Id(id) => Self::Id(id.set_primary_key(primary_key)),
            Self::Builder(builder) => Self::Builder(builder.set_primary_key(primary_key)),
        }
    }
}

impl<Id, Builder> Attributed for IdOrBuilder<Id, Builder>
where
    Builder: Attributed,
{
    type Attribute = Builder::Attribute;
}

impl<C, Id, Builder> TryInsertGeneric<C> for IdOrBuilder<Id, Builder>
where
    Id: SetPrimaryKey<PrimaryKey = Id>,
    Builder: TryInsertGeneric<C, PrimaryKey = Id>,
{
    fn mint_primary_key(
        self,
        user_id: i32,
        conn: &mut C,
    ) -> Result<Self::PrimaryKey, InsertError<Self::Attribute>> {
        match self {
            Self::Id(id) => Ok(id),
            Self::Builder(builder) => builder.mint_primary_key(user_id, conn),
        }
    }
}
