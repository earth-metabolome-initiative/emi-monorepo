//! Submodule representing a directory containing migrations.

use std::{collections::HashMap, path::Path};

use algebra::{
    impls::SquareCSR2D,
    prelude::{Johnson, Kahn, MatrixMut, RaggedVector},
};
use graph::{
    prelude::{
        Builder, GenericGraph, GenericMonoplexMonopartiteGraphBuilder, MonoplexGraph,
        MonoplexMonopartiteGraph,
    },
    traits::{MonopartiteGraphBuilder, MonoplexGraphBuilder},
};

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
                        if entry.path().is_dir() { Some(entry.path()) } else { None }
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

        let migrations = MigrationDirectory { migrations, directory };

        let graph = migrations.graph()?;
        if let Some(circuit) = graph.edges().johnson().next() {
            return Err(Error::CircularDependency(
                circuit
                    .into_iter()
                    .map(|migration_id| {
                        let migration = &migrations.migrations[migration_id];
                        (migration.number(), migration.name().to_owned())
                    })
                    .collect(),
            ));
        }

        Ok(migrations)
    }
}

impl<'a> TryFrom<&'a str> for MigrationDirectory {
    type Error = Error;

    fn try_from(path: &'a str) -> Result<Self, Self::Error> {
        MigrationDirectory::try_from(Path::new(path))
    }
}

impl MigrationDirectory {
    /// Returns the graph of dependencies between the migrations.
    ///
    /// # Implementation details
    ///
    /// The edges of this graph go from the parent migrations to the child
    /// migrations. The nodes of the graph are the migrations.
    ///
    /// # Errors
    ///
    /// * If the up migrations cannot be parsed with `sqlparser`
    fn graph(
        &self,
    ) -> Result<GenericGraph<&[Migration], SquareCSR2D<RaggedVector<usize, usize, usize>>>, Error>
    {
        let path: &Path = Path::new(&self.directory);
        // We create a map between the table names and the migrations.
        let mut migration_map: HashMap<String, usize> = HashMap::new();

        for (migration_number, migration) in self.migrations().enumerate() {
            // We get the tables created by the migration.
            for table in migration.tables(path)? {
                migration_map.insert(table, migration_number);
            }
        }

        // We create the graph from the parent migrations to their children.
        let dependency_edges: SquareCSR2D<RaggedVector<usize, usize, usize>> =
            SquareCSR2D::from_entries(self.migrations().enumerate().flat_map(
                |(child_migration_id, migration)| {
                    let mut parent_migration_ids: Vec<usize> = migration
                        .foreign_tables(path)
                        .ok()
                        .into_iter()
                        .flatten()
                        .filter_map(|foreign_table| {
                            // We get the migration number of the foreign table.
                            migration_map.get(&foreign_table)
                        })
                        // We remove self-loops
                        .filter(|&&parent_migration_id| parent_migration_id != child_migration_id)
                        .copied()
                        .collect();
                    parent_migration_ids.sort_unstable();
                    parent_migration_ids.dedup();
                    parent_migration_ids
                        .into_iter()
                        .map(move |parent_migration_id| (parent_migration_id, child_migration_id))
                },
            ))
            .expect("Failed to create the dependency graph");

        Ok(GenericMonoplexMonopartiteGraphBuilder::default()
            .nodes(self.migrations.as_slice())
            .edges(dependency_edges)
            .build()
            .unwrap())
    }

    /// Iterates over the migrations in the directory in topological order.
    ///
    /// # Implementation details
    ///
    /// For each migration, by using `sqlparser`, we determine whether it
    /// contains a `CREATE TABLE` statement. If it does, we determine the
    /// foreign keys of the table which are treated as the dependencies of
    /// the migration. Then, we sort the migrations in topological order.
    /// This approach may be extended to include other types of statements
    /// such as indices, views, etc.
    pub fn topological_order(&self) -> Result<impl Iterator<Item = &Migration>, Error> {
        let graph = self.graph()?;

        let topological_order_from_roots = graph.edges().kahn().map_err(|_| Error::NotDAG)?;
        let mut reverse_index = vec![0; self.migrations.len()];
        topological_order_from_roots.into_iter().enumerate().for_each(|(i, migration_id)| {
            reverse_index[migration_id] = i;
        });

        Ok(reverse_index.into_iter().map(|migration_id| &self.migrations[migration_id]))
    }

    /// Iterates on the migrations.
    pub fn migrations(&self) -> impl Iterator<Item = &Migration> {
        self.migrations.iter()
    }

    /// Returns the largest migration number.
    #[must_use]
    pub fn max_migration_number(&self) -> Option<u64> {
        self.migrations.iter().map(super::migration::Migration::number).max()
    }

    /// Returns whether the migrations are currently ordered topologically,
    /// meaning that the dependencies of each migration are satisfied.
    ///
    /// # Errors
    ///
    /// * If the up migrations cannot be parsed with `sqlparser`
    pub fn is_topologically_sorted(&self) -> Result<bool, Error> {
        let graph = self.graph()?;
        Ok(graph.is_topologically_sorted())
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
    ///
    /// # Errors
    ///
    /// * If the down migration cannot be read.
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

    /// Redensifies the migrations and returns the newly densified migrations.
    ///
    /// # Errors
    ///
    /// * If the migrations cannot be moved
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

    /// Recreates the migrations following the topological order.
    ///
    /// # Errors
    ///
    /// * If the migrations syntax cannot be parsed with `sqlparser`
    /// * If the migrations cannot be moved
    pub fn topologically_sort(self) -> Result<MigrationDirectory, Error> {
        let path: &Path = Path::new(&self.directory);
        // First, we shift all of the migrations to the largest migration number + 1
        // so to avoid any conflicts while proceeding with the topological sorting.
        let max_migration_number = self.max_migration_number().unwrap_or(0) + 1;

        let shifted_directory = MigrationDirectory {
            directory: self.directory.clone(),
            migrations: self
                .migrations
                .into_iter()
                .enumerate()
                .map(|(number, migration)| {
                    migration.move_to(max_migration_number + number as u64, path)
                })
                .collect::<Result<Vec<Migration>, Error>>()?,
        };

        // We obtain the topological order of the migrations.
        Ok(MigrationDirectory {
            directory: self.directory.clone(),
            migrations: shifted_directory
                .topological_order()?
                .cloned()
                .enumerate()
                .map(|(number, migration)| migration.move_to(number as u64, path))
                .collect::<Result<Vec<Migration>, Error>>()?,
        })
    }
}
