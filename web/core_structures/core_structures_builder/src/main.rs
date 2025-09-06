//! Build the core structures.
use std::path::Path;

use common_traits::builder::Builder;
use diesel::PgConnection;
use init_db::init_database;
use procedure_codegen::ProcedureCodegen;
use reference_docker::reference_docker_with_connection;
use time_requirements::prelude::*;
use webcodegen::{Codegen, PgExtension, Table, errors::WebCodeGenError};

pub(crate) const DATABASE_NAME: &str = "development.db";
pub(crate) const DATABASE_PORT: u16 = 17032;

fn build_core_structures(conn: &mut PgConnection) -> Result<TimeTracker, anyhow::Error> {
    // Generate the code associated with the database
    let out_dir = Path::new("../src");
    let users = Table::load(conn, "users", "public", DATABASE_NAME)?;
    let projects = Table::load(conn, "projects", "public", DATABASE_NAME)?;
    let teams = Table::load(conn, "teams", "public", DATABASE_NAME)?;
    let team_members = Table::load(conn, "team_members", "public", DATABASE_NAME)?;
    let team_projects = Table::load(conn, "team_projects", "public", DATABASE_NAME)?;
    let Some(pgrx_validation) = PgExtension::load("pgrx_validation", "public", conn)? else {
        return Err(WebCodeGenError::MissingExtension("pgrx_validation".to_owned()).into());
    };

    // First, we delete the old `/src/codegen` directory
    let out_codegen_dir = out_dir.join("codegen");
    if out_codegen_dir.exists() {
        std::fs::remove_dir_all(out_codegen_dir)
            .expect("Failed to remove the old codegen directory");
    }

    let mut time_tracker = TimeTracker::new("Code Generation");

    time_tracker.extend(
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
            .generate(conn, DATABASE_NAME)?,
    );

    let procedures_tracker = ProcedureCodegen::builder()
        .output_directory(out_dir.as_ref())
        .generate_procedure_impls()
        .generate_procedure_template_impls()
        .beautify()
        .build()?
        .generate(conn, DATABASE_NAME)?;

    time_tracker.extend(procedures_tracker);

    Ok(time_tracker)
}

#[tokio::main]
#[allow(clippy::too_many_lines)]
/// Main function to build the core structures.
pub async fn main() {
    // We ensure that the migrations directory has all expected properties.
    let mut time_tracker = TimeTracker::new("Building Core Structures");

    let task = Task::new("Setting up Docker and Database Connection");
    // Get the output directory
    let (docker, mut conn) = reference_docker_with_connection(DATABASE_NAME, DATABASE_PORT)
        .await
        .expect("Failed to connect to the database");
    time_tracker.add_completed_task(task);

    // We initialize the database into the docker container
    match init_database(DATABASE_NAME, true, &mut conn).await {
        Ok(tracker) => time_tracker.extend(tracker),
        Err(err) => {
            docker.stop().await.expect("Failed to stop the docker container");
            eprintln!("Failed to initialize the database: {err}");
            return;
        }
    };

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
    Report::new(time_tracker)
        .write(Path::new("time_requirements_report.md"), Path::new("time_requirements_report.png"))
        .unwrap();
}
