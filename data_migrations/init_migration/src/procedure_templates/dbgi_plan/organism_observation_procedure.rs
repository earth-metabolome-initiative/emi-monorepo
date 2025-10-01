use core_structures::{
    GeolocationProcedureTemplate, PhotographProcedureTemplate, ProcedureTemplate,
    ProcedureTemplateAssetModel, User,
    tables::insertables::{
        GeolocationProcedureTemplateSettable, PhotographProcedureTemplateSettable,
        ProcedureTemplateAssetModelSettable, ProcedureTemplateSettable,
    },
    traits::AppendProcedureTemplate,
};
use diesel::OptionalExtension;
use web_common_traits::database::{DispatchableInsertableVariant, Insertable, PrimaryKeyLike};

use crate::procedure_template_asset_models::{
    organism::organism_builder, phone::phone_builder, photograph::photograph_builder,
};

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
pub fn organism_observation_procedure(
    user: &User,
    conn: &mut diesel::PgConnection,
) -> anyhow::Result<(ProcedureTemplate, ProcedureTemplateAssetModel, ProcedureTemplateAssetModel)> {
    let name = "Organism observation procedure";

    const ORGANISM_PTAM_NAME: &str = "Organism";
    const PHONE_PTAM_NAME: &str = "Phone";

    if let Some(existing) = ProcedureTemplate::from_name(name, conn).optional()? {
        let organism = ProcedureTemplateAssetModel::from_name_and_procedure_template(
            ORGANISM_PTAM_NAME,
            existing.primary_key(),
            conn,
        )?;
        let phone = ProcedureTemplateAssetModel::from_name_and_procedure_template(
            PHONE_PTAM_NAME,
            existing.primary_key(),
            conn,
        )?;
        return Ok((existing, organism, phone));
    }

    let observation_procedure = ProcedureTemplate::new()
        .name(name)?
        .description(
			"Procedure for observing an organism, and relevant details for identification and study.",
        )?
        .created_by(user)?
        .insert(user.id, conn)?;

    // Place the colored cardboard arrow in the field pointing towards the organism
    let arrow_reminder = ProcedureTemplate::new()
        .name("Place Arrow")?
        .description(
			"Place a colored cardboard arrow in the field pointing towards the organism to facilitate its identification later.",
        )?
        .created_by(user)?
        .insert(user.id, conn)?;

    // Take a picture of organism and its panel
    let organism_and_panel_picture = PhotographProcedureTemplate::new()
        .name("Organism and Panel Picture")?
        .description("Photograph of the organism and its panel in the botanical garden")?
        .procedure_template_photographed_with_model(
            phone_builder(user, conn)?.name(PHONE_PTAM_NAME)?,
        )?
        .procedure_template_photographed_asset_model(
            organism_builder(user, conn)?.name(ORGANISM_PTAM_NAME)?,
        )?
        .procedure_template_photograph_model(
            photograph_builder(user, conn)?.name("Organism and Panel Picture")?,
        )?
        .created_by(user)?
        .insert(user.id, conn)?;
    let organism = organism_and_panel_picture.procedure_template_photographed_asset_model(conn)?;
    let phone = organism_and_panel_picture.procedure_template_photographed_with_model(conn)?;

    // Take a picture of the full organism
    let organism_picture = PhotographProcedureTemplate::new()
        .name("Organism Picture")?
        .description("Photograph of the full organism for identification.")?
        .procedure_template_photographed_with_model(&phone)?
        .procedure_template_photographed_asset_model(&organism)?
        .procedure_template_photograph_model(
            photograph_builder(user, conn)?.name("Organism Picture")?,
        )?
        .created_by(user)?
        .insert(user.id, conn)?;

    // Take a picture of details of the organism to facilitate
    // identification (e.g. flowers)
    let organism_details_picture = PhotographProcedureTemplate::new()
        .name("Organism Details Picture")?
        .description("Photograph of details of the organism to facilitate identification.")?
        .procedure_template_photographed_with_model(&phone)?
        .procedure_template_photographed_asset_model(&organism)?
        .procedure_template_photograph_model(
            photograph_builder(user, conn)?.name("Organism Details Picture")?,
        )?
        .created_by(user)?
        .insert(user.id, conn)?;

    // Take a picture of the part of the organism that will be
    // collected as a sample
    // (e.g. a leaf, a flower, a piece of bark)
    let organism_collected_part_picture = PhotographProcedureTemplate::new()
        .name("Organism Collected Part Picture")?
        .description(
            "Photograph of the part of the organism that will be collected as a sample (e.g. a leaf, a flower, a piece of bark).",
        )?
        .procedure_template_photographed_with_model(&phone)?
        .procedure_template_photographed_asset_model(&organism)?
        .procedure_template_photograph_model(
            photograph_builder(user, conn)?.name("Organism Collected Part Picture")?,
        )?
        .created_by(user)?
        .insert(user.id, conn)?;

    // Logging geolocation
    let organism_geolocation = GeolocationProcedureTemplate::new()
        .name("Organism Geolocation")?
        .description("Geolocation of the organism observation.")?
        .procedure_template_geolocated_with_model(&phone)?
        .procedure_template_geolocated_asset_model(&organism)?
        .created_by(user)?
        .insert(user.id, conn)?;

    observation_procedure.extend(
        &[
            arrow_reminder.into(),
            organism_and_panel_picture.into(),
            organism_picture.into(),
            organism_details_picture.into(),
            organism_collected_part_picture.into(),
            organism_geolocation.into(),
        ],
        user,
        conn,
    )?;

    Ok((observation_procedure, organism, phone))
}
