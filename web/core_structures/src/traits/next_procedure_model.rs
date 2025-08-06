#![cfg(feature = "postgres")]

//! Submodule defining the `AppendProcedureModel` trait.

use web_common_traits::{
    database::{InsertError, Insertable, InsertableVariant},
    prelude::ExtensionTable,
};

use crate::{
    NextProcedureModel, ProcedureModel, tables::insertables::InsertableNextProcedureModelAttributes,
};

/// Trait defining the methods for managing parent-child relationships in
/// procedure models.
pub trait AppendProcedureModel: ExtensionTable<ProcedureModel>
where
    for<'a> &'a Self: diesel::Identifiable<Id = &'a i32>,
{
    /// Creates a new parent-child relationship for a procedure.
    ///
    /// # Arguments
    ///
    /// * `current_procedure`: The child procedure model to be added.
    /// * `successor_procedure`: The procedure model that will be the successor.
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
    ) -> Result<crate::NextProcedureModel, InsertError<InsertableNextProcedureModelAttributes>>
    where
        C1: ExtensionTable<ProcedureModel>,
        C2: ExtensionTable<ProcedureModel>,
        for<'a> &'a C1: diesel::Identifiable<Id = &'a i32>,
        for<'a> &'a C2: diesel::Identifiable<Id = &'a i32>,
    {
        use diesel::Identifiable;

        // Then, we create a new NextProcedureModel entry linking the parent
        // procedure to the new child procedure.
        let next_procedure = NextProcedureModel::new()
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
    /// * `children`: The child procedure models to be added.
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
    ) -> Result<Vec<crate::NextProcedureModel>, InsertError<InsertableNextProcedureModelAttributes>>
    where
        C: ExtensionTable<ProcedureModel>,
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

impl<T> AppendProcedureModel for T
where
    T: ExtensionTable<ProcedureModel>,
    for<'a> &'a T: diesel::Identifiable<Id = &'a i32>,
{
}
