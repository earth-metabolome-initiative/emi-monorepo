//! Submodule providing a trait for describing SQL Function-like entities.

use std::{fmt::Debug, hash::Hash};

use crate::{
    traits::{DatabaseLike, Metadata},
    utils::normalize_postgres_type,
};

/// A trait for describing SQL Function-like entities.
pub trait FunctionLike: Metadata + Debug + Clone + Hash + Ord + Eq {
    /// The associated database type.
    type DB: DatabaseLike<Function = Self>;

    /// The name of the function.
    ///
    /// # Example
    ///
    /// ```rust
    /// #  fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// use sql_traits::prelude::*;
    ///
    /// let db = ParserDB::try_from(
    ///     r#"
    /// CREATE FUNCTION add_one(x INT) RETURNS INT AS 'SELECT x + 1;';
    /// "#,
    /// )?;
    /// let function = db.functions().next().expect("Function should exist");
    /// assert_eq!(function.name(), "add_one");
    /// # Ok(())
    /// # }
    /// ```
    fn name(&self) -> &str;

    /// Returns the argument type names (if any) of the function as strings.
    ///
    /// # Example
    ///
    /// ```rust
    /// #  fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// use sql_traits::prelude::*;
    ///
    /// let db = ParserDB::try_from(
    ///     r#"
    /// CREATE FUNCTION add(x INT, y INT) RETURNS INT AS 'SELECT x + y;';
    /// CREATE FUNCTION greet(name TEXT) RETURNS TEXT AS 'SELECT "Hello, " || name;';
    /// "#,
    /// )?;
    /// let add_fn = db.functions().find(|f| f.name() == "add").expect("Function should exist");
    /// let greet_fn = db.functions().find(|f| f.name() == "greet").expect("Function should exist");
    /// assert_eq!(add_fn.argument_type_names(&db).collect::<Vec<_>>(), vec!["INT", "INT"]);
    /// assert_eq!(greet_fn.argument_type_names(&db).collect::<Vec<_>>(), vec!["TEXT"]);
    /// # Ok(())
    /// # }
    /// ```
    fn argument_type_names<'db>(
        &'db self,
        database: &'db Self::DB,
    ) -> impl Iterator<Item = &'db str>;

    /// Returns the normalized argument type names (if any) of the function as
    /// strings.
    ///
    /// # Example
    ///
    /// ```rust
    /// #  fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// use sql_traits::prelude::*;
    ///
    /// let db = ParserDB::try_from(
    ///     r#"
    /// CREATE FUNCTION add(x INTEGER, y INT) RETURNS INT AS 'SELECT x + y;';
    /// CREATE FUNCTION greet(name TEXT) RETURNS TEXT AS 'SELECT "Hello, " || name;';
    /// "#,
    /// )?;
    /// let add_fn = db.function("add").expect("Function should exist");
    /// let greet_fn = db.function("greet").expect("Function should exist");
    /// assert_eq!(add_fn.normalized_argument_type_names(&db), vec!["INT", "INT"]);
    /// assert_eq!(greet_fn.normalized_argument_type_names(&db), vec!["TEXT"]);
    /// # Ok(())
    /// # }
    /// ```
    fn normalized_argument_type_names<'db>(&'db self, database: &'db Self::DB) -> Vec<&'db str> {
        self.argument_type_names(database).map(normalize_postgres_type).collect()
    }

    /// Returns the return type name of the function as a string.
    ///
    /// # Example
    ///
    /// ```rust
    /// #  fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// use sql_traits::prelude::*;
    ///
    /// let db = ParserDB::try_from(
    ///     r#"
    /// CREATE FUNCTION add_one(x INT) RETURNS INT AS 'SELECT x + 1;';
    /// CREATE FUNCTION greet(name TEXT) RETURNS TEXT AS 'SELECT "Hello, " || name;';
    /// CREATE FUNCTION do_nothing() AS 'SELECT;';
    /// "#,
    /// )?;
    /// let add_one_fn = db.function("add_one").expect("Function should exist");
    /// let greet_fn = db.function("greet").expect("Function should exist");
    /// let do_nothing_fn = db.function("do_nothing").expect("Function should exist");
    /// assert_eq!(do_nothing_fn.return_type_name(&db), None);
    /// assert_eq!(add_one_fn.return_type_name(&db).as_deref(), Some("INT"));
    /// assert_eq!(greet_fn.return_type_name(&db).as_deref(), Some("TEXT"));
    /// # Ok(())
    /// # }
    /// ```
    fn return_type_name<'db>(&'db self, database: &'db Self::DB) -> Option<&'db str>;

    /// Returns the normalized return type name of the function as a string.
    ///
    /// # Example
    ///
    /// ```rust
    /// #  fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// use sql_traits::prelude::*;
    ///
    /// let db = ParserDB::try_from(
    ///     r#"
    /// CREATE FUNCTION add_one(x INT) RETURNS INTEGER AS 'SELECT x + 1;';
    /// CREATE FUNCTION greet(name TEXT) RETURNS TEXT AS 'SELECT "Hello, " || name;';
    /// CREATE FUNCTION do_nothing() AS 'SELECT;';
    /// "#,
    /// )?;
    /// let add_one_fn = db.function("add_one").expect("Function should exist");
    /// let greet_fn = db.function("greet").expect("Function should exist");
    /// let do_nothing_fn = db.function("do_nothing").expect("Function should exist");
    /// assert_eq!(do_nothing_fn.normalized_return_type_name(&db), None);
    /// assert_eq!(add_one_fn.normalized_return_type_name(&db).as_deref(), Some("INT"));
    /// assert_eq!(greet_fn.normalized_return_type_name(&db).as_deref(), Some("TEXT"));
    /// # Ok(())
    /// # }
    /// ```
    #[inline]
    fn normalized_return_type_name<'db>(&'db self, database: &'db Self::DB) -> Option<&'db str> {
        self.return_type_name(database).map(normalize_postgres_type)
    }
}
