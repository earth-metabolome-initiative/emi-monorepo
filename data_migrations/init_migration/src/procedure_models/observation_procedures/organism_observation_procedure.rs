use core_structures::{
    CameraModel, GeolocationProcedureModel, PhotographProcedureModel, PositioningDeviceModel,
    ProcedureModel, ProcedureModelTrackable, Trackable, User,
    traits::{AppendProcedureModel, ChildOptions, ParentProcedureModel},
};
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
) -> ProcedureModel {
    if let Some(existing) = ProcedureModel::from_name(ORGANISM_OBSERVATION, conn).unwrap() {
        return existing;
    }

    let camera = CameraModel::from_name(CAMERA, conn).unwrap().unwrap();
    let positioning_device =
        PositioningDeviceModel::from_name(GEOLOCATION_INSTRUMENT, conn).unwrap().unwrap();
    let organism = Trackable::from_name(ORGANISM, conn).unwrap().unwrap();

    let camera_builder = ProcedureModelTrackable::new()
        .name(CAMERA)
        .unwrap()
        .created_by(user.id)
        .unwrap()
        .trackable(camera.id)
        .unwrap();

    let positioning_device_builder = ProcedureModelTrackable::new()
        .name(GEOLOCATION_INSTRUMENT)
        .unwrap()
        .created_by(user.id)
        .unwrap()
        .trackable(positioning_device.id)
        .unwrap();

    let organism_builder = ProcedureModelTrackable::new()
        .name(ORGANISM)
        .unwrap()
        .created_by(user.id)
        .unwrap()
        .trackable(organism.id)
        .unwrap();

    let observation_procedure = ProcedureModel::new()
        .name(ORGANISM_OBSERVATION)
        .unwrap()
        .description(
			"Procedure for observing an organism, and relevant details for identification and study.",
        )
        .unwrap()
        .created_by(user.id)
        .unwrap()
        .insert(user.id, conn)
		.unwrap();

    // Place the colored cardboard arrow in the field pointing towards the organism
    let arrow_reminder = ProcedureModel::new()
        .name("Place Arrow")
        .unwrap()
        .description(
			"Place a colored cardboard arrow in the field pointing towards the organism to facilitate its identification later.",
        )
        .unwrap()
        .created_by(user.id)
        .unwrap()
        .insert(user.id, conn)
		.unwrap();

    // Take a picture of organism and surrounding ecosystem
    let organism_in_ecosystem_picture = PhotographProcedureModel::new()
        .name("Organism in Ecosystem Picture")
        .unwrap()
        .description("Photograph of the organism in its surrounding ecosystem.")
        .unwrap()
        .procedure_photographed_with(camera_builder.clone())
        .unwrap()
        .trackable(organism_builder.clone())
        .unwrap()
        .created_by(user.id)
        .unwrap()
        .insert(user.id, conn)
        .unwrap();

    // Take a picture of the full organism
    let organism_picture = PhotographProcedureModel::new()
        .name("Organism Picture")
        .unwrap()
        .description("Photograph of the full organism for identification.")
        .unwrap()
        .procedure_photographed_with(camera_builder.clone())
        .unwrap()
        .trackable(organism_builder.clone())
        .unwrap()
        .created_by(user.id)
        .unwrap()
        .insert(user.id, conn)
        .unwrap();

    // Take one of more picture of details of the organism to facilitate
    // identification (e.g. flowers)
    let organism_details_picture = PhotographProcedureModel::new()
        .name("Organism Details Picture")
        .unwrap()
        .description("Photograph of details of the organism to facilitate identification.")
        .unwrap()
        .procedure_photographed_with(camera_builder.clone())
        .unwrap()
        .trackable(organism_builder.clone())
        .unwrap()
        .created_by(user.id)
        .unwrap()
        .insert(user.id, conn)
        .unwrap();

    // Logging geolocation
    let organism_geolocation = GeolocationProcedureModel::new()
        .name("Organism Geolocation")
        .unwrap()
        .description("Geolocation of the organism observation.")
        .unwrap()
        .procedure_geolocated_with(positioning_device_builder.clone())
        .unwrap()
        .trackable(organism_builder.clone())
        .unwrap()
        .created_by(user.id)
        .unwrap()
        .insert(user.id, conn)
        .unwrap();

    observation_procedure
        .child(&arrow_reminder, ChildOptions::default().skippable(), user, conn)
        .unwrap();

    for procedure in [
        &organism_in_ecosystem_picture.procedure_model(conn).unwrap(),
        &organism_picture.procedure_model(conn).unwrap(),
        &organism_details_picture.procedure_model(conn).unwrap(),
        &organism_geolocation.procedure_model(conn).unwrap(),
    ] {
        observation_procedure
            .child(procedure, ChildOptions::default().inherit_trackables(), user, conn)
            .unwrap();
    }

    observation_procedure
        .extend(
            &[
                &arrow_reminder,
                &organism_in_ecosystem_picture.procedure_model(conn).unwrap(),
                &organism_picture.procedure_model(conn).unwrap(),
                &organism_details_picture.procedure_model(conn).unwrap(),
                &organism_geolocation.procedure_model(conn).unwrap(),
            ],
            user,
            conn,
        )
        .unwrap();

    observation_procedure
}
