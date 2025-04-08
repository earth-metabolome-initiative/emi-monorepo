//! Submodule representing a directory containing migrations.

use std::path::Path;

use crate::{errors::Error, migration::Migration, prelude::MigrationKind};

#[derive(Debug)]
/// Struct representing a directory containing migrations.
pub struct MigrationDirectory {
    /// List of migrations in the directory.
    migrations: Vec<Migration>,
    /// The path of the directory.
    directory: String,
}

impl<'a> TryFrom<&'a Path> for MigrationDirectory {
    type Error = Error;

    fn try_from(path: &'a Path) -> Result<Self, Self::Error> {
        let directory = path.to_str().unwrap().to_string();

        // We iterate on the subdirectories within the directory.
        let mut migrations =
            path.read_dir()?
                .filter_map(|entry| {
                    entry.ok().and_then(|entry| {
                        if entry.path().is_dir() {
                            Some(entry.path())
                        } else {
                            None
                        }
                    })
                })
                .map(|path| Migration::try_from(&path))
                .collect::<Result<Vec<Migration>, Error>>()?;

        // We check that there are no migrations with the same number.
        migrations.sort_unstable();

        for i in 0..migrations.len() - 1 {
            if migrations[i].number() == migrations[i + 1].number() {
                return Err(Error::DuplicateMigrationNumber(migrations[i].number()));
            }
        }

        Ok(MigrationDirectory { migrations, directory })
    }
}

impl<'a> TryFrom<&'a str> for MigrationDirectory {
    type Error = Error;

    fn try_from(path: &'a str) -> Result<Self, Self::Error> {
        MigrationDirectory::try_from(Path::new(path))
    }
}

impl MigrationDirectory {
    /// Iterates on the migrations.
    pub fn migrations(&self) -> impl Iterator<Item = &Migration> {
        self.migrations.iter()
    }

    /// Iterates on the up migrations.
    pub fn ups(&self) -> impl Iterator<Item = Result<String, Error>> + '_ {
        let path: &Path = Path::new(&self.directory);
        self.migrations.iter().map(|migration| migration.up(path))
    }

    /// Executes all the up migrations.
    ///
    /// # Arguments
    ///
    /// * `conn` - The connection to the database.
    ///
    /// # Returns
    ///
    /// The result of the execution of the migrations
    ///
    /// # Errors
    ///
    /// * If the execution of the migrations fails
    pub fn execute_ups<C: diesel::Connection>(&self, conn: &mut C) -> Result<(), Error> {
        for (migration_number, migration) in self.ups().enumerate() {
            conn.batch_execute(&migration?).map_err(|error| {
                Error::ExecutingMigrationFailed(migration_number as u64, MigrationKind::Up, error)
            })?;
        }
        Ok(())
    }

    /// Connects and executes all the up migrations.
    ///
    /// # Arguments
    ///
    /// * `url` - The URL of the database.
    ///
    /// # Returns
    ///
    /// The result of the execution of the migrations
    ///
    /// # Errors
    ///
    /// * If the connection to the database fails
    /// * If the execution of the migrations fails
    pub fn connect_and_execute_ups<C: diesel::Connection>(&self, url: &str) -> Result<(), Error> {
        let mut attempts = 0;
        loop {
            match C::establish(url) {
                Err(err) => {
                    if attempts >= 10 {
                        return Err(err.into());
                    }
                    std::thread::sleep(std::time::Duration::from_secs(1));
                    attempts += 1;
                }
                Ok(mut conn) => return self.execute_ups(&mut conn),
            }
        }
    }

    /// Iterates on the down migrations.
    pub fn downs(&self) -> Result<Vec<String>, Error> {
        let path: &Path = Path::new(&self.directory);
        self.migrations
            .iter()
            .map(|migration| migration.down(path))
            .collect::<Result<Vec<String>, Error>>()
    }

    /// Executes all the down migrations.
    ///
    /// # Arguments
    ///
    /// * `conn` - The connection to the database.
    ///
    /// # Returns
    ///
    /// The result of the execution of the migrations
    ///
    /// # Errors
    ///
    /// * If the execution of the migrations fails
    pub fn execute_downs<C: diesel::Connection>(&self, conn: &mut C) -> Result<(), Error> {
        for (migration_number, migration) in self.downs()?.iter().enumerate() {
            conn.batch_execute(migration).map_err(|error| {
                Error::ExecutingMigrationFailed(migration_number as u64, MigrationKind::Down, error)
            })?;
        }
        Ok(())
    }

    /// Connects and executes all the down migrations.
    ///
    /// # Arguments
    ///
    /// * `url` - The URL of the database.
    ///
    /// # Returns
    ///
    /// The result of the execution of the migrations
    ///
    /// # Errors
    ///
    /// * If the connection to the database fails
    /// * If the execution of the migrations fails
    pub fn connect_and_execute_downs<C: diesel::Connection>(&self, url: &str) -> Result<(), Error> {
        let mut attempts = 0;
        loop {
            match C::establish(url) {
                Err(err) => {
                    if attempts >= 10 {
                        return Err(err.into());
                    }
                    std::thread::sleep(std::time::Duration::from_secs(1));
                    attempts += 1;
                }
                Ok(mut conn) => return self.execute_downs(&mut conn),
            }
        }
    }

    #[must_use]
    /// Returns whether the migrations have dense identifiers.
    pub fn is_dense(&self) -> bool {
        for (number, migration) in self.migrations.iter().enumerate() {
            if migration.number() != number as u64 {
                return false;
            }
        }
        true
    }

    #[must_use]
    /// Returns the number of migrations.
    pub fn len(&self) -> usize {
        self.migrations.len()
    }

    #[must_use]
    /// Returns whether the migrations are empty.
    pub fn is_empty(&self) -> bool {
        self.migrations.is_empty()
    }

    #[must_use]
    /// Redensifies the migrations and returns the newly densified migrations.
    pub fn redensify(self) -> Result<Self, Error> {
        let path = Path::new(&self.directory);
        Ok(MigrationDirectory {
            migrations: self
                .migrations
                .into_iter()
                .enumerate()
                .map(|(number, migration)| migration.move_to(number as u64, path))
                .collect::<Result<Vec<Migration>, Error>>()?,
            directory: self.directory,
        })
    }
}
