//! Submodule defining and implementing the `ColumnSyn` trait, which provide
//! methods to facilitate the rust code generation starting from a SQL column
//! representation, building on top of the
//! [`ColumnLike`](sql_traits::traits::ColumnLike) trait.

use std::borrow::Borrow;

use quote::quote;
use sql_relations::traits::{
    HorizontalSameAsColumnLike, TriangularSameAsColumnLike, TriangularSameAsForeignKeyLike,
    VerticalSameAsColumnLike,
};
use sql_traits::traits::{ColumnLike, ForeignKeyLike, TableLike};
use syn::{Ident, Type};

use crate::{
    structs::{ExternalTypeRef, Workspace},
    traits::TableSynLike,
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
    /// use sql_traits::prelude::*;
    /// use synql::prelude::*;
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
    fn column_acronym_generic(&self) -> syn::GenericParam {
        syn::GenericParam::Type(syn::TypeParam::from(Ident::new(
            &self.column_acronym(),
            proc_macro2::Span::call_site(),
        )))
    }

    /// Returns the snake-cased name of this column.
    ///
    /// # Example
    ///
    /// ```rust
    /// #  fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// use sql_traits::prelude::*;
    /// use synql::prelude::*;
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
    /// use sql_traits::prelude::*;
    /// use synql::prelude::*;
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
    /// use sql_traits::prelude::*;
    /// use synql::prelude::*;
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
    /// use sql_traits::prelude::*;
    /// use synql::prelude::*;
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
    /// use sql_traits::prelude::*;
    /// use synql::prelude::*;
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
    fn external_postgres_type<'workspace>(
        &self,
        workspace: &'workspace Workspace,
        database: &Self::DB,
    ) -> Option<ExternalTypeRef<'workspace>> {
        workspace.external_postgres_type(self.normalized_data_type(database))
    }

    /// Returns the Diesel type of this column.
    fn diesel_type(&self, workspace: &Workspace, database: &Self::DB) -> Option<Type> {
        let external_type = self.external_postgres_type(workspace, database)?;
        let diesel_type = external_type.diesel_type();
        if self.is_nullable(database) {
            Some(syn::parse_quote!(diesel::sql_types::Nullable<#diesel_type>))
        } else {
            Some(diesel_type.clone())
        }
    }

    /// Returns the Rust type of this column.
    fn rust_type(&self, workspace: &Workspace, database: &Self::DB) -> Option<Type> {
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
    fn supports_copy(&self, database: &Self::DB, workspace: &Workspace) -> bool {
        match self.external_postgres_type(workspace, database) {
            Some(external_type) => external_type.supports_copy(),
            None => false,
        }
    }

    /// Returns whether the column type supports the given core trait in Rust.
    ///
    /// # Arguments
    ///
    /// * `core_trait` - The core trait to check support for.
    /// * `database` - The database connection to use to query the column type.
    fn supports(
        &self,
        core_trait: crate::structs::Trait,
        workspace: &Workspace,
        database: &Self::DB,
    ) -> bool {
        match self.external_postgres_type(workspace, database) {
            Some(external_type) => external_type.supports_trait(core_trait),
            None => false,
        }
    }

    /// Generates the struct field tokens for this column.
    fn generate_struct_field(
        &self,
        workspace: &Workspace,
        database: &Self::DB,
    ) -> Result<proc_macro2::TokenStream, crate::Error> {
        let column_ident = self.column_snake_ident();
        let external_postgres_type =
            self.external_postgres_type(workspace, database).ok_or_else(|| {
                crate::Error::ColumnTypeNotFound {
                    table_name: self.table(database).table_name().to_string(),
                    column_name: self.column_name().to_string(),
                    sql_type: self.data_type(database).to_string(),
                }
            })?;
        let documentation = self.column_doc(database).map(|doc| {
            quote! {
                #[doc = #doc]
            }
        });
        let rust_type = external_postgres_type.rust_type();
        let diesel_type = external_postgres_type.diesel_type();
        let mut sql_type_decorator = None;
        if !["std", "core"].contains(&external_postgres_type.crate_name()) {
            sql_type_decorator = Some(quote! {
                #[diesel(sql_type = #diesel_type)]
            });
        }

        // If the column has vertical same-as constraint, we add the
        // ` #[same_as(parent::parent_column)]` decorators
        let mut vertical_same_as_decorators = vec![];
        for same_as in self.dominant_vertical_same_as_columns(database) {
            let parent_table = same_as.table(database);
            let parent_table_ident = parent_table.table_snake_ident();
            let parent_column_ident = same_as.column_snake_ident();
            let parent_table_crate = parent_table.crate_ident(workspace);
            vertical_same_as_decorators.push(quote! {
                #[same_as(#parent_table_crate::#parent_table_ident::#parent_column_ident)]
            });
        }

        // If the column has horizontal same-as constraint, we add the
        // ` #[same_as(satellite::table::column, key)]` decorators
        let mut horizontal_same_as_decorators = vec![];
        for same_as in self.horizontal_same_as_foreign_keys(database) {
            let host_columns =
                same_as.host_columns(database).map(Borrow::borrow).collect::<Vec<&Self>>();
            let referenced_columns =
                same_as.referenced_columns(database).map(Borrow::borrow).collect::<Vec<&Self>>();

            let [key, other] = host_columns.as_slice() else {
                unimplemented!(
                    "Only binary foreign keys are supported for horizontal same-as constraints, found a foreign key in table `{}` to table `{}` with `{}` columns: {}",
                    same_as.host_table(database).table_name(),
                    same_as.referenced_table(database).table_name(),
                    host_columns.len(),
                    host_columns.iter().map(|col| col.column_name()).collect::<Vec<_>>().join(", ")
                );
            };
            let [_foreign_pk, referenced_column] = referenced_columns.as_slice() else {
                unreachable!();
            };

            if !same_as.is_horizontal_same_as_of_triangular(database) {
                continue;
            }

            if key == &self {
                // The key is handled separately in the discretionary/mandatory
                // decorators
                continue;
            }
            if other != &self {
                unreachable!(
                    "The column must be either the key or the other column in the foreign key"
                );
            }
            let key_ident = key.column_snake_ident();
            let satellite_table = same_as.referenced_table(database);
            let satellite_table_ident = satellite_table.table_snake_ident();
            let satellite_table_crate = satellite_table.crate_ident(workspace);
            let referenced_column_ident = referenced_column.column_snake_ident();

            horizontal_same_as_decorators.push(quote! {
                #[same_as(#satellite_table_crate::#satellite_table_ident::#referenced_column_ident, #key_ident)]
            });
        }

        // If the handle has foreign keys which define discretionary/mandatory
        // same-as constraints, we add the corresponding decorators
        let mut triangular_same_as_decorators = vec![];
        for foreign_key in self.triangular_same_as_foreign_keys(database) {
            let Some(triangular) = foreign_key.triangular_same_as(database) else {
                continue;
            };

            let referenced_table = foreign_key.referenced_table(database);
            let referenced_table_ident = referenced_table.table_snake_ident();
            let referenced_table_crate = referenced_table.crate_ident(workspace);
            let referenced_table_path = quote! { #referenced_table_crate::#referenced_table_ident };

            triangular_same_as_decorators.push(if triangular.is_mandatory() {
                quote! {
                    #[mandatory(#referenced_table_path)]
                }
            } else {
                quote! {
                    #[discretionary(#referenced_table_path)]
                }
            });
        }

        Ok(quote! {
            #documentation
            #(#vertical_same_as_decorators)*
            #(#horizontal_same_as_decorators)*
            #(#triangular_same_as_decorators)*
            #sql_type_decorator
            #column_ident: #rust_type
        })
    }
}

impl<T: ColumnLike> ColumnSynLike for T {}
