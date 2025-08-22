//! This module contains direct implementations of methods for determining
//! equivalent `ProcedureModelTrackable` instances within a `ProcedureModel`.
use diesel::PgConnection;

use crate::{ProcedureModel, ProcedureModelTrackable, SharedProcedureModelTrackable};

impl ProcedureModel {
    /// Returns the most high level procedure model trackable.
    ///
    /// # Implementation Details
    ///
    /// Finds the equivalent `ProcedureModelTrackable` in the current procedure
    /// model if it exists. The equivalence is determined by search the
    /// provided trackable within the procedure model's
    /// `SharedProcedureModelTrackable` collection, meaning the provided
    /// Trackable should always be from a child procedure model.
    ///
    /// # Arguments
    ///
    /// * `conn`: The connection to the database.
    ///
    /// # Errors
    ///
    /// * If the connection to the database fails.
    pub fn procedure_model_trackable_equivalent(
        &self,
        procedure_model_trackable: &ProcedureModelTrackable,
        parent_trackable: Option<&ProcedureModelTrackable>,
        conn: &mut PgConnection,
    ) -> Result<Option<ProcedureModelTrackable>, diesel::result::Error> {
        for shared_trackables in
            SharedProcedureModelTrackable::from_parent_procedure_model_id(&self.id, conn)?
        {
            // First, we check whether the parent-child relationship defined
            // by the `SharedProcedureModelTrackable` matches the provided
            // `ProcedureModelTrackable`. If it does, we return the parent
            // `ProcedureModelTrackable`.

            if let Some(parent_trackable) = parent_trackable {
                // If the parent trackable is provided, we check if the
                // shared trackables match the parent trackable.
                if shared_trackables.parent_id != parent_trackable.id {
                    continue;
                }
            }

            // Otherwise, we need to recursively check the child procedure model
            // trackables to see if the provided `ProcedureModelTrackable` exists
            // in any of them.
            // If we find a match in a child procedure model, then the parent trackable
            // associated with the current shared procedure model trackable is the
            // equivalent.
            if shared_trackables.child_id == procedure_model_trackable.id {
                let parent_trackable = shared_trackables.parent(conn)?;

                assert_eq!(parent_trackable.name, procedure_model_trackable.name);

                return Ok(Some(parent_trackable));
            }

            let child_trackable = shared_trackables.child(conn)?;
            if shared_trackables
                .child_procedure_model(conn)?
                .procedure_model_trackable_equivalent(
                    procedure_model_trackable,
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
