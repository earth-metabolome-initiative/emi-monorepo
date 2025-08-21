//! Test to check whether the database can indeed be initialized in the
//! reference docker and populated with the `init_migration`.

use core_structures::{LoginProvider, ProcedureModel, traits::ProcedureModelDot};
use core_structures_vis::MermaidDB;
use init_db::init_database;
use init_migration::{DBGI_PLAN, init_migration};
use reference_docker::reference_docker_with_connection;
use web_common_traits::database::BoundedRead;

const DATABASE_NAME: &str = "test_init_migration.db";
const DATABASE_PORT: u16 = 12132;

#[tokio::test]
async fn test_init_migration() {
    dotenvy::from_path("../../web/.env_develop").unwrap();

    // Get the output directory
    let (docker, mut conn) = reference_docker_with_connection(DATABASE_NAME, DATABASE_PORT)
        .await
        .expect("Failed to connect to the database");

    // We initialize the database into the docker container
    if let Err(err) = init_database(DATABASE_NAME, &mut conn).await {
        docker.stop().await.expect("Failed to stop the docker container");
        panic!("Failed to initialize the database: {err:?}");
    }

    // We try to populate the DB with the init initialization
    if let Err(err) = init_migration(&mut conn) {
        docker.stop().await.expect("Failed to stop the docker container");
        panic!("Failed to initialize the database: {err:?}");
    }

    match LoginProvider::bounded_read(0, 16, &mut conn) {
        Ok(login_providers) => {
            if login_providers.len() != 1 {
                docker.stop().await.expect("Failed to stop the docker container");
                panic!("Expected 1 login provider, but found {}", login_providers.len());
            }
        }
        Err(err) => {
            docker.stop().await.expect("Failed to stop the docker container");
            panic!("Failed to read login providers: {err:?}");
        }
    }

    let procedure_model = ProcedureModel::from_name(DBGI_PLAN, &mut conn).unwrap();

    let dot = procedure_model
        .to_dot(&mut conn)
        .expect("Failed to convert the procedure model to DOT format");

    let flowchart = procedure_model
        .to_mermaid(&mut conn)
        .expect("Failed to convert the procedure model to Mermaid format");
    let er = core_structures_vis::trackables_hierarchy(&mut conn)
        .expect("Failed to generate the trackables hierarchy ERD");

    // We write out the DOT file to a file.
    std::fs::write("test_init_migration.dot", dot).expect("Failed to write the DOT file");

    // We write out the flowchart to a file.
    std::fs::write("test_init_migration_flowchart.mermaid", flowchart.to_string())
        .expect("Failed to write the flowchart file");

    // We write out the ERD to a file.
    std::fs::write("test_init_migration_erd.mermaid", er.to_string())
        .expect("Failed to write the ERD file");

    docker.stop().await.expect("Failed to stop the docker container");
}
