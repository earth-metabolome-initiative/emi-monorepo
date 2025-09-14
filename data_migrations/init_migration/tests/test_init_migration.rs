//! Test to check whether the database can indeed be initialized in the
//! reference docker and populated with the `init_migration`.

use core_structures_vis::MermaidDB;
use init_db::init_database;
use init_migration::{init_dbgi_plan, init_migration, init_root_user};
use reference_docker::reference_docker_with_connection;
use time_requirements::prelude::{Task, TimeTracker};

const DATABASE_NAME: &str = "test_init_migration.db";
const DATABASE_PORT: u16 = 12132;

#[tokio::test]
async fn test_init_migration() {
    let mut test_time_tracker = TimeTracker::new("Test Init Migration");
    let task = Task::new("Load environment variables");
    dotenvy::from_path("../../web/.env_develop").unwrap();
    test_time_tracker.add_completed_task(task);

    // Get the output directory
    let task = Task::new("Initialize the reference docker");
    let (docker, mut conn) = reference_docker_with_connection(DATABASE_NAME, DATABASE_PORT)
        .await
        .expect("Failed to connect to the database");
    test_time_tracker.add_completed_task(task);

    // We initialize the database into the docker container
    let task = Task::new("Initialize the database");
    if let Err(err) = init_database(DATABASE_NAME, true, &mut conn).await {
        docker.stop().await.expect("Failed to stop the docker container");
        panic!("Failed to initialize the database: {err}");
    }
    test_time_tracker.add_completed_task(task);

    // We try to populate the DB with the init initialization
    let task = Task::new("Initialize the migration");
    if let Err(err) = init_migration(&mut conn) {
        docker.stop().await.expect("Failed to stop the docker container");
        panic!("Failed to initialize the database: {err}");
    }
    test_time_tracker.add_completed_task(task);

    let user = init_root_user(&mut conn).expect("Failed to initialize the root user");
    let procedure_template =
        init_dbgi_plan(&user, &mut conn).expect("Failed to initialize the DBGI plan");

    let flowchart = procedure_template
        .to_mermaid(&mut conn)
        .expect("Failed to convert the procedure template to Mermaid format");
    let er = core_structures_vis::asset_model_hierarchy(&mut conn)
        .expect("Failed to generate the asset model hierarchy ERD");

    // We write out the flowchart to a file.
    std::fs::write("test_init_migration_flowchart.mermaid", flowchart.to_string())
        .expect("Failed to write the flowchart file");

    // We write out the ERD to a file.
    std::fs::write("test_init_migration_erd.mermaid", er.to_string())
        .expect("Failed to write the ERD file");

    docker.stop().await.expect("Failed to stop the docker container");
}
