//! Submodule defining and implementing the `TableSynLike` trait, which provides
//! methods to facilitate the rust code generation starting from a SQL table
//! representation, building on top of the
//! [`TableLike`](sql_traits::traits::TableLike) trait and the traits from the
//! [`sql_relations`](sql_relations) crate.

use sql_traits::traits::{DatabaseLike, TableLike};
use syn::Ident;

use crate::{
    traits::{ColumnSynLike, ForeignKeySynLike},
    utils::{
        camel_case_name, is_reserved_rust_word, singular_camel_case_name, singular_snake_name,
        snake_case_name,
    },
};

/// Trait implemented by types that represent SQL tables and can be used to
/// generate Rust code for them.
pub trait TableSynLike: TableLike
where
    <Self::DB as DatabaseLike>::Column: ColumnSynLike,
    <Self::DB as DatabaseLike>::ForeignKey: ForeignKeySynLike,
{
    /// Returns the snake-cased name of this table.
    ///
    /// # Example
    ///
    /// ```rust
    /// #  fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// use synql_core::prelude::*;
    ///
    /// let db = ParserDB::try_from("CREATE TABLE _my__table (id INT);")?;
    /// let table = db.table(None, "_my__table").unwrap();
    /// assert_eq!(table.table_snake_name(), "my_table");
    /// # Ok(())
    /// # }
    /// ```
    fn table_snake_name(&self) -> String {
        snake_case_name(self.table_name())
    }

    /// Returns the singular snake-cased name of this table.
    ///
    /// # Example
    ///
    /// ```rust
    /// #  fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// use synql_core::prelude::*;
    ///
    /// let db = ParserDB::try_from("CREATE TABLE users (id INT);")?;
    /// let table = db.table(None, "users").unwrap();
    /// assert_eq!(table.table_singular_snake_name(), "user");
    /// # Ok(())
    /// # }
    /// ```
    fn table_singular_snake_name(&self) -> String {
        singular_snake_name(self.table_name())
    }

    /// Returns whether the table name is snake case.
    ///
    /// # Example
    ///
    /// ```rust
    /// #  fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// use synql_core::prelude::*;
    /// let db = ParserDB::try_from(
    ///     r#"
    /// 	CREATE TABLE my_table (id INT);
    /// 	CREATE TABLE MyTable (id INT);
    /// "#,
    /// )?;
    /// let table1 = db.table(None, "my_table").unwrap();
    /// let table2 = db.table(None, "MyTable").unwrap();
    /// assert!(table1.has_snake_case_table_name());
    /// assert!(!table2.has_snake_case_table_name());
    /// # Ok(())
    /// # }
    /// ```
    fn has_snake_case_table_name(&self) -> bool {
        self.table_snake_name() == self.table_name()
    }

    /// Returns the snake-cased ident of this table.
    ///
    /// # Example
    ///
    /// ```rust
    /// #  fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// use synql_core::prelude::*;
    ///
    /// let db = ParserDB::try_from("CREATE TABLE box (id INT);")?;
    /// let table = db.table(None, "box").unwrap();
    /// assert_eq!(table.table_snake_ident().to_string(), "r#box");
    /// # Ok(())
    /// # }
    /// ```
    fn table_snake_ident(&self) -> Ident {
        let snake_name = self.table_snake_name();
        if is_reserved_rust_word(&snake_name) {
            Ident::new_raw(&snake_name, proc_macro2::Span::call_site())
        } else {
            Ident::new(&snake_name, proc_macro2::Span::call_site())
        }
    }

    /// Returns the singular snake-cased ident of this table.
    ///
    /// # Example
    ///
    /// ```rust
    /// #  fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// use synql_core::prelude::*;
    ///
    /// let db = ParserDB::try_from("CREATE TABLE users (id INT);")?;
    /// let table = db.table(None, "users").unwrap();
    /// assert_eq!(table.table_singular_snake_ident().to_string(), "user");
    /// # Ok(())
    /// # }
    /// ```
    fn table_singular_snake_ident(&self) -> Ident {
        Ident::new(&self.table_singular_snake_name(), proc_macro2::Span::call_site())
    }

    /// Returns the camel-cased name of this table.
    ///
    /// # Example
    ///
    /// ```rust
    /// #  fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// use synql_core::prelude::*;
    ///
    /// let db = ParserDB::try_from("CREATE TABLE my_table (id INT);")?;
    /// let table = db.table(None, "my_table").unwrap();
    /// assert_eq!(table.table_camel_name(), "MyTable");
    /// # Ok(())
    /// # }
    /// ```
    fn table_camel_name(&self) -> String {
        camel_case_name(self.table_name())
    }

    /// Returns the singular camel-cased name of this table.
    ///
    /// # Example
    ///
    /// ```rust
    /// #  fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// use synql_core::prelude::*;
    ///
    /// let db = ParserDB::try_from("CREATE TABLE users (id INT);")?;
    /// let table = db.table(None, "users").unwrap();
    /// assert_eq!(table.table_singular_camel_name(), "User");
    /// # Ok(())
    /// # }
    /// ```
    fn table_singular_camel_name(&self) -> String {
        singular_camel_case_name(self.table_name())
    }

    /// Returns the camel-cased ident of this table.
    ///
    /// # Example
    ///
    /// ```rust
    /// #  fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// use synql_core::prelude::*;
    /// let db = ParserDB::try_from("CREATE TABLE struct (id INT);")?;
    /// let table = db.table(None, "struct").unwrap();
    /// assert_eq!(table.table_camel_ident().to_string(), "Struct");
    /// # Ok(())
    /// # }
    /// ```
    fn table_camel_ident(&self) -> Ident {
        let camel_name = self.table_camel_name();
        if is_reserved_rust_word(&camel_name) {
            Ident::new_raw(&camel_name, proc_macro2::Span::call_site())
        } else {
            Ident::new(&camel_name, proc_macro2::Span::call_site())
        }
    }

    /// Returns the singular camel-cased ident of this table.
    ///
    /// # Example
    ///
    /// ```rust
    /// #  fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// use synql_core::prelude::*;
    /// let db = ParserDB::try_from("CREATE TABLE users (id INT);")?;
    /// let table = db.table(None, "users").unwrap();
    /// assert_eq!(table.table_singular_camel_ident().to_string(), "User");
    /// # Ok(())
    /// # }
    /// ```
    fn table_singular_camel_ident(&self) -> Ident {
        let camel_name = self.table_singular_camel_name();
        if is_reserved_rust_word(&camel_name) {
            Ident::new_raw(&camel_name, proc_macro2::Span::call_site())
        } else {
            Ident::new(&camel_name, proc_macro2::Span::call_site())
        }
    }

    /// Iterates over the identifiers of the primary key columns of this table.
    fn primary_key_idents<'db>(
        &'db self,
        database: &'db Self::DB,
    ) -> impl Iterator<Item = Ident> + 'db {
        self.primary_key_columns(database).map(move |col| col.column_snake_ident())
    }
}

impl<T: TableLike> TableSynLike for T
where
    <T::DB as DatabaseLike>::Column: ColumnSynLike,
    <T::DB as DatabaseLike>::ForeignKey: ForeignKeySynLike,
{
}
