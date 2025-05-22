//! Test to check whether the database can indeed be initialized in the
//! reference docker and populated with the init_migration.

use core_structures::LoginProvider;
use init_db::init_database;
use init_migration::init_migration;
use reference_docker::reference_docker_with_connection;
use web_common_traits::database::AsyncBoundedRead;

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
    if let Err(err) = init_migration(&mut conn).await {
        docker.stop().await.expect("Failed to stop the docker container");
        panic!("Failed to initialize the database: {err:?}");
    }

    match LoginProvider::read_all_async(&mut conn).await {
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

    docker.stop().await.expect("Failed to stop the docker container");
}
