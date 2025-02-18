//! Build the core structures.
use std::{env, path::Path};

use csqlv::{CSVSchema, CSVSchemaBuilder};
use diesel::{connection::SimpleConnection, pg::PgConnection, Connection};
use diesel_migrations_utils::prelude::*;
use taxonomy_fetcher::{
    impls::ncbi::{NCBIRank, NCBITaxonomy, NCBITaxonomyBuilder},
    Rank, Taxonomy, TaxonomyBuilder,
};
use time_requirements::prelude::*;
use webcodegen::{
    AuthorizationFunctionBuilder, Codegen, CompatibleForeignTypeConstraint, CustomColumnConstraint,
    CustomTableConstraint, LowercaseColumnConstraint, LowercaseTableConstraint, Table,
};

const DATABASE_NAME: &str = "development.db";
const DATABASE_PASSWORD: &str = "password";
const DATABASE_USER: &str = "user";
const DATABASE_PORT: u16 = 15032;

fn establish_connection_to_postgres() -> PgConnection {
    let database_url = format!(
        "postgres://{DATABASE_USER}:{DATABASE_PASSWORD}@localhost:{DATABASE_PORT}/{DATABASE_NAME}",
    );

    let mut number_of_attempts = 0;

    while let Err(e) = PgConnection::establish(&database_url) {
        eprintln!("Failed to establish connection: {:?}", e);
        std::thread::sleep(std::time::Duration::from_secs(1));
        if number_of_attempts > 10 {
            eprintln!("Failed to establish connection after 10 attempts");
            std::process::exit(1);
        }
        number_of_attempts += 1;
    }

    PgConnection::establish(&database_url).unwrap()
}

#[tokio::main]
pub async fn main() {
    // Get the output directory
    let out_dir = env::var("OUT_DIR").unwrap();

    // Path to the file to create
    let path = Path::new(&out_dir).join("core_structs.rs");

    // If the file already exists, we skip the build
    if path.exists() {
        return;
    }

    // We ensure that the migrations directory has all expected properties.
    let mut time_tracker = TimeTracker::new("Building Core Structures");
    let task = Task::new("Checking Migrations Directory");
    let mut extension_migrations = MigrationDirectory::try_from("extensions_migrations").unwrap();
    if !extension_migrations.is_dense() {
        extension_migrations = extension_migrations.redensify().unwrap();
    }
    let mut migrations = MigrationDirectory::try_from("migrations").unwrap();
    if !migrations.is_dense() {
        migrations = migrations.redensify().unwrap();
    }
    time_tracker.add_completed_task(task);

    // First, we create the CSV for the font-awesome icons
    let task = Task::new("Creating Font Awesome Icons CSV");
    font_awesome::Icon::to_csv("csvs/icons.csv").unwrap();
    time_tracker.add_completed_task(task);

    // Next, we create the CSV for the taxonomical ranks
    let task = Task::new("Creating Taxonomical Ranks CSV");
    NCBIRank::to_csv("csvs/ranks.csv").unwrap();
    time_tracker.add_completed_task(task);

    // We retrieve and build the latest version of the NCBI taxonomy
    let task = Task::new("Fetching NCBI Taxonomy");
    let taxonomy: NCBITaxonomy = NCBITaxonomyBuilder::latest().build().await.unwrap();
    time_tracker.add_completed_task(task);
    let task = Task::new("Creating Taxonomy CSV");
    taxonomy.to_csv("csvs/taxa.csv").unwrap();
    time_tracker.add_completed_task(task);

    // Next, we build the SQL associated with the CSVs present in the 'csvs'
    // directory
    let task = Task::new("Building SQL from CSVs");
    let schema: CSVSchema = CSVSchemaBuilder::default()
        // To show a loading bar while processing the CSVs
        .verbose()
        // To include compressed files such as .gz
        .include_gz()
        // For supporting running the tests within
        // containers such as Docker
        .singularize()
        .container_directory("/app/csvs")
        .from_dir("./csvs")
        .unwrap();

    // We create the SQL to create the tables and populate them
    let sql: String = schema.to_postgres().unwrap();
    time_tracker.add_completed_task(task);

    // And obtain a connection to the database
    let task = Task::new("Establishing Connection to Postgres");
    let mut conn = establish_connection_to_postgres();
    time_tracker.add_completed_task(task);

    let task = Task::new("Executing CSQLV SQL");
    // We execute the SQL
    if let Err(error) = conn.batch_execute(&sql) {
        // We write out to file the sql we have just executed that has failed.
        std::fs::write("failed.sql", &sql).unwrap();

        eprintln!("Failed to execute SQL: {:?}", error);
        std::process::exit(1);
    }
    time_tracker.add_completed_task(task);

    // We execute the migrations
    let task = Task::new("Executing Migrations");
    for migration in extension_migrations.ups().unwrap() {
        conn.batch_execute(&migration).unwrap();
    }
    for migration in migrations.ups().unwrap() {
        conn.batch_execute(&migration).unwrap();
    }
    time_tracker.add_completed_task(task);

    // Now that the preliminary database setup is done, we can execute the Meta-SQL
    // which takes care of the roles tables and canx functions, which determine
    // whether a user can insert or update entries in a given table.
    let task = Task::new("Executing Meta-SQL");
    Table::create_roles_tables(&mut conn, DATABASE_NAME, None).unwrap();
    AuthorizationFunctionBuilder::default()
        .add_childless_table(Table::load(&mut conn, "users", None, DATABASE_NAME).unwrap())
        .create_authorization_functions_and_triggers(&mut conn, DATABASE_NAME, None)
        .unwrap();
    Table::create_update_triggers(&mut conn, DATABASE_NAME, None).unwrap();
    time_tracker.add_completed_task(task);

    // We check that the database follows the expected constraints.
    let task = Task::new("Checking Constraints");
    CompatibleForeignTypeConstraint::default().check_all(DATABASE_NAME, None, &mut conn).unwrap();
    LowercaseColumnConstraint::default().check_all(DATABASE_NAME, None, &mut conn).unwrap();
    LowercaseTableConstraint::default().check_all(DATABASE_NAME, None, &mut conn).unwrap();
    time_tracker.add_completed_task(task);

    // We write to the target directory the generated structs

    // Generate the code associated with the database
    let task = Task::new("Generating Code");
    Codegen::default()
        .set_output_path(path.as_ref())
        .generate(&mut conn, DATABASE_NAME, None)
        .unwrap();
    time_tracker.add_completed_task(task);

    // We save the time tracker
    time_tracker.save(Path::new("time_tracker")).unwrap();

    // We print the report
    let mut report = Report::new(time_tracker);
    report.add_directory(Path::new("time_tracker")).unwrap();
    report
        .write(
            &Path::new("time_requirements_report.md"),
            &Path::new("time_requirements_report.png"),
        )
        .unwrap();
}
