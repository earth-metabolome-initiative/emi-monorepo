use core_structures::{tables::insertables::{
        GeolocationProcedureTemplateSettable, PhotographProcedureTemplateSettable,
        ProcedureTemplateAssetModelSettable, ProcedureTemplateSettable,
    }, traits::AppendProcedureTemplate, GeolocationProcedureTemplate, PhotographProcedureTemplate, ProcedureTemplate, User, ProcedureTemplateAssetModel
};
use diesel::OptionalExtension;
use web_common_traits::database::{Insertable, InsertableVariant};

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
pub(crate) fn organism_observation_procedure(
    user: &User,
    conn: &mut diesel::PgConnection,
) -> anyhow::Result<(ProcedureTemplate, ProcedureTemplateAssetModel)> {
    let name = "Organism observation procedure";
    let photograph_procedure_name = "Organism in Ecosystem Picture";

    if let Some(existing) = ProcedureTemplate::from_name(name, conn).optional()? {
        let photograph_procedure = PhotographProcedureTemplate::from_name(photograph_procedure_name, conn)?;
        let organism = photograph_procedure.procedure_template_photographed_asset_model(conn)?;
        return Ok((existing, organism));
    }

    let observation_procedure = ProcedureTemplate::new()
        .name(name)?
        .description(
			"Procedure for observing an organism, and relevant details for identification and study.",
        )?
        .created_by(user.id)?
        .insert(user.id, conn)?;

    // Place the colored cardboard arrow in the field pointing towards the organism
    let arrow_reminder = ProcedureTemplate::new()
        .name("Place Arrow")?
        .description(
			"Place a colored cardboard arrow in the field pointing towards the organism to facilitate its identification later.",
        )?
        .created_by(user.id)?
        .insert(user.id, conn)?;

    // Take a picture of organism and surrounding ecosystem
    let organism_in_ecosystem_picture = PhotographProcedureTemplate::new()
        .name("Organism in Ecosystem Picture")?
        .description("Photograph of the organism in its surrounding ecosystem.")?
        .procedure_template_photographed_with_model(phone_builder(user, conn)?)?
        .procedure_template_photographed_asset_model(organism_builder(user, conn)?)?
        .procedure_template_photograph_model(
            photograph_builder(user, conn)?.name("Organism in Ecosystem Picture")?,
        )?
        .created_by(user.id)?
        .insert(user.id, conn)?;
    let organism = organism_in_ecosystem_picture.procedure_template_photographed_asset_model(conn)?;
    let phone = organism_in_ecosystem_picture.procedure_template_photographed_with_model;

    // Take a picture of the full organism
    let organism_picture = PhotographProcedureTemplate::new()
        .name("Organism Picture")?
        .description("Photograph of the full organism for identification.")?
        .procedure_template_photographed_with_model(phone)?
        .procedure_template_photographed_asset_model(&organism)?
        .procedure_template_photograph_model(
            photograph_builder(user, conn)?.name("Organism Picture")?,
        )?
        .created_by(user.id)?
        .insert(user.id, conn)?;

    // Take one of more picture of details of the organism to facilitate
    // identification (e.g. flowers)
    let organism_details_picture = PhotographProcedureTemplate::new()
        .name("Organism Details Picture")?
        .description("Photograph of details of the organism to facilitate identification.")?
        .procedure_template_photographed_with_model(phone)?
        .procedure_template_photographed_asset_model(&organism)?
        .procedure_template_photograph_model(
            photograph_builder(user, conn)?.name("Organism Details Picture")?,
        )?
        .created_by(user.id)?
        .insert(user.id, conn)?;

    // Logging geolocation
    let organism_geolocation = GeolocationProcedureTemplate::new()
        .name("Organism Geolocation")?
        .description("Geolocation of the organism observation.")?
        .procedure_template_geolocated_with_model(phone)?
        .procedure_template_geolocated_asset_model(&organism)?
        .created_by(user.id)?
        .insert(user.id, conn)?;

    observation_procedure.extend(
        &[
            &arrow_reminder,
            &organism_in_ecosystem_picture.procedure_template(conn)?,
            &organism_picture.procedure_template(conn)?,
            &organism_details_picture.procedure_template(conn)?,
            &organism_geolocation.procedure_template(conn)?,
        ],
        user,
        conn,
    )?;

    Ok((observation_procedure, organism))
}
