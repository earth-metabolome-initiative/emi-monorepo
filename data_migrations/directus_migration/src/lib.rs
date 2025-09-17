//! This module contains the main logic for migrating data from a Directus
//! database to the portal database.

mod directus_templates;
mod errors;
mod impls;
mod migrations;
mod sample_source_kind;
mod structs;

use core_structures::{
    Photograph, PhotographProcedure, ProcedureAsset, Sample, User,
    tables::insertables::{
        AssetSettable, PhotographProcedureSettable, ProcedureAssetSettable, SampleSettable,
    },
};
use diesel::{Connection, PgConnection};
use guided_procedures::{GuidedProcedure, ProcedureTemplateGraph};
use init_migration::asset_models::{
    instruments::phone::phone_model, organisms::organism_model, photographs::photograph_model,
};
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
pub fn directus_connection() -> Result<PgConnection, anyhow::Error> {
    let conn = PgConnection::establish(DIRECTUS_DATABASE_URL)?;
    Ok(conn)
}

/// Executes the data migration from the Directus database to the portal
/// database.
pub fn directus_migration(
    user: &User,
    directus_conn: &mut PgConnection,
    portal_conn: &mut PgConnection,
) -> anyhow::Result<()> {
    let procedure_template = init_migration::init_dbgi_plan(user, portal_conn)
        .expect("Failed to initialize the DBGI plan");
    let procedure_graph = ProcedureTemplateGraph::new(&procedure_template, portal_conn)?;

    let photograph_model = photograph_model(user, portal_conn)?;
    let organism_model = organism_model(user, portal_conn)?;
    let phone_model = phone_model(user, portal_conn)?;

    let guided_procedure = GuidedProcedure::new()
        .user_id(user.id)
        .graph(&procedure_graph)
        .connection(portal_conn)
        .build()?;

    guided_procedure
        .and_then::<PhotographProcedure, anyhow::Error>(|mut builder, conn| {
            let photograph = Photograph::new()
                .model(&photograph_model)?
                .created_by(user)?
                .insert(user.id, conn)?;
            builder = builder
                .procedure_photographed_with(ProcedureAsset::new().asset_model(phone_model)?)?
                .procedure_photographed_asset(ProcedureAsset::new().asset_model(organism_model)?)?
                .procedure_photograph(ProcedureAsset::new().asset(photograph)?)?;
            Ok(builder)
        })?
        .and_then::<PhotographProcedure, anyhow::Error>(|mut builder, conn| {
            let photograph = Photograph::new()
                .model(&photograph_model)?
                .created_by(user)?
                .insert(user.id, conn)?;
            builder = builder.procedure_photograph(ProcedureAsset::new().asset(photograph)?)?;
            Ok(builder)
        })?
        .finish()?;

    for field_data_row in directus_codegen::FieldDatum::bounded_read(0, 1000, directus_conn)? {
        let field_data_row: FieldDatumWrapper = field_data_row.into();
        if field_data_row.should_skip() {
            continue;
        }
        let user = field_data_row.author(portal_conn)?;
        let mut sample_builder = Sample::new()
            .name(field_data_row.sample_id()?)?
            .sample_source(field_data_row.sample_source(&user, portal_conn)?)?
            .created_by(&user)?;

        sample_builder = SampleSettable::model(
            sample_builder,
            field_data_row.sample_model(&user, portal_conn)?,
        )?;

        let sample: Sample = sample_builder.insert(user.id, portal_conn).unwrap();
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
