#![cfg(feature = "postgres")]

//! Submodule defining the `ParentProcedureModel` trait.

use web_common_traits::{
    database::{InsertError, Insertable, InsertableVariant},
    prelude::ExtensionTable,
};

use crate::{
    ProcedureModel, Trackable, tables::insertables::InsertableProcedureModelTrackableAttributes,
};

/// Trait defining the methods for managing parent-child relationships in
/// procedure models.
pub trait Track: ExtensionTable<ProcedureModel>
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
        trackable: &T,
        name: &str,
        user: &crate::User,
        conn: &mut diesel::PgConnection,
    ) -> Result<
        crate::ProcedureModelTrackable,
        InsertError<InsertableProcedureModelTrackableAttributes>,
    >
    where
        T: ExtensionTable<Trackable>,
        for<'a> &'a T: diesel::Identifiable<Id = &'a rosetta_uuid::Uuid>,
    {
        use diesel::Identifiable;
        crate::ProcedureModelTrackable::new()
            .name(name)?
            .procedure_model(*self.id())?
            .trackable(*trackable.id())?
            .created_by(user.id)?
            .insert(user.id, conn)
    }
}

impl<T> Track for T
where
    T: ExtensionTable<ProcedureModel>,
    for<'a> &'a T: diesel::Identifiable<Id = &'a i32>,
{
}
