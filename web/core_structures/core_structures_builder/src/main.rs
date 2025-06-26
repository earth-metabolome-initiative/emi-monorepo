//! Build the core structures.
use std::path::Path;

use diesel::PgConnection;
use init_db::init_database;
use reference_docker::reference_docker_with_connection;
use time_requirements::prelude::*;
use webcodegen::{Codegen, ColumnSameAsNetwork, PgExtension, Table, errors::WebCodeGenError};

pub(crate) const DATABASE_NAME: &str = "development.db";
pub(crate) const DATABASE_PORT: u16 = 17032;

fn build_core_structures(conn: &mut PgConnection) -> Result<TimeTracker, WebCodeGenError> {
    // Generate the code associated with the database
    let out_dir = Path::new("../src");
    let users = Table::load(conn, "users", None, DATABASE_NAME)?;
    let projects = Table::load(conn, "projects", None, DATABASE_NAME)?;
    let teams = Table::load(conn, "teams", None, DATABASE_NAME)?;
    let team_members = Table::load(conn, "team_members", None, DATABASE_NAME)?;
    let team_projects = Table::load(conn, "team_projects", None, DATABASE_NAME)?;
    let Some(pgrx_validation) = PgExtension::load("pgrx_validation", "public", conn)? else {
        return Err(WebCodeGenError::MissingExtension("pgrx_validation".to_owned()));
    };

    // First, we delete the old `/src/codegen` directory
    let out_codegen_dir = out_dir.join("codegen");
    if out_codegen_dir.exists() {
        std::fs::remove_dir_all(out_codegen_dir)
            .expect("Failed to remove the old codegen directory");
    }

    Codegen::default()
        .users(&users)
        .projects(&projects)
        .teams(&teams)
        .team_members(&team_members)
        .team_projects(&team_projects)
        .add_check_constraint_extension(&pgrx_validation)
        .set_output_directory(out_dir.as_ref())
        .enable_deletable_trait()
        .enable_insertable_trait()
        .enable_foreign_trait()
        .enable_updatable_trait()
        .enable_upsertable_trait()
        .enable_crud_operations()
        .enable_yew()
        .beautify()
        .generate(conn, DATABASE_NAME, None)
}

#[tokio::main]
#[allow(clippy::too_many_lines)]
/// Main function to build the core structures.
pub async fn main() {
    // Get the output directory
    let (docker, mut conn) = reference_docker_with_connection(DATABASE_NAME, DATABASE_PORT)
        .await
        .expect("Failed to connect to the database");

    // We ensure that the migrations directory has all expected properties.
    let mut time_tracker = TimeTracker::new("Building Core Structures");

    // We initialize the database into the docker container
    if let Err(err) = init_database(DATABASE_NAME, &mut conn).await {
        docker.stop().await.expect("Failed to stop the docker container");
        eprintln!("Failed to initialize the database: {err:?}");
        return;
    }

    // We save the time tracker
    time_tracker.save(Path::new("./time_tracker")).unwrap();

    let same_as_graph = ColumnSameAsNetwork::new(&mut conn, DATABASE_NAME, None)
        .expect("Failed to build the column same-as network");

    let same_as_dot = same_as_graph
        .to_dot(&mut conn)
        .expect("Failed to convert the column same-as network to dot format");
    let same_as_path = Path::new("column_same_as_network.dot");
    std::fs::write(same_as_path, same_as_dot)
        .expect("Failed to write the column same-as network dot file");

    match build_core_structures(&mut conn) {
        Ok(tracker) => {
            time_tracker.extend(tracker);
        }
        Err(err) => {
            docker.stop().await.expect("Failed to stop the docker container");
            eprintln!("Failed to build core structures: {err:?}");
            return;
        }
    }

    // We stop the docker container
    if let Err(err) = docker.stop().await {
        eprintln!("Failed to stop the docker container: {err:?}");
    }

    // We run the `cargo fmt --all` command to format the generated code
    if let Err(err) = std::process::Command::new("cargo").arg("fmt").arg("--all").output() {
        eprintln!("Failed to run cargo fmt: {err:?}");
    }

    // We print the report
    let mut report = Report::new(time_tracker);
    report.add_directory(Path::new("./time_tracker")).unwrap();
    report
        .write(Path::new("time_requirements_report.md"), Path::new("time_requirements_report.png"))
        .unwrap();
}
