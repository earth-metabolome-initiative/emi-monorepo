use core_structures::{
    GeolocationProcedureTemplate, PhotographProcedureTemplate, ProcedureTemplate,
    RegisteringProcedureTemplate, User,
    tables::insertables::{
        GeolocationProcedureTemplateSettable, PhotographProcedureTemplateSettable,
        ProcedureTemplateSettable, RegisteringProcedureTemplateSettable,
    },
    traits::{AppendProcedureTemplate, ChildOptions, ParentProcedureTemplate},
};
use diesel::OptionalExtension;
use web_common_traits::database::{Insertable, InsertableVariant};

use crate::procedure_template_asset_models::{organism::organism_builder, phone::phone_builder};

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
pub(crate) fn init_organism_observation_procedure(
    user: &User,
    conn: &mut diesel::PgConnection,
) -> anyhow::Result<ProcedureTemplate> {
    let name = "Organism observation procedure";

    if let Some(existing) = ProcedureTemplate::from_name(name, conn).optional()? {
        return Ok(existing);
    }

    let observation_procedure = ProcedureTemplate::new()
        .name(name)?
        .description(
			"Procedure for observing an organism, and relevant details for identification and study.",
        )?
        .created_by(user.id)?
        .insert(user.id, conn)?;

    let registering_procedure = RegisteringProcedureTemplate::new()
        .name("Organism Registration Procedure")?
        .description("Procedure for registering an organism.")?
        .procedure_template_registered_asset_model(organism_builder(user, conn)?)?
        .created_by(user.id)?
        .insert(user.id, conn)?;
    let organism = registering_procedure.procedure_template_registered_asset_model;

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
        .procedure_template_photographed_asset_model(organism)?
        .created_by(user.id)?
        .insert(user.id, conn)?;

    // Take a picture of the full organism
    let organism_picture = PhotographProcedureTemplate::new()
        .name("Organism Picture")?
        .description("Photograph of the full organism for identification.")?
        .procedure_template_photographed_with_model(phone_builder(user, conn)?)?
        .procedure_template_photographed_asset_model(organism)?
        .created_by(user.id)?
        .insert(user.id, conn)?;

    // Take one of more picture of details of the organism to facilitate
    // identification (e.g. flowers)
    let organism_details_picture = PhotographProcedureTemplate::new()
        .name("Organism Details Picture")?
        .description("Photograph of details of the organism to facilitate identification.")?
        .procedure_template_photographed_with_model(phone_builder(user, conn)?)?
        .procedure_template_photographed_asset_model(organism)?
        .created_by(user.id)?
        .insert(user.id, conn)?;

    // Logging geolocation
    let organism_geolocation = GeolocationProcedureTemplate::new()
        .name("Organism Geolocation")?
        .description("Geolocation of the organism observation.")?
        .procedure_template_geolocated_with_model(phone_builder(user, conn)?)?
        .procedure_template_geolocated_asset_model(organism)?
        .created_by(user.id)?
        .insert(user.id, conn)?;

    observation_procedure.child(
        &arrow_reminder,
        ChildOptions::default().skippable(),
        user,
        conn,
    )?;

    for procedure in [
        &organism_in_ecosystem_picture.procedure_template(conn)?,
        &organism_picture.procedure_template(conn)?,
        &organism_details_picture.procedure_template(conn)?,
        &organism_geolocation.procedure_template(conn)?,
    ] {
        observation_procedure.child(procedure, ChildOptions::default(), user, conn)?;
    }

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

    Ok(observation_procedure)
}
