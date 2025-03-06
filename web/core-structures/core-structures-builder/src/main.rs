//! Build the core structures.
use std::path::Path;

use csqlv::CSVSchemaBuilder;
use diesel::{Connection, pg::PgConnection};
use diesel_migrations_utils::prelude::*;
use taxonomy_fetcher::{
    Rank, Taxonomy, TaxonomyBuilder,
    impls::ncbi::{NCBIRank, NCBITaxonomy, NCBITaxonomyBuilder},
};
use time_requirements::prelude::*;
use webcodegen::{
    Codegen, CompatibleForeignTypeConstraint, CustomColumnConstraint, CustomTableConstraint,
    LowercaseColumnConstraint, LowercaseTableConstraint, Table,
};

const DATABASE_NAME: &str = "development.db";
const DATABASE_PASSWORD: &str = "password";
const DATABASE_USER: &str = "user";
const DATABASE_PORT: u16 = 15032;
const DATABASE_URL: &str = const_format::formatcp!(
    "postgres://{DATABASE_USER}:{DATABASE_PASSWORD}@localhost:{DATABASE_PORT}/{DATABASE_NAME}",
);

#[tokio::main]
pub async fn main() {
    // Get the output directory
    let out_dir = Path::new("../src");

    // We ensure that the migrations directory has all expected properties.
    let mut time_tracker = TimeTracker::new("Building Core Structures");
    let task = Task::new("Checking Migrations Directory");
    let mut extension_migrations =
        MigrationDirectory::try_from("../extensions_migrations").unwrap();
    if !extension_migrations.is_dense() {
        extension_migrations = extension_migrations.redensify().unwrap();
    }
    let mut migrations = MigrationDirectory::try_from("../migrations").unwrap();
    if !migrations.is_dense() {
        migrations = migrations.redensify().unwrap();
    }
    time_tracker.add_completed_task(task);

    // First, we create the CSV for the font-awesome icons
    let task = Task::new("Creating Font Awesome Icons CSV");
    font_awesome::Icon::to_csv("../csvs/icons.csv").unwrap();
    time_tracker.add_completed_task(task);

    // Next, we create the CSV for the taxonomical ranks
    let task = Task::new("Creating Taxonomical Ranks CSV");
    NCBIRank::to_csv("../csvs/ranks.csv").unwrap();
    time_tracker.add_completed_task(task);

    // We retrieve and build the latest version of the NCBI taxonomy
    let task = Task::new("Fetching NCBI Taxonomy");
    let taxonomy: NCBITaxonomy = NCBITaxonomyBuilder::latest().build().await.unwrap();
    time_tracker.add_completed_task(task);
    let task = Task::new("Creating Taxonomy CSV");
    taxonomy.to_csv("../csvs/taxa.csv").unwrap();
    time_tracker.add_completed_task(task);

    // Next, we build the SQL associated with the CSVs present in the 'csvs'
    // directory
    let task = Task::new("Building Schema from CSVs");
    CSVSchemaBuilder::default()
        // To show a loading bar while processing the CSVs
        .verbose()
        // To include compressed files such as .gz
        .include_gz()
        // For supporting running the tests within
        // containers such as Docker
        .singularize()
        .container_directory("/app/csvs")
        .from_dir("../csvs")
        .unwrap()
        .connect_and_create::<diesel::PgConnection>(DATABASE_URL)
        .unwrap();

    time_tracker.add_completed_task(task);

    // We execute the migrations
    let task = Task::new("Executing Migrations");
    extension_migrations.connect_and_execute_ups::<diesel::PgConnection>(DATABASE_URL).unwrap();
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
    CompatibleForeignTypeConstraint::default().check_all(DATABASE_NAME, None, &mut conn).unwrap();
    LowercaseColumnConstraint::default().check_all(DATABASE_NAME, None, &mut conn).unwrap();
    LowercaseTableConstraint::default().check_all(DATABASE_NAME, None, &mut conn).unwrap();
    time_tracker.add_completed_task(task);

    // We write to the target directory the generated structs

    // Generate the code associated with the database
    let task = Task::new("Generating Code");
    let users_table =
        Table::load(&mut conn, "users", None, DATABASE_NAME).expect("Failed to load `users` table");
    Codegen::default()
        .users(&users_table)
        .set_output_directory(out_dir.as_ref())
        .enable_loadable_trait()
        .enable_deletable_trait()
        .enable_insertable_trait()
        .enable_foreign_trait()
        // .beautify()
        .generate(&mut conn, DATABASE_NAME, None)
        .unwrap();
    time_tracker.add_completed_task(task);

    // We save the time tracker
    time_tracker.save(Path::new("./time_tracker")).unwrap();

    // We print the report
    let mut report = Report::new(time_tracker);
    report.add_directory(Path::new("./time_tracker")).unwrap();
    report
        .write(
            &Path::new("time_requirements_report.md"),
            &Path::new("time_requirements_report.png"),
        )
        .unwrap();
}
