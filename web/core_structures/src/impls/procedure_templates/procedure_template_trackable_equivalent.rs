//! This module contains direct implementations of methods for determining
//! equivalent `ProcedureTemplateAssetModel` instances within a
//! `ProcedureTemplate`.
use diesel::PgConnection;

use crate::{ProcedureTemplate, ProcedureTemplateAssetModel, SharedProcedureTemplateAssetModel};

impl ProcedureTemplate {
    /// Returns the most high level procedure template trackable.
    ///
    /// # Implementation Details
    ///
    /// Finds the equivalent `ProcedureTemplateAssetModel` in the current
    /// procedure model if it exists. The equivalence is determined by
    /// search the provided trackable within the procedure template's
    /// `SharedProcedureTemplateAssetModel` collection, meaning the provided
    /// Trackable should always be from a child procedure template.
    ///
    /// # Arguments
    ///
    /// * `conn`: The connection to the database.
    ///
    /// # Errors
    ///
    /// * If the connection to the database fails.
    pub fn procedure_template_trackable_equivalent(
        &self,
        procedure_template_trackable: &ProcedureTemplateAssetModel,
        parent_trackable: Option<&ProcedureTemplateAssetModel>,
        conn: &mut PgConnection,
    ) -> Result<Option<ProcedureTemplateAssetModel>, diesel::result::Error> {
        for shared_trackables in
            SharedProcedureTemplateAssetModel::from_parent_procedure_template(&self.id, conn)?
        {
            // First, we check whether the parent-child relationship defined
            // by the `SharedProcedureTemplateAssetModel` matches the provided
            // `ProcedureTemplateAssetModel`. If it does, we return the parent
            // `ProcedureTemplateAssetModel`.

            if let Some(parent_trackable) = parent_trackable {
                // If the parent trackable is provided, we check if the
                // shared trackables match the parent trackable.
                if shared_trackables.parent != parent_trackable.id {
                    continue;
                }
            }

            // Otherwise, we need to recursively check the child procedure template
            // trackables to see if the provided `ProcedureTemplateAssetModel` exists
            // in any of them.
            // If we find a match in a child procedure template, then the parent trackable
            // associated with the current shared procedure template trackable is the
            // equivalent.
            if shared_trackables.child_id == procedure_template_trackable.id {
                let parent_trackable = shared_trackables.parent(conn)?;

                assert_eq!(parent_trackable.name, procedure_template_trackable.name);

                return Ok(Some(parent_trackable));
            }

            let child_trackable = shared_trackables.child(conn)?;
            if shared_trackables
                .child_procedure_template(conn)?
                .procedure_template_trackable_equivalent(
                    procedure_template_trackable,
                    Some(&child_trackable),
                    conn,
                )?
                .is_some()
            {
                return Ok(Some(shared_trackables.parent(conn)?));
            }
        }
        Ok(None)
    }
}
