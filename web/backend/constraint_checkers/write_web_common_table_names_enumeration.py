"""Writes the table names enumeration to the web_common crate."""

import os
from typing import List, Dict
from constraint_checkers.struct_metadata import StructMetadata
from constraint_checkers.table_metadata import TableStructMetadata


def write_web_common_table_names_enumeration(
    struct_metadatas: List[StructMetadata],
    new_model_structs: List[StructMetadata],
    update_model_structs: List[StructMetadata],
) -> List[TableStructMetadata]:
    """Writes the table names enumeration to the web_common crate."""
    imports = ["use serde::Deserialize;", "use serde::Serialize;"]

    # The derives to apply to the structs in the `src/database/tables.rs` document
    derives = ["Deserialize", "Serialize", "Clone", "Debug", "PartialEq", "Eq", "Copy"]

    # We check that we are currently executing in the `backend` crate
    # so to make sure that the relative path to the `web_common` crate
    # is correct.
    if not os.getcwd().endswith("backend"):
        raise RuntimeError("This script must be executed in the `backend` crate.")

    document = open("../web_common/src/database/table_names.rs", "w", encoding="utf8")

    # Preliminarly, we write a docstring at the very head
    # of this submodule to explain what it does and warn the
    # reader not to write anything in this file as it is
    # automatically generated.

    document.write(
        "//! This module contains the table names enumeration.\n"
        "//!\n"
        "//! This module is automatically generated. Do not write anything here.\n\n"
    )

    for import_statement in imports:
        document.write(f"{import_statement}\n")

    tables: Dict[str, TableStructMetadata] = {}
    for struct in struct_metadatas:
        if struct.table_name not in tables:
            tables[struct.table_name] = TableStructMetadata(struct.table_name)
        tables[struct.table_name].set_richest_struct(struct)
        if not struct.is_nested():
            tables[struct.table_name].set_flat_variant(struct)

    # We set the new flat model struct variant for each of the tables,
    # when it is available.
    for struct in new_model_structs:
        assert struct.table_name in tables, f"Table {struct.table_name} not found."
        assert struct.is_new_variant(), (
            f"Struct {struct.name} is not a new variant. "
            f"Expected a new variant for table {struct.table_name}. "
            f"Its flat variant is {struct.get_flat_variant().name}."
        )
        tables[struct.table_name].set_new_flat_variant(struct)
        richest_variant = tables[struct.table_name].get_richest_struct()
        if richest_variant.is_nested():
            struct.set_richest_variant(richest_variant)

    # We set the update flat model struct variant for each of the tables,
    # when it is available.
    for struct in update_model_structs:
        assert struct.table_name in tables, f"Table {struct.table_name} not found."
        assert struct.is_update_variant(), (
            f"Struct {struct.name} is not an update variant. "
            f"Expected an update variant for table {struct.table_name}. "
            f"Its flat variant is {struct.get_flat_variant().name}."
        )
        tables[struct.table_name].set_update_flat_variant(struct)
        richest_variant = tables[struct.table_name].get_richest_struct()
        if richest_variant.is_nested():
            struct.set_richest_variant(richest_variant)

    tables: List[TableStructMetadata] = sorted(
        list(tables.values()), key=lambda x: x.name
    )

    document.write("#[derive(" + ", ".join(derives) + ")]\n")
    document.write("pub enum Table {\n")
    for table in tables:
        document.write(f"    {table.camel_cased()},\n")
    document.write("}\n\n")

    # We implement the `AsRef` trait for the `Table` enum
    # to convert it into &str.
    document.write(
        "impl AsRef<str> for Table {\n"
        "    fn as_ref(&self) -> &str {\n"
        "        match self {\n"
    )
    for table in tables:
        document.write(f'            Table::{table.camel_cased()} => "{table.name}",\n')
    document.write("        }\n    }\n}\n")

    # We implement display

    document.write(
        "impl std::fmt::Display for Table {\n"
        "    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {\n"
        '        write!(f, "{}", self.as_ref())\n'
        "    }\n"
        "}\n"
    )

    # We implement the conversion From<Table> for String

    document.write(
        "impl From<Table> for String {\n"
        "    fn from(table: Table) -> Self {\n"
        "        table.to_string()\n"
        "    }\n"
        "}\n"
    )

    # We implement the TryFrom trait from String to Table

    document.write("impl std::convert::TryFrom<&str> for Table {\n")
    document.write("    type Error = String;\n")
    document.write("    fn try_from(value: &str) -> Result<Self, Self::Error> {\n")
    document.write("        match value {\n")
    for table in tables:
        document.write(
            f'            "{table.name}" => Ok(Table::{table.camel_cased()}),\n'
        )
    document.write(
        '            table_name => Err(format!("Unknown table name: {table_name}")),\n'
    )
    document.write("        }\n    }\n}\n")

    # We implement the TryFrom trait from String to Table

    document.write(
        "impl std::convert::TryFrom<String> for Table {\n"
        "    type Error = String;\n"
        "    fn try_from(value: String) -> Result<Self, Self::Error> {\n"
        "        Self::try_from(value.as_str())\n"
        "    }\n"
        "}\n"
    )

    # We implement methods for the frontend when the frontend feature is enabled.
    # These methods include:
    # - delete, which receives a primary key and a connection to the GlueSQL database

    document.write(
        '#[cfg(feature = "frontend")]\n'
        "impl Table {\n"
        "    /// Delete the row from the table.\n"
        "    ///\n"
        "    /// # Arguments\n"
        "    /// * `primary_key` - The primary key of the row.\n"
        "    /// * `connection` - The database connection.\n"
        "    ///\n"
        "    /// # Returns\n"
        "    /// The number of rows deleted.\n"
        "    pub async fn delete<C>(\n"
        "        &self,\n"
        "        primary_key: crate::database::operations::PrimaryKey,\n"
        "        connection: &mut gluesql::prelude::Glue<C>,\n"
        "    ) -> Result<usize, crate::api::ApiError> where\n"
        "        C: gluesql::core::store::GStore + gluesql::core::store::GStoreMut,\n"
        "    {\n"
        "        match self {\n"
    )

    for table in tables:
        document.write(
            f"            Table::{table.camel_cased()} => {{\n"
            f"                crate::database::{table.flat_variant_name()}::delete_from_id(primary_key.into(), connection).await\n"
            "            },\n"
        )

    document.write("        }\n    }\n")

    # Next, we implement the `get` method for the Table enum, which receives a primary key
    # enum and a connection to the database. The method returns a Result, where the Ok variant
    # is a bincode-serialized version of the row of the table variant, while the Err variant
    # contains an ApiError. The get method is available for all tables with a primary key.
    # For the others, we panic with an unimplemented!() macro.

    document.write(
        "    /// Get the row from the table by the primary key.\n"
        "    ///\n"
        "    /// # Arguments\n"
        "    /// * `primary_key` - The primary key of the row.\n"
        "    /// * `connection` - The database connection.\n"
        "    ///\n"
        "    /// # Returns\n"
        "    /// The row of the table.\n"
        "    pub async fn get<C>(\n"
        "        &self,\n"
        "        primary_key: crate::database::operations::PrimaryKey,\n"
        "        connection: &mut gluesql::prelude::Glue<C>,\n"
        "    ) -> Result<Option<Vec<u8>>, crate::api::ApiError> where\n"
        "        C: gluesql::core::store::GStore + gluesql::core::store::GStoreMut,\n"
        "    {\n"
        "        Ok(match self {\n"
    )

    for table in tables:
        document.write(
            f"            Table::{table.camel_cased()} => crate::database::{table.richest_struct_name()}::get(primary_key.into(), connection).await?.map(|row| bincode::serialize(&row)).transpose()?,\n"
        )

    document.write("        })\n    }\n")

    # Next, we implement the all method for the Table enum, which receives a connection
    # to the database and returns a Result, where the Ok variant is a bincode-serialized
    # vector of the rows of the table variant, while the Err variant contains an ApiError.
    # The all method is available for all tables.

    document.write(
        "    /// Get all the rows from the table.\n"
        "    ///\n"
        "    /// # Arguments\n"
        "    /// * `filter` - The filter to apply to the rows.\n"
        "    /// * `limit` - The maximum number of rows to return.\n"
        "    /// * `offset` - The number of rows to skip. By default `0`.\n"
        "    /// * `connection` - The database connection.\n"
        "    ///\n"
        "    /// # Returns\n"
        "    /// A vector of the rows of the table.\n"
        "    pub async fn all<C>(\n"
        "        &self,\n"
        "        filter: Option<Vec<u8>>,\n"
        "        limit: Option<i64>,\n"
        "        offset: Option<i64>,\n"
        "        connection: &mut gluesql::prelude::Glue<C>,\n"
        "    ) -> Result<Vec<Vec<u8>>, crate::api::ApiError> where\n"
        "        C: gluesql::core::store::GStore + gluesql::core::store::GStoreMut,\n"
        "    {\n"
        "        match self {\n"
    )

    for table in tables:
        document.write(f"            Table::{table.camel_cased()} => {{\n")
        if table.has_filter_variant():
            filter_variant = table.get_filter_variant()
            document.write(
                f"                let filter: Option<crate::database::{filter_variant.name}> = filter.map(|filter| bincode::deserialize(&filter).map_err(crate::api::ApiError::from)).transpose()?;\n"
                f"                crate::database::{table.richest_struct_name()}::all(filter.as_ref(), limit, offset, connection).await?.into_iter().map(|row| bincode::serialize(&row).map_err(crate::api::ApiError::from)).collect()\n"
            )
        else:
            document.write(
                '                 assert!(filter.is_none(), "Filter not implemented for this table.");\n'
                f"                crate::database::{table.richest_struct_name()}::all(limit, offset, connection).await?.into_iter().map(|row| bincode::serialize(&row).map_err(crate::api::ApiError::from)).collect()\n"
            )

        document.write("            },\n")

    document.write("        }\n    }\n")

    # Next, we implement the insert method for the Table enum, which receives a bincode-serialized
    # row of the new flat table variant and a connection to the database. The method returns a Result,
    # where the Ok variant is the bincode-serialized version of the richest struct of the table variant,
    # associated with the newly inserted row, while the Err variant contains an ApiError. The insert
    # method is available only for a subset of the tables, namely those that have a column with the
    # information of which user inserted the row.
    # Each variant deserializes the received row, which is the flat
    # new variant, and calls its insert method providing to it the connection, which returns the flat
    # standard struct. When the table has a richer variant than the flat one, we convert the flat struct
    # to the richest struct using the `from_flat` method. We then serialize the struct and return it.

    document.write(
        "    /// Insert a new row into the table.\n"
        "    ///\n"
        "    /// # Arguments\n"
        "    /// * `new_row` - The bincode-serialized row of the table.\n"
        "    /// * `user_id` - The user ID of the user performing the operation.\n"
        "    /// * `connection` - The database connection.\n"
        "    ///\n"
        "    /// # Returns\n"
        "    /// The bincode-serialized row of the table.\n"
        "    pub async fn insert<C>(\n"
        "        &self,\n"
        "        new_row: Vec<u8>,\n"
        "        user_id: i32,\n"
        "        connection: &mut gluesql::prelude::Glue<C>,\n"
        "    ) -> Result<Vec<u8>, crate::api::ApiError> where\n"
        "        C: gluesql::core::store::GStore + gluesql::core::store::GStoreMut,\n"
        "    {\n"
        "        Ok(match self {\n"
    )

    for table in tables:
        if not table.is_insertable():
            document.write(
                f'            Table::{table.camel_cased()} => unimplemented!("Insert not implemented for {table.name}."),\n'
            )
            continue

        # As another check, if the primary key of this table is NOT a UUID, it does not make sense to
        # have it insertable from the frontend, as the primary key is generated by the database itself
        # and does not support a distributed index.
        if not table.has_uuid_primary_key():
            document.write(
                f'            Table::{table.camel_cased()} => unimplemented!("Insert not implemented for {table.name} in frontend as it does not have a UUID primary key."),\n'
            )
            continue

        document.write(
            f"            Table::{table.camel_cased()} => {{\n"
            f"                let new_row: super::{table.new_flat_variant_name()} = bincode::deserialize::<super::{table.new_flat_variant_name()}>(&new_row).map_err(crate::api::ApiError::from)?;\n"
            f"                let inserted_row: super::{table.flat_variant_name()} = new_row.insert(user_id, connection).await?;\n"
        )

        # If the table has a richer variant than the flat one, we convert the flat struct
        # to the richest struct using the `from_flat` method.
        if table.get_richest_struct().is_nested():
            document.write(
                f"                let nested_row = super::{table.get_richest_struct().name}::from_flat(inserted_row, connection).await?;\n"
                "                 bincode::serialize(&nested_row).map_err(crate::api::ApiError::from)?\n"
            )
        else:
            document.write(
                "                bincode::serialize(&inserted_row).map_err(crate::api::ApiError::from)?\n"
            )

        document.write("            },\n")

    document.write("})\n    }\n")

    # Next, we implement the update method for the Table enum, which receives a bincode-serialized
    # row of the update flat table variant and a connection to the database. The method returns a Result,
    # where the Ok variant is the bincode-serialized version of the richest struct of the table variant,
    # associated with the newly updated row, while the Err variant contains an ApiError. The update
    # method is available only for a subset of the tables, namely those that have a column with the
    # information of which user updated the row.
    # Each variant deserializes the received row, which is the flat new variant,
    # and calls its update method providing to it the connection, which returns the flat
    # standard struct. When the table has a richer variant than the flat one, we convert the flat struct
    # to the richest struct using the `from_flat` method. We then serialize the struct and return it.

    document.write(
        "    /// Update a row in the table.\n"
        "    ///\n"
        "    /// # Arguments\n"
        "    /// * `update_row` - The bincode-serialized row of the table.\n"
        "    /// * `user_id` - The user ID of the user performing the operation.\n"
        "    /// * `connection` - The database connection.\n"
        "    ///\n"
        "    /// # Returns\n"
        "    /// The bincode-serialized row of the table.\n"
        "    pub async fn update<C>(\n"
        "        &self,\n"
        "        update_row: Vec<u8>,\n"
        "        user_id: i32,\n"
        "        connection: &mut gluesql::prelude::Glue<C>,\n"
        "    ) -> Result<Vec<u8>, crate::api::ApiError> where\n"
        "        C: gluesql::core::store::GStore + gluesql::core::store::GStoreMut,\n"
        "    {\n"
        "        Ok(match self {\n"
    )

    for table in tables:
        if table.is_junktion_table():
            document.write(
                f'            Table::{table.camel_cased()} => unimplemented!("Update not implemented for {table.name}."),\n'
            )
            continue

        if not table.is_updatable():
            document.write(
                f'            Table::{table.camel_cased()} => unimplemented!("Update not implemented for {table.name}."),\n'
            )
            continue

        flat_variant: StructMetadata = table.get_flat_variant()

        document.write(
            f"            Table::{table.camel_cased()} => {{\n"
            f"                let update_row: super::{table.update_flat_variant_name()} = bincode::deserialize::<super::{table.update_flat_variant_name()}>(&update_row).map_err(crate::api::ApiError::from)?;\n"
            f"                let {flat_variant.get_formatted_primary_keys(include_prefix=False)} = {flat_variant.get_formatted_primary_keys(include_prefix=True, prefix='update_row')};\n"
            f"                update_row.update("
        )

        if table.name != "users":
            document.write("user_id, ")

        document.write("connection).await?;\n")

        document.write(
            f"                let updated_row: super::{table.flat_variant_name()} = super::{table.flat_variant_name()}::get({flat_variant.get_formatted_primary_keys(include_prefix=False)}, connection).await?.unwrap();\n"
        )

        # If the table has a richer variant than the flat one, we convert the flat struct
        # to the richest struct using the `from_flat` method.
        if table.get_richest_struct().is_nested():
            document.write(
                f"                let nested_row = super::{table.get_richest_struct().name}::from_flat(updated_row, connection).await?;\n"
                "                 bincode::serialize(&nested_row).map_err(crate::api::ApiError::from)?\n"
            )
        else:
            document.write(
                "                bincode::serialize(&updated_row).map_err(crate::api::ApiError::from)?\n"
            )

        document.write("            },\n")

    document.write("})\n    }\n")

    # Next, we implement the update or insert method for the Table enum, which receives a bincode-serialized
    # rows of the richest table variant (not a new one) and a connection to the database. This method is used
    # to sync the client-side database with information newly provided by the server. It does not receive,
    # differently from the insert method, the user ID, as the user ID is already present in the row. The method
    # returns a Result, where the Ok variant is an empty tuple, while the Err variant contains an ApiError.
    # This method is available for ALL tables, not only those that have a column with the information of which
    # user inserted the row.

    document.write(
        "    /// Update or insert a row into the table.\n"
        "    ///\n"
        "    /// # Arguments\n"
        "    /// * `rows` - The bincode-serialized rows of the table.\n"
        "    /// * `connection` - The database connection.\n"
        "    ///\n"
        "    /// # Returns\n"
        "    /// An empty tuple.\n"
        "    pub async fn update_or_insert<C>(\n"
        "        &self,\n"
        "        rows: Vec<Vec<u8>>,"
        "        connection: &mut gluesql::prelude::Glue<C>,\n"
        "    ) -> Result<(), crate::api::ApiError> where\n"
        "        C: gluesql::core::store::GStore + gluesql::core::store::GStoreMut,\n"
        "    {\n"
        "        match self {\n"
    )

    for table in tables:
        document.write(
            f"            Table::{table.camel_cased()} => {{\n"
            f"                for row in rows {{\n"
            f"                    let row: super::{table.richest_struct_name()} = bincode::deserialize::<super::{table.richest_struct_name()}>(&row).map_err(crate::api::ApiError::from)?;\n"
            f"                    row.update_or_insert(connection).await?;\n"
            "                }\n"
            "            },\n"
        )

    document.write("        }\n    Ok(())}\n}\n")

    document.flush()
    document.close()

    return tables
