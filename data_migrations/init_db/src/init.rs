use std::path::Path;

use diesel::Connection;
use time_requirements::prelude::{Task, TimeTracker};

use crate::{
    consistency_constraints::execute_consistency_constraint_checks,
    csv_migrations::{init_csvs, retrieve_csvs},
    migrations::init_migrations,
};

/// Initializes the database.
///
/// # Arguments
///
/// * `database_name` - The name of the database.
/// * `conn` - A mutable reference to the database connection.
///
/// # Errors
///
/// * If the connection cannot be established.
/// * If the migrations cannot be applied.
pub async fn init_database(
    database_name: &str,
    conn: &mut diesel::PgConnection,
) -> Result<TimeTracker, crate::errors::Error> {
    let cargo_directory = env!("CARGO_MANIFEST_DIR");
    let migrations_directory = Path::new(cargo_directory).join("migrations");
    let extension_migrations_directory = Path::new(cargo_directory).join("extension_migrations");
    let csv_directory = Path::new(cargo_directory).join("csvs");
    let container_directory = Path::new("/app/data_migrations/init_db/csvs");
    retrieve_csvs(&csv_directory).await?;
    let transaction_time_tracker = conn.transaction(|portal_conn| {
        let mut time_tracker = TimeTracker::new("Init DB Transaction");
        let task = Task::new("Initialize CSVs");
        init_csvs(&csv_directory, container_directory, portal_conn)?;
        time_tracker.add_completed_task(task);
        let task = Task::new("Initialize Migrations");
        init_migrations(&migrations_directory, &extension_migrations_directory, portal_conn)?;
        time_tracker.add_completed_task(task);
        time_tracker.extend(execute_consistency_constraint_checks(database_name, portal_conn)?);
        Ok::<_, crate::errors::Error>(time_tracker)
    })?;
    Ok(transaction_time_tracker)
}
