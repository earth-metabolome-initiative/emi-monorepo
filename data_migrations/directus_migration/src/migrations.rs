//! Submodule providing the methods for all of the directus migrations.

mod utils;
use core_structures::{
    ProcedureTemplate, User, tables::insertables::InsertableProcedureTemplateBuilder,
};
use diesel::PgConnection;
// mod insert_missing_brands;
// pub use insert_missing_brands::insert_missing_brands;
// mod insert_missing_users;
// pub use insert_missing_users::insert_missing_users;
// mod insert_missing_instrument_models;
// pub(crate) use insert_missing_instrument_models::insert_missing_instrument_models;
use init_migration::init_dbgi_plan;
pub(crate) use utils::{get_room, get_user};
mod insert_collection_procedures;
pub use insert_collection_procedures::insert_directus_collection_procedures;

use directus_codegen::Container as DirectusContainer;

fn process_sample(
    user: &User,
    sample_container: &DirectusContainer,
    directus_conn: &mut PgConnection,
    portal_conn: &mut PgConnection,
) -> anyhow::Result<()> {
    // let factory = raccolta_funghi_template.factory();
    // let raccolta_funghi_instance_builder = factory.new_builder();

    // if let Some(GeolocateFungo(geolocation_procedure_builder)) =
    // raccolta_funghi_instance_builder.next()
    // {
    // assert_eq!(procedure_builder.name, "First step of the DBGI plan");
    // geolocation_procedure_builder
    // .geolocation(Point(6, 87))
    // .insert_first_trackable(trackable)
    // .insert(portal_conn)?;
    // } else {
    // unreachable!("Expected First step of the DBGI plan");
    // }
    //
    // let Some(GeolocateFungo(geolocation_procedure_builder)) =
    // raccolta_funghi_instance_builder.next()
    // else {
    // unreachable!("Expected First step of the DBGI plan");
    // };
    // assert_eq!(procedure_builder.name, "First step of the DBGI plan");
    // geolocation_procedure_builder
    // .geolocation(Point(6, 87))
    // .insert_first_trackable(trackable)
    // .insert(portal_conn)?;

    // let geolocation_procedure = raccolta_funghi_instance_builder
    //     .try_next_into::<GeolocationProcedureBuilder>()?
    //     .geolocation(Point(6, 87))
    //     .insert_first_trackable(trackable)
    //     .insert(portal_conn)?;

    // if let Some(ExpectedVariant2(procedure_builder)) =
    // raccolta_funghi_instance_builder.next() {     assert_eq!
    // (procedure_builder.name, "Second step of the DBGI plan");
    //     procedure_builder.insert_first_trackable(trackable).insert(portal_conn)?;
    // } else {
    //     unreachable!("Expected Second step of the DBGI plan");
    // }

    // if let Some(ExpectedVariant3(procedure_builder)) =
    // raccolta_funghi_instance_builder.next() {     assert_eq!
    // (procedure_builder.name, "Third step of the DBGI plan");
    //     procedure_builder.insert_first_trackable(trackable).insert(portal_conn)?;
    // } else {
    //     unreachable!("Expected Third step of the DBGI plan");
    // }

    Ok(())
}

pub fn directus_migration(
    user: &User,
    directus_conn: &mut PgConnection,
    portal_conn: &mut PgConnection,
) -> Result<(), anyhow::Error> {
    // let dbgi_pm = init_dbgi_plan(user, portal_conn)?;
    // let procedure_builders: impl Iterator<Item =
    // Enum<InsertableProcedureBuilder>> =     dbgi_pm.
    // subprocedures(portal_conn);

    // // Create solvant

    // for sample_container in DirectusContainer::bounded_read() {
    //     process_sample(user, sample, directus_conn, portal_conn)?;
    // }

    Ok(())
}
