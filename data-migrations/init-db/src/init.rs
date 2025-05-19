use std::path::Path;

use diesel_async::AsyncConnection;

use crate::{
    consistency_constraints::execute_consistency_constraint_checks, csv_migrations::init_csvs,
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
    conn: &mut diesel_async::AsyncPgConnection,
) -> Result<(), crate::errors::Error> {
    let cargo_directory = env!("CARGO_MANIFEST_DIR");
    let migrations_directory = Path::new(cargo_directory).join("migrations");
    let extension_migrations_directory = Path::new(cargo_directory).join("extension_migrations");
    let csv_directory = Path::new(cargo_directory).join("csvs");
    let container_directory = Path::new("/app/data-migrations/init-db/csvs");

    conn.transaction(|portal_conn| {
        Box::pin(async move {
            init_csvs(&csv_directory, container_directory, portal_conn).await?;
            init_migrations(&migrations_directory, &extension_migrations_directory, portal_conn)
                .await?;
            execute_consistency_constraint_checks(database_name, portal_conn).await?;
            Ok::<(), crate::errors::Error>(())
        })
    })
    .await?;

    Ok(())
}
