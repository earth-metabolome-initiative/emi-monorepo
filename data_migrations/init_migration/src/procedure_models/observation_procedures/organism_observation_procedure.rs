use core_structures::{
    CameraModel, GeolocationProcedureModel, PhotographProcedureModel, PositioningDeviceModel,
    ProcedureModel, ProcedureModelTrackable, Trackable, User,
    traits::{AppendProcedureModel, ChildOptions, ParentProcedureModel},
};
use diesel::OptionalExtension;
use web_common_traits::database::{Insertable, InsertableVariant};

use crate::trackables::{
    instruments::{CAMERA, GEOLOCATION_INSTRUMENT},
    organisms::ORGANISM,
};

/// The name of the Organism observation procedure model.
const ORGANISM_OBSERVATION: &str = "Organism observation procedure";

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
    if let Some(existing) = ProcedureModel::from_name(ORGANISM_OBSERVATION, conn).optional()? {
        return Ok(existing);
    }

    let camera = CameraModel::from_name(CAMERA, conn)?;
    let positioning_device = PositioningDeviceModel::from_name(GEOLOCATION_INSTRUMENT, conn)?;
    let organism = Trackable::from_name(ORGANISM, conn)?;

    let camera_builder =
        ProcedureModelTrackable::new().name(CAMERA)?.created_by(user.id)?.trackable(camera.id)?;

    let positioning_device_builder = ProcedureModelTrackable::new()
        .name(GEOLOCATION_INSTRUMENT)?
        .created_by(user.id)?
        .trackable(positioning_device.id)?;

    let organism_builder = ProcedureModelTrackable::new()
        .name(ORGANISM)?
        .created_by(user.id)?
        .trackable(organism.id)?;

    let observation_procedure = ProcedureModel::new()
        .name(ORGANISM_OBSERVATION)
        ?
        .description(
			"Procedure for observing an organism, and relevant details for identification and study.",
        )
        ?
        .created_by(user.id)
        ?
        .insert(user.id, conn)
		?;

    // Place the colored cardboard arrow in the field pointing towards the organism
    let arrow_reminder = ProcedureModel::new()
        .name("Place Arrow")
        ?
        .description(
			"Place a colored cardboard arrow in the field pointing towards the organism to facilitate its identification later.",
        )
        ?
        .created_by(user.id)
        ?
        .insert(user.id, conn)
		?;

    // Take a picture of organism and surrounding ecosystem
    let organism_in_ecosystem_picture = PhotographProcedureModel::new()
        .name("Organism in Ecosystem Picture")?
        .description("Photograph of the organism in its surrounding ecosystem.")?
        .procedure_photographed_with(camera_builder.clone())?
        .trackable(organism_builder.clone())?
        .created_by(user.id)?
        .insert(user.id, conn)?;

    // Take a picture of the full organism
    let organism_picture = PhotographProcedureModel::new()
        .name("Organism Picture")?
        .description("Photograph of the full organism for identification.")?
        .procedure_photographed_with(camera_builder.clone())?
        .trackable(organism_builder.clone())?
        .created_by(user.id)?
        .insert(user.id, conn)?;

    // Take one of more picture of details of the organism to facilitate
    // identification (e.g. flowers)
    let organism_details_picture = PhotographProcedureModel::new()
        .name("Organism Details Picture")?
        .description("Photograph of details of the organism to facilitate identification.")?
        .procedure_photographed_with(camera_builder.clone())?
        .trackable(organism_builder.clone())?
        .created_by(user.id)?
        .insert(user.id, conn)?;

    // Logging geolocation
    let organism_geolocation = GeolocationProcedureModel::new()
        .name("Organism Geolocation")?
        .description("Geolocation of the organism observation.")?
        .procedure_geolocated_with(positioning_device_builder.clone())?
        .trackable(organism_builder.clone())?
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
