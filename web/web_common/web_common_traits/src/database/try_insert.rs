//! Submodule defining the `TryInsert` trait, which allows to convert an
//! insertable builder into an insertable object after having processed the
//! necessary parent builders, if any.

use std::fmt::Debug;

use common_traits::builder::IsCompleteBuilder;

use crate::{
    database::{IdOrBuilder, InsertError},
    prelude::SetPrimaryKey,
};

/// Trait allowing to define a default value for an ancestor attribute type.
pub trait DefaultExtensionAttribute<ExtensionAttribute> {
    /// Returns the default value for the target attribute.
    fn target_default() -> Self;
}

/// Trait allowing to convert an attribute into another attribute type,
/// typically used to convert an attribute of an extension table into an
/// attribute of the base table.
pub trait FromExtensionAttribute<ExtensionAttribute, Ancestor>:
    DefaultExtensionAttribute<ExtensionAttribute>
{
    /// The effective attribute type that the ancestor has.
    type EffectiveExtensionAttribute;

    /// Converts the current attribute into the target attribute type.
    ///
    /// # Arguments
    ///
    /// * `extension_attribute` - The attribute of the extension table to
    ///   convert.
    fn from_extension_attribute(extension_attribute: Self::EffectiveExtensionAttribute) -> Self;
}

/// Trait defining the properties that any generic associated with a type
/// implementing `TryInsert` must have.
pub trait TryInsertGeneric<C>: SetPrimaryKey + IsCompleteBuilder {
    /// Attributes enumeration for the insertable object.
    type Attribute: Debug;

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
    type Attribute = ();

    fn mint_primary_key(
        self,
        _user_id: i32,
        _conn: &mut C,
    ) -> Result<Self::PrimaryKey, InsertError<Self::Attribute>> {
        self.ok_or(InsertError::BuilderError(
            common_traits::prelude::BuilderError::IncompleteBuild(()),
        ))
    }
}

impl<ExtensionAttribute, DescendantAttribute, T>
    FromExtensionAttribute<ExtensionAttribute, Option<T>> for DescendantAttribute
where
    DescendantAttribute: DefaultExtensionAttribute<ExtensionAttribute>,
{
    type EffectiveExtensionAttribute = ();

    fn from_extension_attribute(_extension_attribute: ()) -> Self {
        Self::target_default()
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

impl<C, Id, Builder> TryInsertGeneric<C> for IdOrBuilder<Id, Builder>
where
    Id: SetPrimaryKey<PrimaryKey = Id>,
    Builder: TryInsertGeneric<C, PrimaryKey = Id>,
{
    type Attribute = Builder::Attribute;

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
