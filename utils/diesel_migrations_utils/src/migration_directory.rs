//! Submodule representing a directory containing migrations.

use std::path::Path;

use crate::{errors::Error, migration::Migration};

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
        let mut migrations = path
            .read_dir()?
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

        Ok(MigrationDirectory {
            migrations,
            directory,
        })
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
    /// Redensifies the migrations and returns the newly densified migrations.
    pub fn redensify(self) -> Self {
        let path = Path::new(&self.directory);
        MigrationDirectory {
            migrations: self
                .migrations
                .into_iter()
                .enumerate()
                .map(|(number, migration)| migration.move_to(number as u64, path))
                .collect(),
            directory: self.directory,
        }
    }
}
