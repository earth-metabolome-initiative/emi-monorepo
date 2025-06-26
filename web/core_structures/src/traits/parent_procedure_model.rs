#![cfg(feature = "postgres")]

//! Submodule defining the `ParentProcedureModel` trait.

use web_common_traits::{
    database::{InsertError, Insertable, InsertableVariant},
    prelude::ExtensionTable,
};

use crate::{
    ProcedureModel, ProcedureModelTrackable, SharedProcedureModelTrackable,
    tables::insertables::InsertableParentProcedureModelAttributes,
};

#[derive(Default, Copy, Clone, Hash, PartialEq, Eq)]
pub struct ChildOptions {
    snoozable: bool,
    copiable: bool,
    repeatable: bool,
    skippable: bool,
    inherit_trackables: bool,
}

impl ChildOptions {
    /// Sets the snoozable option.
    pub fn snoozable(mut self) -> Self {
        self.snoozable = true;
        self
    }

    /// Sets the copiable option.
    pub fn copiable(mut self) -> Self {
        self.copiable = true;
        self
    }

    /// Sets the repeatable option.
    pub fn repeatable(mut self) -> Self {
        self.repeatable = true;
        self
    }

    /// Sets the skippable option.
    pub fn skippable(mut self) -> Self {
        self.skippable = true;
        self
    }

    /// Sets whether to inherit trackables
    pub fn inherit_trackables(mut self) -> Self {
        self.inherit_trackables = true;
        self
    }
}

/// Trait defining the methods for managing parent-child relationships in
/// procedure models.
pub trait ParentProcedureModel: ExtensionTable<ProcedureModel>
where
    for<'a> &'a Self: diesel::Identifiable<Id = &'a i32>,
{
    /// Creates a new parent-child relationship for a procedure.
    ///
    /// # Arguments
    ///
    /// * `child_procedure`: The child procedure model to be added.
    /// * `user`: The user who is creating the tool category.
    /// * `conn`: The database connection to use for the insertion.
    ///
    /// # Errors
    ///
    /// * If the insertion fails, an `InsertError` is returned.
    fn child<C>(
        &self,
        child_procedure: &C,
        options: ChildOptions,
        user: &crate::User,
        conn: &mut diesel::PgConnection,
    ) -> Result<crate::ParentProcedureModel, InsertError<InsertableParentProcedureModelAttributes>>
    where
        C: ExtensionTable<ProcedureModel>,
        for<'a> &'a C: diesel::Identifiable<Id = &'a i32>,
    {
        use diesel::Identifiable;
        let parent_procedure_model = crate::ParentProcedureModel::new()
            .parent_procedure_model_id(*self.id())?
            .child_procedure_model_id(*child_procedure.id())?
            .snoozable(options.snoozable)?
            .copiable(options.copiable)?
            .repeatable(options.repeatable)?
            .skippable(options.skippable)?
            .created_by(user.id)?
            .insert(user.id, conn)?;

        if options.inherit_trackables {
            for child_trackable in
                ProcedureModelTrackable::from_procedure_model_id(child_procedure.id(), conn)?
            {
                let parent_trackable = if let Some(parent_trackable) =
                    ProcedureModelTrackable::from_name_and_procedure_model_id(
                        &child_trackable.name,
                        self.id(),
                        conn,
                    )? {
                    parent_trackable
                } else {
                    ProcedureModelTrackable::new()
                        .name(child_trackable.name)
                        .unwrap()
                        .procedure_model_id(*self.id())
                        .unwrap()
                        .trackable_id(child_trackable.trackable_id)
                        .unwrap()
                        .created_by(user.id)
                        .unwrap()
                        .insert(user.id, conn)
                        .unwrap()
                };

                SharedProcedureModelTrackable::new()
                    .parent_id(parent_trackable.id)
                    .unwrap()
                    .child_id(child_trackable.id)
                    .unwrap()
                    .parent_trackable_id(parent_trackable.trackable_id)
                    .unwrap()
                    .child_trackable_id(child_trackable.trackable_id)
                    .unwrap()
                    .parent_procedure_model_id(parent_trackable.procedure_model_id)
                    .unwrap()
                    .child_procedure_model_id(child_trackable.procedure_model_id)
                    .unwrap()
                    .created_by(user.id)
                    .unwrap()
                    .insert(user.id, conn)
                    .unwrap();
            }
        }

        Ok(parent_procedure_model)
    }
}

impl<T> ParentProcedureModel for T
where
    T: ExtensionTable<ProcedureModel>,
    for<'a> &'a T: diesel::Identifiable<Id = &'a i32>,
{
}
