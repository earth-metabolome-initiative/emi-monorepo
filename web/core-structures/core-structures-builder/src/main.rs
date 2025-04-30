//! Build the core structures.
use std::path::Path;

use diesel::{
    Connection, connection::SimpleConnection, pg::PgConnection, result::DatabaseErrorKind,
};
use diesel_migrations_utils::prelude::*;
use taxonomy_fetcher::{
    Rank, Taxonomy, TaxonomyBuilder,
    impls::ncbi::{NCBIRank, NCBITaxonomy, NCBITaxonomyBuilder},
};
use time_requirements::prelude::*;
use webcodegen::{Codegen, PgExtension, Table};

mod consistency_constraints;
use consistency_constraints::execute_consistency_constraint_checks;
mod constants;
use constants::{DATABASE_NAME, DATABASE_URL};
use csqlv_macro::load_populating_sql_from_csvs;

const CSV_SQL: &str = load_populating_sql_from_csvs!(("../csvs", "/app/web/core-structures/csvs"));

#[tokio::main]
#[allow(clippy::too_many_lines)]
/// Main function to build the core structures.
pub async fn main() {
    // Get the output directory
    let out_dir = Path::new("../src");

    // We ensure that the migrations directory has all expected properties.
    let mut time_tracker = TimeTracker::new("Building Core Structures");
    let task = Task::new("Checking Migrations Directory");

    // We delete empty directories in the `migrations` directory which may occur
    // when some git collision occurs.
    for entry in std::fs::read_dir("../migrations").unwrap() {
        let entry = entry.unwrap();
        if entry.file_type().unwrap().is_dir() {
            let path = entry.path();
            if path.read_dir().unwrap().count() == 0 {
                std::fs::remove_dir_all(path).unwrap();
            }
        }
    }

    let mut extension_migrations =
        MigrationDirectory::try_from("../extensions_migrations").unwrap();
    if !extension_migrations.is_dense() {
        extension_migrations = extension_migrations.redensify().unwrap();
    }
    let mut migrations = MigrationDirectory::try_from("../migrations").unwrap();
    if !migrations.is_topologically_sorted().unwrap() {
        migrations = migrations.topologically_sort().unwrap();
    }
    assert!(
        migrations.is_topologically_sorted().unwrap(),
        "The migrations are not topologically sorted.",
    );

    if !migrations.is_dense() {
        migrations = migrations.redensify().unwrap();
    }
    time_tracker.add_completed_task(task);

    // Next, we create the CSV for the taxonomical ranks
    let task = Task::new("Creating Taxonomical Ranks CSV");
    NCBIRank::to_csv("../csvs/ranks.csv").unwrap();
    time_tracker.add_completed_task(task);

    // We retrieve and build the latest version of the NCBI taxonomy
    if !Path::new("../csvs/taxa.csv").exists() {
        let task = Task::new("Fetching NCBI Taxonomy");
        let taxonomy: NCBITaxonomy = NCBITaxonomyBuilder::latest().build().await.unwrap();
        time_tracker.add_completed_task(task);
        let task = Task::new("Creating Taxonomy CSV");
        taxonomy.to_csv("../csvs/taxa.csv").unwrap();
        time_tracker.add_completed_task(task);
    }

    // Next, we build the SQL associated with the CSVs present in the 'csvs'
    // directory
    let task = Task::new("Building Schema from CSVs");
    let mut connection = PgConnection::establish(DATABASE_URL).unwrap();
    connection.batch_execute(CSV_SQL).unwrap();

    time_tracker.add_completed_task(task);

    // We execute the migrations
    let task = Task::new("Executing Migrations");
    match extension_migrations.connect_and_execute_ups::<diesel::PgConnection>(DATABASE_URL) {
        Ok(()) => {}
        Err(MigrationError::ExecutingMigrationFailed(
            _,
            MigrationKind::Up,
            diesel::result::Error::DatabaseError(DatabaseErrorKind::Unknown, error),
        )) => {
            // This error is expected when the database is empty and the
            // migration is not the first one.
            if error.message() == "extension \"pgrx_validation\" is not available" {
                panic!(concat!(
                    "You have forgotten to build the pgrx_validation extension. ",
                    "Please navigate to the `/web/web_common/pgrx_validation` crate and read the ",
                    "README.md file to build the extension. Do remember to copy the ",
                    "extension to the `core-structures` directory as at this time the ",
                    "Docker is not able to load the extension from the `pgrx_validation` ",
                    "directory."
                ));
            } else {
                panic!("{error:?}");
            }
        }
        error => error.unwrap(),
    }
    migrations.connect_and_execute_ups::<diesel::PgConnection>(DATABASE_URL).unwrap();
    time_tracker.add_completed_task(task);

    let mut conn = PgConnection::establish(DATABASE_URL).unwrap();

    // Now that the preliminary database setup is done, we can execute the Meta-SQL
    // which takes care of the roles tables and canx functions, which determine
    // whether a user can insert or update entries in a given table.
    let task = Task::new("Executing Meta-SQL");
    // Table::create_roles_tables(&mut conn, DATABASE_NAME, None).unwrap();
    // AuthorizationFunctionBuilder::default()
    //     .add_childless_table(Table::load(&mut conn, "users", None,
    // DATABASE_NAME).unwrap())
    //     .create_authorization_functions_and_triggers(&mut conn, DATABASE_NAME,
    // None)     .unwrap();
    Table::create_update_triggers(&mut conn, DATABASE_NAME, None).unwrap();
    time_tracker.add_completed_task(task);

    // We check that the database follows the expected constraints.
    let task = Task::new("Checking Constraints");

    execute_consistency_constraint_checks(&mut conn).unwrap();

    time_tracker.add_completed_task(task);

    // We write to the target directory the generated structs

    // Generate the code associated with the database
    let users =
        Table::load(&mut conn, "users", None, DATABASE_NAME).expect("Failed to load `users` table");
    let projects = Table::load(&mut conn, "projects", None, DATABASE_NAME)
        .expect("Failed to load `projects` table");
    let teams =
        Table::load(&mut conn, "teams", None, DATABASE_NAME).expect("Failed to load `teams` table");
    let team_members = Table::load(&mut conn, "team_members", None, DATABASE_NAME)
        .expect("Failed to load `team_members` table");
    let team_projects = Table::load(&mut conn, "team_projects", None, DATABASE_NAME)
        .expect("Failed to load `team_projects` table");
    let pgrx_validation = PgExtension::load("pgrx_validation", "public", &mut conn)
        .expect("Failed to query the database")
        .expect("Failed to load `pgrx_validation` extension, maybe it is not installed");
    time_tracker.extend(
        Codegen::default()
            .users(&users)
            .projects(&projects)
            .teams(&teams)
            .team_members(&team_members)
            .team_projects(&team_projects)
            .add_check_constraint_extension(&pgrx_validation)
            .set_output_directory(out_dir.as_ref())
            .enable_loadable_trait()
            .enable_deletable_trait()
            .enable_insertable_trait()
            .enable_foreign_trait()
            .enable_updatable_trait()
            .enable_upsertable_trait()
            .enable_crud_operations()
            .beautify()
            .generate(&mut conn, DATABASE_NAME, None)
            .unwrap(),
    );

    // We save the time tracker
    time_tracker.save(Path::new("./time_tracker")).unwrap();

    // We print the report
    let mut report = Report::new(time_tracker);
    report.add_directory(Path::new("./time_tracker")).unwrap();
    report
        .write(Path::new("time_requirements_report.md"), Path::new("time_requirements_report.png"))
        .unwrap();
}
