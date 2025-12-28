#![cfg(feature = "postgres")]

//! Submodule defining the `AppendProcedureTemplate` trait.

use web_common_traits::{
    database::{DispatchableInsertableVariant, InsertError, Insertable},
    prelude::ExtensionTable,
};

use crate::{
    NextProcedureTemplate, ProcedureTemplate, User,
    codegen::structs_codegen::tables::insertables::NextProcedureTemplateSettable,
    tables::insertables::NextProcedureTemplateAttribute,
};

/// Trait defining the methods for managing parent-child relationships in
/// procedure templates.
pub trait AppendProcedureTemplate: ExtensionTable<ProcedureTemplate> {
    /// Creates a new parent-child relationship for a procedure.
    ///
    /// # Arguments
    ///
    /// * `current_procedure`: The child procedure template to be added.
    /// * `successor_procedure`: The procedure template that will be the
    ///   successor.
    /// * `user`: The user who is creating the tool category.
    /// * `conn`: The database connection to use for the insertion.
    ///
    /// # Errors
    ///
    /// * If the insertion fails, an `InsertError` is returned.
    fn append(
        &self,
        current_procedure: &dyn ExtensionTable<ProcedureTemplate>,
        successor_procedure: &dyn ExtensionTable<ProcedureTemplate>,
        user: &User,
        conn: &mut diesel::PgConnection,
    ) -> Result<crate::NextProcedureTemplate, InsertError<NextProcedureTemplateAttribute>> {
        // Then, we create a new NextProcedureTemplate entry linking the parent
        // procedure to the new child procedure.
        let next_procedure = NextProcedureTemplate::new()
            .parent(self.primary_key())?
            .predecessor(current_procedure.primary_key())?
            .successor(successor_procedure.primary_key())?
            .created_by(user.id)?
            .insert(user.id, conn)?;

        Ok(next_procedure)
    }

    /// Creates a new parent-child relationship for a procedure.
    ///
    /// # Arguments
    ///
    /// * `children`: The child procedure templates to be added.
    /// * `user`: The user who is creating the tool category.
    /// * `conn`: The database connection to use for the insertion.
    ///
    /// # Errors
    ///
    /// * If the insertion fails, an `InsertError` is returned.
    fn extend(
        &self,
        children: &[Box<dyn ExtensionTable<ProcedureTemplate>>],
        user: &User,
        conn: &mut diesel::PgConnection,
    ) -> Result<Vec<crate::NextProcedureTemplate>, InsertError<NextProcedureTemplateAttribute>>
    {
        children
            .iter()
            .zip(children.iter().skip(1))
            .map(|(current_procedure, successor_procedure)| {
                self.append(&**current_procedure, &**successor_procedure, user, conn)
            })
            .collect()
    }
}

impl<T> AppendProcedureTemplate for T where T: ExtensionTable<ProcedureTemplate> {}
