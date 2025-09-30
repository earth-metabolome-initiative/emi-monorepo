//! This module contains the main logic for migrating data from a Directus
//! database to the portal database.

mod directus_template;
mod errors;
mod impls;
mod migrations;
mod sample_source_kind;
mod structs;

use chrono::format;
use core_structures::{
    Photograph, PhotographProcedure, ProcedureAsset, Sample, User,
    tables::insertables::{
        AssetSettable, GeolocationProcedureSettable, PhotographProcedureSettable,
        ProcedureAssetSettable, SampleSettable,
    },
};
use diesel::{Connection, PgConnection};
use guided_procedures::{GuidedProcedure, ProcedureTemplateGraph};
use init_migration::asset_models::{
    instruments::phone::phone_model, organisms::organism_model, photographs::photograph_model,
};
use postgis_diesel::types::Point;
use web_common_traits::{
    database::{BoundedRead, DispatchableInsertableVariant},
    prelude::{Builder, Insertable},
};

use crate::structs::FieldDatumWrapper;

const DIRECTUS_DATABASE_NAME: &str = "directus";
const DIRECTUS_DATABASE_PASSWORD: &str = "directus_dbgi";
const DIRECTUS_DATABASE_USER: &str = "directus";
const DIRECTUS_DATABASE_PORT: u16 = 5434;
const DIRECTUS_HOSTNAME: &str = "134.21.20.118";
const DIRECTUS_DATABASE_URL: &str = const_format::formatcp!(
    "postgres://{DIRECTUS_DATABASE_USER}:{DIRECTUS_DATABASE_PASSWORD}@{DIRECTUS_HOSTNAME}:{DIRECTUS_DATABASE_PORT}/{DIRECTUS_DATABASE_NAME}",
);
/// Establishes a connection to the Directus database.
///
/// # Errors
///
/// * Returns an `anyhow::Error` if there is an issue connecting to the
///   database.
pub fn directus_connection() -> Result<PgConnection, anyhow::Error> {
    let conn = PgConnection::establish(DIRECTUS_DATABASE_URL)?;
    Ok(conn)
}

#[allow(clippy::too_many_lines)]
/// Executes the data migration from the Directus database to the portal
/// database.
///
/// # Arguments
///
/// * `user` - The user performing the migration.
/// * `directus_conn` - A mutable reference to the Directus database connection.
/// * `portal_conn` - A mutable reference to the portal database connection.
///
/// # Errors
///
/// * Returns an `anyhow::Error` if there is an issue during the migration
///   process.
pub fn directus_migration(
    user: &User,
    directus_conn: &mut PgConnection,
    portal_conn: &mut PgConnection,
) -> anyhow::Result<()> {
    // let pseudocode =
    //     guided_procedures::GuidedProcedurePseudocode::new().graph(&
    // procedure_graph)?.build()?; println!("{}",
    // pseudocode.pseudocode::<anyhow::Error>());

    for field_data_row in directus_codegen::FieldDatum::bounded_read(0, 1000, directus_conn)? {
        let field_data_row: FieldDatumWrapper = field_data_row.into();
        if field_data_row.should_skip() {
            continue;
        }
        let user = field_data_row.author(portal_conn)?;

        field_data_row.procedure_dispatch(&user, portal_conn)?;
    }
    // insert_missing_users(directus_conn, portal_conn)?;
    // insert_missing_brands(directus_conn, portal_conn)?;
    // insert_missing_instrument_models(directus_conn, portal_conn)?;
    // insert_directus_collection_procedures(directus_conn, portal_conn)?;
    Ok(())
}

// #[tokio::main]
// async fn main() -> Result<(), anyhow::Error> {
//     let mut directus_conn = PgConnection::establish(DIRECTUS_DATABASE_URL)?;
//     let mut portal_conn = PgConnection::establish(PORTAL_DATABASE_URL)?;

//     portal_conn.transaction(|portal_conn| transact_migration(&mut
// directus_conn, portal_conn))?;

//     Ok(())
// }
