#![cfg(feature = "postgres")]

//! Submodule defining the `AppendProcedureTemplate` trait.

use web_common_traits::{
    database::{InsertError, Insertable, InsertableVariant},
    prelude::ExtensionTable,
};

use crate::{
    NextProcedureTemplate, ProcedureTemplate,
    codegen::structs_codegen::tables::insertables::NextProcedureTemplateSettable,
    tables::insertables::NextProcedureTemplateAttribute,
};

/// Trait defining the methods for managing parent-child relationships in
/// procedure templates.
pub trait AppendProcedureTemplate: ExtensionTable<ProcedureTemplate>
where
    for<'a> &'a Self: diesel::Identifiable<Id = &'a i32>,
{
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
    fn append<C1, C2>(
        &self,
        current_procedure: &C1,
        successor_procedure: &C2,
        user: &crate::User,
        conn: &mut diesel::PgConnection,
    ) -> Result<crate::NextProcedureTemplate, InsertError<NextProcedureTemplateAttribute>>
    where
        C1: ExtensionTable<ProcedureTemplate>,
        C2: ExtensionTable<ProcedureTemplate>,
        for<'a> &'a C1: diesel::Identifiable<Id = &'a i32>,
        for<'a> &'a C2: diesel::Identifiable<Id = &'a i32>,
    {
        use diesel::Identifiable;

        // Then, we create a new NextProcedureTemplate entry linking the parent
        // procedure to the new child procedure.
        let next_procedure = NextProcedureTemplate::new()
            .parent(*self.id())?
            .current(*current_procedure.id())?
            .successor(*successor_procedure.id())?
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
    fn extend<C>(
        &self,
        children: &[&C],
        user: &crate::User,
        conn: &mut diesel::PgConnection,
    ) -> Result<Vec<crate::NextProcedureTemplate>, InsertError<NextProcedureTemplateAttribute>>
    where
        C: ExtensionTable<ProcedureTemplate>,
        for<'a> &'a C: diesel::Identifiable<Id = &'a i32>,
    {
        children
            .iter()
            .zip(children.iter().skip(1))
            .map(|(current_procedure, successor_procedure)| {
                self.append(*current_procedure, *successor_procedure, user, conn)
            })
            .collect()
    }
}

impl<T> AppendProcedureTemplate for T
where
    T: ExtensionTable<ProcedureTemplate>,
    for<'a> &'a T: diesel::Identifiable<Id = &'a i32>,
{
}
