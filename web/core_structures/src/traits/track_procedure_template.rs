#![cfg(feature = "postgres")]

//! Submodule defining the `ParentProcedureTemplate` trait.

use web_common_traits::{
    database::{DispatchableInsertableVariant, InsertError, Insertable},
    prelude::ExtensionTable,
};

use crate::{
    AssetModel, ProcedureTemplate,
    codegen::structs_codegen::tables::insertables::ProcedureTemplateAssetModelSettable,
    tables::insertables::ProcedureTemplateAssetModelAttribute,
};

/// Trait defining the methods for managing parent-child relationships in
/// procedure templates.
pub trait Track: ExtensionTable<ProcedureTemplate>
where
    for<'a> &'a Self: diesel::Identifiable<Id = &'a i32>,
{
    /// Creates a new parent-child relationship for a procedure.
    ///
    /// # Arguments
    ///
    /// * `user`: The user who is creating the tool category.
    /// * `conn`: The database connection to use for the insertion.
    ///
    /// # Errors
    ///
    /// * If the insertion fails, an `InsertError` is returned.
    fn track<T>(
        &self,
        asset_model: &T,
        name: &str,
        user: &crate::User,
        conn: &mut diesel::PgConnection,
    ) -> Result<crate::ProcedureTemplateAssetModel, InsertError<ProcedureTemplateAssetModelAttribute>>
    where
        T: ExtensionTable<AssetModel>,
        for<'a> &'a T: diesel::Identifiable<Id = &'a i32>,
    {
        use diesel::Identifiable;
        crate::ProcedureTemplateAssetModel::new()
            .name(name)?
            .procedure_template(*self.id())?
            .asset_model(*asset_model.id())?
            .insert(user.id, conn)
    }
}

impl<T> Track for T
where
    T: ExtensionTable<ProcedureTemplate>,
    for<'a> &'a T: diesel::Identifiable<Id = &'a i32>,
{
}
