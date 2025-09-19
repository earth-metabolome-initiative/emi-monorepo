//! Submodule defining the main translator struct.

use sqlparser::ast::Statement;

use crate::{
    options::Pg2SqliteOptions,
    prelude::{PgSchema, Translator},
};

#[derive(Debug, Clone, Default)]
/// Struct to translate between a `PostgreSQL` entry and a `SQLite` entry.
pub struct Pg2Sqlite {
    /// The set of `PostgreSQL` statements to be translated.
    pub pg_statements: Vec<Statement>,
    /// Inner schema.
    inner_schema: PgSchema,
}

impl Pg2Sqlite {
    #[must_use]
    /// Adds a new SQL statement to the set of `PostgreSQL` statements to be
    /// translated.
    pub fn statement(mut self, statement: Statement) -> Self {
        self.pg_statements.push(statement);
        self
    }

    /// Adds a new SQL statement to be parsed and added to the set of
    /// `PostgreSQL` statements to be translated.
    ///
    /// # Arguments
    ///
    /// * `sql` - The SQL statement to be parsed and added to the set of
    ///   `PostgreSQL` statements to be translated.
    ///
    /// # Returns
    ///
    /// A Result containing the updated `Pg2Sqlite` struct or an error if the
    /// SQL statement could not be parsed.
    ///
    /// # Errors
    ///
    /// * If the SQL statement could not be parsed.
    pub fn sql(mut self, sql: &str) -> Result<Self, crate::errors::Error> {
        let stmt =
            sqlparser::parser::Parser::parse_sql(&sqlparser::dialect::PostgreSqlDialect {}, sql)
                .map_err(|e| crate::errors::Error::ParserError(sql.to_owned(), e))?;
        for statement in stmt {
            self = self.statement(statement);
        }
        Ok(self)
    }

    /// Adds a new path with an SQL file to be parsed and added to the set of
    /// `PostgreSQL` statements to be translated.
    ///
    /// # Arguments
    ///
    /// * `path` - The path to the SQL file to be parsed and added to the set of
    ///   `PostgreSQL` statements to be translated.
    ///
    /// # Returns
    ///
    /// A Result containing the updated `Pg2Sqlite` struct or an error if the
    /// SQL file could not be read or parsed.
    ///
    /// # Errors
    ///
    /// * If the SQL file could not be read.
    /// * If the SQL file could not be parsed.
    pub fn file<P: AsRef<std::path::Path>>(self, path: P) -> Result<Self, crate::errors::Error> {
        let path = path.as_ref();
        let content = std::fs::read_to_string(path)?;
        self.sql(&content)
    }

    /// Adds all of the `up.sql` migrations found under the given directory to
    /// the set of `PostgreSQL` statements to be translated.
    ///
    /// # Arguments
    ///
    /// * `path` - The path to the directory containing the `up.sql` migrations.
    ///
    /// # Returns
    ///
    /// A Result containing the updated `Pg2Sqlite` struct or an error if the
    /// SQL files could not be read or parsed.
    ///
    /// # Errors
    ///
    /// * If the SQL files could not be read.
    /// * If the SQL files could not be parsed.
    pub fn ups<P: AsRef<std::path::Path>>(
        mut self,
        directory: P,
    ) -> Result<Self, crate::errors::Error> {
        // We iterate recursively over the migrations directory.
        for entry in std::fs::read_dir(directory)? {
            let entry = entry?;
            let path = entry.path();
            if path.is_file() {
                // If the file name is `up.sql` we parse it and add it to the
                // set of PostgreSQL statements to be translated.
                if let Some(file_name) = path.file_name()
                    && file_name == "up.sql"
                {
                    self = self.file(path)?;
                }
            } else if path.is_dir() {
                self = self.ups(path)?;
            }
        }
        Ok(self)
    }

    /// Translates the set of `PostgreSQL` statements to `SQLite` statements.
    ///
    /// # Returns
    ///
    /// * A Result containing the set of `SQLite` statements or an error if the
    ///   translation could not be performed.
    ///
    /// # Errors
    ///
    /// * If the translation could not be performed.
    ///
    /// # Panics
    ///
    /// * If the progress bar could not be created.
    pub fn translate(
        mut self,
        options: &Pg2SqliteOptions,
    ) -> Result<Vec<Statement>, crate::errors::Error> {
        self.pg_statements
            .iter()
            .map(|statement| statement.translate(&mut self.inner_schema, options))
            .collect::<Result<Vec<Vec<Statement>>, crate::errors::Error>>()
            .map(|statements| statements.into_iter().flatten().collect())
    }
}
