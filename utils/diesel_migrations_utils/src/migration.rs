//! Submodule defining the migration struct and its methods.

use std::path::{Path, PathBuf};

use sqlparser::ast::Statement;

use crate::errors::Error;

#[derive(Debug, PartialEq, Eq, Clone, Hash)]
/// Struct representing a migration.
pub struct Migration {
    /// The name of the migration.
    name: String,
    /// The number of the migration.
    number: u64,
}

impl PartialOrd for Migration {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Migration {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.number.cmp(&other.number)
    }
}

impl<'a> TryFrom<&'a Path> for Migration {
    type Error = Error;

    fn try_from(path: &'a Path) -> Result<Self, Self::Error> {
        // We get the name of the migration.
        let name =
            path.file_name().ok_or(Error::InvalidMigration(path.to_string_lossy().to_string()))?;
        let name =
            name.to_str().ok_or(Error::InvalidMigration(path.to_string_lossy().to_string()))?;
        let mut fragmented_name = name.split('_');
        let number = fragmented_name
            .next()
            .ok_or(Error::InvalidMigration(path.to_string_lossy().to_string()))?
            .parse::<u64>()
            .map_err(|_| Error::InvalidMigration(path.to_string_lossy().to_string()))?;

        let name = fragmented_name.collect::<Vec<&str>>().join("_");

        // We check whether the provided path contains the up and down migrations.
        let up = path.join("up.sql");
        let down = path.join("down.sql");
        if !up.exists() {
            return Err(Error::MissingUpMigration(number));
        }
        if !down.exists() {
            return Err(Error::MissingDownMigration(number));
        }

        // We check whether the syntax of the `up.sql` document is correct.
        let up = std::fs::read_to_string(up)?;
        if let Err(up_error) = sqlparser::parser::Parser::parse_sql(
            &sqlparser::dialect::PostgreSqlDialect {},
            up.as_str(),
        ) {
            return Err(Error::InvalidSql(
                number,
                crate::prelude::MigrationKind::Up,
                up_error.to_string(),
            ));
        }
        let down = std::fs::read_to_string(down)?;
        if let Err(down_error) = sqlparser::parser::Parser::parse_sql(
            &sqlparser::dialect::PostgreSqlDialect {},
            down.as_str(),
        ) {
            return Err(Error::InvalidSql(
                number,
                crate::prelude::MigrationKind::Down,
                down_error.to_string(),
            ));
        }

        Ok(Migration { name: name.to_string(), number })
    }
}

impl<'a> TryFrom<&'a PathBuf> for Migration {
    type Error = Error;

    fn try_from(path: &'a PathBuf) -> Result<Self, Self::Error> {
        Migration::try_from(path.as_path())
    }
}

impl<'a> TryFrom<&'a str> for Migration {
    type Error = Error;

    fn try_from(path: &'a str) -> Result<Self, Self::Error> {
        // We determine the path of the migration.
        let path = Path::new(path);

        Migration::try_from(path)
    }
}

impl Migration {
    #[must_use]
    /// Returns the name of the migration.
    ///
    /// # Returns
    ///
    /// The name of the migration.
    pub fn name(&self) -> &str {
        &self.name
    }

    #[must_use]
    /// Returns the number of the migration.
    ///
    /// # Returns
    ///
    /// The number of the migration.
    pub fn number(&self) -> u64 {
        self.number
    }

    #[must_use]
    /// Returns the name of the directory containing the migration.
    pub fn directory(&self) -> String {
        format!("{:014}_{}", self.number, self.name)
    }

    /// Returns an iterator over all of the tables created in the up migration.
    ///
    /// # Arguments
    ///
    /// * `parent` - The parent path of the migration.
    ///
    /// # Raises
    ///
    /// * `Error::ReadingMigrationFailed` - If the migration cannot be read.
    /// * `Error::ParsingMigrationFailed` - If the migration cannot be parsed.
    pub fn tables(&self, parent: &Path) -> Result<impl Iterator<Item = String>, Error> {
        Ok(self.up_statements(parent)?.into_iter().filter_map(|statement| {
            if let Statement::CreateTable(create_table) = statement {
                Some(create_table.name.to_string())
            } else {
                None
            }
        }))
    }

