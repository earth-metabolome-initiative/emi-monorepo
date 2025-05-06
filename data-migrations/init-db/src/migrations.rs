use std::path::Path;

use diesel::result::DatabaseErrorKind;
use diesel_migrations_utils::prelude::*;

pub(crate) async fn init_migrations(
    migrations_directory: &Path,
    extension_migrations_directory: &Path,
    conn: &mut diesel_async::AsyncPgConnection,
) -> Result<(), crate::errors::Error> {
    // We delete empty directories in the `migrations` directory which may occur
    // when some git collision occurs.
    for entry in std::fs::read_dir(migrations_directory)? {
        let entry = entry?;
        if entry.file_type()?.is_dir() {
            let path = entry.path();
            if path.read_dir()?.count() == 0 {
                std::fs::remove_dir_all(path)?;
            }
        }
    }

    let mut extension_migrations = MigrationDirectory::try_from(extension_migrations_directory)?;
    if !extension_migrations.is_dense() {
        extension_migrations = extension_migrations.redensify()?;
    }
    let mut migrations = MigrationDirectory::try_from(migrations_directory)?;
    if !migrations.is_topologically_sorted()? {
        migrations = migrations.topologically_sort()?;
    }
    assert!(
        migrations.is_topologically_sorted()?,
        "The migrations are not topologically sorted: {:?}",
        migrations.not_topologically_sorted()?
    );

    if !migrations.is_dense() {
        migrations = migrations.redensify()?;
    }

    // We execute the migrations
    match extension_migrations.execute_ups(conn).await {
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
    migrations.execute_ups(conn).await?;

    Ok(())
}
