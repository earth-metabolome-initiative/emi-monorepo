//! Test to check whether the database can indeed be initialized in the
//! reference docker.

use std::path::Path;

use init_db::init_database;
use reference_docker::reference_docker_with_connection;
use time_requirements::{
    prelude::{Report, TimeTracker},
    task::Task,
};
use webcodegen::{PgExtension, PgStatStatement};

const DATABASE_NAME: &str = "test_init_db.db";
const DATABASE_PORT: u16 = 12032;

#[tokio::test]
async fn test_init_db() {
    let mut test_time_tracker = TimeTracker::new("Test Init DB");
    let task = Task::new("Booting up Docker");
    // Get the output directory
    let (docker, mut conn) = reference_docker_with_connection(DATABASE_NAME, DATABASE_PORT)
        .await
        .expect("Failed to connect to the database");
    test_time_tracker.add_completed_task(task);

    // We initialize the database into the docker container
    match init_database(DATABASE_NAME, &mut conn).await {
        Ok(time_tracker) => {
            test_time_tracker.extend(time_tracker);
            Report::new(test_time_tracker)
                .write(
                    Path::new("time_requirements_report.md"),
                    Path::new("time_requirements_report.png"),
                )
                .unwrap();
        }
        Err(err) => {
            let expensive_statements =
                PgStatStatement::most_expensive_queries(&mut conn).unwrap_or_default();

            // We only keep the top 100 most expensive statements
            let expensive_statements =
                expensive_statements.into_iter().take(100).collect::<Vec<_>>();

            // We serialize the expensive statements to JSON and save them to a file
            let json = serde_json::to_string_pretty(&expensive_statements).unwrap_or_default();
            std::fs::write("most_expensive_queries.json", json)
                .expect("Failed to write most_expensive_queries.json");

            docker.stop().await.expect("Failed to stop the docker container");
            panic!("Failed to initialize the database: {err}");
        }
    }

    for extension in ["pgrx_validation", "iso_codes"] {
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
