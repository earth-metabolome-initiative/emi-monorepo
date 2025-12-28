#![cfg(feature = "postgres")]

//! Submodule defining the `ParentProcedureTemplate` trait.

use web_common_traits::{
    database::{DispatchableInsertableVariant, InsertError, Insertable},
    prelude::ExtensionTable,
};

use crate::{
    ProcedureTemplate,
    codegen::structs_codegen::tables::insertables::ParentProcedureTemplateSettable,
    tables::insertables::ParentProcedureTemplateAttribute,
};

/// Trait defining the methods for managing parent-child relationships in
/// procedure templates.
pub trait ParentProcedureTemplate: ExtensionTable<ProcedureTemplate>
where
    for<'a> &'a Self: diesel::Identifiable<Id = &'a i32>,
{
    /// Creates a new parent-child relationship for a procedure.
    ///
    /// # Arguments
    ///
    /// * `child_procedure`: The child procedure template to be added.
    /// * `user`: The user who is creating the tool category.
    /// * `conn`: The database connection to use for the insertion.
    ///
    /// # Errors
    ///
    /// * If the insertion fails, an `InsertError` is returned.
    fn child<C>(
        &self,
        child_procedure: &C,
        user: &crate::User,
        conn: &mut diesel::PgConnection,
    ) -> Result<crate::ParentProcedureTemplate, InsertError<ParentProcedureTemplateAttribute>>
    where
        C: ExtensionTable<ProcedureTemplate>,
        for<'a> &'a C: diesel::Identifiable<Id = &'a i32>,
    {
        use diesel::Identifiable;
        let parent_procedure_template = crate::ParentProcedureTemplate::new()
            .parent(*self.id())?
            .child(*child_procedure.id())?
            .created_by(user.id)?
            .insert(user.id, conn)?;

        Ok(parent_procedure_template_id)
    }
}

impl<T> ParentProcedureTemplate for T
where
    T: ExtensionTable<ProcedureTemplate>,
    for<'a> &'a T: diesel::Identifiable<Id = &'a i32>,
{
}
