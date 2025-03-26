//! Build the core structures.
use std::path::Path;

use config::{Config, File};
use diesel::{Connection, pg::PgConnection};
use time_requirements::prelude::*;
use webcodegen::{Codegen, Table};

#[tokio::main]
pub async fn main() {
    // Get the output directory
    let out_dir = Path::new("../src");

    // Load the configuration
    let settings = Config::builder()
        .add_source(File::with_name("config.ini"))
        .build()
        .expect("Failed to load configuration file");

    // Extract the database settings
    let database_name: String = settings.get("database.name").expect("Missing database.name");
    let database_password: String =
        settings.get("database.password").expect("Missing database.password");
    let database_user: String = settings.get("database.user").expect("Missing database.user");
    let database_port: u16 = settings.get("database.port").expect("Missing database.port");
    let database_host: String = settings.get("database.host").expect("Missing database.host");

    let database_url = format!(
        "postgres://{database_user}:{database_password}@{database_host}:{database_port}/{database_name}",
    );

    // We ensure that the migrations directory has all expected properties.
    let mut time_tracker = TimeTracker::new("Building Directus Structures");

    let mut conn = PgConnection::establish(&database_url).unwrap();

    // We write to the target directory the generated structs
    let curation_data = Table::load(&mut conn, "Curation_Data", None, "directus").unwrap();

    // Generate the code associated with the database
    let task = Task::new("Generating Code");
    Codegen::default()
        .add_table_to_deny_list(&curation_data)
        .set_output_directory(out_dir.as_ref())
        .enable_loadable_trait()
        .enable_foreign_trait()
        .beautify()
        .generate(&mut conn, "directus", None)
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
