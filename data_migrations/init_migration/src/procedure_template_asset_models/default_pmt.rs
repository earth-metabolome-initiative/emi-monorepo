//! Submodule defining default partial builder initialization for
//! a standard procedure template trackable.

use core_structures::{
    ProcedureTemplateAssetModel, Trackable, User,
    tables::insertables::InsertableProcedureTemplateAssetModelBuilder,
};
use web_common_traits::database::Insertable;

/// Returns a partial builder for a default procedure template trackable.
///
/// # Arguments
///
/// * `user` - The user who is creating the trackable.
/// * `trackable` - The trackable to associate with the procedure template
///   trackable.
///
/// # Errors
///
/// * If the connection to the database fails.
pub(super) fn default_pmt<T>(
    user: &User,
    trackable: T,
) -> anyhow::Result<InsertableProcedureTemplateAssetModelBuilder>
where
    T: AsRef<Trackable>,
{
    let trackable = trackable.as_ref();

    let Some(name) = &trackable.name else {
        return Err(anyhow::anyhow!("Trackable must have a name, provided: {:?}", trackable));
    };

    Ok(ProcedureTemplateAssetModel::new()
        .name(name)?
        .trackable(trackable.id)?
        .created_by(user.id)?)
}
