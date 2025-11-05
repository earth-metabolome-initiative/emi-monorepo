use core_structures::{
    PhotographProcedureTemplate, ProcedureTemplate, User,
    tables::insertables::{
        PhotographProcedureTemplateSettable, ProcedureTemplateAssetModelSettable,
        ProcedureTemplateSettable,
    },
};
use diesel::OptionalExtension;
use web_common_traits::database::{DispatchableInsertableVariant, Insertable, Read};

use crate::procedure_template_asset_models::photograph::photograph_builder;

use crate::procedure_template_asset_models::panels::panel_model_builder;
use crate::procedure_templates::organism_observation_procedure;

/// Initializes the Organism observation procedure template in the database.
///
/// # Arguments
///
/// * `user` - The user who is creating the procedure template.
/// * `conn` - The database connection to use for the insertion.
///
/// # Panics
///
/// * If the connection fails to insert the procedure template.
/// * If the procedure template building fails.
pub(super) fn panel_observation_procedure(
    user: &User,
    conn: &mut diesel::PgConnection,
) -> anyhow::Result<ProcedureTemplate> {
    let name = "Panel observation procedure";

    if let Some(existing) = ProcedureTemplate::from_name(name, conn).optional()? {
        return Ok(existing);
    }

    let (_, _organism, phone) = organism_observation_procedure(user, conn)?;

    // Take a picture of organism and its panel
    let organism_and_panel_picture = PhotographProcedureTemplate::new()
        .name("Photograph the panel")?
        .description(
            "Photograph the information panel associated to the organism being collected.",
        )?
        .procedure_template_photographed_with_model(&phone)?
        .procedure_template_photographed_asset_model(panel_model_builder(user, conn)?)?
        .procedure_template_photograph_model(
            photograph_builder(user, conn)?.name("Panel Picture")?,
        )?
        .created_by(user)?
        .insert(user.id, conn)?;

    Ok(ProcedureTemplate::read(organism_and_panel_picture.procedure_template, conn)?)
}
