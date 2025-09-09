//! Test to check whether the database can indeed be initialized in the
//! reference docker and populated with the `init_migration`.

use core_structures::LoginProvider;
use core_structures_vis::MermaidDB;
use init_db::init_database;
use init_migration::{init_dbgi_plan, init_migration, init_root_user};
use reference_docker::reference_docker_with_connection;
use web_common_traits::database::BoundedRead;
use directus_migration::directus_migration;

const DATABASE_NAME: &str = "test_directus_migration.db";
const DATABASE_PORT: u16 = 12132;

#[tokio::test]
async fn test_directus_migration() {
    dotenvy::from_path("../../web/.env_develop").unwrap();

    // Get the output directory
    let (docker, mut conn) = reference_docker_with_connection(DATABASE_NAME, DATABASE_PORT)
        .await
        .expect("Failed to connect to the database");

    // We initialize the database into the docker container
    if let Err(err) = init_database(DATABASE_NAME, true, &mut conn).await {
        docker.stop().await.expect("Failed to stop the docker container");
        panic!("Failed to initialize the database: {err}");
    }

    // We try to populate the DB with the init initialization
    if let Err(err) = directus_migration(&mut conn) {
        docker.stop().await.expect("Failed to stop the docker container");
        panic!("Failed to execute the Directus migration: {err}");
    }

    docker.stop().await.expect("Failed to stop the docker container");
}
