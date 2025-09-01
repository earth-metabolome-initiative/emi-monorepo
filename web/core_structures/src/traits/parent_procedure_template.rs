#![cfg(feature = "postgres")]

//! Submodule defining the `ParentProcedureTemplate` trait.

use diesel::OptionalExtension;
use web_common_traits::{
    database::{InsertError, Insertable, InsertableVariant, Read},
    prelude::ExtensionTable,
};

use crate::{
    ProcedureTemplate, ProcedureTemplateAssetModel, SharedProcedureTemplateAssetModel,
    tables::insertables::InsertableParentProcedureTemplateAttributes,
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
        options: ChildOptions,
        user: &crate::User,
        conn: &mut diesel::PgConnection,
    ) -> Result<
        crate::ParentProcedureTemplate,
        InsertError<InsertableParentProcedureTemplateAttributes>,
    >
    where
        C: ExtensionTable<ProcedureTemplate>,
        for<'a> &'a C: diesel::Identifiable<Id = &'a i32>,
    {
        use diesel::Identifiable;
        let child_procedure = ProcedureTemplate::read(*child_procedure.id(), conn)?
            .expect("Child procedure template not found");
        let parent_procedure_template = crate::ParentProcedureTemplate::new()
            .parent_procedure_template(*self.id())?
            .child_procedure_template(child_procedure.id)?
            .snoozable(options.snoozable)?
            .copiable(options.copiable)?
            .repeatable(options.repeatable)?
            .skippable(options.skippable)?
            .created_by(user.id)?
            .insert(user.id, conn)?;

        if options.inherit_trackables {
            for child_trackable in
                ProcedureTemplateAssetModel::from_procedure_template(&child_procedure.id, conn)?
            {
                let parent_trackable = if let Some(parent_trackable) =
                    ProcedureTemplateAssetModel::from_name_and_procedure_template(
                        &child_trackable.name,
                        self.id(),
                        conn,
                    )
                    .optional()?
                {
                    parent_trackable
                } else {
                    ProcedureTemplateAssetModel::new()
                        .name(&child_trackable.name)
                        .unwrap()
                        .procedure_template(*self.id())
                        .unwrap()
                        .trackable(child_trackable.trackable_id)
                        .unwrap()
                        .created_by(user.id)
                        .unwrap()
                        .insert(user.id, conn)
                        .unwrap()
                };

                SharedProcedureTemplateAssetModel::new()
                    .parent(parent_trackable.id)
                    .unwrap()
                    .child(child_trackable.id)
                    .unwrap()
                    .parent_trackable(parent_trackable.trackable_id)
                    .unwrap()
                    .child_trackable(child_trackable.trackable_id)
                    .unwrap()
                    .parent_procedure_template(parent_trackable.procedure_template)
                    .unwrap()
                    .child_procedure_template(child_trackable.procedure_template)
                    .unwrap()
                    .created_by(user.id)
                    .unwrap()
                    .insert(user.id, conn)
                    .unwrap();
            }
        }

        Ok(parent_procedure_template)
    }
}

impl<T> ParentProcedureTemplate for T
where
    T: ExtensionTable<ProcedureTemplate>,
    for<'a> &'a T: diesel::Identifiable<Id = &'a i32>,
{
}
