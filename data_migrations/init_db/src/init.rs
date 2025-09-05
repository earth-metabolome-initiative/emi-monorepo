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
    let mut init_db_time_tracker = TimeTracker::new("Init DB");
    let cargo_directory = env!("CARGO_MANIFEST_DIR");
    let migrations_directory = Path::new(cargo_directory).join("migrations");
    let extension_migrations_directory = Path::new(cargo_directory).join("extension_migrations");
    let csv_directory = Path::new(cargo_directory).join("csvs");
    let container_directory = Path::new("/app/data_migrations/init_db/csvs");
    let task = Task::new("Retrieve CSVs");
    retrieve_csvs(&csv_directory).await?;
    init_db_time_tracker.add_completed_task(task);
    // let transaction_time_tracker = conn.transaction(|conn| {
    let mut transaction_time_tracker = TimeTracker::new("Init DB Transaction");
    let task = Task::new("Initialize CSVs");
    init_csvs(&csv_directory, container_directory, conn)?;
    transaction_time_tracker.add_completed_task(task);
    let task = Task::new("Initialize Migrations");
    init_migrations(&migrations_directory, &extension_migrations_directory, conn)?;
    transaction_time_tracker.add_completed_task(task);
    transaction_time_tracker.extend(execute_consistency_constraint_checks(database_name, conn)?);
    //     Ok::<_, crate::errors::Error>(transaction_time_tracker)
    // })?;
    init_db_time_tracker.extend(transaction_time_tracker);

    Ok(init_db_time_tracker)
}
