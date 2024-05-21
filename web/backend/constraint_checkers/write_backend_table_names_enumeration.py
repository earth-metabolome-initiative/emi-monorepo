"""Write the table enumeration in the backend crate."""

from typing import List
import os
from constraint_checkers.table_metadata import TableStructMetadata
from constraint_checkers.indices import PGIndices

def write_backend_table_names_enumeration(
    tables: List[TableStructMetadata],
):
    """Write the table names enumeration in the backend crate."""
    # We check that we are currently executing in the `backend` crate
    # so to make sure that the relative path to the `web_common` crate
    # is correct.
    if not os.getcwd().endswith("backend"):
        raise RuntimeError("This script must be executed in the `backend` crate.")

    path = "src/table_enumeration.rs"

    # Since the table enumeration is defined in the web_common crate, we cannot
    # directly implement methods in the backend on the Table enum. Instead, we
    # define traits in the backend that are implemented by the Table enum in the
    # backend.

    document = open(path, "w", encoding="utf8")

    # Preliminarly, we write a docstring at the very head
    # of this submodule to explain what it does and warn the
    # reader not to write anything in this file as it is
    # automatically generated.

    document.write(
        "//! This module contains the table names enumeration.\n"
        "//!\n"
        "//! This module is automatically generated. Do not write anything here.\n\n"
    )

    # We start with the necessary imports.
    imports = [
        "use crate::models::*;",
        "use crate::nested_models::*;",
        # "use crate::views::*;",
        "use diesel::r2d2::PooledConnection;",
        "use diesel::r2d2::ConnectionManager;",
        "use crate::new_variants::InsertRow;",
        "use crate::update_variants::UpdateRow;",
    ]

    document.write("\n".join(imports) + "\n\n")

    # We start with the first trait, the SearchableTable trait, which provides
    # a search method receiving a &str query and a number of rows to return (i32).
    # The method returns a Result, where the Ok variant is a bincode-serialized
    # vector of the rows of the i-th table variant, while the Err version contains
    # and ApiError. The search method is not available for all tables, but only
    # those that have a similarity index. For the ones that do not have a similarity
    # index, we panic with an unimplemented!() macro.

    document.write(
        "/// Trait providing the search method for the Table enum.\n"
        "pub trait SearchableTable {\n"
    )
    for similarity_method, _, _ in PGIndices.SIMILARITY_METHODS:
        document.write(
            f"    /// Search the table by the query using the {similarity_method} method from PostgreSQL.\n"
            "    ///\n"
            "    /// # Arguments\n"
            "    /// * `query` - The string to search for.\n"
            "    /// * `limit` - The maximum number of results, by default `10`.\n"
            "    /// * `connection` - The database connection.\n"
            "    ///\n"
            "    /// # Returns\n"
            "    /// A serialized vector of the rows of the table, using bincode.\n"
            f"    fn {similarity_method}_search(\n"
            "         &self,\n"
            "         query: &str,\n"
            "         limit: Option<i32>,\n"
            "         connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>\n"
            ") -> Result<Vec<Vec<u8>>, web_common::api::ApiError>;\n\n"
        )

        document.write(
            f"    /// Search editables rows by the query using the {similarity_method} method from PostgreSQL.\n"
            "    ///\n"
            "    /// # Arguments\n"
            "    /// * `user_id` - The user ID of the user performing the operation.\n"
            "    /// * `query` - The string to search for.\n"
            "    /// * `limit` - The maximum number of results, by default `10`.\n"
            "    /// * `connection` - The database connection.\n"
            "    ///\n"
            "    /// # Returns\n"
            "    /// A serialized vector of the rows of the table, using bincode.\n"
            f"    fn {similarity_method}_search_editables(\n"
            "         &self,\n"
            "         user_id: i32,\n"
            "         query: &str,\n"
            "         limit: Option<i32>,\n"
            "         connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>\n"
            ") -> Result<Vec<Vec<u8>>, web_common::api::ApiError>;\n\n"
        )

    document.write("}\n\n")

    document.write("impl SearchableTable for web_common::database::Table {\n")
    for similarity_method, _, _ in PGIndices.SIMILARITY_METHODS:
        document.write(
            f"    fn {similarity_method}_search(&self, query: &str, limit: Option<i32>, connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>) -> Result<Vec<Vec<u8>>, web_common::api::ApiError> {{\n"
            "        match self {\n"
        )

        for table in tables:
            if table.is_searchable():
                document.write(
                    f"            web_common::database::Table::{table.camel_cased()} => {table.richest_struct_name()}::{similarity_method}_search(query, limit, connection)?.iter().map(|row| bincode::serialize(row).map_err(web_common::api::ApiError::from)).collect(),\n"
                )
            else:
                document.write(
                    f'            web_common::database::Table::{table.camel_cased()} => unimplemented!("Table `{table.name}` does not have a GIN similarity index."),\n'
                )

        document.write("        }\n    }\n")

        document.write(
            f"    fn {similarity_method}_search_editables(&self, user_id: i32, query: &str, limit: Option<i32>, connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>) -> Result<Vec<Vec<u8>>, web_common::api::ApiError> {{\n"
            "        match self {\n"
        )

        for table in tables:
            if table.is_searchable():
                if table.has_associated_roles() and table.name != "users":
                    document.write(
                        f"            web_common::database::Table::{table.camel_cased()} => {table.richest_struct_name()}::{similarity_method}_search_editables(user_id, query, limit, connection)?.iter().map(|row| bincode::serialize(row).map_err(web_common::api::ApiError::from)).collect(),\n"
                    )
                else:
                    document.write(
                        f'            web_common::database::Table::{table.camel_cased()} => unimplemented!("Table `{table.name}` does not have associated roles."),\n'
                    )
            else:
                document.write(
                    f'            web_common::database::Table::{table.camel_cased()} => unimplemented!("Table `{table.name}` does not have a GIN similarity index."),\n'
                )

        document.write("        }\n    }\n")
    document.write("}\n")

    # Next, we implement the `get` method for the Table enum, which receives a primary key
    # enum and a connection to the database. The method returns a Result, where the Ok variant
    # is a bincode-serialized version of the row of the table variant, while the Err variant
    # contains an ApiError. The get method is available for all tables with a primary key -
    # tables that do not have a primary key will raise a panic with the unimplemented!() macro.

    document.write(
        "/// Trait providing the get method for the Table enum.\n"
        "pub trait IdentifiableTable {\n"
        "    /// Get the row from the table by the primary key.\n"
        "    ///\n"
        "    /// # Arguments\n"
        "    /// * `primary_key` - The primary key of the row.\n"
        "    /// * `connection` - The database connection.\n"
        "    ///\n"
        "    /// # Returns\n"
        "    /// A serialized version of the row of the table, using bincode.\n"
        "    fn get(\n"
        "         &self,\n"
        "         primary_key: web_common::database::operations::PrimaryKey,\n"
        "         connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>\n"
        ") -> Result<Vec<u8>, web_common::api::ApiError>;\n"
        "}\n\n"
    )

    document.write(
        "impl IdentifiableTable for web_common::database::Table {\n\n"
        "    fn get(\n"
        "        &self,\n"
        "        primary_key: web_common::database::operations::PrimaryKey,\n"
        "        connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>\n"
        "    ) -> Result<Vec<u8>, web_common::api::ApiError> {\n"
        "        Ok(match self {\n"
    )

    for table in tables:
        document.write(
            f"            web_common::database::Table::{table.camel_cased()} => bincode::serialize(&{table.richest_struct_name()}::get(primary_key.into(), connection)?)?,\n"
        )

    document.write("        })\n    }\n}\n")

    # We implement the `can_view` method for the Table enum, which receives a primary key
    # enum and a user ID. The method returns a Result, where the Ok variant is a boolean
    # indicating whether the user can view the row, while the Err variant contains an ApiError.
    # For all tables that do not have a `public` column, we always return true.
    # For the entries that do have a `public` column, we return true if the column is true, or
    # if the provided user is a valid viewer of that entry as determined by the associated roles
    # method from the flat struct which is called by the `is_viewer_by_id` method.

    document.write(
        "/// Trait providing the can_view method for the Table enum.\n"
        "pub trait ViewableTable {\n"
        "    /// Check whether the user can view the row.\n"
        "    ///\n"
        "    /// # Arguments\n"
        "    /// * `primary_key` - The primary key of the row.\n"
        "    /// * `user_id` - The user ID of the user performing the operation.\n"
        "    /// * `connection` - The database connection.\n"
        "    ///\n"
        "    /// # Returns\n"
        "    /// A boolean indicating whether the user can view the row.\n"
        "    fn can_view(\n"
        "         &self,\n"
        "         primary_key: web_common::database::operations::PrimaryKey,\n"
        "         user_id: Option<i32>,\n"
        "         connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>\n"
        ") -> Result<bool, web_common::api::ApiError>;\n"
        "}\n\n"
    )

    document.write(
        "impl ViewableTable for web_common::database::Table {\n\n"
        "    fn can_view(\n"
        "        &self,\n"
        "        primary_key: web_common::database::operations::PrimaryKey,\n"
        "        user_id: Option<i32>,\n"
        "        connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>\n"
        "    ) -> Result<bool, web_common::api::ApiError> {\n"
        "        Ok(match self {\n"
    )

    for table in tables:
        if table.has_public_column():
            document.write(
                f"            web_common::database::Table::{table.camel_cased()} => {{\n"
                f"                {table.flat_variant_name()}::get(primary_key.into(), connection)?.public ||\n"
                f"                user_id.map_or(Ok(false), |user_id| {table.flat_variant_name()}::is_viewer_by_id(primary_key.into(), user_id, connection))?\n"
                "            },\n"
            )
        else:
            document.write(
                f"            web_common::database::Table::{table.camel_cased()} => true,\n"
            )

    document.write("        })\n    }\n}\n")

    # We implement the can_update method for the Table enum, which receives a primary key
    # enum and a user ID. The method returns a Result, where the Ok variant is a boolean
    # indicating whether the user can edit the row, while the Err variant contains an ApiError.
    # For the tables that do not have an associated roles, we always return false.
    # For the users table specifically, we return true if the user ID is the same as the
    # primary key, and false otherwise. For the other tables, we call the `is_editor_by_id` method
    # from the flat struct, which returns true if the user is an editor of the entry.

    document.write(
        "/// Trait providing the can_update method for the Table enum.\n"
        "pub trait EditableTable {\n"
        "    /// Check whether the user can edit the row.\n"
        "    ///\n"
        "    /// # Arguments\n"
        "    /// * `primary_key` - The primary key of the row.\n"
        "    /// * `user_id` - The user ID of the user performing the operation.\n"
        "    /// * `connection` - The database connection.\n"
        "    ///\n"
        "    /// # Returns\n"
        "    /// A boolean indicating whether the user can edit the row.\n"
        "    fn can_update(\n"
        "         &self,\n"
        "         primary_key: web_common::database::operations::PrimaryKey,\n"
        "         user_id: i32,\n"
        "         connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>\n"
        ") -> Result<bool, web_common::api::ApiError>;\n"
        "}\n\n"
    )

    document.write(
        "impl EditableTable for web_common::database::Table {\n\n"
        "    fn can_update(\n"
        "        &self,\n"
        "        primary_key: web_common::database::operations::PrimaryKey,\n"
        "        user_id: i32,\n"
        "        connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>\n"
        "    ) -> Result<bool, web_common::api::ApiError> {\n"
        "        Ok(match self {\n"
    )

    for table in tables:
        if table.has_associated_roles() and table.name != "users":
            document.write(
                f"            web_common::database::Table::{table.camel_cased()} => {table.flat_variant_name()}::is_editor_by_id(primary_key.into(), user_id, connection)?,\n"
            )
        elif table.name == "users":
            document.write(
                f"            web_common::database::Table::{table.camel_cased()} => {{\n"
                "                let primary_key: i32 = primary_key.into();\n"
                "                primary_key == user_id\n"
                "            },\n"
            )
        else:
            document.write(
                f"            web_common::database::Table::{table.camel_cased()} => false,\n"
            )

    document.write("        })\n    }\n}\n")

    # We implement the can_delete method for the Table enum, which receives a primary key
    # enum and a user ID. The method returns a Result, where the Ok variant is a boolean
    # indicating whether the user can delete the row, while the Err variant contains an ApiError.
    # For the tables that do not have an associated roles, we always return false.
    # For the users table specifically, we always return false. For the other tables, we call
    # the `is_admin_by_id` method from the flat struct, which returns true if the user is an admin of
    # the entry.

    # We also implement the `delete` method for the Table enum, which receives a primary key
    # enum and a connection to the database. The method returns a Result, where the Ok variant
    # is the number of rows deleted, while the Err variant contains an ApiError. The delete
    # method is available for all tables with a primary key - tables that do not have a primary
    # key will raise a panic with the unimplemented!() macro.

    document.write(
        "/// Trait providing the can_delete method for the Table enum.\n"
        "pub trait DeletableTable {\n"
        "    /// Check whether the user can delete the row.\n"
        "    ///\n"
        "    /// # Arguments\n"
        "    /// * `primary_key` - The primary key of the row.\n"
        "    /// * `user_id` - The user ID of the user performing the operation.\n"
        "    /// * `connection` - The database connection.\n"
        "    ///\n"
        "    /// # Returns\n"
        "    /// A boolean indicating whether the user can delete the row.\n"
        "    fn can_delete(\n"
        "         &self,\n"
        "         primary_key: web_common::database::operations::PrimaryKey,\n"
        "         user_id: i32,\n"
        "         connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>\n"
        ") -> Result<bool, web_common::api::ApiError>;\n\n"
        "    /// Delete the row from the table by the primary key.\n"
        "    ///\n"
        "    /// # Arguments\n"
        "    /// * `primary_key` - The primary key of the row.\n"
        "    /// * `author_user_id` - The user ID of the user performing the operation.\n"
        "    /// * `connection` - The database connection.\n"
        "    ///\n"
        "    /// # Returns\n"
        "    /// The number of rows deleted.\n"
        "    fn delete(\n"
        "         &self,\n"
        "         primary_key: web_common::database::operations::PrimaryKey,\n"
        "         author_user_id: i32,\n"
        "         connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>\n"
        ") -> Result<usize, web_common::api::ApiError>;\n"
        "}\n\n"
    )

    document.write(
        "impl DeletableTable for web_common::database::Table {\n\n"
        "    fn can_delete(\n"
        "        &self,\n"
        "        primary_key: web_common::database::operations::PrimaryKey,\n"
        "        user_id: i32,\n"
        "        connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>\n"
        "    ) -> Result<bool, web_common::api::ApiError> {\n"
        "        Ok(match self {\n"
    )

    for table in tables:
        if table.has_associated_roles() and table.name != "users":
            document.write(
                f"            web_common::database::Table::{table.camel_cased()} => {table.flat_variant_name()}::is_admin_by_id(primary_key.into(), user_id, connection)?,\n"
            )
        else:
            document.write(
                f"            web_common::database::Table::{table.camel_cased()} => false,\n"
            )

    document.write("        })\n    }\n")

    document.write(
        "    fn delete(\n"
        "        &self,\n"
        "        primary_key: web_common::database::operations::PrimaryKey,\n"
        "        author_user_id: i32,\n"
        "        connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>\n"
        "    ) -> Result<usize, web_common::api::ApiError> {\n"
        "        if !self.can_delete(primary_key, author_user_id, connection)? {\n"
        "            return Err(web_common::api::ApiError::unauthorized());\n"
        "        }\n"
        "        Ok(match self {\n"
    )

    for table in tables:

        if table.has_associated_roles() and table.name != "users":
            document.write(
                f"            web_common::database::Table::{table.camel_cased()} => {table.flat_variant_name()}::delete_by_id(primary_key.into(), author_user_id, connection)?,\n"
            )
        else:
            document.write(
                f'            web_common::database::Table::{table.camel_cased()} => unimplemented!("Delete not implemented for {table.name}."),\n'
            )

    document.write("        })\n")
    document.write("    }\n")
    document.write("}\n")

    # Next, we implement the all method for the Table enum, which receives a connection
    # to the database and returns a Result, where the Ok variant is a bincode-serialized
    # vector of the rows of the table variant, while the Err variant contains an ApiError.
    # The all method is available for all tables. It also receives a limit parameter, which
    # is the maximum number of rows to return, and an offset parameter, which is the number
    # of rows to skip.

    document.write(
        "/// Trait providing the all method for the Table enum.\n"
        "pub trait AllTable {\n"
        "    /// Get all the rows from the table.\n"
        "    ///\n"
        "    /// # Arguments\n"
        "    /// * `filter` - The filter to apply to the rows.\n"
        "    /// * `limit` - The maximum number of rows to return.\n"
        "    /// * `offset` - The number of rows to skip.\n"
        "    /// * `connection` - The database connection.\n"
        "    ///\n"
        "    /// # Returns\n"
        "    /// A vector of the rows of the table.\n"
        "    fn all(\n"
        "         &self,\n"
        "         filter: Option<Vec<u8>>,\n"
        "         limit: Option<i64>,\n"
        "         offset: Option<i64>,\n"
        "         connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>\n"
        ") -> Result<Vec<Vec<u8>>, web_common::api::ApiError>;\n"
        "}\n\n"
    )

    document.write(
        "impl AllTable for web_common::database::Table {\n\n"
        "    fn all(\n"
        "        &self,\n"
        "        filter: Option<Vec<u8>>,\n"
        "        limit: Option<i64>,\n"
        "        offset: Option<i64>,\n"
        "        connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>\n"
        "    ) -> Result<Vec<Vec<u8>>, web_common::api::ApiError> {\n"
        "        match self {\n"
    )

    for table in tables:
        document.write(
            f"            web_common::database::Table::{table.camel_cased()} => {{"
        )
        if table.has_filter_variant():
            filter_variant = table.get_filter_variant()
            document.write(
                f"let filter: Option<web_common::database::{filter_variant.name}> = filter.map(|filter| bincode::deserialize::<web_common::database::{filter_variant.name}>(&filter).map_err(web_common::api::ApiError::from)).transpose()?;\n"
                f"{table.richest_struct_name()}::all(filter.as_ref(), limit, offset, connection)?.iter().map(|row| bincode::serialize(row).map_err(web_common::api::ApiError::from)).collect()\n"
            )
        else:
            document.write(
                f'assert!(filter.is_none(), "Filter not implemented for {table.name}.");\n'
                f"{table.richest_struct_name()}::all(limit, offset, connection)?.iter().map(|row| bincode::serialize(row).map_err(web_common::api::ApiError::from)).collect()\n"
            )
        document.write("},\n")

    document.write("        }\n    }\n}\n")

    # We define a trait for the Table enum, which provides the all_by_updated_at method.
    # The method receives a connection to the database and returns a Result, where the Ok
    # variant is a bincode-serialized vector of the rows of the table variant, while the
    # Err variant contains an ApiError. The all_by_updated_at method is available for all
    # tables that have an updated_at column. For the tables that do not have an updated_at
    # column, we panic with an unimplemented!() macro.

    document.write(
        "/// Trait providing the all_by_updated_at method for the Table enum.\n"
        "pub trait AllByUpdatedAtTable {\n"
        "    /// Get all the rows from the table ordered by the `updated_at` column.\n"
        "    ///\n"
        "    /// # Arguments\n"
        "    /// * `filter` - The filter to apply to the rows.\n"
        "    /// * `limit` - The maximum number of rows to return.\n"
        "    /// * `offset` - The number of rows to skip.\n"
        "    /// * `connection` - The database connection.\n"
        "    ///\n"
        "    /// # Returns\n"
        "    /// A vector of the rows of the table.\n"
        "    fn all_by_updated_at(\n"
        "         &self,\n"
        "         filter: Option<Vec<u8>>,\n"
        "         limit: Option<i64>,\n"
        "         offset: Option<i64>,\n"
        "         connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>\n"
        ") -> Result<Vec<Vec<u8>>, web_common::api::ApiError>;\n"
        "}\n\n"
    )

    # Next, for all the tables that have an updated_at column, we implement the
    # `all_by_updated_at` method, which returns all of the rows ordered by the
    # `updated_at` column. When the table does not have an `updated_at` column,
    # we panic with an unimplemented!() macro.

    document.write(
        "impl AllByUpdatedAtTable for web_common::database::Table {\n\n"
        "    fn all_by_updated_at(\n"
        "        &self,\n"
        "        filter: Option<Vec<u8>>,\n"
        "        limit: Option<i64>,\n"
        "        offset: Option<i64>,\n"
        "        connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>\n"
        "    ) -> Result<Vec<Vec<u8>>, web_common::api::ApiError> {\n"
        "        match self {\n"
    )

    for table in tables:
        if table.has_updated_at_column():
            document.write(
                f"            web_common::database::Table::{table.camel_cased()} => {{"
            )
            if table.has_filter_variant():
                filter_variant = table.get_filter_variant()
                document.write(
                    f"let filter: Option<web_common::database::{filter_variant.name}> = filter.map(|filter| bincode::deserialize::<web_common::database::{filter_variant.name}>(&filter).map_err(web_common::api::ApiError::from)).transpose()?;\n"
                    f"{table.richest_struct_name()}::all_by_updated_at(filter.as_ref(), limit, offset, connection)?.iter().map(|row| bincode::serialize(row).map_err(web_common::api::ApiError::from)).collect()\n"
                )
            else:
                document.write(
                    f'assert!(filter.is_none(), "Filter not implemented for {table.name}.");\n'
                    f"{table.richest_struct_name()}::all_by_updated_at(limit, offset, connection)?.iter().map(|row| bincode::serialize(row).map_err(web_common::api::ApiError::from)).collect()\n"
                )
            document.write("},\n")
        else:
            document.write(
                f'            web_common::database::Table::{table.camel_cased()} => unimplemented!("all_by_updated_at not implemented for {table.name}."),\n'
            )

    document.write("        }\n    }\n}\n")

    # We create a method to insert new rows in the database. The method receives a bincode-serialized
    # row of the new flat variant of the table, the id of the user inserting the row, and a connection
    # to the database. The method returns a Result, where the Ok variant is the bincode-serialized
    # version of the richest struct of the table variant, associated with the newly inserted row, while
    # the Err variant contains an ApiError. The insert method is only available for a subset of the tables
    # as determined by the is_insertable method of the table. When the table is not insertable, we panic
    # with the unreachable!() macro, which explains that the table is not insertable as it does not have
    # a known column associated to a creator user id.

    document.write(
        "/// Trait providing the insert method for the Table enum.\n"
        "pub trait InsertableTable {\n"
        "    /// Insert a new row into the table.\n"
        "    ///\n"
        "    /// # Arguments\n"
        "    /// * `row` - The bincode-serialized row of the table.\n"
        "    /// * `user_id` - The id of the user inserting the row.\n"
        "    /// * `connection` - The database connection.\n"
        "    ///\n"
        "    /// # Returns\n"
        "    /// The bincode-serialized row of the table.\n"
        "    fn insert(\n"
        "         &self,\n"
        "         row: Vec<u8>,\n"
        "         user_id: i32,\n"
        "         connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>\n"
        ") -> Result<Vec<u8>, web_common::api::ApiError>;\n"
        "}\n\n"
    )

    document.write(
        "impl InsertableTable for web_common::database::Table {\n\n"
        "    fn insert(\n"
        "        &self,\n"
        "        row: Vec<u8>,\n"
        "        user_id: i32,\n"
        "        connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>\n"
        "    ) -> Result<Vec<u8>, web_common::api::ApiError> {\n"
        "        Ok(match self {\n"
    )

    for table in tables:
        if not table.is_insertable() or table.is_junktion_table():
            document.write(
                f'            web_common::database::Table::{table.camel_cased()} => unreachable!("Table `{table.name}` is not insertable as it does not have a known column associated to a creator user id."),\n'
            )
            continue

        document.write(
            f"            web_common::database::Table::{table.camel_cased()} => {{\n"
            f"                let row: web_common::database::{table.new_flat_variant_name()} = bincode::deserialize::<web_common::database::{table.new_flat_variant_name()}>(&row).map_err(web_common::api::ApiError::from)?;\n"
            f"                let inserted_row: crate::models::{table.flat_variant_name()} = <web_common::database::{table.new_flat_variant_name()} as InsertRow>::insert(row, user_id, connection)?;\n"
        )

        # If the table has a richer variant than the flat one, we convert the flat struct
        # to the richest struct using the `from_flat` method.
        if table.get_richest_struct().is_nested():
            document.write(
                f"                let nested_row = crate::nested_models::{table.get_richest_struct().name}::from_flat(inserted_row, connection)?;\n"
                "                 bincode::serialize(&nested_row).map_err(web_common::api::ApiError::from)?\n"
            )
        else:
            document.write(
                "                 bincode::serialize(&inserted_row).map_err(web_common::api::ApiError::from)?\n"
            )

        document.write("            },\n")

    document.write("})\n")

    document.write("    }\n")

    document.write("}\n")

    # We create a method to update rows in the database. The method receives a bincode-serialized
    # row of the update flat variant of the table, the id of the user updating the row, and a connection
    # to the database. The method returns a Result, where the Ok variant is the bincode-serialized
    # version of the richest struct of the table variant, associated with the newly updated row, while
    # the Err variant contains an ApiError. The update method is only available for a subset of the tables
    # as determined by the is_updatable method of the table. When the table is not updatable, we panic
    # with the unreachable!() macro, which explains that the table is not updatable as it does not have
    # a known column associated to a updated_by user id.

    document.write(
        "/// Trait providing the update method for the Table enum.\n"
        "pub trait UpdatableTable {\n"
        "    /// Update a row in the table.\n"
        "    ///\n"
        "    /// # Arguments\n"
        "    /// * `row` - The bincode-serialized row of the table.\n"
        "    /// * `user_id` - The id of the user updating the row.\n"
        "    /// * `connection` - The database connection.\n"
        "    ///\n"
        "    /// # Returns\n"
        "    /// The bincode-serialized row of the table.\n"
        "    fn update(\n"
        "         &self,\n"
        "         row: Vec<u8>,\n"
        "         user_id: i32,\n"
        "         connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>\n"
        ") -> Result<Vec<u8>, web_common::api::ApiError>;\n"
        "}\n\n"
    )

    document.write(
        "impl UpdatableTable for web_common::database::Table {\n\n"
        "    fn update(\n"
        "        &self,\n"
        "        row: Vec<u8>,\n"
        "        user_id: i32,\n"
        "        connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>\n"
        "    ) -> Result<Vec<u8>, web_common::api::ApiError> {\n"
        "        Ok(match self {\n"
    )

    for table in tables:
        if not table.is_updatable() or table.is_junktion_table():
            document.write(
                f'            web_common::database::Table::{table.camel_cased()} => unreachable!("Table `{table.name}` is not updatable as it does not have a known column associated to an updater user id."),\n'
            )
            continue

        document.write(
            f"            web_common::database::Table::{table.camel_cased()} => {{\n"
            f"                let row: web_common::database::{table.update_flat_variant_name()} = bincode::deserialize::<web_common::database::{table.update_flat_variant_name()}>(&row).map_err(web_common::api::ApiError::from)?;\n"
            f"                let updated_row: crate::models::{table.flat_variant_name()} = <web_common::database::{table.update_flat_variant_name()} as UpdateRow>::update(row, user_id, connection)?;\n"
        )

        # If the table has a richer variant than the flat one, we convert the flat struct
        # to the richest struct using the `from_flat` method.
        if table.get_richest_struct().is_nested():
            document.write(
                f"                let nested_row = crate::nested_models::{table.get_richest_struct().name}::from_flat(updated_row, connection)?;\n"
                "                 bincode::serialize(&nested_row).map_err(web_common::api::ApiError::from)?\n"
            )
        else:
            document.write(
                "                 bincode::serialize(&updated_row).map_err(web_common::api::ApiError::from)?\n"
            )

        document.write("            },\n")

    document.write("})\n    }\n}\n")

    # Next, we define a trait providing the from_flat_str method for the Table enum. The method receives
    # a &str containing the json serialized row of the flat variant of the table and a connection to the
    # database. The method returns a Result, where the Ok variant is the bincode-serialized version of the
    # richest struct of the table variant, associated with the newly inserted row, while the Err variant
    # contains an ApiError. The from_flat_str method is available for all tables. Where the table does not
    # have a richer variant than the flat one, the flat one is deserialized from json and reserialized to
    # bincode. This method is primarily used in the context of notifications.

    document.write(
        "/// Trait providing the from_flat_str method for the Table enum.\n"
        "pub trait FromFlatStrTable {\n"
        "    /// Convert a JSON serialized row of the flat variant of the table to the richest struct.\n"
        "    ///\n"
        "    /// # Arguments\n"
        "    /// * `row` - The JSON serialized row of the table.\n"
        "    /// * `connection` - The database connection.\n"
        "    ///\n"
        "    /// # Returns\n"
        "    /// The bincode-serialized row of the table.\n"
        "    fn from_flat_str(\n"
        "         &self,\n"
        "         row: &str,\n"
        "         connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>\n"
        ") -> Result<Vec<u8>, web_common::api::ApiError>;\n"
        "}\n\n"
    )

    document.write(
        "impl FromFlatStrTable for web_common::database::Table {\n\n"
        "    fn from_flat_str(\n"
        "        &self,\n"
        "        row: &str,\n"
        "        connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>\n"
        "    ) -> Result<Vec<u8>, web_common::api::ApiError> {\n"
        "        Ok(match self {\n"
    )

    for table in tables:
        flat_variant = table.get_flat_variant()
        richest_variant = table.get_richest_struct()

        if not richest_variant.is_nested():
            document.write(
                f"            web_common::database::Table::{table.camel_cased()} => bincode::serialize(&serde_json::from_str::<crate::models::{table.flat_variant_name()}>(row).map_err(web_common::api::ApiError::from)?).map_err(web_common::api::ApiError::from)?,\n"
            )
            continue

        document.write(
            f"            web_common::database::Table::{table.camel_cased()} => {{\n"
            f"                let flat_row: crate::models::{flat_variant.name} = serde_json::from_str::<crate::models::{flat_variant.name}>(row).map_err(web_common::api::ApiError::from)?;\n"
            f"                let richest_row = crate::nested_models::{richest_variant.name}::from_flat(flat_row, connection)?;\n"
            "                 bincode::serialize(&richest_row).map_err(web_common::api::ApiError::from)?\n"
            "            },\n"
        )

    document.write("        })\n    }\n}\n")

    document.flush()
    document.close()
