#![cfg(feature = "postgres")]

//! Submodule defining the `AppendProcedureTemplate` trait.

use web_common_traits::{
    database::{InsertError, Insertable, InsertableVariant},
    prelude::ExtensionTable,
};

use crate::{
    AssetCompatibilityRule, AssetModel,
    codegen::structs_codegen::tables::insertables::AssetCompatibilityRuleBuildable,
    tables::insertables::InsertableAssetCompatibilityRuleAttributes,
};

/// Trait defining the methods for managing parent-child relationships in
/// procedure templates.
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
    fn compatible_with<T>(
        &self,
        other: &T,
        user: &crate::User,
        conn: &mut diesel::PgConnection,
    ) -> Result<AssetCompatibilityRule, InsertError<InsertableAssetCompatibilityRuleAttributes>>
    where
        T: ExtensionTable<AssetModel>,
        for<'a> &'a T: diesel::Identifiable<Id = &'a i32>,
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
