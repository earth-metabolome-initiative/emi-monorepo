use core_structures::{
    GeolocationProcedureModel, PhotographProcedureModel, ProcedureModel, User,
    traits::{AppendProcedureModel, ChildOptions, ParentProcedureModel},
};
use diesel::OptionalExtension;
use web_common_traits::database::{Insertable, InsertableVariant};

use crate::procedure_model_trackables::{organism::organism_builder, phone::phone_builder};

/// Initializes the Organism observation procedure model in the database.
///
/// # Arguments
///
/// * `user` - The user who is creating the procedure model.
/// * `conn` - The database connection to use for the insertion.
///
/// # Panics
///
/// * If the connection fails to insert the procedure model.
/// * If the procedure model building fails.
pub(crate) fn init_organism_observation_procedure(
    user: &User,
    conn: &mut diesel::PgConnection,
) -> anyhow::Result<ProcedureModel> {
    let name = "Organism observation procedure";

    if let Some(existing) = ProcedureModel::from_name(name, conn).optional()? {
        return Ok(existing);
    }

    let observation_procedure = ProcedureModel::new()
        .name(name)?
        .description(
			"Procedure for observing an organism, and relevant details for identification and study.",
        )?
        .created_by(user.id)?
        .insert(user.id, conn)?;

    // Place the colored cardboard arrow in the field pointing towards the organism
    let arrow_reminder = ProcedureModel::new()
        .name("Place Arrow")?
        .description(
			"Place a colored cardboard arrow in the field pointing towards the organism to facilitate its identification later.",
        )?
        .created_by(user.id)?
        .insert(user.id, conn)?;

    // Take a picture of organism and surrounding ecosystem
    let organism_in_ecosystem_picture = PhotographProcedureModel::new()
        .name("Organism in Ecosystem Picture")?
        .description("Photograph of the organism in its surrounding ecosystem.")?
        .procedure_photographed_with(phone_builder(user, conn)?)?
        .trackable(organism_builder(user, conn)?)?
        .created_by(user.id)?
        .insert(user.id, conn)?;

    // Take a picture of the full organism
    let organism_picture = PhotographProcedureModel::new()
        .name("Organism Picture")?
        .description("Photograph of the full organism for identification.")?
        .procedure_photographed_with(phone_builder(user, conn)?)?
        .trackable(organism_builder(user, conn)?)?
        .created_by(user.id)?
        .insert(user.id, conn)?;

    // Take one of more picture of details of the organism to facilitate
    // identification (e.g. flowers)
    let organism_details_picture = PhotographProcedureModel::new()
        .name("Organism Details Picture")?
        .description("Photograph of details of the organism to facilitate identification.")?
        .procedure_photographed_with(phone_builder(user, conn)?)?
        .trackable(organism_builder(user, conn)?)?
        .created_by(user.id)?
        .insert(user.id, conn)?;

    // Logging geolocation
    let organism_geolocation = GeolocationProcedureModel::new()
        .name("Organism Geolocation")?
        .description("Geolocation of the organism observation.")?
        .procedure_geolocated_with(phone_builder(user, conn)?)?
        .trackable(organism_builder(user, conn)?)?
        .created_by(user.id)?
        .insert(user.id, conn)?;

    observation_procedure.child(
        &arrow_reminder,
        ChildOptions::default().skippable(),
        user,
        conn,
    )?;

    for procedure in [
        &organism_in_ecosystem_picture.procedure_model(conn)?,
        &organism_picture.procedure_model(conn)?,
        &organism_details_picture.procedure_model(conn)?,
        &organism_geolocation.procedure_model(conn)?,
    ] {
        observation_procedure.child(
            procedure,
            ChildOptions::default().inherit_trackables(),
            user,
            conn,
        )?;
    }

    observation_procedure.extend(
        &[
            &arrow_reminder,
            &organism_in_ecosystem_picture.procedure_model(conn)?,
            &organism_picture.procedure_model(conn)?,
            &organism_details_picture.procedure_model(conn)?,
            &organism_geolocation.procedure_model(conn)?,
        ],
        user,
        conn,
    )?;

    Ok(observation_procedure)
}
