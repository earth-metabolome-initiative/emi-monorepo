//! Submodule defining and implementing the `ColumnSyn` trait, which provide
//! methods to facilitate the rust code generation starting from a SQL column
//! representation, building on top of the
//! [`ColumnLike`](sql_traits::traits::ColumnLike) trait.

use sql_traits::traits::ColumnLike;
use syn::{Ident, Type};

use crate::{
    structs::{ExternalTypeRef, Workspace},
    utils::{camel_case_name, is_reserved_rust_word, snake_case_name},
};

/// Trait implemented by types that represent SQL columns and can be used to
/// generate Rust code for them.
pub trait ColumnSynLike: ColumnLike {
    /// Returns the uppercased acronym of this column.
    /// 
    /// # Example
    /// 
    /// ```rust
    /// #  fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// use synql_core::prelude::*;
    /// let db = ParserDB::try_from("CREATE TABLE my_table (my_column INT);")?;
    /// let table = db.table(None, "my_table").unwrap();
    /// let column = table.column("my_column", &db).unwrap();
    /// assert_eq!(column.column_acronym(), "MC");
    /// # Ok(())
    /// # }
    /// ```
    fn column_acronym(&self) -> String {
        let snake_name = self.column_snake_name();
        snake_name
            .split('_')
            .filter_map(|part| part.chars().next())
            .collect::<String>()
            .to_uppercase()
    }

    /// Returns the uppercased acronym ident of this column.
    fn column_acronym_ident(&self) -> Ident {
        Ident::new(&self.column_acronym(), proc_macro2::Span::call_site())
    }

    /// Returns the snake-cased name of this column.
    ///
    /// # Example
    ///
    /// ```rust
    /// #  fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// use synql_core::prelude::*;
    ///
    /// let db = ParserDB::try_from("CREATE TABLE my_table (_my__column INT);")?;
    /// let table = db.table(None, "my_table").unwrap();
    /// let column = table.column("_my__column", &db).unwrap();
    /// assert_eq!(column.column_snake_name(), "my_column");
    /// # Ok(())
    /// # }
    /// ```
    fn column_snake_name(&self) -> String {
        snake_case_name(self.column_name())
    }

    /// Returns whether the column name is snake case.
    ///
    /// # Example
    ///
    /// ```rust
    /// #  fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// use synql_core::prelude::*;
    /// let db = ParserDB::try_from("CREATE TABLE my_table (my_column INT, MyColumn INT);")?;
    /// let table = db.table(None, "my_table").unwrap();
    /// let column1 = table.column("my_column", &db).unwrap();
    /// let column2 = table.column("MyColumn", &db).unwrap();
    /// assert!(column1.has_snake_case_column_name());
    /// assert!(!column2.has_snake_case_column_name());
    /// # Ok(())
    /// # }
    /// ```
    fn has_snake_case_column_name(&self) -> bool {
        self.column_snake_name() == self.column_name()
    }

    /// Returns the snake-cased ident of this column.
    ///
    /// # Example
    ///
    /// ```rust
    /// #  fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// use synql_core::prelude::*;
    ///
    /// let db = ParserDB::try_from("CREATE TABLE my_table (type INT);")?;
    /// let table = db.table(None, "my_table").unwrap();
    /// let column = table.column("type", &db).unwrap();
    /// assert_eq!(column.column_snake_ident().to_string(), "r#type");
    /// # Ok(())
    /// # }
    /// ```
    fn column_snake_ident(&self) -> Ident {
        let snake_name = self.column_snake_name();
        if is_reserved_rust_word(&snake_name) {
            Ident::new_raw(&snake_name, proc_macro2::Span::call_site())
        } else {
            Ident::new(&snake_name, proc_macro2::Span::call_site())
        }
    }

    /// Returns the camel-cased name of this column.
    ///
    /// # Example
    ///
    /// ```rust
    /// #  fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// use synql_core::prelude::*;
    ///
    /// let db = ParserDB::try_from("CREATE TABLE my_table (my_column INT);")?;
    /// let table = db.table(None, "my_table").unwrap();
    /// let column = table.column("my_column", &db).unwrap();
    /// assert_eq!(column.column_camel_name(), "MyColumn");
    /// # Ok(())
    /// # }
    /// ```
    fn column_camel_name(&self) -> String {
        camel_case_name(self.column_name())
    }

    /// Returns the camel-cased ident of this column.
    ///
    /// # Example
    ///
    /// ```rust
    /// #  fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// use synql_core::prelude::*;
    /// let db = ParserDB::try_from("CREATE TABLE my_table (struct INT);")?;
    /// let table = db.table(None, "my_table").unwrap();
    /// let column = table.column("struct", &db).unwrap();
    /// assert_eq!(column.column_camel_ident().to_string(), "Struct");
    /// # Ok(())
    /// # }
    /// ```
    fn column_camel_ident(&self) -> Ident {
        Ident::new(&self.column_camel_name(), proc_macro2::Span::call_site())
    }

    /// Returns the type ref curresponding to the postgres type of this column.
    ///
    /// # Arguments
    ///
    /// * `workspace` - The workspace where the column is defined.
    fn external_postgres_type<'workspace, 'data>(
        &self,
        workspace: &'workspace Workspace<'data>,
        database: &Self::DB,
    ) -> Option<ExternalTypeRef<'data>> {
        workspace.external_postgres_type(&self.normalized_data_type(database))
    }

    /// Returns the Diesel type of this column.
    fn diesel_type<'workspace, 'data>(
        &self,
        workspace: &'workspace Workspace<'data>,
        database: &Self::DB,
    ) -> Option<Type> {
        let external_type = self.external_postgres_type(workspace, database)?;
        let diesel_type = external_type.diesel_type()?;
        if self.is_nullable(database) {
            Some(syn::parse_quote!(diesel::sql_types::Nullable<#diesel_type>))
        } else {
            Some(diesel_type.clone())
        }
    }

    /// Returns the Rust type of this column.
    fn rust_type<'workspace, 'data>(
        &self,
        workspace: &'workspace Workspace<'data>,
        database: &Self::DB,
    ) -> Option<Type> {
        let external_type = self.external_postgres_type(workspace, database)?;
        let rust_type = external_type.rust_type();
        if self.is_nullable(database) {
            Some(syn::parse_quote!(Option<#rust_type>))
        } else {
            Some(rust_type.clone())
        }
    }

    /// Returns whether the column type supports the `Copy` trait in Rust.
    ///
    /// # Arguments
    ///
    /// * `database` - The database connection to use to query the column type.
    /// * `workspace` - The workspace where the column is defined.
    fn supports_copy<'workspace, 'data>(
        &self,
        database: &Self::DB,
        workspace: &'workspace Workspace<'data>,
    ) -> bool {
        match self.external_postgres_type(workspace, database) {
            Some(external_type) => external_type.supports_copy(),
            None => return false,
        }
    }
}

impl<T: ColumnLike> ColumnSynLike for T {}
