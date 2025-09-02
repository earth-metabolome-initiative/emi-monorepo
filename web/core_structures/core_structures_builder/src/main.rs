//! Build the core structures.
use std::path::Path;

use common_traits::builder::Builder;
use diesel::PgConnection;
use init_db::init_database;
use procedure_codegen::ProcedureCodegen;
use reference_docker::reference_docker_with_connection;
use time_requirements::prelude::*;
use webcodegen::{Codegen, PgExtension, PgStatStatement, Table, errors::WebCodeGenError};

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

    let mut codegen = Codegen::default()
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
        .beautify();
    codegen.print_same_as_network(conn, DATABASE_NAME, "columns_same_as_network.dot")?;
    time_tracker.extend(codegen.generate(conn, DATABASE_NAME)?);

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
    // Get the output directory
    let (docker, mut conn) = reference_docker_with_connection(DATABASE_NAME, DATABASE_PORT)
        .await
        .expect("Failed to connect to the database");

    // We ensure that the migrations directory has all expected properties.
    let mut time_tracker = TimeTracker::new("Building Core Structures");

    // We initialize the database into the docker container
    match init_database(DATABASE_NAME, &mut conn).await {
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
            eprintln!("Failed to build core structures: {err:?}");
            return;
        }
    }

    let expensive_statements =
        PgStatStatement::most_expensive_queries(&mut conn).unwrap_or_default();

    // We only keep the top 100 most expensive statements
    let expensive_statements = expensive_statements.into_iter().take(100).collect::<Vec<_>>();

    // We serialize the expensive statements to JSON and save them to a file
    let json = serde_json::to_string_pretty(&expensive_statements).unwrap_or_default();
    std::fs::write("most_expensive_queries.json", json)
        .expect("Failed to write most_expensive_queries.json");

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