    /// Returns an iterator over all of the foreign tables referenced in the
    /// up migration.
    ///
    /// # Arguments
    ///
    /// * `parent` - The parent path of the migration.
    ///
    /// # Raises
    ///
    /// * `Error::ReadingMigrationFailed` - If the migration cannot be read.
    /// * `Error::ParsingMigrationFailed` - If the migration cannot be parsed.
    pub fn foreign_tables(&self, parent: &Path) -> Result<impl Iterator<Item = String>, Error> {
        Ok(self
            .up_statements(parent)?
            .into_iter()
            .filter_map(|statement| {
                if let Statement::CreateTable(create_table) = statement {
                    Some(create_table)
                } else {
                    None
                }
            })
            .flat_map(|create_table| {
                create_table
                    .columns
                    .into_iter()
                    .flat_map(|column| column.options.into_iter())
                    .filter_map(|option| {
                        if let sqlparser::ast::ColumnOption::ForeignKey { foreign_table, .. } =
                            option.option
                        {
                            Some(foreign_table.to_string())
                        } else {
                            None
                        }
                    })
                    .chain(create_table.constraints.into_iter().filter_map(|constraint| {
                        if let sqlparser::ast::TableConstraint::ForeignKey {
                            foreign_table, ..
                        } = constraint
                        {
                            Some(foreign_table.to_string())
                        } else {
                            None
                        }
                    }))
            }))
    }

    #[must_use]
    /// Returns the SQL content of the up migration.
    ///
    /// # Arguments
    ///
    /// * `parent` - The parent path of the migration.
    ///
    /// # Errors
    ///
    /// * `Error::ReadingMigrationFailed` - If the migration cannot be read.
    pub fn up(&self, parent: &Path) -> Result<String, Error> {
        let path = parent.join(self.directory()).join("up.sql");
        std::fs::read_to_string(path).map_err(|error| {
            Error::ReadingMigrationFailed(
                self.number,
                crate::prelude::MigrationKind::Up,
                error.to_string(),
            )
        })
    }

    /// Returns the statements in the up migration.
    ///
    /// # Arguments
    ///
    /// * `parent` - The parent path of the migration.
    ///
    /// # Errors
    ///
    /// * `Error::ReadingMigrationFailed` - If the migration cannot be read.
    /// * `Error::ParsingMigrationFailed` - If the migration cannot be parsed.
    pub fn up_statements(&self, parent: &Path) -> Result<Vec<Statement>, Error> {
        let statements = self.up(parent)?;
        let statements = sqlparser::parser::Parser::parse_sql(
            &sqlparser::dialect::PostgreSqlDialect {},
            statements.as_str(),
        )
        .map_err(|error| {
            Error::ParsingMigrationFailed(self.number, crate::prelude::MigrationKind::Up, error)
        })?;
        Ok(statements)
    }

    /// Returns the SQL content of the down migration.
    pub fn down(&self, parent: &Path) -> Result<String, Error> {
        let path = parent.join(self.directory()).join("down.sql");
        std::fs::read_to_string(path).map_err(|error| {
            Error::ReadingMigrationFailed(
                self.number,
                crate::prelude::MigrationKind::Down,
                error.to_string(),
            )
        })
    }

    /// Moves the up and down migrations to the newly provided number and
    /// returns the new migration.
    ///
    /// # Implementation details
    ///
    /// This method does not take into account whether the new number is already
    /// taken by another migration. It is the responsibility of the caller
    /// to ensure that the new number is not already taken.
    ///
    /// # Arguments
    ///
    /// * `number` - The new number of the migration.
    /// * `parent` - The parent path of the migration.
    pub(crate) fn move_to(self, number: u64, parent: &Path) -> Result<Self, Error> {
        let current_migration_directory = parent.join(self.directory());
        let updated_migration = Migration { name: self.name.clone(), number };
        let updated_migration_directory = parent.join(updated_migration.directory());
        std::fs::rename(&current_migration_directory, &updated_migration_directory).map_err(
            |_| {
                Error::MovingMigrationFailed {
                    source: current_migration_directory.to_string_lossy().to_string(),
                    destination: updated_migration_directory.to_string_lossy().to_string(),
                }
            },
        )?;
        Ok(updated_migration)
    }
}
