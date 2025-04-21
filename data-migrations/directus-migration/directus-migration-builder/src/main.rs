//! Build the core structures.
use std::path::Path;

use diesel::{Connection, pg::PgConnection};
use time_requirements::prelude::*;
use webcodegen::{Codegen, Table};

const DATABASE_NAME: &str = "directus";
const DATABASE_PASSWORD: &str = "directus_dbgi";
const DATABASE_USER: &str = "directus";
const DATABASE_PORT: u16 = 5434;
const HOSTNAME: &str = "134.21.20.118";
const DATABASE_URL: &str = const_format::formatcp!(
    "postgres://{DATABASE_USER}:{DATABASE_PASSWORD}@{HOSTNAME}:{DATABASE_PORT}/{DATABASE_NAME}",
);

#[tokio::main]
/// Executable to generate the code for the Directus database.
pub async fn main() {
    // Get the output directory
    let out_dir = Path::new("../src");

    // We ensure that the migrations directory has all expected properties.
    let mut time_tracker = TimeTracker::new("Building Directus Structures");

    let mut conn = PgConnection::establish(DATABASE_URL).unwrap();

    // We write to the target directory the generated structs
    let curation_data = Table::load(&mut conn, "Curation_Data", None, DATABASE_NAME).unwrap();

    // Generate the code associated with the database
    let task = Task::new("Generating Code");
    Codegen::default()
        .add_table_to_deny_list(&curation_data)
        .set_output_directory(out_dir.as_ref())
        .enable_loadable_trait()
        .enable_foreign_trait()
        .beautify()
        .generate(&mut conn, DATABASE_NAME, None)
        .unwrap();
    time_tracker.add_completed_task(task);

    // We create the directory `time_tracker` if it does not already exist
    std::fs::create_dir_all("./time_tracker").unwrap();

    // We save the time tracker
    time_tracker.save(Path::new("./time_tracker")).unwrap();

    // We print the report
    let mut report = Report::new(time_tracker);
    report.add_directory(Path::new("./time_tracker")).unwrap();
    report
        .write(Path::new("time_requirements_report.md"), Path::new("time_requirements_report.png"))
        .unwrap();
}
