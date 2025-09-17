#![cfg(feature = "postgres")]

use web_common_traits::{
    database::{DispatchableInsertableVariant, InsertError, Insertable},
    prelude::ExtensionTable,
};

use crate::{
    AssetCompatibilityRule, AssetModel, ContainerCompatibilityRule, ContainerModel,
    codegen::structs_codegen::tables::insertables::{
        AssetCompatibilityRuleSettable, ContainerCompatibilityRuleSettable,
    },
    tables::insertables::{AssetCompatibilityRuleAttribute, ContainerCompatibilityRuleAttribute},
};

/// A trait for asset models that can be compatible with other asset models.
pub trait CompatibleWith: ExtensionTable<AssetModel>
where
    for<'a> &'a Self: diesel::Identifiable<Id = &'a i32>,
{
    /// Creates a new `AssetCompatibilityRule` linking the current trackable
    /// with another
    ///
    /// # Arguments
    ///
    /// * `other` - A reference to another trackable item that this one is
    ///   compatible with.
    /// * `user` - The user performing the operation, used for tracking who
    ///   created the compatibility rule.
    /// * `conn` - A mutable reference to the database connection where the
    ///   operation will be performed.
    ///
    /// # Errors
    ///
    /// * If the insertion fails, an `InsertError` is returned.
    fn compatible_with<AM>(
        &self,
        other: &AM,
        user: &crate::User,
        conn: &mut diesel::PgConnection,
    ) -> Result<AssetCompatibilityRule, InsertError<AssetCompatibilityRuleAttribute>>
    where
        AM: ExtensionTable<AssetModel>,
        for<'a> &'a AM: diesel::Identifiable<Id = &'a i32>,
    {
        use diesel::Identifiable;

        // Then, we create a new NextProcedureTemplate entry linking the parent
        // procedure to the new child procedure.
        AssetCompatibilityRule::new()
            .left_asset_model(*self.id())?
            .right_asset_model(*other.id())?
            .created_by(user.id)?
            .insert(user.id, conn)
    }
}

impl<T> CompatibleWith for T
where
    T: ExtensionTable<AssetModel>,
    for<'a> &'a T: diesel::Identifiable<Id = &'a i32>,
{
}

/// A trait for container models that can contain other asset models.
pub trait CanContain: ExtensionTable<ContainerModel>
where
    for<'a> &'a Self: diesel::Identifiable<Id = &'a i32>,
{
    /// Creates a new `AssetCompatibilityRule` linking the current trackable
    /// with another
    ///
    /// # Arguments
    ///
    /// * `other` - A reference to another trackable item that this one is
    ///   compatible with.
    /// * `user` - The user performing the operation, used for tracking who
    ///   created the compatibility rule.
    /// * `conn` - A mutable reference to the database connection where the
    ///   operation will be performed.
    ///
    /// # Errors
    ///
    /// * If the insertion fails, an `InsertError` is returned.
    fn can_contain<AM>(
        &self,
        other: &AM,
        quantity: i16,
        user: &crate::User,
        conn: &mut diesel::PgConnection,
    ) -> Result<ContainerCompatibilityRule, InsertError<ContainerCompatibilityRuleAttribute>>
    where
        AM: ExtensionTable<AssetModel>,
        for<'a> &'a AM: diesel::Identifiable<Id = &'a i32>,
    {
        use diesel::Identifiable;

        // Then, we create a new NextProcedureTemplate entry linking the parent
        // procedure to the new child procedure.
        ContainerCompatibilityRule::new()
            .container_model(*self.id())?
            .contained_asset_model(*other.id())?
            .quantity(quantity)?
            .created_by(user.id)?
            .insert(user.id, conn)
    }
}

impl<T> CanContain for T
where
    T: ExtensionTable<ContainerModel>,
    for<'a> &'a T: diesel::Identifiable<Id = &'a i32>,
{
}
