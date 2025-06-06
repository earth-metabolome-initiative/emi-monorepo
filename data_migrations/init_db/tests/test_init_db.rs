//! Test to check whether the database can indeed be initialized in the
//! reference docker.

use init_db::init_database;
use reference_docker::reference_docker_with_connection;
use webcodegen::PgExtension;

const DATABASE_NAME: &str = "test_init_db.db";
const DATABASE_PORT: u16 = 12032;

#[tokio::test]
async fn test_init_db() {
    // Get the output directory
    let (docker, mut conn) = reference_docker_with_connection(DATABASE_NAME, DATABASE_PORT)
        .await
        .expect("Failed to connect to the database");

    // We initialize the database into the docker container
    if let Err(err) = init_database(DATABASE_NAME, &mut conn).await {
        docker.stop().await.expect("Failed to stop the docker container");
        panic!("Failed to initialize the database: {err:?}");
    }

    for extension in ["pgrx_validation", "iso_codes", "tool_categories", "instrument_categories"] {
        if PgExtension::load(extension, "public", &mut conn).unwrap().is_none() {
            docker.stop().await.expect("Failed to stop the docker container");
            panic!(
                "Failed to load extension {extension} in the database. \
                Please check if the extension is installed in the database."
            );
        }
    }

    docker.stop().await.expect("Failed to stop the docker container");
}
