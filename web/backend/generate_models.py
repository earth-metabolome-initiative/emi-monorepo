"""Python script to run a patched version of the diesel extended CLI to generate models.

Implementation details
----------------------
The diesel extended CLI can be used to generated the structs associated to the database tables. However, the
generated structs are not complete most commonly, as it does not come equipped with some of the postgres types.
Fortunately, this can be easily patched with some replace statements.

We start by running the extended CLI command:

```bash
diesel_ext --model --add-table-name > src/models.rs
```

Then, we need to handle the following replacements, plus adding on the top of the file the associated imports.
The imports need to be added solely when the replacements are effective, i.e. there is actually a change in the file,
otherwise we would add unnecessary imports and cause compilation warnings.

The replacements are defined in the file `replacements.json` and are applied to the generated file `src/models.rs`.
The structure of the JSON file is as follows:

```json
[
    {
        "search": "search_string",
        "replace": "replace_string",
        "imports": ["import1", "import2"]
    }
]
```

With a more concrete example:

```json
[
    {
        "search": "Option</* TODO: unknown type Nullable<Numrange> */>",
        "replace": "Option<Range<Numeric>>",
        "imports": [
            "use diesel::sql_types::Numeric;",
            "use diesel::sql_types::Range;"
        ]
    }
]
```

Since we also need to interface with the Frontend database which are NOT based
on Postgres, we also need to duplicate the code in the web commons and generate
the From implementations for the structs in the `src/models.rs` file. The `web_common`
structs will be generated in the `src/database/tables.rs` file in the `web_common` crate.
Since these structs are field-by-field identical, we can simply copy the structs while
ignoring the `#[derive(...)]` and `#[table_name = "..."]` attributes which would not be
applicable in the `web_common` crate. The `From` implementations will be generated in the
`src/models.rs` file in the `backend` crate, below each of the diesel-generated structs and
will make use of the full path to the struct in the `web_common` crate so to avoid conflicts.

"""

import os
import re
import io
import copy
import shutil
from typing import List, Optional, Tuple, Union, Dict
from userinput import userinput
import compress_json
import pandas as pd
import psycopg2
from time import sleep
from densify_directory_counter import densify_directory_counter
from dotenv import load_dotenv
from retrieve_taxons import retrieve_taxons
from tqdm.auto import tqdm
from insert_migration import insert_migration
from functools import cache
import glob
from constraint_checkers import (
    find_foreign_keys,
    ensures_all_update_at_trigger_exists,
    TableMetadata,
    ViewColumn,
    get_cursor,
)
from constraint_checkers import ensure_created_at_columns, ensure_updated_at_columns
from constraint_checkers import handle_minimal_revertion
from constraint_checkers import (
    replace_serial_indices,
    PGIndex,
    PGIndices,
    find_pg_trgm_indices,
)
from constraint_checkers import TableStructMetadata, StructMetadata, AttributeMetadata
from constraint_checkers import write_frontend_pages, write_frontend_router_page
from constraint_checkers import (
    enforce_migration_naming_convention,
    ensure_updatable_tables_have_roles_tables,
    ensure_tables_have_creation_notification_trigger,
)
from constraint_checkers.regroup_tables import regroup_tables
from constraint_checkers import generate_view_schema


TEXTUAL_DATA_TYPES = ["String"]

GLUESQL_TYPES_MAPPING = {
    "i8": "gluesql::core::ast_builder::num({})",
    "i16": "gluesql::core::ast_builder::num({})",
    "i32": "gluesql::core::ast_builder::num({})",
    "i64": "gluesql::core::ast_builder::num({})",
    "i128": "gluesql::core::ast_builder::num({})",
    "u8": "gluesql::core::ast_builder::num({})",
    "u16": "gluesql::core::ast_builder::num({})",
    "u32": "gluesql::core::ast_builder::num({})",
    "u64": "gluesql::core::ast_builder::num({})",
    "u128": "gluesql::core::ast_builder::num({})",
    "f32": "gluesql::core::ast_builder::num({})",
    "f64": "gluesql::core::ast_builder::num({})",
    "String": "gluesql::core::ast_builder::text({})",
    "Uuid": "gluesql::core::ast_builder::uuid({}.to_string())",
    "bool": "({}.into())",
    "NaiveDateTime": "gluesql::core::ast_builder::timestamp({}.to_string())",
    "DateTime<Utc>": "gluesql::core::ast_builder::timestamp({}.to_string())",
    "Vec<u8>": "gluesql::core::ast_builder::bytea({})",
}

INPUT_TYPE_MAP = {
    "String": "Text",
    "f32": "Numeric",
    "f64": "Numeric",
    "i8": "Numeric",
    "i16": "Numeric",
    "i32": "Numeric",
    "i64": "Numeric",
    "i128": "Numeric",
    "u8": "Numeric",
    "u16": "Numeric",
    "u32": "Numeric",
    "u64": "Numeric",
    "u128": "Numeric",
}

TEMPORARELY_IGNORED_TABLES = [
    "derived_samples",
    "spectra",
    "spectra_collections",
]


def struct_name_from_table_name(table_name: str) -> str:
    """Convert the table name to the struct name."""
    if table_name.endswith("s"):
        table_name = table_name[:-1]
    return "".join(word.capitalize() for word in table_name.split("_"))


def sql_type_to_rust_type(sql_type: str) -> str:
    """Convert the SQL type to the Rust type."""
    if sql_type == "uuid":
        return "uuid::Uuid"
    if sql_type == "integer":
        return "i32"
    raise NotImplementedError(f"The SQL type {sql_type} is not supported.")


def write_backend_structs(
    path: str,
    table_type: str,
    struct_metadatas: List[StructMetadata],
    table_metadatas: TableMetadata,
):
    """Write the `From` implementations for the structs in the `src/models.rs` file."""

    if len(struct_metadatas) == 0:
        return

    similarity_indices: PGIndices = find_pg_trgm_indices()

    # After each struct ends, as defined by the `}` character, after
    # we have found a `struct` keyword, we write the `From` implementation
    # for the struct where we implement the conversion to the struct in the
    # `web_common` crate.

    impl_from_line = "impl From<{struct_name}> for web_common::database::{table_type}::{struct_name} {{\n"
    reverse_from = "impl From<web_common::database::{table_type}::{struct_name}> for {struct_name} {{\n"

    imports = [
        "use diesel::Queryable;",
        "use diesel::QueryableByName;",
        "use diesel::Identifiable;",
        "use diesel::Insertable;",
        "use crate::schema::*;",
        "use diesel::Selectable;",
        "use serde::Deserialize;",
        "use serde::Serialize;",
        "use diesel::r2d2::ConnectionManager;",
        "use diesel::r2d2::PooledConnection;",
        "use diesel::prelude::*;",
        "use uuid::Uuid;",
        "use chrono::NaiveDateTime;",
    ]

    if table_type == "views":
        imports.append("use crate::views::schema::*;")

    with open(path, "w") as file:
        # Preliminarly, we write out a few docstrings explaining that
        # the file is generated and should not be modified.
        file.write(
            "//! This file is generated automatically and should not be modified.\n"
            "//!\n"
            "//! Any edits you may apply to this document will be overwritten next time the\n"
            "//! backend is generated. Refrain from making any changes to this file.\n\n"
            "//! If you need to make changes to the backend, please modify the `generate_models`\n"
            "//! document in the `migrations` folder.\n\n"
        )

        # First, we write out the macros for clippy.
        file.write("#![allow(unused)]\n" "#![allow(clippy::all)]\n\n")

        # Then, we write the import statements.
        for import_statement in imports:
            file.write(f"{import_statement}\n")

        # Then, we write the structs.
        file.write("\n")

        for struct in tqdm(
            struct_metadatas,
            desc=f"Writing {table_type} to backend",
            unit="struct",
            leave=False,
        ):

            # First of all, we write out the struct.
            struct.write_to(file, diesel=table_type)

            file.write(
                impl_from_line.format(struct_name=struct.name, table_type=table_type)
            )
            file.write(
                f"    fn from(item: {struct.name}) -> Self {{\n" "        Self {\n"
            )
            for attribute in struct.attributes:
                file.write(f"            {attribute.name}: item.{attribute.name},\n")
            file.write("        }\n" "    }\n" "}\n\n")

            file.write(
                reverse_from.format(struct_name=struct.name, table_type=table_type)
            )
            file.write(
                f"    fn from(item: web_common::database::{table_type}::{struct.name}) -> Self {{\n"
                "        Self {\n"
            )
            for attribute in struct.attributes:
                file.write(f"            {attribute.name}: item.{attribute.name},\n")
            file.write("        }\n" "    }\n" "}\n\n")

            # We now generate the `get` method for the diesel struct.
            # This method receives the ID of the struct and returns the
            # struct from the database.
            #
            # ```rust
            # pub fn get(
            #     id: Uuid,
            #     connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>
            # ) -> Result<Self, diesel::Error> {
            #     login_providers::dsl::login_providers
            #         .filter(login_providers::dsl::id.eq(provider_id))
            #         .first::<Self>(&mut conn)
            # }
            # ```

            file.write(f"impl {struct.name} {{\n")

            # For all tables we implement a `all` method that retrieves all of
            # the rows in the table structured as a vector of the struct.

            file.write(
                "    /// Get all of the structs from the database.\n"
                "    ///\n"
                "    /// # Arguments\n"
                "    /// * `limit` - The maximum number of structs to retrieve. By default, this is 10.\n"
                "    /// * `offset` - The number of structs to skip. By default, this is 0.\n"
                "    /// * `connection` - The connection to the database.\n"
                "    ///\n"
                f"    pub fn all(\n"
                "        limit: Option<i64>,\n"
                "        offset: Option<i64>,\n"
                "        connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>\n"
                "    ) -> Result<Vec<Self>, diesel::result::Error> {\n"
            )
            if table_type == "tables":
                file.write(f"        use crate::schema::{struct.table_name};\n")
            else:
                file.write(f"        use crate::views::schema::{struct.table_name};\n")
            # If the limit is None, we do not apply any limit to the query.
            file.write(
                f"       {struct.table_name}::dsl::{struct.table_name}\n"
                "            .offset(offset.unwrap_or(0))\n"
                "            .limit(limit.unwrap_or(10))\n"
                "            .load::<Self>(connection)\n"
                "    }\n"
            )

            # For the tables that have an updated_at column, we implement the
            # `all_by_updated_at` method that retrieves all of the rows in the
            # table structured as a vector of the struct ordered by the updated_at
            # column in descending order.

            if table_metadatas.has_updated_at_column(struct.table_name):
                file.write(
                    "    /// Get all of the structs from the database ordered by the updated_at column.\n"
                    "    ///\n"
                    "    /// # Arguments\n"
                    "    /// * `limit` - The maximum number of structs to retrieve. By default, this is 10.\n"
                    "    /// * `offset` - The number of structs to skip. By default, this is 0.\n"
                    "    /// * `connection` - The connection to the database.\n"
                    "    ///\n"
                    f"    pub fn all_by_updated_at(\n"
                    "        limit: Option<i64>,\n"
                    "        offset: Option<i64>,\n"
                    "        connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>\n"
                    "    ) -> Result<Vec<Self>, diesel::result::Error> {\n"
                )
                if table_type == "tables":
                    file.write(f"        use crate::schema::{struct.table_name};\n")
                else:
                    file.write(
                        f"        use crate::views::schema::{struct.table_name};\n"
                    )
                file.write(
                    f"        {struct.table_name}::dsl::{struct.table_name}\n"
                    f"            .order_by({struct.table_name}::dsl::updated_at.desc())\n"
                    "            .offset(offset.unwrap_or(0))\n"
                    "            .limit(limit.unwrap_or(10))\n"
                    "            .load::<Self>(connection)\n"
                    "    }\n"
                )

            if table_metadatas.has_primary_key(struct.table_name):
                primary_keys = struct.get_primary_keys()
            elif table_metadatas.is_view(struct.table_name):
                primary_keys = struct.get_attribute_by_name("id")

            if primary_keys is not None:
                file.write(
                    "    /// Delete the struct from the database.\n"
                    "    ///\n"
                    "    /// # Arguments\n"
                    "    /// * `connection` - The connection to the database.\n"
                    "    ///\n"
                    "    pub fn delete(\n"
                    "        &self,\n"
                    "        connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>\n"
                    "    ) -> Result<usize, diesel::result::Error> {\n"
                    f"        Self::delete_by_id({struct.get_formatted_primary_keys(include_prefix=True)}, connection)\n"
                    "    }\n"
                )

                file.write(
                    "    /// Delete the struct from the database by its ID.\n"
                    "    ///\n"
                    "    /// # Arguments\n"
                    f"    /// * `{struct.get_formatted_primary_keys(include_prefix=False)}` - The primary key(s) of the struct to delete.\n"
                    "    /// * `connection` - The connection to the database.\n"
                    "    ///\n"
                    "    pub fn delete_by_id(\n"
                    f"       {struct.get_formatted_primary_keys(include_prefix=False)}: {struct.get_formatted_primary_key_data_types()},\n"
                    "        connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>\n"
                    "    ) -> Result<usize, diesel::result::Error> {\n"
                    f"        diesel::delete({struct.table_name}::dsl::{struct.table_name}\n"
                )
                for primary_key in primary_keys:
                    file.write(
                        f"            .filter({struct.table_name}::dsl::{primary_key.name}.eq({primary_key.name}))\n"
                    )
                file.write("        ).execute(connection)\n" "    }\n")

                file.write(
                    "    /// Get the struct from the database by its ID.\n"
                    "    ///\n"
                    "    /// # Arguments\n"
                    f"    /// * `{struct.get_formatted_primary_keys(include_prefix=False)}` - The primary key(s) of the struct to get.\n"
                    "    /// * `connection` - The connection to the database.\n"
                    "    ///\n"
                    "    pub fn get(\n"
                    f"       {struct.get_formatted_primary_keys(include_prefix=False)}: {struct.get_formatted_primary_key_data_types()},\n"
                    "        connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>\n"
                    "    ) -> Result<Self, diesel::result::Error> {\n"
                )
                if table_type == "tables":
                    file.write(f"        use crate::schema::{struct.table_name};\n")
                else:
                    file.write(
                        f"        use crate::views::schema::{struct.table_name};\n"
                    )
                file.write(f"        {struct.table_name}::dsl::{struct.table_name}\n")
                for primary_key in primary_keys:
                    file.write(
                        f"            .filter({struct.table_name}::dsl::{primary_key.name}.eq({primary_key.name}))\n"
                    )
                file.write("            .first::<Self>(connection)\n" "    }\n")

            # For each of the columns in the struct that are required to be UNIQUE
            # in the SQL, as defined by the get_unique_constraint_columns method, we
            # implement methods of the form `from_{column_name}` that retrieves the
            # struct by the unique column. In the case of a view, we first retrieve
            # the associated base table and secondarily we execute with a get the
            # struct by the id column from the obtained base table.
            for unique_columns in struct.get_unique_constraints():
                reference_unique_columns = [
                    unique_column.as_ref()
                    for unique_column in unique_columns
                ]

                if len(reference_unique_columns) == 1:
                    human_readable_unique_columns = reference_unique_columns[0].name
                else:
                    human_readable_unique_columns = ", ".join(
                        [column.name for column in reference_unique_columns[:-1]]
                    ) + f" and {reference_unique_columns[-1].name}"


                from_method_name = f"from_{'_and_'.join([column.name for column in reference_unique_columns])}"

                if table_type == "views":
                    raise NotImplementedError(
                        "The `from_{column_name}` method is not implemented for views."
                    )
                else:
                    file.write(
                        f"    /// Get the struct from the database by its {human_readable_unique_columns}.\n"
                        "    ///\n"
                        "    /// # Arguments\n"
                    )
                    for unique_column in reference_unique_columns:
                        file.write(f"    /// * `{unique_column.name}` - The {unique_column.name} of the struct to get.\n")
                    
                    file.write(
                        "    /// * `connection` - The connection to the database.\n"
                        "    ///\n"
                        f"    pub fn {from_method_name}(\n"
                    )
                    for unique_column in reference_unique_columns:
                        file.write(f"        {unique_column.name}: {unique_column.format_data_type()},\n")
                    
                    file.write(
                        "        connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>\n"
                        "    ) -> Result<Self, diesel::result::Error> {\n"
                        f"        use crate::schema::{struct.table_name};\n"
                        f"        {struct.table_name}::dsl::{struct.table_name}\n"
                    )
                    for unique_column in reference_unique_columns:
                        file.write(
                            f"            .filter({struct.table_name}::dsl::{unique_column.name}.eq({unique_column.name}))\n"
                        )
                    file.write(
                        "            .first::<Self>(connection)\n"
                        "    }\n"
                    )

            # If this table implements the `pg_trgm` index, we also
            # provide the `search` method to search for the struct
            # by a given string. The method also receives a limit
            # parameter to limit the number of results.
            if similarity_indices.has_table(struct.table_name):
                similarity_index: PGIndex = similarity_indices.get_table(
                    struct.table_name
                )

                for (
                    method_name,
                    similarity_operator,
                    distance_operator,
                ) in PGIndices.SIMILARITY_METHODS:

                    file.write(
                        f"    /// Search for the struct by a given string by Postgres's `{method_name}`.\n"
                        "    ///\n"
                        "    /// # Arguments\n"
                        "    /// * `query` - The string to search for.\n"
                        "    /// * `limit` - The maximum number of results, by default `10`.\n"
                        "    /// * `connection` - The connection to the database.\n"
                        "    ///\n"
                        f"    pub fn {method_name}_search(\n"
                        "        query: &str,\n"
                        "        limit: Option<i32>,\n"
                        "        connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>\n"
                        "    ) -> Result<Vec<Self>, diesel::result::Error> {\n"
                        "        let limit = limit.unwrap_or(10);\n"
                        "        // If the query string is empty, we run an all query with the\n"
                        "        // limit parameter provided instead of a more complex similarity\n"
                        "        // search.\n"
                        "        if query.is_empty() {\n"
                        "            return Self::all(Some(limit as i64), None, connection);\n"
                        "        }\n"
                    )

                    joined_field_names = ", ".join(
                        attribute.name for attribute in struct.attributes
                    )

                    if table_type == "views":
                        file.write(
                            "        let similarity_query = concat!(\n"
                            f'            "WITH selected_ids AS (",\n'
                            f'            "SELECT {similarity_index.table_name}.{primary_key.name} AS id FROM {similarity_index.table_name} ",\n'
                        )
                        if similarity_index.is_gin():
                            file.write(
                                f'            "WHERE $1 {similarity_operator} {similarity_index.arguments}  ",\n'
                                f'            "ORDER BY {method_name}($1, {similarity_index.arguments}) DESC LIMIT $2",\n'
                            )
                        else:
                            file.write(
                                f'            "ORDER BY $1 {distance_operator} {similarity_index.arguments} LIMIT $2",\n'
                            )
                        file.write(
                            '         ")",\n'
                            f'            "SELECT {joined_field_names} FROM {struct.table_name} ",\n'
                            f'            "JOIN selected_ids ON selected_ids.id = {struct.table_name}.id"\n'
                            "        );\n"
                        )
                    else:
                        file.write(
                            "        let similarity_query = concat!(\n"
                            f'            "SELECT {joined_field_names} FROM {struct.table_name} ",\n'
                        )
                        if similarity_index.is_gin():
                            file.write(
                                f'            "WHERE $1 {similarity_operator} {similarity_index.arguments} ",\n'
                                f'            "ORDER BY {method_name}($1, {similarity_index.arguments}) DESC LIMIT $2",\n'
                            )
                        else:
                            file.write(
                                f'            "ORDER BY $1 {distance_operator} {similarity_index.arguments} LIMIT $2;"\n'
                            )
                        file.write("        );\n")

                    file.write(
                        "        diesel::sql_query(similarity_query)\n"
                        "            .bind::<diesel::sql_types::Text, _>(query)\n"
                        "            .bind::<diesel::sql_types::Integer, _>(limit)\n"
                        "            .load(connection)\n"
                        "}\n"
                    )

            # Finally, we cluse the struct implementation.
            file.write("}\n")


def extract_structs(path: str, table_metadata: TableMetadata) -> List[StructMetadata]:
    # A dictionary to store the table names and their
    # respective structs.
    struct_metadatas: List[StructMetadata] = []
    derives = []
    last_table_name = None
    inside_struct = False

    with open(path, "r", encoding="utf8") as file:
        document = file.read()

    for line in document.split("\n"):
        # We skip all lines beginning with `//!` as they are comments
        if line.startswith("//!"):
            continue

        # We find the table name by searching lines like
        # #[diesel(table_name = item_continuous_quantities)]
        if "table_name =" in line:
            last_table_name = line.split("=")[1].strip(" )]").split(":")[-1]

        # If we are in a derive line, we extract the derives:
        if line.startswith("#[derive("):
            derives = line.split("(")[1].strip(")]").split(", ")

        # We determine whether a new struct has started
        # by checking if the `struct` keyword is present
        # in the line.
        if "struct" in line:
            struct_name = line.split(" ")[2]

            struct_metadata = StructMetadata(
                table_name=last_table_name,
                struct_name=struct_name,
                table_metadata=table_metadata,
            )

            for derive in derives:
                struct_metadata.add_derive(derive)

            inside_struct = True

        if inside_struct:
            # If the current line contains the id field,
            # we store the type of the id field.
            if "pub" in line and ":" in line:
                field_name = line.strip().split(" ")[1].strip(":")
                field_type = line.split(":")[1].strip(", ")
                option = False
                if field_type.startswith("Option<"):
                    option = True
                    field_type = field_type[7:-1]
                struct_metadata.add_attribute(
                    AttributeMetadata(
                        original_name=field_name,
                        name=field_name,
                        data_type=field_type,
                        optional=option,
                    )
                )

            # We determine whether the struct has ended
            # by checking if the `}` keyword is present
            # in the line.
            if "}" in line:
                inside_struct = False
                struct_metadatas.append(struct_metadata)

    return struct_metadatas


def write_update_method_for_gluesql(
    struct: StructMetadata,
    writer: "io.TextIOWrapper",
):
    """Write the `update` method for the struct in the GlueSQL database."""

    if struct.is_update_variant() and struct.table_name != "users":
        updator_user_id_attribute: AttributeMetadata = (
            struct.get_updator_user_id_attribute()
        )
    else:
        updator_user_id_attribute = None

    update_types_and_methods = GLUESQL_TYPES_MAPPING.copy()
    update_types_and_methods["bool"] = "{}"

    # We implement the `update` method for the struct. This method updates
    # the struct in the GlueSQL database.
    writer.write(
        "    /// Update the struct in the database.\n"
        "    ///\n"
        "    /// # Arguments\n"
    )

    if updator_user_id_attribute is not None:
        writer.write(
            "    /// * `user_id` - The ID of the user who is updating the struct.\n"
        )

    writer.write(
        "    /// * `connection` - The connection to the database.\n"
        "    ///\n"
        "    /// # Returns\n"
        "    /// The number of rows updated.\n"
        "    pub async fn update<C>(\n"
        "        self,\n"
    )

    if updator_user_id_attribute is not None:
        writer.write("        user_id: i32,\n")

    writer.write(
        "        connection: &mut gluesql::prelude::Glue<C>,\n"
        "    ) -> Result<usize, gluesql::prelude::Error> where\n"
        "        C: gluesql::core::store::GStore + gluesql::core::store::GStoreMut,\n"
        "    {\n"
        "        use gluesql::core::ast_builder::*;\n"
    )
    # We use the AST builder as much as possible so to avoid SQL injection attacks.

    # First, we determine whether the current struct has at least an optional field.

    if struct.contains_optional_fields():
        writer.write(f'        let mut update_row = table("{struct.table_name}")\n')
    else:
        writer.write(f'        table("{struct.table_name}")\n')
    writer.write("            .update()")

    if struct.contains_only_optional_fields():
        raise NotImplementedError(
            f"The struct {struct.name} does not contain any non-optional attributes. "
            "It is not well defined how to update a struct without any attributes."
        )

    for attribute in struct.attributes:
        if attribute.optional:
            # We handle this in the next loop
            continue
        if attribute.data_type() in update_types_and_methods:
            conversion = update_types_and_methods[attribute.data_type()].format(
                f"self.{attribute.name}"
            )
            writer.write(f'        \n.set("{attribute.name}", {conversion})')
        else:
            raise NotImplementedError(
                f"The type {attribute.data_type()} is not supported."
                f"The struct {struct.name} contains an {attribute.data_type()}."
            )

    if updator_user_id_attribute is not None:
        conversion = update_types_and_methods["i32"].format("user_id")
        writer.write(
            f'        \n.set("{updator_user_id_attribute.name}", {conversion})'
        )

    if struct.contains_optional_fields():
        writer.write(";\n")

    # After all of the non-optional fields, we handle the optional fields.
    for attribute in struct.attributes:
        if not attribute.optional:
            continue
        conversion = update_types_and_methods[attribute.data_type()].format(
            f"self.{attribute.name}"
        )
        if attribute.data_type() in update_types_and_methods:
            writer.write(
                f"        if let Some({attribute.name}) = self.{attribute.name} {{\n"
                f'            update_row = update_row.set("{attribute.name}", {update_types_and_methods[attribute.data_type()].format(attribute.name)});\n'
                "        }\n"
            )
        else:
            raise NotImplementedError(
                f"The type {attribute.data_type()} is not supported. "
                f"The struct {attribute.name} contains an {attribute.data_type()}. "
            )

    if struct.contains_optional_fields():
        writer.write("            update_row.execute(connection)\n")
    else:
        writer.write("            .execute(connection)\n")
    writer.write(
        "            .await\n"
        "             .map(|payload| match payload {\n"
        "                 gluesql::prelude::Payload::Update(number_of_updated_rows) => number_of_updated_rows,\n"
        '                 _ => unreachable!("Expected Payload::Update")\n'
        "})\n"
        "    }\n\n"
    )


def write_web_common_structs(
    structs: List[StructMetadata],
    target: str,
    enumeration: str,
    table_metadatas: TableMetadata,
):
    """Write the structs in the target file in the `web_common` crate.

    Parameters
    ----------
    structs : List[StructMetadata]
        The list of structs to write in the target file.
    target : str
        The path where to write the structs in the `web_common` crate.
    enumeration : str
        The name of the enumeration to write in the target file.
    table_metadatas : TableMetadata
        The metadata of the tables.
    """
    # The derive statements to include in the `src/database/tables.rs` document
    imports = [
        "use serde::Deserialize;",
        "use serde::Serialize;",
        "use uuid::Uuid;",
        "use chrono::NaiveDateTime;",
    ]

    # We check that we are currently executing in the `backend` crate
    # so to make sure that the relative path to the `web_common` crate
    # is correct.
    if not os.getcwd().endswith("backend"):
        raise Exception("This script must be executed in the `backend` crate.")

    document = open(f"../web_common/src/database/{target}.rs", "w", encoding="utf8")

    for import_statament in imports:
        document.write(f"{import_statament}\n")

    similarity_indices: PGIndices = find_pg_trgm_indices()

    for struct in tqdm(
        structs,
        desc="Writing frontend structs",
        unit="struct",
        leave=False,
    ):
        struct_has_just_finished = False

        document.write("#[derive(")
        document.write(", ".join(struct.derives()))
        document.write(")]\n")
        # We also write conditional derives for the frontend feature
        # that ask for the `frontend` feature to be enabled and derive
        # the yew::html::Properties trait for the struct.
        document.write(
            '#[cfg_attr(feature = "frontend", derive(yew::html::Properties))]\n'
        )

        document.write(f"pub struct {struct.name} {{\n")
        for attribute in struct.attributes:
            document.write(
                f"    pub {attribute.name}: {attribute.format_data_type()},\n"
            )
        document.write("}\n")

        # This variant of the struct implementation is only
        # available when in the web_common is enabled the frontend
        # feature. It provides several methods including the use
        # of GlueSQL. Fortunately, it does not force us like Diesel
        # to create yet again another duplicate of the struct.
        document.write('#[cfg(feature = "frontend")]\n')
        document.write(f"impl {struct.name} {{\n")
        columns = ", ".join([attribute.name for attribute in struct.attributes])

        # As first thing, we implement the `into_row` method for the struct. This method
        # converts the struct into a vector of `gluesql::core::ast_builder::ExprList`
        # variants, which are used to insert the struct into the GlueSQL database.

        document.write(
            "    pub fn into_row(self) -> Vec<gluesql::core::ast_builder::ExprNode<'static>> {\n"
        )

        document.write("        vec![\n")
        for attribute in struct.attributes:

            if attribute.optional:
                if attribute.data_type() in GLUESQL_TYPES_MAPPING:
                    document.write(f"            match self.{attribute.name} {{\n")
                    document.write(
                        f"                Some({attribute.name}) => {GLUESQL_TYPES_MAPPING[attribute.data_type()].format(attribute.name)},\n"
                    )
                    document.write(
                        "                None => gluesql::core::ast_builder::null(),\n"
                    )
                    document.write("            },\n")
                else:
                    raise NotImplementedError(
                        f"The type {attribute.data_type()} is not supported. "
                        f"The struct {struct.name} contains an {attribute.data_type()}. "
                    )
            elif attribute.data_type() in GLUESQL_TYPES_MAPPING:
                document.write(
                    f"            {GLUESQL_TYPES_MAPPING[attribute.data_type()].format(f'self.{attribute.name}')},\n"
                )
            else:
                raise NotImplementedError(
                    f"The type {attribute.data_type()} is not supported."
                )

        document.write("        ]\n")

        document.write("    }\n\n")

        # We implement the `insert` method for the struct. This method
        # receives a connection to the GlueSQL database and inserts the
        # struct into the database.
        document.write(f"    /// Insert the {struct.name} into the database.\n")
        document.write("    ///\n")
        document.write("    /// # Arguments\n")
        document.write("    /// * `connection` - The connection to the database.\n")
        document.write("    ///\n")
        document.write("    /// # Returns\n")
        document.write(f"    /// The number of rows inserted in table {struct.name}\n")
        document.write("    pub async fn insert<C>(\n")
        document.write("        self,\n")
        document.write("        connection: &mut gluesql::prelude::Glue<C>,\n")
        document.write("    ) -> Result<usize, gluesql::prelude::Error> where\n")
        document.write(
            "        C: gluesql::core::store::GStore + gluesql::core::store::GStoreMut,\n"
        )
        document.write("    {\n")
        document.write("        use gluesql::core::ast_builder::*;\n")
        # We use the AST builder as much as possible so to avoid SQL injection attacks.
        document.write(f'        table("{struct.table_name}")\n')
        document.write("            .insert()\n")
        document.write(f'            .columns("{columns}")\n')
        document.write("            .values(vec![self.into_row()])\n")
        document.write("            .execute(connection)\n")
        document.write("            .await\n")
        document.write("             .map(|payload| match payload {\n")
        document.write(
            "                 gluesql::prelude::Payload::Insert ( number_of_inserted_rows ) => number_of_inserted_rows,\n"
        )
        document.write(
            '                 _ => unreachable!("Payload must be an Insert"),\n'
        )
        document.write("             })\n")
        document.write("    }\n\n")

        # We implement the `get` method for the struct. This method
        # receives the ID of the struct and a connection to the GlueSQL
        # database. The method returns the struct from the database.
        document.write(f"    /// Get {struct.name} from the database by its ID.\n")
        document.write("    ///\n")
        document.write("    /// # Arguments\n")
        primary_keys = struct.get_primary_keys()

        document.write(
            f"    /// * `{struct.get_formatted_primary_keys(include_prefix=False)}` - The primary key(s) of the struct to get.\n"
        )
        document.write(
            "    /// * `connection` - The connection to the database.\n"
            "    ///\n"
            "    pub async fn get<C>(\n"
            f"        {struct.get_formatted_primary_keys(include_prefix=False)}: {struct.get_formatted_primary_key_data_types()},\n"
            "        connection: &mut gluesql::prelude::Glue<C>,\n"
            "    ) -> Result<Option<Self>, gluesql::prelude::Error> where\n"
            "        C: gluesql::core::store::GStore + gluesql::core::store::GStoreMut,\n"
            "    {\n"
            "        use gluesql::core::ast_builder::*;\n"
            f'        let select_row = table("{struct.table_name}")\n'
            "            .select()\n"
        )
        for primary_key in primary_keys:
            document.write(
                f'            .filter(col("{primary_key.name}").eq({primary_key.name}.to_string()))\n'
            )
        document.write(
            f'            .project("{columns}")\n'
            "            .limit(1)\n"
            "            .execute(connection)\n"
            "            .await?;\n"
            "         Ok(select_row.select()\n"
            "            .unwrap()\n"
            "            .map(Self::from_row)\n"
            "            .collect::<Vec<_>>()\n"
            "            .pop())\n"
            "    }\n\n"
        )

        # We implement the `delete` method for the struct. This method deletes
        # the struct from the GlueSQL database.
        document.write(
            f"    /// Delete {struct.name} from the database.\n"
            "    ///\n"
            "    /// # Arguments\n"
            f"    /// * `{struct.get_formatted_primary_keys(include_prefix=False)}` - The primary key(s) of the struct to delete.\n"
            "    /// * `connection` - The connection to the database.\n"
            "    ///\n"
            "    /// # Returns\n"
            "    /// The number of rows deleted.\n"
            "    pub async fn delete_from_id<C>(\n"
            f"        {struct.get_formatted_primary_keys(include_prefix=False)}: {struct.get_formatted_primary_key_data_types()},\n"
            "        connection: &mut gluesql::prelude::Glue<C>,\n"
            "    ) -> Result<usize, gluesql::prelude::Error> where\n"
            "        C: gluesql::core::store::GStore + gluesql::core::store::GStoreMut,\n"
            "    {\n"
            "        use gluesql::core::ast_builder::*;\n"
            f'        table("{struct.table_name}")\n'
            "            .delete()\n"
        )
        for primary_key in primary_keys:
            document.write(
                f'            .filter(col("{primary_key.name}").eq({primary_key.name}.to_string()))\n'
            )
        document.write(
            "            .execute(connection)\n"
            "            .await\n"
            "             .map(|payload| match payload {\n"
            "                 gluesql::prelude::Payload::Delete(number_of_deleted_rows) => number_of_deleted_rows,\n"
            '                 _ => unreachable!("Payload must be a Delete"),\n'
            "             })\n"
            "    }\n\n"
        )

        # We implement the `delete` method for the struct. This method deletes
        # the current instance of the struct from the GlueSQL database.
        document.write(
            f"    /// Delete the current instance of {struct.name} from the database.\n"
            "    ///\n"
            "    /// # Arguments\n"
            "    /// * `connection` - The connection to the database.\n"
            "    ///\n"
            "    /// # Returns\n"
            "    /// The number of rows deleted.\n"
            "    pub async fn delete<C>(\n"
            "        self,\n"
            "        connection: &mut gluesql::prelude::Glue<C>,\n"
            "    ) -> Result<usize, gluesql::prelude::Error> where\n"
            "        C: gluesql::core::store::GStore + gluesql::core::store::GStoreMut,\n"
            "    {\n"
            f"        Self::delete_from_id({struct.get_formatted_primary_keys(include_prefix=True)}, connection).await\n"
            "    }\n"
        )

        # We implement the `update` method for the struct. This method updates
        # the struct in the GlueSQL database.
        write_update_method_for_gluesql(struct, document)

        # Next, we implement the `update_or_insert` method for the struct. This method
        # inserts the struct into the GlueSQL database if it does not exist, otherwise
        # it updates the struct in the database.
        document.write(
            "    /// Update the struct in the database if it exists, otherwise insert it.\n"
            "    ///\n"
            "    /// # Arguments\n"
            "    /// * `connection` - The connection to the database.\n"
            "    ///\n"
            "    /// # Returns\n"
            "    /// The number of rows updated or inserted.\n"
            "    pub async fn update_or_insert<C>(\n"
            "        self,\n"
            "        connection: &mut gluesql::prelude::Glue<C>,\n"
            "    ) -> Result<usize, gluesql::prelude::Error> where\n"
            "        C: gluesql::core::store::GStore + gluesql::core::store::GStoreMut,\n"
            "    {\n"
            "        let number_of_rows = self.clone().update(connection).await?;\n"
            "        if number_of_rows == 0 {\n"
            "            self.insert(connection).await\n"
            "        } else {\n"
            "            Ok(number_of_rows)\n"
            "        }\n"
            "    }\n"
        )

        # We implement the `all` method for the struct. This method returns all of the
        # structs in the GlueSQL database.
        document.write(
            f"    /// Get all {struct.name} from the database.\n"
            "    ///\n"
            "    /// # Arguments\n"
            "    /// * `limit` - The maximum number of results, by default `10`.\n"
            "    /// * `offset` - The offset of the results, by default `0`.\n"
            "    /// * `connection` - The connection to the database.\n"
            "    ///\n"
            "    pub async fn all<C>(\n"
            "        limit: Option<i64>,\n"
            "        offset: Option<i64>,\n"
            "        connection: &mut gluesql::prelude::Glue<C>,\n"
            "    ) -> Result<Vec<Self>, gluesql::prelude::Error> where\n"
            "        C: gluesql::core::store::GStore + gluesql::core::store::GStoreMut,\n"
            "    {\n"
            "        use gluesql::core::ast_builder::*;\n"
            f'        let select_row = table("{struct.table_name}")\n'
            "            .select()\n"
            f'            .project("{columns}")\n'
            "            .offset(offset.unwrap_or(0))\n"
            "            .limit(limit.unwrap_or(10))\n"
            "            .execute(connection)\n"
            "            .await?;\n"
            "        Ok(select_row.select()\n"
            "            .unwrap()\n"
            "            .map(Self::from_row)\n"
            "            .collect::<Vec<_>>())\n"
            "    }\n"
        )

        # We implement for all tables that implement the `updated_at` column
        # the `all_by_updated_at` method. This method returns all of the structs
        # in the GlueSQL database ordered by the `updated_at` column.
        if table_metadatas.has_updated_at_column(struct.table_name):
            document.write(
                f"    /// Get all {struct.name} from the database ordered by the `updated_at` column.\n"
                "    ///\n"
                "    /// # Arguments\n"
                "    /// * `limit` - The maximum number of results, by default `10`.\n"
                "    /// * `offset` - The offset of the results, by default `0`.\n"
                "    /// * `connection` - The connection to the database.\n"
                "    ///\n"
                "    pub async fn all_by_updated_at<C>(\n"
                "        limit: Option<i64>,\n"
                "        offset: Option<i64>,\n"
                "        connection: &mut gluesql::prelude::Glue<C>,\n"
                "    ) -> Result<Vec<Self>, gluesql::prelude::Error> where\n"
                "        C: gluesql::core::store::GStore + gluesql::core::store::GStoreMut,\n"
                "    {\n"
                "        use gluesql::core::ast_builder::*;\n"
                f'        let select_row = table("{struct.table_name}")\n'
                "            .select()\n"
                f'            .project("{columns}")\n'
                '            .order_by("updated_at desc")\n'
                "            .offset(offset.unwrap_or(0))\n"
                "            .limit(limit.unwrap_or(10))\n"
                "            .execute(connection)\n"
                "            .await?;\n"
                "        Ok(select_row.select()\n"
                "            .unwrap()\n"
                "            .map(Self::from_row)\n"
                "            .collect::<Vec<_>>())\n"
                "    }\n"
            )

        # We implement the `from_row` method for the struct. This method
        # receives a row from the GlueSQL database, which is a `HashMap<&str, &&Value>`.
        # The method returns the struct from the row.
        document.write(
            "    pub fn from_row(row: std::collections::HashMap<&str, &gluesql::prelude::Value>) -> Self {\n"
        )
        document.write("        Self {\n")

        clonables = {
            "bool": "Bool",
            "i8": "I8",
            "i16": "I16",
            "i32": "I32",
            "i64": "I64",
            "i128": "I128",
            "u8": "U8",
            "u16": "U16",
            "u32": "U32",
            "u64": "U64",
            "u128": "U128",
            "f32": "F32",
            "f64": "F64",
            "String": "Str",
            "NaiveDateTime": "Timestamp",
            "Vec<u8>": "Bytea",
        }

        for attribute in struct.attributes:
            if attribute.format_data_type() == "Uuid":
                document.write(
                    f'            {attribute.name}: match row.get("{attribute.name}").unwrap() {{\n'
                    f"                gluesql::prelude::Value::Uuid({attribute.name}) => Uuid::from_u128(*{attribute.name}),\n"
                    '                _ => unreachable!("Expected Uuid"),\n'
                    "            },\n"
                )
            elif attribute.format_data_type() == "Option<Uuid>":
                document.write(
                    f'            {attribute.name}: match row.get("{attribute.name}").unwrap() {{\n'
                )
                document.write(
                    "                gluesql::prelude::Value::Null => None,\n"
                )
                document.write(
                    f"                gluesql::prelude::Value::Uuid({attribute.name}) => Some(Uuid::from_u128(*{attribute.name})),\n"
                )
                document.write('                _ => unreachable!("Expected Uuid"),\n')
                document.write("            },\n")
            elif attribute.implements_clone():
                if attribute.optional:
                    document.write(
                        f'            {attribute.name}: match row.get("{attribute.name}").unwrap() {{\n'
                    )
                    document.write(
                        "                gluesql::prelude::Value::Null => None,\n"
                    )
                    document.write(
                        f"                gluesql::prelude::Value::{clonables[attribute.data_type()]}({attribute.name}) => Some({attribute.name}.clone()),\n"
                    )
                    document.write(
                        f'                _ => unreachable!("Expected {clonables[attribute.data_type()]}")\n'
                    )
                    document.write("            },\n")
                else:
                    document.write(
                        f'            {attribute.name}: match row.get("{attribute.name}").unwrap() {{\n'
                    )
                    document.write(
                        f"                gluesql::prelude::Value::{clonables[attribute.data_type()]}({attribute.name}) => {attribute.name}.clone(),\n"
                    )
                    document.write(
                        f'                _ => unreachable!("Expected {clonables[attribute.data_type()]}")\n'
                    )
                    document.write("            },\n")
            else:
                raise NotImplementedError(
                    f"Found an unsupported attribute type for the struct {struct.name}: {attribute.data_type()} "
                    f"for the attribute {attribute.name}."
                )
        document.write("        }\n")
        document.write("    }\n")

        # And finally we close the struct implementation
        document.write("}\n")

    document.close()


def get_view_names() -> List[str]:
    """Returns list of view names.

    Implementative details
    -------------------------
    The view names are extracted from the `migrations` directory. The view names are extracted
    from the `up.sql` file in each of the directories. The view names are extracted by searching
    for the `CREATE VIEW` statements in the SQL file.
    """
    view_names = []
    for directory in os.listdir("migrations"):
        if not os.path.isdir(f"migrations/{directory}"):
            continue
        with open(f"migrations/{directory}/up.sql", "r", encoding="utf8") as file:
            content = file.read()
        for line in content.split("\n"):
            if "CREATE VIEW" in line or "CREATE MATERIALIZED VIEW" in line:
                view_name = line.rsplit(" ", maxsplit=2)[1]
                view_names.append(view_name)
    return view_names


def map_postgres_to_rust_type(pg_type):
    pg_to_rust_types = {
        "uuid": "diesel::sql_types::Uuid",
        "text": "diesel::sql_types::Text",
        "timestamp without time zone": "diesel::sql_types::Timestamp",
        "character varying": "diesel::sql_types::Text",
        "integer": "diesel::sql_types::Integer",
    }

    if pg_type in pg_to_rust_types:
        return pg_to_rust_types[pg_type]

    raise NotImplementedError(f'Postgres type "{pg_type}" is not supported.')


def generate_diesel_schema(view_name: str, columns: List[ViewColumn]) -> str:
    schema_code = "diesel::table! {\n"
    schema_code += f"    {view_name} (id) {{\n"
    for column in columns:
        if column.nullable:
            schema_code += f"        {column.alias_name} -> diesel::sql_types::Nullable<{map_postgres_to_rust_type(column.data_type)}>,\n"
        else:
            schema_code += f"        {column.alias_name} -> {map_postgres_to_rust_type(column.data_type)},\n"
    schema_code += "    }\n"
    schema_code += "}\n"
    return schema_code


def check_schema_completion():
    """Check the view schema completion.

    Implementative details
    -------------------------
    Diesel does not support the `CREATE VIEW` statements, and as such we need to manually
    create the views in the schema file `src/views/schema.rs`. This script check that all
    of the view names are present in the schema file.
    """
    view_names = get_view_names()
    with open("src/views/schema.rs", "r", encoding="utf8") as file:
        content = file.read()
    for view_name in view_names:
        if view_name not in content:
            raise NotImplementedError(
                f"View {view_name} is not present in the schema file."
            )


def generate_view_structs():
    """Generate the view structs.

    Implementative details
    -------------------------
    Since Diesel solely supports the generation of the table structs, we need to use
    this custom script to generate the view structs. The view structs are generated
    starting from the schema file `src/views/schema.rs` and are written to the file
    `src/views/views.rs`. The view structs are generated by copying the views structs
    defined in the views schema, with the data types appropriately changed to match the
    view schema.

    An example of a schema entry is:

    ```rust
    diesel::table! {
        users_view (id) {
            id -> Uuid,
            first_name -> Nullable<Text>,
            middle_name -> Nullable<Text>,
            last_name -> Nullable<Text>,
            created_at -> Timestamp,
            updated_at -> Timestamp,
        }
    }
    ```
    """

    with open("src/views/schema.rs", "r", encoding="utf8") as file:
        schema = file.read()

    views = open("src/views/views.rs", "w", encoding="utf8")

    if len(schema) == 0:
        views.close()
        return

    data_types = {
        "diesel::sql_types::Uuid": "Uuid",
        "diesel::sql_types::Text": "String",
        "diesel::sql_types::Timestamp": "NaiveDateTime",
        "diesel::sql_types::Integer": "i32",
    }

    imports = [
        "use serde::Deserialize;",
        "use serde::Serialize;",
        "use diesel::Queryable;",
        "use diesel::QueryableByName;",
        "use uuid::Uuid;",
        "use chrono::NaiveDateTime;",
        "use diesel::r2d2::PooledConnection;",
        "use diesel::r2d2::ConnectionManager;",
        "use diesel::prelude::*;",
    ]

    derives = [
        "Deserialize",
        "Serialize",
        "Debug",
        "PartialEq",
        "Queryable",
        "QueryableByName",
    ]

    for import_statement in imports:
        views.write(f"{import_statement}\n")

    view_structs = []
    last_line_was_table = False
    brackets_count = 0

    for line in schema.split("\n"):
        if "{" in line:
            brackets_count += 1
        if "}" in line:
            brackets_count -= 1

        if last_line_was_table:
            view_name = line.split("(")[0].strip(" ")
            view_struct = StructMetadata(
                struct_name=struct_name_from_table_name(view_name),
                table_name=view_name,
            )
            view_structs.append(view_struct)

        if "diesel::table! {" in line:
            last_line_was_table = True
            continue
        else:
            last_line_was_table = False

        if "->" in line:
            (attribute, data_type) = line.strip(" ,").split(" -> ")
            optional = False
            if "Nullable<" in data_type:
                optional = True
                data_type = data_type.split("Nullable<", maxsplit=1)[1].strip(">")
            view_struct.add_attribute(
                AttributeMetadata(
                    original_name=attribute,
                    name=attribute,
                    data_type=data_types[data_type],
                    optional=optional,
                )
            )

        # If we have found the end of the struct, we write the struct
        # to the file.
        if brackets_count == 0 and view_name is not None:
            # First, we generate the derives.
            for derive in derives:
                view_struct.add_derive(derive)

            views.write("#[derive(")
            views.write(", ".join(view_struct.derives()))
            views.write(")]\n")

            # Then, we write the table_name attribute to link
            # the struct to the view.
            views.write(
                f"#[diesel(table_name = crate::views::schema::{view_struct.table_name})]\n"
            )

            # We write the struct definition
            views.write(f"pub struct {view_struct.name} {{\n")
            for attribute in view_struct.attributes:
                views.write(
                    f"    pub {attribute.name}: {attribute.format_data_type()},\n"
                )
            views.write("}\n\n")

        if brackets_count == 0:
            view_name = None

    view_names_from_sql = get_view_names()
    for view_struct in view_structs:
        assert (
            view_struct.table_name in view_names_from_sql
        ), f'View "{view_struct.table_name}" is not present in the "schema.rs" file.'

    views.close()


def generate_nested_structs(
    path: str, struct_metadatas: List[StructMetadata], tables_metadata: TableMetadata
) -> List[StructMetadata]:
    """Generate the nested structs.

    Implementative details
    -------------------------
    Normally, a table struct is generated from a row in the database. However, in some cases,
    a table row may contain a reference id to another table. In this case, we generate a nested
    struct for the referenced table. Depending on whether this referenced row contains also a
    reference to another table, we may generate the nested struct version of the referenced row
    or the flat version, i.e. the row itself.

    For each table, we query the postgres to get the foreign keys. We then generate the nested
    structs for the referenced tables. The nested structs are written to the file `src/models.rs`.
    """
    similarity_indices: PGIndices = find_pg_trgm_indices()

    # We open the file to write the nested structs
    document = open(path, "w", encoding="utf8")

    # Preliminarly, we write a docstring at the very head
    # of this submodule to explain what it does and warn the
    # reader not to write anything in this file as it is
    # automatically generated.
    document.write(
        "//! This module contains the nested structs for the database tables.\n"
        "//!\n"
        "//! This file is automatically generated. Do not write anything here.\n\n"
    )

    # We start with the necessary imports.
    imports = [
        "use serde::Deserialize;",
        "use serde::Serialize;",
        "use diesel::r2d2::ConnectionManager;",
        "use diesel::r2d2::PooledConnection;",
        "use uuid::Uuid;",
        "use crate::models::*;",
        "use crate::views::views::*;",
    ]

    for import_statement in imports:
        document.write(f"{import_statement}\n")

    def get_struct_by_table_name(table_name: str) -> StructMetadata:
        for struct in struct_metadatas:
            if struct.table_name == table_name:
                return struct
        raise ValueError(f"Table name {table_name} not found in the struct metadata.")

    nested_structs = []

    # For each of the struct, we generated the Nested{struct_name} version
    # if the struct contains a reference to another struct.
    for struct in tqdm(
        struct_metadatas,
        desc="Generating nested structs",
        leave=False,
        unit="struct"
    ):
        foreign_keys = struct.get_foreign_keys()

        if len(foreign_keys) == 0:
            continue

        nested_struct = StructMetadata(
            f"Nested{struct.name}",
            struct.table_name,
            table_metadata=tables_metadata
        )
        nested_struct.set_flat_variant(struct)

        if not struct.has_only_foreign_keys():
            nested_struct.add_attribute(
                AttributeMetadata(
                    original_name="inner",
                    name="inner",
                    data_type=struct,
                    optional=False,
                )
            )

        for attribute in foreign_keys:
            # If the current attribute is among the foreign keys, we replace it
            # with the foreign struct. This struct may be also nested if the foreign
            # table has foreign keys, which we check by using the `has_foreign_keys`
            # method of the `tables_metadata` object.
            foreign_key_table_name = tables_metadata.get_foreign_key_table_name(
                struct.table_name, attribute.name
            )
            
            normalized_attribute_name = attribute.name
            foreign_struct = get_struct_by_table_name(foreign_key_table_name)

            if normalized_attribute_name.endswith("_id"):
                normalized_attribute_name = normalized_attribute_name[:-3]

            if struct.name == foreign_struct.name or not foreign_struct.has_foreign_keys():
                nested_struct.add_attribute(
                    AttributeMetadata(
                        original_name=attribute.name,
                        name=normalized_attribute_name,
                        data_type=foreign_struct,
                        optional=attribute.optional,
                    )
                )
            else:
                nested_struct.add_attribute(
                    AttributeMetadata(
                        original_name=attribute.name,
                        name=normalized_attribute_name,
                        data_type=f"Nested{foreign_struct.name}",
                        optional=attribute.optional,
                    )
                )

        foreign_keys = nested_struct.get_foreign_keys()
        for foreign_key in foreign_keys:
            assert any(
                attribute.original_name == foreign_key.name
                for attribute in nested_struct.attributes
            ), (
                f"Foreign key {foreign_key.name} not found in the nested struct {nested_struct.name}. "
                f"The struct has the following attributes: {nested_struct.attributes}."
            )
        
        nested_structs.append(nested_struct)

    # We replace until convergence the data type of the structs with the structs themselves.
    # This is necessary as the nested structs may contain references to other structs, which
    # in turn may contain references to other structs and so on.

    changed = True

    while changed:
        changed = False
        updated_struct_metadatas = []
        for nested_struct in nested_structs:
            if not nested_struct.has_undefined_nested_dependencies():
                updated_struct_metadatas.append(nested_struct)
                continue
            new_attributes = []
            converged = True
            for attribute in nested_struct.attributes:
                if attribute.is_undefined_nested_dependencies():
                    struct_identified = False
                    for struct in nested_structs + struct_metadatas:
                        if struct.name == attribute.data_type():
                            struct_identified = True
                            if struct.has_undefined_nested_dependencies():
                                converged = False
                                continue
                            new_attributes.append(
                                AttributeMetadata(
                                    original_name=attribute.original_name,
                                    name=attribute.name,
                                    data_type=struct,
                                    optional=attribute.optional,
                                )
                            )
                            changed = True
                            break
                    if not struct_identified:
                        raise RuntimeError(
                            f"Could not identify the struct with the name {attribute.data_type()}. "
                            f"Found while processing the nested struct {nested_struct.name}."
                        )
                else:
                    new_attributes.append(attribute)
            if converged:
                assert len(new_attributes) == len(nested_struct.attributes), (
                    f"Expected the length of the new attributes to be equal to the length of the old attributes. "
                    f"Expected {len(new_attributes)}, got {len(nested_struct.attributes)}."
                )
                nested_struct.attributes = new_attributes
            updated_struct_metadatas.append(nested_struct)
        nested_structs = updated_struct_metadatas

    # Consistency check:
    # We check that all the nested struct contain the foreign keys
    # present in the original struct.
    for nested_struct in nested_structs:
        assert nested_struct.is_nested()
        foreign_keys = nested_struct.get_foreign_keys()
        for foreign_key in foreign_keys:
            assert any(
                attribute.original_name == foreign_key.name
                for attribute in nested_struct.attributes
            ), (
                f"Foreign key {foreign_key.name} not found in the nested struct {nested_struct.name}. "
                f"The struct has the following attributes: {nested_struct.attributes}."
            )


    for nested_struct in nested_structs:
        nested_struct.write_to(document)
        flat_struct = nested_struct.get_flat_variant()

        # We implement the all for the nested structs

        # First, we implement a method that will be reused by several of the following methods,
        # including the all, get and search ones: a method that given the flat struct and a connection
        # to the database returns a result containing the nested struct.
        document.write(
            f"impl {nested_struct.name} {{\n"
            "    /// Convert the flat struct to the nested struct.\n"
            "    ///\n"
            "    /// # Arguments\n"
            "    /// * `flat_struct` - The flat struct.\n"
            "    /// * `connection` - The database connection.\n"
            "    pub fn from_flat(\n"
            f"        flat_struct: {flat_struct.name},\n"
            "        connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>,\n"
            "    ) -> Result<Self, diesel::result::Error> {\n"
            "        Ok(Self {\n"
        )
        for attribute in nested_struct.attributes:
            if attribute.name == "inner":
                continue
            if attribute.data_type() == nested_struct.name or flat_struct.has_attribute(
                attribute
            ):
                document.write(
                    f"            {attribute.name}: flat_struct.{attribute.name},\n"
                )
                continue
            if attribute.optional:
                document.write(
                    f"            {attribute.name}: flat_struct.{attribute.original_name}.map(|flat_struct| {attribute.data_type()}::get(flat_struct, connection)).transpose()?,\n"
                )
            else:
                document.write(
                    f"            {attribute.name}: {attribute.data_type()}::get(flat_struct.{attribute.original_name}, connection)?,\n"
                )

        document.write(
            "                inner: flat_struct,\n" "        })\n" "    }\n" "}\n"
        )

        # Then we implement the all query.

        document.write(
            f"impl {nested_struct.name} {{\n"
            "    /// Get all the nested structs from the database.\n"
            "    ///\n"
            "    /// # Arguments\n"
            "    /// * `limit` - The maximum number of rows to return. By default `10`.\n"
            "    /// * `offset` - The offset of the rows to return. By default `0`.\n"
            "    /// * `connection` - The database connection.\n"
            "    pub fn all(\n"
            "        limit: Option<i64>,\n"
            "        offset: Option<i64>,\n"
            "        connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>,\n"
            "    ) -> Result<Vec<Self>, diesel::result::Error> {\n"
            f"        {flat_struct.name}::all(limit, offset, connection)?.into_iter().map(|flat_struct| Self::from_flat(flat_struct, connection)).collect()\n"
            "    }\n"
            "}\n"
        )

        # Then, for all the tables that have an updated_at column, we implement the
        # `all_by_updated_at` method, which returns all of the nested structs ordered
        # by the `updated_at` column.
        if tables_metadata.has_updated_at_column(flat_struct.table_name):
            document.write(
                f"impl {nested_struct.name} {{\n"
                "    /// Get all the nested structs from the database ordered by the `updated_at` column.\n"
                "    ///\n"
                "    /// # Arguments\n"
                "    /// * `limit` - The maximum number of rows to return. By default `10`.\n"
                "    /// * `offset` - The offset of the rows to return. By default `0`.\n"
                "    /// * `connection` - The database connection.\n"
                "    pub fn all_by_updated_at(\n"
                "        limit: Option<i64>,\n"
                "        offset: Option<i64>,\n"
                "        connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>,\n"
                "    ) -> Result<Vec<Self>, diesel::result::Error> {\n"
                f"        {flat_struct.name}::all_by_updated_at(limit, offset, connection)?.into_iter().map(|flat_struct| Self::from_flat(flat_struct, connection)).collect()\n"
                "    }\n"
                "}\n"
            )

        document.write(
            f"impl {nested_struct.name} {{\n"
            "    /// Get the nested struct from the provided primary key.\n"
            "    ///\n"
            "    /// # Arguments\n"
            f"    /// * `{nested_struct.get_formatted_primary_keys(include_prefix=False)}` - The primary key(s) of the row.\n"
            "    /// * `connection` - The database connection.\n"
            "    pub fn get(\n"
            f"        {nested_struct.get_formatted_primary_keys(include_prefix=False)}: {nested_struct.get_formatted_primary_key_data_types()},\n"
            "        connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>,\n"
            "    ) -> Result<Self, diesel::result::Error>\n"
            "    {\n"
            f"       {flat_struct.name}::get({nested_struct.get_formatted_primary_keys(include_prefix=False)}, connection).and_then(|flat_struct| Self::from_flat(flat_struct, connection))\n"
            "    }\n"
            "}\n"
        )

        # For each of the columns in the struct that have a UNIQUE constraint,
        # we implement the methods `from_{column_name}` by employing the method
        # of the same name available for the main struct associated to this struct
        for unique_columns in flat_struct.get_unique_constraints():

            unique_column_references = [
                unique_column.as_ref()
                for unique_column in unique_columns
            ]

            joined = '_and_'.join([
                unique_column.name
                for unique_column in unique_column_references
            ])
            from_method_name = f"from_{joined}"

            if len(unique_column_references) == 1:
                human_readable_column_names = unique_column_references[0].name
            else:
                human_readable_column_names = ', '.join([
                    unique_column.name
                    for unique_column in unique_column_references[:-1]
                ]) + f" and {unique_column_references[-1].name}"

            comma_separated_column_names = ', '.join([
                unique_column.name
                for unique_column in unique_column_references
            ])

            document.write(
                f"impl {nested_struct.name} {{\n"
                f"    /// Get the nested struct from the provided {human_readable_column_names}.\n"
                "    ///\n"
                f"    /// # Arguments\n"
            )
            for unique_column in unique_column_references:
                document.write(
                    f"    /// * `{unique_column.name}` - The {unique_column.name} of the row.\n"
                )
            document.write(         
                "    /// * `connection` - The database connection.\n"
                f"    pub fn {from_method_name}(\n"
            )
            for unique_column in unique_column_references:
                document.write(
                f"        {unique_column.name}: {unique_column.format_data_type()},\n"
                )
            document.write(
                "        connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>,\n"
                "    ) -> Result<Self, diesel::result::Error>\n"
                "    {\n"
                f"        {flat_struct.name}::{from_method_name}({comma_separated_column_names}, connection).and_then(|flat_struct| Self::from_flat(flat_struct, connection))\n"
                "    }\n"
                "}\n"
            )

        # If there is an index on the table, we implement the search method that
        # calls search on the flat version of the struct and then iterates on the
        # primary keys of the results and constructs the nested structs by calling
        # the `get` method several times.
        if similarity_indices.has_table(flat_struct.table_name):
            for method_name, _, _ in PGIndices.SIMILARITY_METHODS:
                document.write(
                    f"impl {nested_struct.name} {{\n"
                    "    /// Search the table by the query.\n"
                    "    ///\n"
                    "    /// # Arguments\n"
                    "    /// * `query` - The string to search for.\n"
                    "    /// * `limit` - The maximum number of results, by default `10`.\n"
                    f"    pub fn {method_name}_search(\n"
                    "        query: &str,\n"
                    "        limit: Option<i32>,\n"
                    "        connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>,\n"
                    "    ) -> Result<Vec<Self>, diesel::result::Error> {\n"
                    f"       {flat_struct.name}::{method_name}_search(query, limit, connection)?.into_iter().map(|flat_struct| Self::from_flat(flat_struct, connection)).collect()\n"
                    "    }\n"
                    "}\n"
                )

        # We implement the bidirectional From methods for the nested struct
        # present in the web_common crate, which does not use Diesel or its
        # structs, but the web_common version of the structs.
        document.write(
            f"impl From<web_common::database::nested_models::{nested_struct.name}> for {nested_struct.name} {{\n"
            f"    fn from(item: web_common::database::nested_models::{nested_struct.name}) -> Self {{\n"
            "        Self {\n"
        )
        for attribute in nested_struct.attributes:
            if attribute.optional:
                document.write(
                    f"            {attribute.name}: item.{attribute.name}.map(|item| item.into()),\n"
                )
            else:
                document.write(
                    f"            {attribute.name}: item.{attribute.name}.into(),\n"
                )
        document.write("        }\n" "    }\n" "}\n")

        document.write(
            f"impl From<{nested_struct.name}> for web_common::database::nested_models::{nested_struct.name} {{\n"
            f"    fn from(item: {nested_struct.name}) -> Self {{\n"
            "        Self {\n"
        )
        for attribute in nested_struct.attributes:
            if attribute.optional:
                document.write(
                    f"            {attribute.name}: item.{attribute.name}.map(|item| item.into()),\n"
                )
            else:
                document.write(
                    f"            {attribute.name}: item.{attribute.name}.into(),\n"
                )
        document.write("        }\n" "    }\n" "}\n")

    document.close()

    return nested_structs


def write_web_common_nested_structs(path: str, nested_structs: List[StructMetadata]):
    """Writes the nested structs to the web_common crate."""

    # We open the file to write the nested structs
    document = open(f"../web_common/src/database/{path}", "w", encoding="utf8")
    table_metadatas = find_foreign_keys()

    # Preliminarly, we write a docstring at the very head
    # of this submodule to explain what it does and warn the
    # reader not to write anything in this file as it is
    # automatically generated.
    document.write(
        "//! This module contains the nested structs for the database tables.\n"
    )
    document.write("//!\n")
    document.write(
        "//! This file is automatically generated. Do not write anything here.\n"
    )
    document.write("\n")

    # We start with the necessary imports.
    imports = [
        "use serde::Deserialize;",
        "use serde::Serialize;",
        "use uuid::Uuid;",
        "use super::tables::*;",
        "use super::views::*;",
    ]

    for import_statement in imports:
        document.write(f"{import_statement}\n")

    for struct_metadata in nested_structs:
        struct_metadata.write_to(document)

        # We implement the `get` method for the struct when the frontend feature
        # is enabled using GlueSQL. This method will be extremely similar to the
        # `get` method for the Diesel-based approach of the backend.

        flat_struct = struct_metadata.get_flat_variant()

        document.write(
            f'#[cfg(feature = "frontend")]\n' f"impl {struct_metadata.name} {{\n"
        )

        # First, we implement the `from_flat` method that will be used to convert
        # the flat struct to the nested struct. This method receives the flat struct
        # and the connection to the database and returns the nested struct.
        document.write(
            "    /// Convert the flat struct to the nested struct.\n"
            "    ///\n"
            "    /// # Arguments\n"
            "    /// * `flat_struct` - The flat struct.\n"
            "    /// * `connection` - The database connection.\n"
            "    pub async fn from_flat(\n"
            f"        flat_struct: {flat_struct.name},\n"
            "        connection: &mut gluesql::prelude::Glue<impl gluesql::core::store::GStore + gluesql::core::store::GStoreMut>,\n"
            "    ) -> Result<Self, gluesql::prelude::Error> {\n"
            "        Ok(Self {\n"
        )
        for attribute in struct_metadata.attributes:
            if attribute.name == "inner":
                continue
            if (
                attribute.data_type() == struct_metadata.name
                or flat_struct.has_attribute(attribute)
            ):
                document.write(
                    f"            {attribute.name}: flat_struct.{attribute.name},\n"
                )
                continue
            if attribute.optional:
                document.write(
                    f"            {attribute.name}: if let Some({attribute.original_name}) = flat_struct.{attribute.original_name} {{ {attribute.data_type()}::get({attribute.original_name}, connection).await? }} else {{ None }},\n"
                )
            else:
                document.write(
                    f"            {attribute.name}: {attribute.data_type()}::get(flat_struct.{attribute.original_name}, connection).await?.unwrap(),\n"
                )

        if any(attribute.name == "inner" for attribute in struct_metadata.attributes):
            document.write(f"            inner: flat_struct,\n")

        document.write("        })\n" "    }\n")

        document.write(
            "    /// Get the nested struct from the provided primary key.\n"
            "    ///\n"
            "    /// # Arguments\n"
            f"    /// * `{flat_struct.get_formatted_primary_keys(include_prefix=False)}` - The primary key(s) of the row.\n"
            "    /// * `connection` - The database connection.\n"
            "    pub async fn get<C>(\n"
            f"        {flat_struct.get_formatted_primary_keys(include_prefix=False)}: {flat_struct.get_formatted_primary_key_data_types()},\n"
            "        connection: &mut gluesql::prelude::Glue<C>,\n"
            "    ) -> Result<Option<Self>, gluesql::prelude::Error> where\n"
            "        C: gluesql::core::store::GStore + gluesql::core::store::GStoreMut,\n"
            "    {\n"
            f"       let flat_struct = {flat_struct.name}::get({flat_struct.get_formatted_primary_keys(include_prefix=False)}, connection).await?;"
            "        match flat_struct {\n"
            "            Some(flat_struct) => Ok(Some(Self::from_flat(flat_struct, connection).await?)),\n"
            "            None => Ok(None),\n"
            "        }\n"
            "    }\n"
        )

        # We implement the all method for the struct when the frontend feature is enabled
        # using GlueSQL. This method will be extremely similar to the `all` method for the
        # Diesel-based approach of the backend.

        document.write(
            "    /// Get all the nested structs from the database.\n"
            "    ///\n"
            "    /// # Arguments\n"
            "    /// * `limit` - The maximum number of rows to return.\n"
            "    /// * `offset` - The number of rows to skip.\n"
            "    /// * `connection` - The database connection.\n"
            "    pub async fn all<C>(\n"
            "        limit: Option<i64>,\n"
            "        offset: Option<i64>,\n"
            "        connection: &mut gluesql::prelude::Glue<C>,\n"
            "    ) -> Result<Vec<Self>, gluesql::prelude::Error> where\n"
            "        C: gluesql::core::store::GStore + gluesql::core::store::GStoreMut,\n"
            "    {\n"
            f"        let flat_structs = {flat_struct.name}::all(limit, offset, connection).await?;\n"
            "         let mut nested_structs = Vec::with_capacity(flat_structs.len());\n"
            "         for flat_struct in flat_structs {\n"
            "             nested_structs.push(Self::from_flat(flat_struct, connection).await?);\n"
            "         }\n"
            "         Ok(nested_structs)\n"
            "    }\n"
        )

        # We implement the all_by_updated_at method for the struct when the frontend feature
        # is enabled using GlueSQL. This method will be extremely similar to the `all_by_updated_at`
        # method for the Diesel-based approach of the backend.

        if table_metadatas.has_updated_at_column(flat_struct.table_name):
            document.write(
                "    /// Get all the nested structs from the database ordered by the `updated_at` column.\n"
                "    ///\n"
                "    /// # Arguments\n"
                "    /// * `limit` - The maximum number of rows to return.\n"
                "    /// * `offset` - The number of rows to skip.\n"
                "    /// * `connection` - The database connection.\n"
                "    pub async fn all_by_updated_at<C>(\n"
                "        limit: Option<i64>,\n"
                "        offset: Option<i64>,\n"
                "        connection: &mut gluesql::prelude::Glue<C>,\n"
                "    ) -> Result<Vec<Self>, gluesql::prelude::Error> where\n"
                "        C: gluesql::core::store::GStore + gluesql::core::store::GStoreMut,\n"
                "    {\n"
                f"        let flat_structs = {flat_struct.name}::all_by_updated_at(limit, offset, connection).await?;\n"
                "         let mut nested_structs = Vec::with_capacity(flat_structs.len());\n"
                "         for flat_struct in flat_structs {\n"
                "             nested_structs.push(Self::from_flat(flat_struct, connection).await?);\n"
                "         }\n"
                "         Ok(nested_structs)\n"
                "    }\n"
            )

        # We implement the update_or_insert method for the struct when the frontend feature
        # is enabled using GlueSQL. This method will be extremely similar to the `update_or_insert`
        # for the flat version of the struct, with the important difference that we will call it
        # on all of its attributes that are nested structs.

        document.write(
            "    /// Update or insert the nested struct into the database.\n"
            "    ///\n"
            "    /// # Arguments\n"
            "    /// * `connection` - The database connection.\n"
            "    pub async fn update_or_insert<C>(\n"
            "        self,\n"
            "        connection: &mut gluesql::prelude::Glue<C>,\n"
            "    ) -> Result<(), gluesql::prelude::Error> where\n"
            "        C: gluesql::core::store::GStore + gluesql::core::store::GStoreMut,\n"
            "    {\n"
        )
        for attribute in struct_metadata.attributes:
            assert isinstance(attribute.raw_data_type(), StructMetadata)

            if attribute.optional:
                document.write(
                    f"        if let Some({attribute.name}) = self.{attribute.name} {{\n"
                    f"            {attribute.name}.update_or_insert(connection).await?;\n"
                    "        }\n"
                )
            else:
                document.write(
                    f"        self.{attribute.name}.update_or_insert(connection).await?;\n"
                )

        document.write("        Ok(())\n" "    }\n")

        document.write("}\n")

    document.close()


def write_webcommons_table_names_enumeration(
    struct_metadatas: List[StructMetadata],
    new_model_structs: List[StructMetadata],
    update_model_structs: List[StructMetadata],
    tables_metadata: TableMetadata,
) -> List[TableStructMetadata]:
    imports = [
        "use serde::Deserialize;",
        "use serde::Serialize;",
    ]

    # The derives to apply to the structs in the `src/database/tables.rs` document
    derives = ["Deserialize", "Serialize", "Clone", "Debug", "PartialEq", "Eq", "Copy"]

    # We check that we are currently executing in the `backend` crate
    # so to make sure that the relative path to the `web_common` crate
    # is correct.
    if not os.getcwd().endswith("backend"):
        raise Exception("This script must be executed in the `backend` crate.")

    document = open(f"../web_common/src/database/table_names.rs", "w", encoding="utf8")

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
            tables[struct.table_name].set_flat_struct(struct)

    # We set the new flat model struct variant for each of the tables,
    # when it is available.
    for struct in new_model_structs:
        assert struct.table_name in tables, f"Table {struct.table_name} not found."
        assert struct.is_new_variant(), (
            f"Struct {struct.name} is not a new variant. "
            f"Expected a new variant for table {struct.table_name}. "
            f"Its flat variant is {struct.get_flat_variant().name}."
        )
        tables[struct.table_name].set_new_flat_struct(struct)
        struct.set_richest_variant(tables[struct.table_name].get_richest_struct())

    # We set the update flat model struct variant for each of the tables,
    # when it is available.
    for struct in update_model_structs:
        assert struct.table_name in tables, f"Table {struct.table_name} not found."
        assert struct.is_update_variant()
        tables[struct.table_name].set_update_flat_struct(struct)
        struct.set_richest_variant(tables[struct.table_name].get_richest_struct())

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
    document.write("impl AsRef<str> for Table {\n")
    document.write("    fn as_ref(&self) -> &str {\n")
    document.write("        match self {\n")
    for table in tables:
        document.write(f'            Table::{table.camel_cased()} => "{table.name}",\n')
    document.write("        }\n")
    document.write("    }\n")
    document.write("}\n")

    # We implement display

    document.write("impl std::fmt::Display for Table {\n")
    document.write(
        "    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {\n"
    )
    document.write('        write!(f, "{}", self.as_ref())\n')
    document.write("    }\n")
    document.write("}\n")

    # We implement the conversion From<Table> for String

    document.write("impl From<Table> for String {\n")
    document.write("    fn from(table: Table) -> Self {\n")
    document.write("        table.to_string()\n")
    document.write("    }\n")
    document.write("}\n")

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
    document.write("        }\n")
    document.write("    }\n")
    document.write("}\n")

    # We implement the TryFrom trait from String to Table

    document.write("impl std::convert::TryFrom<String> for Table {\n")
    document.write("    type Error = String;\n")
    document.write("    fn try_from(value: String) -> Result<Self, Self::Error> {\n")
    document.write("        Self::try_from(value.as_str())\n")
    document.write("    }\n")
    document.write("}\n")

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
        "    ) -> Result<usize, gluesql::prelude::Error> where\n"
        "        C: gluesql::core::store::GStore + gluesql::core::store::GStoreMut,\n"
        "    {\n"
        "        match self {\n"
    )

    for table in tables:
        document.write(
            f"            Table::{table.camel_cased()} => {{\n"
            f"                crate::database::{table.flat_struct_name()}::delete_from_id(primary_key.into(), connection).await\n"
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
        "    /// * `limit` - The maximum number of rows to return.\n"
        "    /// * `offset` - The number of rows to skip. By default `0`.\n"
        "    /// * `connection` - The database connection.\n"
        "    ///\n"
        "    /// # Returns\n"
        "    /// A vector of the rows of the table.\n"
        "    pub async fn all<C>(\n"
        "        &self,\n"
        "        limit: Option<i64>,\n"
        "        offset: Option<i64>,\n"
        "        connection: &mut gluesql::prelude::Glue<C>,\n"
        "    ) -> Result<Vec<Vec<u8>>, crate::api::ApiError> where\n"
        "        C: gluesql::core::store::GStore + gluesql::core::store::GStoreMut,\n"
        "    {\n"
        "        match self {\n"
    )

    for table in tables:
        document.write(
            f"            Table::{table.camel_cased()} => crate::database::{table.richest_struct_name()}::all(limit, offset, connection).await?.into_iter().map(|row| bincode::serialize(&row).map_err(crate::api::ApiError::from)).collect(),\n"
        )

    document.write("        }\n    }\n")

    # Next, for all the tables that have an updated_at column, we implement the
    # `all_by_updated_at` method, which returns all of the rows ordered by the
    # `updated_at` column. When the table does not have an `updated_at` column,
    # we panic with an unimplemented!() macro.

    document.write(
        "    /// Get all the rows from the table ordered by the `updated_at` column.\n"
        "    ///\n"
        "    /// # Arguments\n"
        "    /// * `limit` - The maximum number of rows to return.\n"
        "    /// * `offset` - The number of rows to skip. By default `0`.\n"
        "    /// * `connection` - The database connection.\n"
        "    ///\n"
        "    /// # Returns\n"
        "    /// A vector of the rows of the table.\n"
        "    pub async fn all_by_updated_at<C>(\n"
        "        &self,\n"
        "        limit: Option<i64>,\n"
        "        offset: Option<i64>,\n"
        "        connection: &mut gluesql::prelude::Glue<C>,\n"
        "    ) -> Result<Vec<Vec<u8>>, crate::api::ApiError> where\n"
        "        C: gluesql::core::store::GStore + gluesql::core::store::GStoreMut,\n"
        "    {\n"
        "        match self {\n"
    )

    for table in tables:
        if table.has_updated_at_column():
            document.write(
                f"            Table::{table.camel_cased()} => crate::database::{table.richest_struct_name()}::all_by_updated_at(limit, offset, connection).await?.into_iter().map(|row| bincode::serialize(&row).map_err(crate::api::ApiError::from)).collect(),\n"
            )
        else:
            document.write(
                f'            Table::{table.camel_cased()} => unimplemented!("all_by_updated_at not implemented for {table.name}."),\n'
            )

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

        if table.name in TEMPORARELY_IGNORED_TABLES:
            document.write(
                f'            Table::{table.camel_cased()} => todo!("Insert not implemented for {table.name}."),\n'
            )
            continue

        document.write(
            f"            Table::{table.camel_cased()} => {{\n"
            f"                let new_row: super::{table.new_flat_struct_name()} = bincode::deserialize::<super::{table.new_flat_struct_name()}>(&new_row).map_err(crate::api::ApiError::from)?;\n"
            f"                let inserted_row: super::{table.flat_struct_name()} = new_row.insert(user_id, connection).await?;\n"
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

    document.write("})\n")

    document.write("    }\n")

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

        if table.name in TEMPORARELY_IGNORED_TABLES:
            document.write(
                f'            Table::{table.camel_cased()} => todo!("Update not implemented for {table.name}."),\n'
            )
            continue

        flat_struct: StructMetadata = table.get_flat_variant()

        document.write(
            f"            Table::{table.camel_cased()} => {{\n"
            f"                let update_row: super::{table.update_flat_struct_name()} = bincode::deserialize::<super::{table.update_flat_struct_name()}>(&update_row).map_err(crate::api::ApiError::from)?;\n"
            f"                let {flat_struct.get_formatted_primary_keys(include_prefix=False)} = {flat_struct.get_formatted_primary_keys(include_prefix=True, prefix='update_row')};\n"
            f"                update_row.update("
        )

        if table.name != "users":
            document.write("user_id, ")

        document.write("connection).await?;\n")

        document.write(
            f"                let updated_row: super::{table.flat_struct_name()} = super::{table.flat_struct_name()}::get({flat_struct.get_formatted_primary_keys(include_prefix=False)}, connection).await?.unwrap();\n"
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

    document.write("})\n")

    document.write("    }\n")

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

    document.write("        }\n    Ok(())}\n")

    document.write("}\n")

    document.flush()
    document.close()

    return tables


def write_diesel_table_names_enumeration(
    tables: List[TableStructMetadata],
):
    # We check that we are currently executing in the `backend` crate
    # so to make sure that the relative path to the `web_common` crate
    # is correct.
    if not os.getcwd().endswith("backend"):
        raise Exception("This script must be executed in the `backend` crate.")

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
        "use crate::views::*;",
        "use diesel::r2d2::PooledConnection;",
        "use diesel::r2d2::ConnectionManager;",
        "use crate::new_variants::InsertRow;",
        "use crate::update_variants::UpdateRow;",
    ]

    for import_statement in imports:
        document.write(f"{import_statement}\n")

    document.write("\n")

    search_indices: PGIndices = find_pg_trgm_indices()

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
            ") -> Result<Vec<Vec<u8>>, web_common::api::ApiError>;\n"
        )
    document.write("}\n\n")

    document.write("impl SearchableTable for web_common::database::Table {\n")
    for similarity_method, _, _ in PGIndices.SIMILARITY_METHODS:
        document.write(
            f"    fn {similarity_method}_search(&self, query: &str, limit: Option<i32>, connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>) -> Result<Vec<Vec<u8>>, web_common::api::ApiError> {{\n"
            "        match self {\n"
        )

        for table in tables:
            if search_indices.has_table(table.name):
                document.write(
                    f"            web_common::database::Table::{table.camel_cased()} => {table.richest_struct_name()}::{similarity_method}_search(query, limit, connection)?.iter().map(|row| bincode::serialize(row).map_err(web_common::api::ApiError::from)).collect(),\n"
                )
            else:
                document.write(
                    f'            web_common::database::Table::{table.camel_cased()} => unimplemented!("Table `{table.name}` does not have a GIN similarity index."),\n'
                )

        document.write("        }\n" "    }\n")
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

    document.write("        })\n" "    }\n" "}\n")

    # We implement the `delete` method for the Table enum, which receives a primary key
    # enum and a connection to the database. The method returns a Result, where the Ok variant
    # is the number of rows deleted, while the Err variant contains an ApiError. The delete
    # method is available for all tables with a primary key - tables that do not have a primary
    # key will raise a panic with the unimplemented!() macro.

    document.write(
        "/// Trait providing the delete method for the Table enum.\n"
        "pub trait DeletableTable {\n"
        "    /// Delete the row from the table by the primary key.\n"
        "    ///\n"
        "    /// # Arguments\n"
        "    /// * `primary_key` - The primary key of the row.\n"
        "    /// * `connection` - The database connection.\n"
        "    ///\n"
        "    /// # Returns\n"
        "    /// The number of rows deleted.\n"
        "    fn delete(\n"
        "         &self,\n"
        "         primary_key: web_common::database::operations::PrimaryKey,\n"
        "         connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>\n"
        ") -> Result<usize, web_common::api::ApiError>;\n"
        "}\n\n"
    )

    document.write(
        "impl DeletableTable for web_common::database::Table {\n\n"
        "    fn delete(\n"
        "        &self,\n"
        "        primary_key: web_common::database::operations::PrimaryKey,\n"
        "        connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>\n"
        "    ) -> Result<usize, web_common::api::ApiError> {\n"
        "        Ok(match self {\n"
    )

    for table in tables:
        document.write(
            f"            web_common::database::Table::{table.camel_cased()} => {table.flat_struct_name()}::delete_by_id(primary_key.into(), connection)?,\n"
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
        "    /// * `limit` - The maximum number of rows to return.\n"
        "    /// * `offset` - The number of rows to skip.\n"
        "    /// * `connection` - The database connection.\n"
        "    ///\n"
        "    /// # Returns\n"
        "    /// A vector of the rows of the table.\n"
        "    fn all(\n"
        "         &self,\n"
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
        "        limit: Option<i64>,\n"
        "        offset: Option<i64>,\n"
        "        connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>\n"
        "    ) -> Result<Vec<Vec<u8>>, web_common::api::ApiError> {\n"
        "        match self {\n"
    )

    for table in tables:
        document.write(
            f"            web_common::database::Table::{table.camel_cased()} => {table.richest_struct_name()}::all(limit, offset, connection)?.iter().map(|row| bincode::serialize(row).map_err(web_common::api::ApiError::from)).collect(),\n"
        )

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
        "    /// * `limit` - The maximum number of rows to return.\n"
        "    /// * `offset` - The number of rows to skip.\n"
        "    /// * `connection` - The database connection.\n"
        "    ///\n"
        "    /// # Returns\n"
        "    /// A vector of the rows of the table.\n"
        "    fn all_by_updated_at(\n"
        "         &self,\n"
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
        "        limit: Option<i64>,\n"
        "        offset: Option<i64>,\n"
        "        connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>\n"
        "    ) -> Result<Vec<Vec<u8>>, web_common::api::ApiError> {\n"
        "        match self {\n"
    )

    for table in tables:
        if table.has_updated_at_column():
            document.write(
                f"            web_common::database::Table::{table.camel_cased()} => {table.richest_struct_name()}::all_by_updated_at(limit, offset, connection)?.iter().map(|row| bincode::serialize(row).map_err(web_common::api::ApiError::from)).collect(),\n"
            )
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

        if table.name in TEMPORARELY_IGNORED_TABLES:
            document.write(
                f'            web_common::database::Table::{table.camel_cased()} => todo!("Insert not implemented for {table.name}."),\n'
            )
            continue

        document.write(
            f"            web_common::database::Table::{table.camel_cased()} => {{\n"
            f"                let row: web_common::database::{table.new_flat_struct_name()} = bincode::deserialize::<web_common::database::{table.new_flat_struct_name()}>(&row).map_err(web_common::api::ApiError::from)?;\n"
            f"                let inserted_row: crate::models::{table.flat_struct_name()} = <web_common::database::{table.new_flat_struct_name()} as InsertRow>::insert(row, user_id, connection)?;\n"
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

        if table.name in TEMPORARELY_IGNORED_TABLES:
            document.write(
                f'            web_common::database::Table::{table.camel_cased()} => todo!("Update not implemented for {table.name}."),\n'
            )
            continue

        document.write(
            f"            web_common::database::Table::{table.camel_cased()} => {{\n"
            f"                let row: web_common::database::{table.update_flat_struct_name()} = bincode::deserialize::<web_common::database::{table.update_flat_struct_name()}>(&row).map_err(web_common::api::ApiError::from)?;\n"
            f"                let updated_row: crate::models::{table.flat_struct_name()} = <web_common::database::{table.update_flat_struct_name()} as UpdateRow>::update(row, user_id, connection)?;\n"
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

    document.write("})\n")

    document.write("    }\n")

    document.write("}\n")

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
                f"            web_common::database::Table::{table.camel_cased()} => bincode::serialize(&serde_json::from_str::<crate::models::{table.flat_struct_name()}>(row).map_err(web_common::api::ApiError::from)?).map_err(web_common::api::ApiError::from)?,\n"
            )
            continue

        document.write(
            f"            web_common::database::Table::{table.camel_cased()} => {{\n"
            f"                let flat_row: crate::models::{flat_variant.name} = serde_json::from_str::<crate::models::{flat_variant.name}>(row).map_err(web_common::api::ApiError::from)?;\n"
            f"                let richest_row = crate::nested_models::{richest_variant.name}::from_flat(flat_row, connection)?;\n"
            "                 bincode::serialize(&richest_row).map_err(web_common::api::ApiError::from)?\n"
            "            },\n"
        )

    document.write("        })\n" "    }\n" "}\n")

    document.flush()
    document.close()


def write_web_common_search_trait_implementations(
    struct_metadatas: List[StructMetadata],
):
    # We check that we are currently executing in the `backend` crate
    # so to make sure that the relative path to the `web_common` crate
    # is correct.
    if not os.getcwd().endswith("backend"):
        raise Exception("This script must be executed in the `backend` crate.")

    document = open(
        f"../web_common/src/database/search_tables.rs", "w", encoding="utf8"
    )
    similarity_indices: PGIndices = find_pg_trgm_indices()

    imports = [
        "use crate::database::*;",
    ]

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

    # First, we create the Searchable trait that will be implemented by all the structs
    # that are searchable.

    document.write(
        "pub trait Searchable {\n"
        "    fn search_task(query: String, limit: u32) -> super::Select;\n"
        "}\n"
    )

    for struct in struct_metadatas:
        if similarity_indices.has_table(struct.table_name):
            document.write(
                f"impl Searchable for {struct.name} {{\n"
                "    fn search_task(query: String, limit: u32) -> super::Select {\n"
                f"        super::Select::search(\n"
                f"             Table::{struct.capitalized_table_name()},\n"
                "              query,\n"
                "              limit,\n"
                "        )\n"
                "    }\n"
                "}\n"
            )

    document.flush()
    document.close()


def derive_new_models(
    struct_metadatas: List[StructMetadata],
    table_metadatas: TableMetadata,
) -> List[StructMetadata]:
    """Returns list of the New{Model} structs.

    Parameters
    ----------
    struct_metadatas : List[StructMetadata]
        The list of the StructMetadata objects.

    Implementation details
    ----------------------
    The New variants are used to either insert data in the database or,
    when used in the frontend, employed as base for the html Forms.
    The primary difference between the New variant and the original
    variant is that the New variant does not contain the primary key
    attribute and the automatically determined columns, such as the
    created_at and updated_at columns, unless the primary key is a UUID.
    """
    # Temporary list of tables for which to refrain from generating
    # the form component.

    for table_name in TEMPORARELY_IGNORED_TABLES:
        if not table_metadatas.is_table(table_name):
            raise RuntimeError(
                f"Table {table_name} is in the deny list but does not exist in the database."
            )

    new_structs = []

    for struct in tqdm(
        struct_metadatas,
        desc="Deriving new structs",
        unit="struct",
        leave=False,
    ):
        if not struct.is_insertable():
            continue

        if struct.is_junktion_table():
            continue

        if struct.table_name in TEMPORARELY_IGNORED_TABLES:
            continue

        assert not struct.is_nested()

        # If the struct has a single attribute,
        # we do not derive a New variant.
        if len(struct.attributes) == 1:
            continue

        new_struct = StructMetadata(
            struct_name=f"New{struct.name}",
            table_name=struct.table_name,
            table_metadata=table_metadatas,
        )

        new_struct.set_flat_variant(struct)

        primary_keys = struct.get_primary_keys()

        for derive in struct.derives():
            new_struct.add_derive(derive)

        for attribute in struct.attributes:
            if attribute.is_automatically_determined_column():
                continue

            if attribute in primary_keys:
                if len(primary_keys) > 1:
                    continue
                if attribute.is_uuid():
                    new_struct.add_attribute(attribute)
                    continue
                continue
            new_struct.add_attribute(attribute)

        new_structs.append(new_struct)

    return new_structs


def derive_update_models(
    struct_metadatas: List[StructMetadata],
) -> List[StructMetadata]:
    """Returns list of the Update{Model} structs.

    Parameters
    ----------
    struct_metadatas : List[StructMetadata]
        The list of the StructMetadata objects.

    Implementation details
    ----------------------
    The Update variants are used to update data in the database.
    The primary difference between the Update variant and the original
    variant is that the Update variant does not contain columns relative
    to the automatically determined columns, such as the created_at and
    updated_at columns. It does contain the primary key column, and as
    such in the case of a UUID primary key, the Update variant is for all
    intents and purposes identical to the New variant, and as such there
    is no need to derive an Update variant.
    """
    update_structs = []

    for struct in tqdm(
        struct_metadatas,
        desc="Deriving update structs",
        unit="struct",
        leave=False,
    ):
        if not struct.is_updatable():
            continue

        if struct.is_junktion_table():
            continue

        if struct.table_name in TEMPORARELY_IGNORED_TABLES:
            continue

        assert not struct.is_nested()

        # If the struct has a single attribute,
        # we do not derive an Update variant.
        if len(struct.attributes) == 1:
            continue

        primary_keys = struct.get_primary_keys()

        if len(primary_keys) == 1 and primary_keys[0].is_uuid():
            continue

        update_struct = StructMetadata(
            struct_name=f"Update{struct.name}",
            table_name=struct.table_name,
            table_metadata=struct.table_metadata,
        )

        update_struct.set_flat_variant(struct)

        for derive in struct.derives():
            update_struct.add_derive(derive)

        for attribute in struct.attributes:
            if attribute.is_automatically_determined_column():
                continue
            update_struct.add_attribute(attribute)

        update_structs.append(update_struct)

    return update_structs


def derive_model_builders(
    new_or_update_struct_metadatas: List[StructMetadata],
) -> List[StructMetadata]:
    """Returns list of the {struct_name}Builder structs.

    Parameters
    ----------
    new_or_update_struct_metadatas : List[StructMetadata]
        The list of the StructMetadata objects.
        This list includes both the New and Update variants.

    Implementation details
    ----------------------
    The {struct_name}Builder structs are used to build either the New{struct_name} or Update{stuct_name} structs.
    Since they are builders, they contain option variants of the attributes
    of the {struct_name} structs. When the original attribute is already optional (Option<T>)
    the {struct_name}Builder attribute is not doubly optional (Option<Option<T>>), but simply optional
    as the original attribute (Option<T>).
    """
    builders = []

    deny_list_tables = [
        "user_emails",
    ]

    for struct in tqdm(
        new_or_update_struct_metadatas,
        desc="Deriving builders",
        unit="struct",
        leave=False,
    ):
        assert not struct.is_nested()
        assert struct.is_new_variant() or struct.is_update_variant()

        if struct.table_name in deny_list_tables:
            continue

        if struct.is_update_variant() and not struct.is_new_variant():
            found = False
            for builder in builders:
                if builder.table_name == struct.table_name:
                    found = True
                    builder.set_update_variant(struct)
                    break

            if found:
                continue

        flat_variant = struct.get_flat_variant()
        richest_variant = struct.get_richest_variant()

        builder = StructMetadata(
            struct_name=f"{flat_variant.name}Builder",
            table_name=struct.table_name,
            table_metadata=struct.table_metadata,
        )

        builder.set_flat_variant(flat_variant)
        builder.set_richest_variant(richest_variant)

        if struct.is_new_variant():
            builder.set_new_variant(struct)

        if struct.is_update_variant():
            builder.set_update_variant(struct)

        builder.add_derive("Store")
        for derive in richest_variant.derives():
            builder.add_derive(derive)

        builder.add_decorator('store(storage = "local", storage_tab_sync)')

        foreign_keys = flat_variant.get_foreign_keys()
        primary_keys = flat_variant.get_primary_keys()

        assert len(primary_keys) == 1

        attribute = copy.deepcopy(primary_keys[0])
        attribute.optional = True
        builder.add_attribute(attribute)
        
        for attribute in flat_variant.attributes:
            if attribute in foreign_keys or attribute in primary_keys:
                continue

            if attribute.is_automatically_determined_column():
                continue

            attribute = copy.deepcopy(attribute)
            attribute.optional = True
            builder.add_attribute(attribute)

        if len(foreign_keys) > 0:
            assert richest_variant.is_nested()

        if richest_variant.is_nested():
            for attribute in richest_variant.attributes:

                if attribute.name == "inner":
                    continue

                if attribute.is_automatically_determined_column():
                    continue

                if attribute.data_type() == flat_variant.name:
                    builder.add_attribute(
                        AttributeMetadata(
                            original_name=attribute.original_name,
                            name=attribute.name,
                            data_type=richest_variant,
                            optional=True,
                        )
                    )
                    continue

                attribute = copy.deepcopy(attribute)
                attribute.optional = True
                builder.add_attribute(attribute)

        # For each attribute, for add new parameters that are vectors
        # for ApiError and collect the errors associated to the specific
        # attribute. This is useful for the frontend, where we can display
        # the errors associated to the specific attribute. In order to
        # easily distinguish these fields from the other fields and avoid
        # name clashes, we prefix the attribute name with `errors_`.
        new_attributes = []
        for attribute in builder.attributes:
            if attribute in primary_keys:
                continue

            new_attributes.append(
                AttributeMetadata(
                    original_name=attribute.original_name,
                    name=f"errors_{attribute.name}",
                    data_type="Vec<ApiError>",
                    optional=False,
                )
            )

        for attribute in new_attributes:
            builder.add_attribute(attribute)

        # Finally, we add an attribute to the builder to be used primarily
        # by yew to detect when the object has been updated. This is needed
        # for cases when the attempted update is invalid, and the data inserted
        # in the form needs to be resetted to the original values. In this attribute
        # we store the datetime of the last update of the object.
        builder.add_attribute(
            AttributeMetadata(
                original_name="form_updated_at",
                name="form_updated_at",
                data_type="NaiveDateTime",
                optional=False,
            )
        )

        # Self-consistency check - all of the foreign keys must appear in the builder,
        # eventually in the normalized version.

        for foreign_key in foreign_keys:

            if foreign_key.is_automatically_determined_column():
                continue

            found = False
            for attribute in builder.attributes:
                if attribute.original_name == foreign_key.name:
                    found = True
                    break

            if not found:
                raise RuntimeError(
                    f"Could not find the foreign key {foreign_key.name} in the builder {builder.name}. "
                    f"The attributes of the flat variant are {flat_variant.attributes}. "
                    f"The attributes of the richest variant are {richest_variant.attributes}."
                )

        builders.append(builder)

    # We run a simple self-consistency check to ensure that the
    # builders have been correctly derived.
    for struct in new_or_update_struct_metadatas:

        if struct.table_name in deny_list_tables:
            continue

        # We identify the curresponding builder by the matching table name.
        found = False
        for builder in builders:
            if builder.table_name == struct.table_name:
                found = True
                break

        if not found:
            raise RuntimeError(f"Could not find the builder for the struct {struct.name}.")

        if struct.is_new_variant():
            assert builder.get_new_variant() == struct

        if struct.is_update_variant():
            assert builder.get_update_variant() == struct

    return builders


def write_web_common_new_structs(
    new_struct_metadatas: List[StructMetadata],
):
    """Writes the new structs to the web_common crate."""

    # For the time being, we simply write out the structs.
    # In the near future, we will also implement several
    # traits for these structs.

    path = "../web_common/src/database/new_variants.rs"

    document = open(path, "w", encoding="utf8")

    # Preliminarly, we write a docstring at the very head
    # of this submodule to explain what it does and warn the
    # reader not to write anything in this file as it is
    # automatically generated.

    document.write(
        "//! This module contains the new variants of the database models.\n"
        "//!\n"
        "//! This module is automatically generated. Do not write anything here.\n\n"
    )

    imports = [
        "use uuid::Uuid;",
        "use serde::{Deserialize, Serialize};",
        "use chrono::NaiveDateTime;",
    ]

    for import_statement in imports:
        document.write(f"{import_statement}\n")

    document.write("\n")

    table_metadatas = find_foreign_keys()

    for struct in tqdm(
        new_struct_metadatas,
        desc="Writing new structs",
        unit="struct",
        leave=False,
    ):
        assert struct.is_new_variant()

        struct.write_to(document)

        if not struct.has_uuid_primary_key():
            continue

        # When the frontend flag is enables, we implement the insert method for the new flat struct.
        # This method receives the user id of the user inserting the row and the connection to the database.
        # The method returns a Result, where the Ok variant is the flat variant of the struct associated with
        # the newly inserted row, while the Err variant contains an ApiError.

        # First thing, we determine the name of the attribute that contains the user id. This is not directly
        # an attribute of the new variant, as it is not set by the user, but it is available as part of the
        # associated flat variant.
        creator_user_id_attribute: AttributeMetadata = (
            struct.get_creator_user_id_attribute()
        )

        updator_user_id_attribute: AttributeMetadata = (
            struct.get_updator_user_id_attribute()
        )

        columns = []

        document.write(f'#[cfg(feature = "frontend")]\n' f"impl {struct.name} {{\n")

        document.write(
            f"    pub fn into_row(self, {creator_user_id_attribute.name}: {creator_user_id_attribute.format_data_type()}) -> Vec<gluesql::core::ast_builder::ExprNode<'static>> {{\n"
        )

        document.write("        vec![\n")
        for attribute in [
            creator_user_id_attribute,
            updator_user_id_attribute,
        ] + struct.attributes:
            columns.append(attribute.name)

            if attribute.name in (
                creator_user_id_attribute.name,
                updator_user_id_attribute.name,
            ):
                self_attribute_name = creator_user_id_attribute.name
            else:
                self_attribute_name = f"self.{attribute.name}"

            if attribute.optional:
                if attribute.data_type() in GLUESQL_TYPES_MAPPING:
                    document.write(
                        f"            match {self_attribute_name} {{\n"
                        f"                Some({attribute.name}) => {GLUESQL_TYPES_MAPPING[attribute.data_type()].format(attribute.name)},\n"
                        "                None => gluesql::core::ast_builder::null(),\n"
                        "            },\n"
                    )
                else:
                    raise NotImplementedError(
                        f"The type {attribute.data_type()} is not supported. "
                        f"The struct {struct.name} contains an {attribute.data_type()}. "
                    )
            elif attribute.data_type() in GLUESQL_TYPES_MAPPING:
                document.write(
                    f"            {GLUESQL_TYPES_MAPPING[attribute.data_type()].format(self_attribute_name)},\n"
                )
            else:
                raise NotImplementedError(
                    f"The type {attribute.data_type()} is not supported."
                )

        document.write("        ]\n    }\n\n")

        # We implement the `insert` method for the struct. This method
        # receives a connection to the GlueSQL database and inserts the
        # struct into the database.
        document.write(
            f"    /// Insert the {struct.name} into the database.\n"
            "    ///\n"
            "    /// # Arguments\n"
            f"    /// * `{creator_user_id_attribute.name}` - The id of the user inserting the row.\n"
            "    /// * `connection` - The connection to the database.\n"
            "    ///\n"
            "    /// # Returns\n"
            f"    /// The number of rows inserted in table {struct.name}\n"
            "    pub async fn insert<C>(\n"
            "        self,\n"
            f"        {creator_user_id_attribute.name}: {creator_user_id_attribute.format_data_type()},\n"
            "        connection: &mut gluesql::prelude::Glue<C>,\n"
            f"    ) -> Result<super::{struct.get_flat_variant().name}, gluesql::prelude::Error> where\n"
            "        C: gluesql::core::store::GStore + gluesql::core::store::GStoreMut,\n"
            "    {\n"
            "        use gluesql::core::ast_builder::*;\n"
            f"        let {struct.get_formatted_primary_keys(include_prefix=False)} = {struct.get_formatted_primary_keys(include_prefix=True)};\n"
            f'        table("{struct.table_name}")\n'
            "            .insert()\n"
            f'            .columns("{",".join(columns)}")\n'
            f"            .values(vec![self.into_row({creator_user_id_attribute.name})])\n"
            "            .execute(connection)\n"
            "            .await\n"
            "             .map(|payload| match payload {\n"
            "                 gluesql::prelude::Payload::Insert ( number_of_inserted_rows ) => number_of_inserted_rows,\n"
            '                 _ => unreachable!("Payload must be an Insert"),\n'
            "             })?;\n"
            f"        super::{struct.get_flat_variant().name}::get({struct.get_formatted_primary_keys(include_prefix=False)}, connection).await.map(|maybe_row| maybe_row.unwrap())\n"
            "    }\n\n"
        )

        if struct.is_updatable():
            write_update_method_for_gluesql(struct, document)

        document.write("}\n")

    document.flush()
    document.close()


def write_web_common_update_structs(
    update_struct_metadatas: List[StructMetadata],
):
    """Writes the update structs to the web_common crate."""

    # For the time being, we simply write out the structs.
    # In the near future, we will also implement several
    # traits for these structs.

    path = "../web_common/src/database/update_variants.rs"

    document = open(path, "w", encoding="utf8")

    # Preliminarly, we write a docstring at the very head
    # of this submodule to explain what it does and warn the
    # reader not to write anything in this file as it is
    # automatically generated.

    document.write(
        "//! This module contains the update variants of the database models.\n"
        "//!\n"
        "//! Some of the update variants would be identical to the new variants, "
        "//! and as such we do not generate them. You will find here the update variants "
        "//! only for the tables that have a primary key that is not a UUID.\n"
        "//! This module is automatically generated. Do not write anything here.\n\n"
    )

    imports = [
        "use serde::{Deserialize, Serialize};",
        "use chrono::NaiveDateTime;",
    ]

    for import_statement in imports:
        document.write(f"{import_statement}\n")

    document.write("\n")

    table_metadatas = find_foreign_keys()

    for struct in tqdm(
        update_struct_metadatas,
        desc="Writing update structs",
        unit="struct",
        leave=False,
    ):
        assert struct.is_update_variant()

        struct.write_to(document)

        # When the frontend flag is enables, we implement the insert method for the new flat struct.
        # This method receives the user id of the user inserting the row and the connection to the database.
        # The method returns a Result, where the Ok variant is the flat variant of the struct associated with
        # the newly inserted row, while the Err variant contains an ApiError.

        # First thing, we determine the name of the attribute that contains the user id. This is not directly
        # an attribute of the new variant, as it is not set by the user, but it is available as part of the
        # associated flat variant.

        attributes = struct.attributes

        if struct.table_name != "users":
            updator_user_id_attribute: AttributeMetadata = (
                struct.get_updator_user_id_attribute()
            )
            attributes = [updator_user_id_attribute] + attributes
        else:
            updator_user_id_attribute = None

        document.write(f'#[cfg(feature = "frontend")]\n' f"impl {struct.name} {{\n")

        if struct.table_name != "users":
            document.write(
                f"    pub fn into_row(self, {updator_user_id_attribute.name}: {updator_user_id_attribute.format_data_type()}) -> Vec<gluesql::core::ast_builder::ExprNode<'static>> {{\n"
            )
        else:
            document.write(
                "    pub fn into_row(self) -> Vec<gluesql::core::ast_builder::ExprNode<'static>> {\n"
            )

        document.write("        vec![\n")

        for attribute in attributes:

            if struct.get_attribute_by_name(attribute.name) is not None:
                self_attribute_name = f"self.{attribute.name}"
            else:
                self_attribute_name = attribute.name

            if attribute.optional:
                if attribute.data_type() in GLUESQL_TYPES_MAPPING:
                    document.write(
                        f"            match {self_attribute_name} {{\n"
                        f"                Some({attribute.name}) => {GLUESQL_TYPES_MAPPING[attribute.data_type()].format(attribute.name)},\n"
                        "                None => gluesql::core::ast_builder::null(),\n"
                        "            },\n"
                    )
                else:
                    raise NotImplementedError(
                        f"The type {attribute.data_type()} is not supported. "
                        f"The struct {struct.name} contains an {attribute.data_type()}. "
                    )
            elif attribute.data_type() in GLUESQL_TYPES_MAPPING:
                document.write(
                    f"            {GLUESQL_TYPES_MAPPING[attribute.data_type()].format(self_attribute_name)},\n"
                )
            else:
                raise NotImplementedError(
                    f"The type {attribute.data_type()} is not supported."
                )

        document.write("        ]\n    }\n\n")

        # We implement the `update` method for the struct. This method
        # receives a connection to the GlueSQL database and updates the
        # struct into the database.
        write_update_method_for_gluesql(struct, document)

        document.write("}\n")

    document.flush()
    document.close()


def write_diesel_new_structs(
    new_struct_metadatas: List[StructMetadata],
):
    """Writes to the backend the diesel methods for the new structs.

    Parameters
    ----------
    new_struct_metadatas : List[StructMetadata]
        The list of the StructMetadata objects.
    """

    path = "./src/new_variants.rs"

    document = open(path, "w", encoding="utf8")

    # First of all, we write a docstring that warns the reader
    # not to write anything in this file as it is automatically
    # generated.

    document.write(
        "//! This module contains the new variants of the database models.\n"
        "//!\n"
        "//! This module is automatically generated. Do not write anything here.\n\n"
    )

    imports = [
        "use diesel::prelude::*;",
        "use crate::models::*;",
        "use crate::schema::*;",
        "use diesel::r2d2::PooledConnection;",
        "use diesel::r2d2::ConnectionManager;",
        "use uuid::Uuid;",
        "use chrono::NaiveDateTime;",
    ]

    for import_statement in imports:
        document.write(f"{import_statement}\n")

    document.write("\n")

    # Since the new variants are defined in the web_common crate, in order to
    # implement methods for the backend we need to import the new variants from
    # the web_common crate and define new traits for the new variants. We also
    # need to implement these traits for the new variants, of course.

    # Because of how Diesel works, we need to define new structs that are an
    # intermediate representation of the row. These structs have all of the
    # attributes of the new variant, plus the attribute associated with the
    # creator user id. These intermediate variants are private to this document
    # as they are not used outside of it. They derive the Insertable trait from
    # Diesel, which is used to insert the row in the database.

    # We start by defining the trait InsertRow, which is implemented by the new
    # variants and provides the insert method for the new variants. The insert
    # method receives the user id of the user inserting the row and the connection
    # to the database. The same trait also has an associated type, which is the
    # intermediate variant that is used to insert the row in the database, and a
    # method that receives the self and user id and returns the intermediate variant.
    # The insert method returns the newly inserted row, which is the flat variant
    # of the new flat variant.

    document.write(
        "/// Trait providing the insert method for the new variants.\n"
        "pub(super) trait InsertRow {\n"
        "    /// The intermediate representation of the row.\n"
        "    type Intermediate;\n\n"
        "    /// The flat variant of the new variant.\n"
        "    type Flat;\n\n"
        "    /// Convert the new variant into the intermediate representation.\n"
        "    fn to_intermediate(self, user_id: i32) -> Self::Intermediate;\n\n"
        "    /// Insert the row into the database.\n"
        "    fn insert(\n"
        "        self,\n"
        "        user_id: i32,\n"
        "        connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>\n"
        "    ) -> Result<Self::Flat, diesel::result::Error>;\n"
        "}\n\n"
    )

    for struct in tqdm(
        new_struct_metadatas,
        desc="Writing new structs",
        unit="struct",
        leave=False,
    ):
        assert struct.is_new_variant()

        if struct.table_name == "users":
            creator_user_id_attribute = None
        else:
            creator_user_id_attribute: AttributeMetadata = (
                struct.get_creator_user_id_attribute()
            )

            assert not creator_user_id_attribute.optional, (
                f"The attribute {creator_user_id_attribute.name} of the struct {struct.name} "
                "is optional, but it should not be. Most likely, you forgot to add NOT NULL "
                f"to the attribute in the database im the table {struct.table_name}."
            )

            assert not isinstance(creator_user_id_attribute, StructMetadata)

        intermediate_struct_name = f"Intermediate{struct.name}"

        # First, we write the intermediate struct that is used to insert the row in the database.
        document.write(
            f"/// Intermediate representation of the new variant {struct.name}.\n"
            "#[derive(Insertable)]\n"
            f"#[diesel(table_name = {struct.table_name})]\n"
            f"pub(super) struct {intermediate_struct_name} {{\n"
        )

        all_attributes: List[AttributeMetadata] = struct.attributes

        if creator_user_id_attribute is not None:
            all_attributes = [creator_user_id_attribute] + all_attributes

        # If this struct also has an updator attribute, we add it to the list of attributes.
        # This value initially will be set to the same value as the creator user id, but it
        # will be updated when the row is updated.

        if struct.is_updatable() and struct.table_name != "users":
            updator_user_id_attribute: AttributeMetadata = (
                struct.get_updator_user_id_attribute()
            )
            all_attributes.append(updator_user_id_attribute)
        else:
            updator_user_id_attribute = creator_user_id_attribute

        for attribute in all_attributes:
            document.write(f"    {attribute.name}: {attribute.format_data_type()},\n")

        document.write("}\n\n")

        underscored_user_id = ""
        if struct.table_name == "users":
            underscored_user_id = "_"

        # Next, we implement the InsertRow trait for the new variant.
        document.write(
            f"impl InsertRow for web_common::database::{struct.name} {{\n"
            f"    type Intermediate = {intermediate_struct_name};\n"
            f"    type Flat = {struct.get_flat_variant().name};\n\n"
            f"    fn to_intermediate(self, {underscored_user_id}user_id: i32) -> Self::Intermediate {{\n"
            f"        {intermediate_struct_name} {{\n"
        )

        for attribute in all_attributes:
            if attribute in (
                creator_user_id_attribute,
                updator_user_id_attribute,
            ):
                document.write(f"            {attribute.name}: user_id,\n")
            else:
                document.write(
                    f"            {attribute.name}: self.{attribute.name},\n"
                )

        document.write("        }\n    }\n\n")

        defaulted_user_id = "user_id"
        if struct.table_name == "users":
            defaulted_user_id = "0"
        
        assert_user_id_check = ""
        if struct.table_name == "users":
            assert_user_id_check = "        assert_eq!(user_id, 0);\n"

        document.write(
            "    fn insert(\n"
            "        self,\n"
            f"       user_id: i32,\n"
            "        connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>\n"
            "    ) -> Result<Self::Flat, diesel::result::Error> {\n"
            f"        use crate::schema::{struct.table_name};\n{assert_user_id_check}"
            f"        diesel::insert_into({struct.table_name}::dsl::{struct.table_name})\n"
            f"            .values(self.to_intermediate({defaulted_user_id}))\n"
            "            .get_result(connection)\n"
            "    }\n"
            "}\n\n"
        )

    document.flush()
    document.close()


def write_diesel_update_structs(
    update_struct_metadatas: List[StructMetadata],
):
    """Writes to the backend the diesel methods for the update structs.

    Parameters
    ----------
    update_struct_metadatas : List[StructMetadata]
        The list of the StructMetadata objects.
    """

    path = "./src/update_variants.rs"

    document = open(path, "w", encoding="utf8")

    # First of all, we write a docstring that warns the reader
    # not to write anything in this file as it is automatically
    # generated.

    document.write(
        "//! This module contains the update variants of the database models.\n"
        "//!\n"
        "//! This module is automatically generated. Do not write anything here.\n\n"
    )

    imports = [
        "use diesel::prelude::*;",
        "use crate::models::*;",
        "use crate::schema::*;",
        "use diesel::r2d2::PooledConnection;",
        "use diesel::r2d2::ConnectionManager;",
        "use uuid::Uuid;",
        "use chrono::NaiveDateTime;",
    ]

    for import_statement in imports:
        document.write(f"{import_statement}\n")

    document.write("\n")

    # Since the update variants are defined in the web_common crate, in order to
    # implement methods for the backend we need to import the update variants from
    # the web_common crate and define new traits for the update variants. We also
    # need to implement these traits for the update variants, of course.

    # Because of how Diesel works, we need to define new structs that are an
    # intermediate representation of the row. These structs have all of the
    # attributes of the update variant, plus the attribute associated with the
    # updator user id. They derive the AsChangeset trait from
    # Diesel, which is used to insert the row in the database.

    # We start by defining the trait UpdateRow, which is implemented by the update
    # variants and provides the update method for the update variants. The update
    # method receives the user id of the user updating the row and the connection
    # to the database. The same trait also has an associated type, which is the
    # intermediate variant that is used to update the row in the database, and a
    # method that receives the self and user id and returns the intermediate variant.
    # The update method returns the newly updated row, which is the flat variant
    # of the update flat variant.

    document.write(
        "/// Trait providing the update method for the update variants.\n"
        "pub(super) trait UpdateRow {\n"
        "    /// The intermediate representation of the row.\n"
        "    type Intermediate;\n\n"
        "    /// The flat variant of the update variant.\n"
        "    type Flat;\n\n"
        "    /// Convert the update variant into the intermediate representation.\n"
        "    fn to_intermediate(self, user_id: i32) -> Self::Intermediate;\n\n"
        "    /// Update the row in the database.\n"
        "    fn update(\n"
        "        self,\n"
        "        user_id: i32,\n"
        "        connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>\n"
        "    ) -> Result<Self::Flat, diesel::result::Error>;\n"
        "}\n\n"
    )

    for struct in tqdm(
        update_struct_metadatas,
        desc="Writing update structs",
        unit="struct",
        leave=False,
    ):
        assert struct.is_update_variant(), (
            f"The struct {struct.name} is not an update variant, but it should be."
            f"It is associated to the table {struct.table_name}."
        )

        if struct.table_name == "users":
            updator_user_id_attribute = None
        else:
            updator_user_id_attribute: AttributeMetadata = (
                struct.get_updator_user_id_attribute()
            )

            assert not updator_user_id_attribute.optional, (
                f"The attribute {updator_user_id_attribute.name} of the struct {struct.name} "
                "is optional, but it should not be. Most likely, you forgot to add NOT NULL "
                f"to the attribute in the database im the table {struct.table_name}."
            )

            assert not isinstance(updator_user_id_attribute, StructMetadata)

        intermediate_struct_name = f"Intermediate{struct.name}"

        primary_keys = struct.get_primary_keys()

        # First, we write the intermediate struct that is used to update the row in the database.
        document.write(
            f"/// Intermediate representation of the update variant {struct.name}.\n"
            "#[derive(AsChangeset)]\n"
            f"#[diesel(table_name = {struct.table_name})]\n"
            f"pub(super) struct {intermediate_struct_name} {{\n"
        )

        all_attributes: List[AttributeMetadata] = struct.attributes

        if struct.table_name != "users":
            all_attributes = [updator_user_id_attribute] + all_attributes

        for attribute in all_attributes:
            if attribute in primary_keys:
                continue
            document.write(f"    {attribute.name}: {attribute.format_data_type()},\n")

        document.write("}\n\n")

        # Next, we implement the UpdateRow trait for the update variant.
        document.write(
            f"impl UpdateRow for web_common::database::{struct.name} {{\n"
            f"    type Intermediate = {intermediate_struct_name};\n"
            f"    type Flat = {struct.get_flat_variant().name};\n\n"
        )
        if struct.table_name != "users":
            document.write(
                "    fn to_intermediate(self, user_id: i32) -> Self::Intermediate {\n"
            )
        else:
            document.write(
                "    fn to_intermediate(self, _user_id: i32) -> Self::Intermediate {\n"
            )
        document.write(f"        {intermediate_struct_name} {{\n")

        for attribute in all_attributes:
            if attribute in primary_keys:
                continue
            if struct.get_attribute_by_name(attribute.name) is not None:
                document.write(
                    f"            {attribute.name}: self.{attribute.name},\n"
                )
            else:
                document.write(f"            {attribute.name}: user_id,\n")

        document.write("        }\n    }\n\n")

        document.write(
            "    fn update(\n"
            "        self,\n"
            "        user_id: i32,\n"
            "        connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>\n"
            "    ) -> Result<Self::Flat, diesel::result::Error> {\n"
            f"        use crate::schema::{struct.table_name};\n"
            f"        diesel::update({struct.table_name}::dsl::{struct.table_name})\n"
        )
        for primary_key in primary_keys:
            document.write(
                "            .filter(\n"
                f"                {struct.table_name}::dsl::{primary_key.name}.eq(self.{primary_key.name})\n"
                "            )\n"
            )
        document.write(
            "            .set(self.to_intermediate(user_id))\n"
            "            .get_result(connection)\n"
            "    }\n"
            "}\n\n"
        )

    document.flush()
    document.close()


def write_frontend_builder_default_implementation(
    builder: StructMetadata,
    table_metadata: TableMetadata,
    document: "io.TextIO",
):
    """Writes the implementation of the Default trait for the builder struct.

    Parameters
    ----------
    builder : StructMetadata
        The builder struct for which to write the implementation.
    table_metadata : TableMetadata
        The metadata of the table associated with the builder struct.
    document : io.TextIO
        The document to write to.

    Implementation details
    ----------------------
    This method extracts from the database the default values set for the
    attributes of the builder struct and writes the implementation of the
    Default trait for the builder struct. If an attribute does not have a
    default value, the attribute is set to None.
    """

    flat_struct = builder.get_flat_variant()

    document.write(
        f"impl Default for {builder.name} {{\n"
        f"    fn default() -> Self {{\n"
        f"        Self {{\n"
    )

    primary_keys = flat_struct.get_primary_keys()

    for attribute in builder.attributes:
        # If this is an error vector, we set it to
        # an empty vector.

        if (
            attribute.name.startswith("errors_")
            and attribute.data_type() == "Vec<ApiError>"
        ):
            document.write(f"            {attribute.name}: Vec::new(),\n")
            continue

        # If this is the primary key, we set it to None.
        if attribute in primary_keys:
            document.write(f"            {attribute.name}: None,\n")
            continue

        # If the current attribute does not exist in the flat struct,
        # we set it to None.
        if flat_struct.get_attribute_by_name(attribute.name) is None:
            if attribute.optional:
                document.write(f"            {attribute.name}: None,\n")
            else:
                # Otherwise, we set it to the default value of the data type.
                document.write(
                    f"            {attribute.name}: <{attribute.data_type()}>::default(),\n"
                )
            continue

        default_value = table_metadata.get_default_column_value(
            builder.table_name, attribute.name
        )

        if default_value is not None:
            default_value = default_value.replace("'", '"')

            if default_value.endswith("::character varying"):
                default_value = default_value.replace(
                    "::character varying", ".to_string()"
                )

            document.write(f"            {attribute.name}: Some({default_value}),\n")
            continue

        # If the current value is an option, we set it to None.
        if attribute.optional:
            document.write(f"            {attribute.name}: None,\n")
            continue

    document.write("        }\n" "    }\n" "}\n\n")


def write_frontend_builder_action_enumeration(
    builder: StructMetadata,
    table_metadata: TableMetadata,
    document: "io.TextIO",
):
    """Writes the enumeration of the builder actions for the builder struct.

    Parameters
    ----------
    builder : StructMetadata
        The builder struct for which to write the enumeration.
    table_metadata : TableMetadata
        The metadata of the table associated with the builder struct.
    document : io.TextIO
        The document to write to.

    Implementation details
    ----------------------
    This method writes the enumeration of the builder actions for the builder struct.
    """

    flat_variant = builder.get_flat_variant()
    rich_variant = builder.get_richest_variant()
    primary_keys = flat_variant.get_primary_keys()

    action_enum_name = f"{flat_variant.name}Actions"

    derives = ", ".join(
        [derive for derive in builder.derives() if derive not in ("Store", "Default")]
    )
    document.write(f"#[derive({derives})]\n" f"pub(super) enum {action_enum_name} {{\n")

    attributes_requiring_operations: List[AttributeMetadata] = []

    for attribute in builder.attributes:
        if attribute in primary_keys:
            continue

        # We do not want to include the errors attribute in the builder actions.
        if (
            attribute.name.startswith("errors_")
            and attribute.data_type() == "Vec<ApiError>"
        ):
            continue

        if attribute.name == "form_updated_at":
            continue

        if attribute.data_type() == rich_variant.name:
            assert rich_variant.is_nested()
            attributes_requiring_operations.append(attribute)

        if (
            attribute.data_type() in INPUT_TYPE_MAP
            or attribute.data_type() == "NaiveDateTime"
        ):
            document.write(f"    Set{attribute.capitalized_name()}(Option<String>),\n")
        else:
            document.write(
                f"    Set{attribute.capitalized_name()}({attribute.format_data_type()}),\n"
            )

    document.write("}\n\n")

    # We implement the FromOperation trait for the action enum. This trait
    # is used to convert a named get operation into an action to apply to
    # the current builder object. The trait only implements a single method,
    # namely from_operation, which receives a generic S that implements AsRef<str>
    # and a vector of bytes which contains the struct corresponding the action
    # that needs to be built. The method panics if none of the expected operation
    # names are supported. The operation name are equal to the attribute names
    # of types that are equal to the richest variant associated to the builder,
    # i.e. the variants for which we must run additional requests to the backend.
    # If the builder in question does not contain any attributes of the richest
    # variant, the method solely contains an unreachable!() macro.

    document.write(f"impl FromOperation for {action_enum_name} {{\n")

    if len(attributes_requiring_operations) == 0:
        document.write(
            "    fn from_operation<S: AsRef<str>>(_operation: S, _row: Vec<u8>) -> Self {\n"
            f'        unreachable!("No operations are expected to be needed for the builder {builder.name}.")\n'
        )
    else:
        document.write(
            "    fn from_operation<S: AsRef<str>>(operation: S, row: Vec<u8>) -> Self {\n"
            "        match operation.as_ref() {\n"
        )

        for attribute in attributes_requiring_operations:
            document.write(
                f'            "{attribute.name}" => {action_enum_name}::Set{attribute.capitalized_name()}(bincode::deserialize(&row).unwrap()),\n'
            )

        document.write(
            "            operation_name => unreachable!(\"The operation name '{}' is not supported.\", operation_name),\n"
            "        }\n"
        )

    document.write("    }\n}\n\n")

    document.write(
        f"impl Reducer<{builder.name}> for {action_enum_name} {{\n"
        f"    fn apply(self, mut state: std::rc::Rc<{builder.name}>) -> std::rc::Rc<{builder.name}> {{\n"
        "        let state_mut = Rc::make_mut(&mut state);\n"
        "        match self {\n"
    )

    largest_type_variants = {
        "i8": "i128",
        "i16": "i128",
        "i32": "i128",
        "i64": "i128",
        "i128": "i128",
        "u8": "u128",
        "u16": "u128",
        "u32": "u128",
        "u64": "u128",
        "u128": "u128",
        "f32": "f64",
        "f64": "f64",
    }

    for attribute in builder.attributes:
        if attribute in primary_keys:
            continue

        # We do not want to include the errors attribute in the builder actions.
        if (
            attribute.name.startswith("errors_")
            and attribute.data_type() == "Vec<ApiError>"
        ):
            continue

        if attribute.name == "form_updated_at":
            continue

        struct_attribute = rich_variant.get_attribute_by_name(attribute.name)

        if struct_attribute is None:
            struct_attribute = flat_variant.get_attribute_by_name(attribute.name)

        assert (
            struct_attribute is not None
        ), f"Attribute {attribute.name} not found in the struct {flat_variant.name}."

        document.write(
            f"            {action_enum_name}::Set{attribute.capitalized_name()}({attribute.name}) => '{attribute.name}: {{\n"
        )

        # First we clear out the existing errors associated with the attribute.
        document.write(f"                state_mut.errors_{attribute.name}.clear();\n")

        # If the attribute is solely optional in the builder, we need to check
        # whether it is currently populated. If it is not, we return an error.
        if not struct_attribute.optional and attribute.optional:
            document.write(
                f"        if {attribute.name}.is_none() {{\n"
                f"            state_mut.errors_{attribute.name}.push(ApiError::BadRequest(vec![\n"
                f'                "The {attribute.human_readable_name()} field is required.".to_string()\n'
                "             ]));\n"
                f"            state_mut.{attribute.name} = None;\n"
                f"             break '{attribute.name};\n"
                f"        }}\n"
            )

        # If the provided value is a String, we need to check whether it is empty.
        # If it is, we add an error to the errors vector.
        if attribute.data_type() == "String":
            document.write(
                f"                if let Some(value) = {attribute.name}.as_ref() {{\n"
                "                    if value.is_empty() {\n"
                f"                        state_mut.errors_{attribute.name}.push(ApiError::BadRequest(vec![\n"
                f'                            "The {attribute.human_readable_name()} field cannot be left empty.".to_string()\n'
                "                        ]));\n"
                f"                         state_mut.{attribute.name} = None;\n"
                f"                          break '{attribute.name};\n"
                "                    }\n"
                "                }\n"
            )

        if attribute.data_type() == "NaiveDateTime":
            # We convert the dates provided from the date picker to the NaiveDateTime format.
            # The dates from a datetime-local input are in the format "YYYY-MM-DDTHH:MM".
            document.write(
                f"                match {attribute.name} {{\n"
                f'                    Some(value) => match NaiveDateTime::parse_from_str(&value, "%Y-%m-%dT%H:%M") {{\n'
                f"                        Ok({attribute.name}) => state_mut.{attribute.name} = Some({attribute.name}),\n"
                f"                        Err(_) => state_mut.errors_{attribute.name}.push(ApiError::BadRequest(vec![\n"
                f'                            "The {attribute.name} field must be a valid date and time.".to_string()\n'
                "                        ])),\n"
                "                    },\n"
                f"                    None => state_mut.{attribute.name} = None,\n"
                "                }\n"
            )
        elif (
            attribute.data_type() in INPUT_TYPE_MAP
            and attribute.data_type() != "String"
        ):
            # We try to convert the values to the largest possible type.
            # If the conversion fails, we add an error to the errors vector. Subsequently, we
            # verify whether the value is within the expected range. If it is not, we add an error to the
            # errors vector. Finally, we set the attribute to the provided value.
            largest_type_variant = largest_type_variants[attribute.data_type()]

            document.write(
                f"                state_mut.form_updated_at = chrono::Utc::now().naive_utc();\n"
                f"                match {attribute.name} {{\n"
                f"                    Some(value) => match value.parse::<{largest_type_variant}>() {{\n"
                "                        Ok(value) => {\n"
            )

            # In the case of floats, we also check for NaN and Infinity.
            if attribute.data_type() in ("f32", "f64"):
                document.write(
                    f"                            if value.is_nan() || value.is_infinite() {{\n"
                    f"                                state_mut.errors_{attribute.name}.push(ApiError::BadRequest(vec![\n"
                    f'                                    "The {attribute.name} field must be a valid {attribute.data_type()}.".to_string()\n'
                    "                                ]));\n"
                    "                            } else "
                )

            document.write(
                f"                            if value < {attribute.data_type()}::MIN as {largest_type_variant} || value > {attribute.data_type()}::MAX as {largest_type_variant} {{\n"
                f"                                state_mut.errors_{attribute.name}.push(ApiError::BadRequest(vec![\n"
                f"                                    format!("
                f'                                            "The {attribute.name} field must be between {{}} and {{}}.",\n'
                f"                                            {attribute.data_type()}::MIN,\n"
                f"                                            {attribute.data_type()}::MAX\n"
                "                                    )\n"
                "                                ]));\n"
                "                            } else {\n"
                f"                                state_mut.{attribute.name} = Some(value as {attribute.data_type()});\n"
                "                            }\n"
                "                        }\n"
                "                        Err(_) => {\n"
                f"                            state_mut.errors_{attribute.name}.push(ApiError::BadRequest(vec![\n"
                f'                                "The {attribute.name} field must be a valid {attribute.data_type()}.".to_string()\n'
                "                            ]));\n"
                "                        }\n"
                "                    },\n"
                f"                    None => state_mut.{attribute.name} = None,\n"
                "                }\n"
            )
        else:
            document.write(
                f"                state_mut.{attribute.name} = {attribute.name};\n"
            )

        document.write(
            f"                // To avoid having a codesmell relative to the cases where we are not\n"
            f"                // yet handling more corner cases, we always use the break here.\n"
            f"                break '{attribute.name};\n"
            "            }\n"
        )

    document.write("        }\n" "        state\n" "    }\n" "}\n")


def write_frontend_form_builder_implementation(
    builder: StructMetadata,
    table_metadata: TableMetadata,
    document: "io.TextIO",
):
    """Writes the implementation of the FormBuilder trait for the builder struct.

    Parameters
    ----------
    builder : StructMetadata
        The builder struct for which to write the implementation.
    table_metadata : TableMetadata
        The metadata of the table associated with the builder struct.
    document : io.TextIO
        The document to write to.

    Implementation details
    ----------------------
    This method implements the FormBuilder trait for the provided builder struct.
    """

    flat_struct = builder.get_flat_variant()
    rich_struct = builder.get_richest_variant()

    variants = []

    flat_struct = builder.get_flat_variant()

    if flat_struct.is_insertable():
        variants.append(builder.get_new_variant())

    # If the new variant is not also used as an update
    # variant, we add it to the list of variants.
    if flat_struct.is_updatable():
        update_variant = builder.get_update_variant()
        if not update_variant.is_new_variant():
            variants.append(update_variant)

    assert len(variants) > 0

    primary_keys = flat_struct.get_primary_keys()

    document.write(
        f"impl FormBuilder for {builder.name} {{\n"
        f"    type Actions = {flat_struct.name}Actions;\n\n"
        f"    type RichVariant = {rich_struct.name};\n\n"
        "    fn has_errors(&self) -> bool {\n"
    )

    error_vectors = [
        attribute
        for attribute in builder.attributes
        if attribute.name.startswith("errors_")
        and attribute.data_type() == "Vec<ApiError>"
    ]

    assert len(error_vectors) > 0

    document.write(
        " || ".join(
            [f"!self.{error_vector.name}.is_empty()" for error_vector in error_vectors]
        )
        + "\n"
    )

    document.write("    }\n\n")

    # We implement the id method, which returns the primary key of the struct.
    document.write(
        "    fn id(&self) -> Option<PrimaryKey> {\n"
        f"        {flat_struct.get_formatted_primary_keys(include_prefix=True)}.map(|id| id.into())\n"
        "    }\n\n"
    )

    # We implement the update method, which operated on a mutable reference of the builder
    # and receives a Rich Variant type, the type associated with the FormBuilder trait,
    # which is the richest variant of the flat struct. The method updates the builder with the
    # values of the rich variant, which do not require validation as they are already validated,
    # being the result of a successful query to the database. This also means that the error
    # vectors are cleared when the method is called.
    #
    # Some of the structs composing the builder may appear Nested in the builder while flat
    # in the rich variant that has been provided when they have the same type of the rich variant.
    # For instance, a NestedProject containts a Project, which is flat in the rich variant so as
    # to avoid an infinitely-sized struct. In this case, we need to run some additional requests
    # to the backend to obtain the nested versions of the object that are not present in the rich
    # variant. These additional request are named get requests.
    document.write(
        "    fn update(dispatcher: &Dispatch<Self>, rich_variant: Self::RichVariant) -> Vec<ComponentMessage> {\n"
    )

    named_requests: List[str] = []

    for attribute in builder.attributes:
        if (
            attribute.name.startswith("errors_")
            and attribute.data_type() == "Vec<ApiError>"
        ):
            continue

        if attribute.name == "form_updated_at":
            # TODO: maybe use the updated_at column to set this value?
            # There is the issue of the timezone to pick though.
            continue

        if attribute in primary_keys:
            # TODO: maybe check that the primary key is equal?
            continue

        # We need to check whether is will be necessary to make a request to the backend
        # to obtain the nested version of the attribute. The request name is always equal
        # to the name of the attribute.
        if attribute.data_type() == rich_struct.name:
            assert rich_struct.is_nested()
            named_requests.append(
                f'ComponentMessage::get_named::<&str, {variants[0].name}>("{attribute.name}", {flat_struct.get_formatted_primary_keys(include_prefix=True, prefix="rich_variant.inner")}.into())'
            )
            continue

        struct_attribute = rich_struct.get_attribute_by_name(attribute.name)

        if struct_attribute is not None:
            if struct_attribute.optional:
                document.write(
                    f"        dispatcher.apply({flat_struct.name}Actions::Set{attribute.capitalized_name()}(rich_variant.{attribute.name}));\n"
                )
            else:
                document.write(
                    f"        dispatcher.apply({flat_struct.name}Actions::Set{attribute.capitalized_name()}(Some(rich_variant.{attribute.name})));\n"
                )
        else:
            struct_attribute = flat_struct.get_attribute_by_name(attribute.name)

            if struct_attribute is not None:
                if (
                    attribute.data_type() in INPUT_TYPE_MAP
                    or attribute.data_type() == "NaiveDateTime"
                ):
                    if struct_attribute.optional:
                        document.write(
                            f"    dispatcher.apply({flat_struct.name}Actions::Set{attribute.capitalized_name()}(rich_variant.inner.{attribute.name}.map(|{attribute.name}| {attribute.name}.to_string())));\n"
                        )
                    else:
                        document.write(
                            f"    dispatcher.apply({flat_struct.name}Actions::Set{attribute.capitalized_name()}(Some(rich_variant.inner.{attribute.name}.to_string())));\n"
                        )
                elif struct_attribute.optional:
                    document.write(
                        f"        dispatcher.apply({flat_struct.name}Actions::Set{attribute.capitalized_name()}(rich_variant.inner.{attribute.name}));\n"
                    )
                else:
                    document.write(
                        f"        dispatcher.apply({flat_struct.name}Actions::Set{attribute.capitalized_name()}(Some(rich_variant.inner.{attribute.name})));"
                    )
            else:
                raise RuntimeError(
                    f"Attribute {attribute.name} present in builder struct {builder.name} "
                    f"not found in neither the rich variant {rich_struct.name} nor the flat variant {flat_struct.name}."
                )

    # We returns the names requests. When the list
    # is empty, the method returns an empty vector,
    # otherwise it returns the list of requests.
    if len(named_requests) == 0:
        document.write("        Vec::new()\n")
    else:
        document.write(f"        vec![{', '.join(named_requests)}]\n")

    document.write("    }\n\n")

    # We implement the can submit method, which checks whether the form
    # contains errors as specified by the has_errors method, plus checks
    # that all non-optional fields are populated.
    document.write(
        "    fn can_submit(&self) -> bool {\n" "        !self.has_errors()\n"
    )

    for attribute in builder.attributes:
        if attribute.name.startswith("errors_"):
            continue

        if attribute in primary_keys:
            continue

        if attribute.name == "form_updated_at":
            continue

        struct_attribute = flat_struct.get_attribute_by_name(attribute.name)

        if struct_attribute is None:
            # We check whether the _id variant of the attribute is present.
            struct_attribute = flat_struct.get_attribute_by_name(f"{attribute.name}_id")

        if struct_attribute is None:
            raise RuntimeError(
                f"Attribute {attribute.name} not found in the build target struct {flat_struct.name}."
            )

        if not struct_attribute.optional and attribute.optional:
            document.write(f"        && self.{attribute.name}.is_some()\n")

    document.write("    }\n\n")

    document.write("}\n\n")

    for variant in variants:

        # We implement the From method to convert the builder to the target struct.
        document.write(
            f"impl From<{builder.name}> for {variant.name} {{\n"
            f"    fn from(builder: {builder.name}) -> Self {{\n"
            "        Self {\n"
        )

        if variant.is_new_variant() and variant.is_update_variant():
            assert len(primary_keys) == 1
            primary_key = primary_keys[0]
            assert primary_key.data_type() == "Uuid"
            document.write(
                f"            {primary_key.name}: builder.{primary_key.name}.unwrap_or_else(Uuid::new_v4),\n"
            )
        elif variant.is_update_variant():
            for primary_key in primary_keys:
                document.write(
                    f"            {primary_key.name}: builder.{primary_key.name}.unwrap(),\n"
                )

        for attribute in flat_struct.attributes:

            if attribute.is_automatically_determined_column():
                continue

            if attribute in primary_keys:
                continue

            # There are 3 cases to consider:
            # 1. The attribute is present in the builder, and is not a nested attribute.
            # 2. The attribute is present in the builder, and is a nested attribute, so we need to recover the inner attribute.
            # 3. The attribute is not present in the builder, so we need to check whether the normalized version, with the _id suffix, is present.

            builder_attribute = builder.get_attribute_by_name(attribute.name)

            if builder_attribute is None and attribute.name.endswith("_id"):
                builder_attribute = builder.get_attribute_by_name(attribute.name[:-3])

            if builder_attribute is None:

                raise RuntimeError(
                    f"It was impossible to find the attribute names '{attribute.name}' in "
                    f"the builder struct {builder.name}. The attributes present in the struct "
                    f"are {[attribute.name for attribute in builder.attributes]}."
                )

            # At this point, we need to handle the case where the attribute is expected to be
            # an option by the build target, and therefore we do not unwrap it, or alternatively
            # the attribute is not expected to be an option by the build target, and therefore we
            # unwrap it.
            if attribute.optional:
                if isinstance(builder_attribute.raw_data_type(), StructMetadata):
                    inner_primary_keys = (
                        builder_attribute.raw_data_type().get_primary_keys()
                    )
                    assert len(inner_primary_keys) == 1
                    inner_primary_key = inner_primary_keys[0]
                    if builder_attribute.raw_data_type().is_nested():
                        document.write(
                            f"            {attribute.name}: builder.{builder_attribute.name}.map(|{builder_attribute.name}| {builder_attribute.name}.inner.{inner_primary_key.name}),\n"
                        )
                    else:
                        document.write(
                            f"            {attribute.name}: builder.{builder_attribute.name}.map(|{builder_attribute.name}| {builder_attribute.name}.{inner_primary_key.name}),\n"
                        )
                else:
                    document.write(
                        f"            {attribute.name}: builder.{attribute.name},\n"
                    )
            else:
                if isinstance(builder_attribute.raw_data_type(), StructMetadata):
                    inner_primary_keys = (
                        builder_attribute.raw_data_type().get_primary_keys()
                    )

                    assert len(inner_primary_keys) == 1
                    inner_primary_key = inner_primary_keys[0]

                    if builder_attribute.raw_data_type().is_nested():
                        document.write(
                            f"            {attribute.name}: builder.{builder_attribute.name}.unwrap().inner.{inner_primary_key.name},\n"
                        )
                    else:
                        document.write(
                            f"            {attribute.name}: builder.{builder_attribute.name}.unwrap().{inner_primary_key.name},\n"
                        )
                else:
                    document.write(
                        f"            {attribute.name}: builder.{attribute.name}.unwrap(),\n"
                    )

        document.write("        }\n" "    }\n" "}\n")


def handle_missing_gin_index(
    attribute: AttributeMetadata,
    builder: StructMetadata,
):
    """Handles the case where a GIN trigram index is missing.

    Parameters
    ----------
    attribute : AttributeMetadata
        The attribute for which the GIN trigram index is missing.
    builder : StructMetadata
        The builder struct associated with the attribute.
    """
    # We prepare a message for the user to ask them whether we should generate the index
    # automatically for them. The exception will be raised nevertheless, as creating an
    # index in-medias-res will change many of the other metadata collected earlier on,
    # and the pipeline has to be re-run from the beginning.

    # First, we identify the migration the migration after which the index should be created.
    # The index has to be created AFTER either the creation of the table or the population of the
    # table with data, as the index will be created on the populated table. These names are the
    # suffixes of the migrations, as the prefix is the number of the migration.
    target_path_names = [
        f"populate_{attribute.raw_data_type().table_name}_table",
        f"create_{attribute.raw_data_type().table_name}_table",
    ]

    # We find the migration after which the index should be created.
    target_migration = None
    for target_path_name in target_path_names:
        for migration in os.listdir("../backend/migrations"):
            if migration.endswith(target_path_name):
                target_migration = migration
                break
        if target_migration is not None:
            break

    migration_number = int(target_migration.split("_")[0])

    index_migration_name = f"create_{attribute.raw_data_type().table_name}_gin_index"

    full_migration_name = (
        f"{(str(migration_number + 1)).zfill(14)}_{index_migration_name}"
    )

    textual_columns = []

    flat_struct = None

    assert not attribute.raw_data_type().has_only_foreign_keys()

    if attribute.raw_data_type().is_nested():
        inner_attribute = attribute.raw_data_type().get_attribute_by_name("inner")

        if inner_attribute is None:
            raise RuntimeError(
                f"The attribute {attribute.name} is nested, but we could not find the inner attribute. "
                f"The builder struct is {builder.name}. "
                f"It is of type {attribute.raw_data_type().name}. "
                f"The other attributes in the struct are {', '.join(inner_attribute.name for inner_attribute in attribute.raw_data_type().attributes)}."
            )

        flat_struct = inner_attribute.raw_data_type()
    else:
        flat_struct = attribute.raw_data_type()

    for inner_attribute in flat_struct.attributes:
        if inner_attribute.data_type() in TEXTUAL_DATA_TYPES:
            textual_columns.append(inner_attribute)

    if len(textual_columns) == 0:
        raise RuntimeError(
            f"The table {attribute.raw_data_type().table_name} is not searchable as "
            "we did not find a GIN trigram index for it. We cannot generate a datalist "
            "for it - please create a GIN trigram index for it and try again. "
            "If you have just created the index, recall that you may still need to run "
            "that particular migration. Furthermore, we have not even found any textual "
            "columns in the table, so we cannot help you creating the index. "
            f"The builder struct is {builder.name}. "
            f"The attribute name is {attribute.name} and is of type {attribute.data_type()}."
        )

    if len(textual_columns) > 0:
        print(
            "The table is not searchable as we did not find a GIN trigram index for it.\n"
            f"The following columns are searchable: {', '.join(textual_column.name for textual_column in textual_columns)}\n"
            f"We can generate a GIN trigram index for the table {attribute.raw_data_type().table_name}.\n"
            f"We will generate part of the index in the migration {full_migration_name}.\n"
            "You will still need to refine the index afterwards to your liking."
        )

        user_answer = userinput(
            name="Create GIN index?",
            default=False,
            validator="human_bool",
            sanitizer="human_bool",
            cache=False,
        )

        assert isinstance(user_answer, bool)

        if user_answer:
            print("We will generate the index in the migration.")
            print("Please re-run pipeline once the index has been created.")
            sleep(2)
            insert_migration(
                counter=migration_number + 1,
                name=index_migration_name,
            )

            concatenate_columns = "_".join(
                textual_column.name for textual_column in textual_columns
            )

            index_name = (
                f"{attribute.raw_data_type().table_name}_{concatenate_columns}_trgm_idx"
            )
            function_name = None

            with open(
                f"./migrations/{full_migration_name}/up.sql", "w", encoding="utf8"
            ) as up_index_migration:

                up_index_migration.write(
                    f"-- Create index to run approximate search queries on the {attribute.raw_data_type().table_name} table.\n"
                    f"-- The search will be case insensitive and will use the trigram index.\n\n"
                    "CREATE EXTENSION IF NOT EXISTS pg_trgm;\n\n"
                )

                if len(textual_columns) > 1:
                    function_name = f"f_concat_{attribute.raw_data_type().table_name}_{concatenate_columns}"

                    up_index_migration.write(f"CREATE FUNCTION {function_name}(\n")
                    for inner_attribute in textual_columns:
                        up_index_migration.write(f"{inner_attribute.name} text,\n")
                    up_index_migration.write(
                        ") RETURNS text AS $$\n"
                        "BEGIN\n"
                        "-- TODO! Add the concatenation logic here!\n"
                        "END;\n"
                        "$$ LANGUAGE plpgsql IMMUTABLE STRICT PARALLEL SAFE;\n\n"
                    )

                    up_index_migration.write(
                        f"CREATE INDEX {index_name} ON {attribute.raw_data_type().table_name} USING gin (\n"
                        f"{function_name}(\n"
                    )
                    for inner_attribute in textual_columns:
                        up_index_migration.write(f"{inner_attribute.name},\n")

                    up_index_migration.write(") gin_trgm_ops\n" ");\n")
                else:
                    up_index_migration.write(
                        f"CREATE INDEX {index_name} ON {attribute.raw_data_type().table_name} USING gin (\n"
                        f"    {textual_columns[0].name} gin_trgm_ops\n"
                        ");\n"
                    )

            with open(
                f"./migrations/{full_migration_name}/down.sql", "w", encoding="utf8"
            ) as down_index_migration:
                down_index_migration.write(
                    f"-- Drop index on the {attribute.raw_data_type().table_name} table.\n"
                    f"-- The index was used to run approximate search queries on the table.\n\n"
                    f"DROP INDEX {index_name};\n"
                )

                if function_name is not None:
                    down_index_migration.write(f"DROP FUNCTION {function_name}(")
                    for inner_attribute in textual_columns:
                        down_index_migration.write(f"{inner_attribute.name} text,\n")
                    down_index_migration.write(");\n")
        else:
            print("Please create the index manually and re-run the pipeline.")
            sleep(2)

    raise RuntimeError(
        f"The table {attribute.raw_data_type().table_name} is not searchable as "
        "we did not find a GIN trigram index for it. We cannot generate a datalist "
        "for it - please create a GIN trigram index for it and try again. "
        "If you have just created the index, recall that you may still need to run "
        "that particular migration. This error was encountered while trying to generate "
        f"the form for the {builder.name} builder."
    )


def implements_row_to_searchable_badge(
    struct: StructMetadata,
):
    """Returns whether the struct implements the RowToSearchableBadge trait.

    Parameters
    ----------
    struct : StructMetadata
        The struct for which to check the implementation.

    Returns
    -------
    bool
        Whether the struct implements the RowToSearchableBadge trait.

    Implementation details
    ----------------------
    This method checks whether the provided struct implements the RowToSearchableBadge trait.
    """
    # The standard position of the RowToSearchableBadge trait implementation is in the
    # frontend crate, in the /src/components/row_to_searchable_badge/{table_name}.rs file.
    # We check whether the file exists, and if it does, we check whether the
    # struct implements the trait, meaning whether there appears therein the
    # implementation of the RowToSearchableBadge trait for the struct:
    #
    # impl RowToSearchableBadge for {struct_name} {

    # We check that this method is called at the correct position, in the
    # backend crate.

    assert os.getcwd().endswith("/backend")

    path = f"../frontend/src/components/database/row_to_searchable_badge/{struct.table_name}.rs"

    if not os.path.exists(path):
        return False

    module_path = f"../frontend/src/components/database/row_to_searchable_badge.rs"

    # We check that the module is imported in the row_to_searchable_badge.rs file.
    with open(module_path, "r", encoding="utf8") as module:
        for line in module:
            if line.startswith(f"pub mod {struct.table_name};"):
                break
        else:
            return False

    with open(path, "r", encoding="utf8") as document:
        for line in document:
            if line.startswith(f"impl RowToSearchableBadge for {struct.name} {{"):
                return True

    return False


def handle_missing_row_to_searchable_badge_implementation(
    struct: StructMetadata,
):
    """Handles the missing implementation of the RowToSearchableBadge trait.

    Parameters
    ----------
    struct : StructMetadata
        The struct for which to handle the missing implementation.

    Implementation details
    ----------------------
    When the implementation of the RowToSearchableBadge trait is missing, this method
    asks to the user whether the implementation should be generated automatically.
    If so, we proceed to create the file at the expected location and write the
    implementation of the trait for the struct, using a very rough first draft.
    """

    path = f"../frontend/src/components/database/row_to_searchable_badge/{struct.table_name}.rs"

    # We check that the target implementation does not appear in some other file
    # with the mistaken name.
    for file in os.listdir(
        "../frontend/src/components/database/row_to_searchable_badge"
    ):
        target_string = f"impl RowToSearchableBadge for {struct.name} {{"
        with open(
            f"../frontend/src/components/database/row_to_searchable_badge/{file}",
            "r",
            encoding="utf8",
        ) as document:
            for line in document:
                if target_string in line:
                    raise Exception(
                        "We expected to find the RowToSearchableBadge implementation for the "
                        f"{struct.name} struct in the {file} file, but it appears in the {path} file."
                    )

    print(
        f"Should we create the RowToSearchableBadge implementation for the {struct.name} struct? "
        f"It would be at the following location: {path}."
    )

    user_answer = userinput(
        name="Create RowToSearchableBadge implementation?",
        default="no",
        validator="human_bool",
        sanitizer="human_bool",
    )

    if user_answer:

        # We retrieve the textual columns of the struct, as we will use them
        # to generate the implementation of the RowToSearchableBadge trait.
        index: PGIndex = find_pg_trgm_indices().get_table(struct.table_name)

        # We retrieve the textual columns of the struct, and we check which of
        # them appear in the index arguments.
        search_columns = [
            column
            for column in (
                struct.get_attribute_by_name("inner").raw_data_type().attributes
                if struct.is_nested()
                else struct.attributes
            )
            if column.data_type() in TEXTUAL_DATA_TYPES
            and column.name in index.arguments
        ]

        assert len(search_columns) > 0

        directory_name = os.path.dirname(path)
        os.makedirs(directory_name, exist_ok=True)

        with open(path, "w", encoding="utf8") as document:

            document.write(
                "use super::RowToSearchableBadge;\n"
                "use crate::traits::format_match::FormatMatch;\n"
                f"use web_common::database::{struct.name};\n"
                "use yew::prelude::*;\n\n"
            )

            document.write(
                f"impl RowToSearchableBadge for {struct.name} {{\n"
                f"    fn to_datalist_badge(&self, query: &str) -> Html {{\n"
                f"        html! {{\n"
                f"            <div>\n"
                f"                <p>\n"
            )

            font_awesome_icon = None

            if struct.get_attribute_by_name("font_awesome_icon") is not None:
                if struct.get_attribute_by_name("color") is not None:
                    font_awesome_icon = '<i class={format!("{} {}", self.font_awesome_icon.name, self.color.name)}></i>'
                else:
                    font_awesome_icon = '<i class={format!("{} grey", self.font_awesome_icon.name)}></i>'
            else:
                font_awesome_icon = '<i class="fas fa-question grey"></i>'

            document.write(f"                {font_awesome_icon}\n")

            # we handle both the case where the column is optional and the case where it is not
            for column in search_columns:
                if column.optional:
                    if struct.is_nested():
                        document.write(
                            f"                if let Some({column.name}) = self.inner.{column.name}.as_ref() {{\n"
                            f"                    <span>{{{column.name}.format_match(query)}}</span>\n"
                            f"                }}\n"
                        )
                    else:
                        document.write(
                            f"                if let Some({column.name}) = self.{column.name}.as_ref() {{\n"
                            f"                    <span>{{{column.name}.format_match(query)}}</span>\n"
                            f"                }}\n"
                        )
                else:
                    if struct.is_nested():
                        document.write(
                            f"                    <span>{{self.inner.{column.name}.format_match(query)}}</span>\n"
                        )
                    else:
                        document.write(
                            f"                    <span>{{self.{column.name}.format_match(query)}}</span>\n"
                        )

            document.write(
                "                </p>\n"
                "            </div>\n"
                "        }\n"
                "    }\n\n"
            )

            document.write(
                f"    fn to_selected_datalist_badge(&self) -> Html {{\n"
                f"        html! {{\n"
                f"            <div>\n"
                f"                <p>\n"
            )

            document.write(f"                {font_awesome_icon}\n")

            # We only want to show a single column in the selected datalist badge.
            # If the struct happens to have column called `name`, we use it, otherwise
            # we use the first column that is searchable.
            if struct.get_attribute_by_name("name") is not None:
                if struct.is_nested():
                    document.write(
                        f"                <span>{{self.inner.name.clone()}}</span>\n"
                    )
                else:
                    document.write(
                        f"                    <span>{{self.name.clone()}}</span>\n"
                    )
            else:
                if struct.is_nested():
                    document.write(
                        f"                <span>{{self.inner.{search_columns[0].name}.clone()}}</span>\n"
                    )
                else:
                    assert not search_columns[0].optional
                    document.write(
                        f"                    <span>{{self.{search_columns[0].name}.clone()}}</span>\n"
                    )

            document.write(
                "                </p>\n" "            </div>\n" "        }\n" "    }\n"
            )

            # We implement the matches method, where we chain the columns that are searchable
            # if the column name is not available.

            document.write(f"    fn matches(&self, query: &str) -> bool {{\n")

            if "name" in [column.name for column in search_columns]:
                if struct.is_nested():
                    document.write(f"        self.inner.name.format_match(query)\n")
                else:
                    document.write(f"        self.name == query\n")
            else:
                column = search_columns[0]
                if column.optional:
                    if struct.is_nested():
                        document.write(
                            f"        self.inner.{column.name}.as_ref().map_or(false, |column| column == query)\n"
                        )
                    else:
                        document.write(
                            f"        self.{column.name}.as_ref().map_or(false, |column| column == query)\n"
                        )
                else:
                    if struct.is_nested():
                        document.write(f"        self.inner.{column.name} == query\n")
                    else:
                        document.write(f"        self.{column.name} == query\n")

            document.write("    }\n")

            document.write(f"    fn similarity_score(&self, query: &str) -> isize {{\n")

            scores = []

            for column in search_columns:
                if column.optional:
                    if struct.is_nested():
                        scores.append(
                            f"self.inner.{column.name}.as_ref().map_or(0, |column| column.similarity_score(query))"
                        )
                    else:
                        scores.append(
                            f"self.{column.name}.as_ref().map_or(0, |column| column.similarity_score(query))"
                        )
                else:
                    if struct.is_nested():
                        scores.append(
                            f"self.inner.{column.name}.similarity_score(query)"
                        )
                    else:
                        scores.append(f"self.{column.name}.similarity_score(query)")

            scores_summatory = " + ".join(scores)
            document.write(f"        {scores_summatory}\n")

            document.write("    }\n")

            document.write("fn primary_color_class(&self) -> &str {\n")

            if struct.get_attribute_by_name("color") is not None:
                document.write("        &self.color.name\n")
            else:
                document.write('        "grey"\n')

            document.write("    }\n")

            document.write("fn description(&self) -> &str {\n")

            if "description" in [column.name for column in search_columns]:
                if struct.is_nested():
                    document.write(f"        &self.inner.description\n")
                else:
                    document.write("        &self.description\n")
            else:
                document.write('        ""\n')

            document.write("    }\n")

            document.write("}\n")

        # We import the new module in the:
        path = "../frontend/src/components/database/row_to_searchable_badge.rs"

        with open(path, "a", encoding="utf8") as document:
            document.write(f"pub mod {struct.table_name};\n")

        print(f"RowToSearchableBadge implementation for {struct.name} created.")
        print("Please refine it and re-run the pipeline.")
        sleep(2)

        raise Exception(
            f"The RowToSearchableBadge implementation for the {struct.name} struct is missing. "
            "We have generated a rough first draft for you. Please refine it and re-run the pipeline."
        )


def write_frontend_yew_form(
    builder: StructMetadata,
    table_metadata: TableMetadata,
    document: "io.TextIO",
):
    """Writes the Yew form for the builder struct.

    Parameters
    ----------
    builder : StructMetadata
        The builder struct for which to write the form.
    document : io.TextIO
        The document to write to.

    Implementation details
    ----------------------
    This method writes the Yew form for the provided builder struct.
    """

    trigram_indices = find_pg_trgm_indices()

    flat_struct = builder.get_flat_variant()
    primary_keys = flat_struct.get_primary_keys()

    variants = []

    if flat_struct.is_insertable():
        variants.append((builder.get_new_variant(), "POST"))

    if flat_struct.is_updatable():
        variants.append((builder.get_update_variant(), "PUT"))

    for variant, method in variants:

        action_name = "Create" if method == "POST" else "Update"

        form_component_name = f"{action_name}{flat_struct.name}Form"

        # We generate the lowercased name of the form component by splitting
        # on the uppercased letters and joining the resulting list with an
        # underscore.
        form_method_name = "_".join(
            re.findall("[A-Z][^A-Z]*", form_component_name)
        ).lower()

        # When we are creating an update variant, the form needs to receive the ID associated
        # to the variant, so that the frontend can request the correct row from the backend,
        # and upon submission, the frontend can send the correct ID to the backend. For this
        # reason, we need to create a struct that derives the Properties trait, which will
        # contain the ID of the row.

        if method == "PUT":
            document.write(
                f"#[derive(Clone, PartialEq, Properties)]\n"
                f"pub struct {form_component_name}Prop {{\n"
            )
            for primary_key in primary_keys:
                document.write(
                    f"    pub {primary_key.name}: {primary_key.format_data_type()},\n"
                )
            document.write("}\n\n")

        document.write(f"#[function_component({form_component_name})]\n")

        if method == "PUT":
            # We need to generate the form method that will receive the ID of the row.
            document.write(
                f"pub fn {form_method_name}(props: &{form_component_name}Prop) -> Html {{\n"
            )
        else:
            # We generate the form method that will not receive the ID of the row.
            document.write(f"pub fn {form_method_name}() -> Html {{\n")

        document.write(
            f"    let (builder_store, builder_dispatch) = use_store::<{builder.name}>();\n"
        )

        if method == "PUT":
            document.write("     builder_dispatch.reduce_mut(|builder| {\n")
            for primary_key in primary_keys:
                document.write(
                    f"         builder.{primary_key.name} = Some(props.{primary_key.name});\n"
                )
            document.write("     });\n")

        for attribute in builder.attributes:
            # We do not want to include the errors attribute in the builder actions.
            if (
                attribute.name.startswith("errors_")
                and attribute.data_type() == "Vec<ApiError>"
            ):
                continue

            if attribute in primary_keys:
                continue

            if attribute.name == "form_updated_at":
                continue

            if attribute.data_type() == "bool":
                document.write(
                    f"    let set_{attribute.name} = builder_dispatch.apply_callback(|{attribute.name}: bool| {flat_struct.name}Actions::Set{attribute.capitalized_name()}(Some({attribute.name})));\n"
                )
            elif (
                attribute.data_type() in INPUT_TYPE_MAP
                or attribute.data_type() == "NaiveDateTime"
            ):
                document.write(
                    f"    let set_{attribute.name} = builder_dispatch.apply_callback(|{attribute.name}: Option<String>| {flat_struct.name}Actions::Set{attribute.capitalized_name()}({attribute.name}));\n"
                )
            elif attribute.data_type() == "Vec<u8>":
                if "picture" in attribute.name:
                    document.write(
                        f"    let set_{attribute.name} = builder_dispatch.apply_callback(|{attribute.name}: Option<Image>| {flat_struct.name}Actions::Set{attribute.capitalized_name()}({attribute.name}.map(|{attribute.name}| {attribute.name}.into())));\n"
                    )
                else:
                    raise RuntimeError(
                        f"Attribute {attribute.name} of type {attribute.data_type()} not supported in the frontend form generation."
                    )
            else:
                document.write(
                    f"    let set_{attribute.name} = builder_dispatch.apply_callback(|{attribute.name}: {attribute.format_data_type()}| {flat_struct.name}Actions::Set{attribute.capitalized_name()}({attribute.name}));\n"
                )

        document.write(
            "    html! {\n"
            f"        <BasicForm<{variant.name}> method={{FormMethod::{method}}} builder={{builder_store.deref().clone()}} builder_dispatch={{builder_dispatch}}>\n"
        )

        for attribute in builder.attributes:

            if (
                attribute.name.startswith("errors_")
                and attribute.data_type() == "Vec<ApiError>"
            ):
                continue

            if attribute in primary_keys:
                continue

            if attribute.name == "form_updated_at":
                continue

            error_attribute = builder.get_attribute_by_name(f"errors_{attribute.name}")

            assert (
                error_attribute is not None
            ), f"Error attribute not found for {attribute.name} in {builder.name}."

            if (
                attribute.data_type() in INPUT_TYPE_MAP
                or attribute.data_type() == "NaiveDateTime"
            ):
                document.write(
                    f'            <BasicInput<{attribute.data_type()}> label="{attribute.human_readable_name()}" errors={{builder_store.{error_attribute.name}.clone()}} builder={{set_{attribute.name}}} value={{builder_store.{attribute.name}.clone()}} />\n'
                )
                continue

            if attribute.data_type() == "bool":
                document.write(
                    f'            <Checkbox label="{attribute.human_readable_name()}" errors={{builder_store.{error_attribute.name}.clone()}} builder={{set_{attribute.name}}} value={{builder_store.{attribute.name}.unwrap_or(false)}} />\n'
                )
                continue

            if attribute.data_type() == "Vec<u8>":
                if "picture" in attribute.name:
                    allowed_formats = ["GenericFileFormat::Image"]

                    document.write(
                        f'            <FileInput<Image> label="{attribute.human_readable_name()}" errors={{builder_store.{error_attribute.name}.clone()}} builder={{set_{attribute.name}}} allowed_formats={{vec![{", ".join(allowed_formats)}]}} value={{builder_store.{attribute.name}.clone().map(|{attribute.name}| {attribute.name}.into())}} />\n'
                    )
                else:
                    raise RuntimeError(
                        f"Attribute {attribute.name} of type {attribute.data_type()} not supported in the frontend form generation."
                    )

                continue

            # If the attribute is a nested struct, we need to generate a Datalist
            # that will allow the user to select the nested struct.
            if isinstance(attribute.raw_data_type(), StructMetadata):
                # We check that the table associated to the nested struct is searchable, otherwise
                # we cannot generate the datalist for it and we need to raise an exception.
                if not trigram_indices.has_table(attribute.raw_data_type().table_name):
                    handle_missing_gin_index(attribute, builder)

                # We check that the nested struct implements the RowToSearchableBadge trait, as we need to
                # be able to convert the nested struct to a badge within the Datalist.
                if not implements_row_to_searchable_badge(attribute.raw_data_type()):
                    handle_missing_row_to_searchable_badge_implementation(
                        attribute.raw_data_type()
                    )
                    raise RuntimeError(
                        f"The struct {attribute.raw_data_type().name} does not implement the RowToSearchableBadge trait."
                    )

                document.write(
                    f'            <Datalist<{attribute.data_type()}> builder={{set_{attribute.name}}} errors={{builder_store.{error_attribute.name}.clone()}} value={{builder_store.{attribute.name}.clone()}} label="{attribute.human_readable_name()}" />\n'
                )
                continue

            # TODO! ADD MORE INPUT TYPES HERE!

            # raise Exception(
            #     f"Attribute {attribute.name} of type {attribute.data_type()} not supported in the frontend form generation."
            # )

        document.write(f"        </BasicForm<{variant.name}>>\n" f"    }}\n" f"}}\n")


def write_frontend_form_buildable_implementation(
    builder: StructMetadata,
    document: "io.TextIO",
):
    """Writes the implementation of the Buildable trait for the target struct.

    Parameters
    ----------
    builder : StructMetadata
        The builder struct for which to write the implementation.
    document : io.TextIO
        The document to write to.

    Implementation details
    ----------------------
    This method implements the Buildable trait for the provided struct.
    """

    flat_struct = builder.get_flat_variant()
    rich_struct = builder.get_richest_variant()

    variants: List[StructMetadata] = []

    if flat_struct.is_insertable():
        variants.append(builder.get_new_variant())

    if flat_struct.is_updatable():
        update_variant = builder.get_update_variant()
        if not update_variant.is_new_variant():
            variants.append(update_variant)

    # We implement the Tabular trait for the struct.
    document.write(
        f"impl Tabular for {rich_struct.name} {{\n"
        f"    const TABLE: Table = Table::{rich_struct.get_capitalized_table_name()};\n"
        "}\n\n"
    )

    for variant in variants:
        # We implement the Tabular trait for the struct.
        document.write(
            f"impl Tabular for {variant.name} {{\n"
            f"    const TABLE: Table = Table::{flat_struct.get_capitalized_table_name()};\n"
            "}\n\n"
        )

        # We implement the FormBuildable trait for the struct.
        document.write(
            f"impl FormBuildable for {variant.name} {{\n"
            f"    type Builder = {builder.name};\n"
            "    fn title() -> &'static str {\n"
            f'        "{variant.human_readable_name()}"\n'  # TODO! Add the title
            "    }\n"
            "    fn task_target() -> &'static str {\n"
            f'        "{variant.human_readable_name()}"\n'  # TODO! Add the task target name
            "    }\n"
            "    fn requires_authentication() -> bool {\n"
            f"        {'true' if variant.requires_authentication() else 'false'}\n"
            "    }\n"
            "    fn can_operate_offline() -> bool {\n"
            f"        {'true' if variant.has_uuid_primary_key() or variant.is_update_variant() else 'false'}\n"
            "    }\n"
            "}\n\n"
        )


def write_frontend_forms(
    builder_structs: List[StructMetadata],
):
    """Writes the frontend forms to the web_common crate."""

    # For the time being, we simply write out the structs.
    # In the near future, we will also implement several
    # traits for these structs.

    path = "../frontend/src/components/forms/automatic_forms.rs"

    document = open(path, "w", encoding="utf8")

    # Preliminarly, we write a docstring at the very head
    # of this submodule to explain what it does and warn the
    # reader not to write anything in this file as it is
    # automatically generated.

    document.write(
        "//! This module contains the forms for the frontend.\n"
        "//!\n"
        "//! This module is automatically generated. Do not write anything here.\n\n"
    )

    imports = [
        "use serde::{Deserialize, Serialize};",
        "use web_common::database::*;",
        "use yew::prelude::*;",
        "use yewdux::{use_store, Reducer, Store};",
        "use crate::components::forms::*;",
        "use web_common::api::form_traits::FormMethod;",
        "use std::rc::Rc;",
        "use uuid::Uuid;",
        "use std::ops::Deref;",
        "use crate::workers::ws_worker::Tabular;",
        "use yewdux::Dispatch;",
        "use chrono::NaiveDateTime;",
        "use web_common::api::ApiError;",
        "use crate::workers::ws_worker::ComponentMessage;",
        "use web_common::custom_validators::Image;",
        "use web_common::file_formats::GenericFileFormat;",
    ]

    for import_statement in imports:
        document.write(f"{import_statement}\n")

    document.write("\n")

    for builder in tqdm(
        builder_structs,
        desc="Writing new buiders",
        unit="struct",
        leave=False,
    ):
        builder.write_to(document, derives_deny_list=["Default"])
        write_frontend_builder_default_implementation(
            builder, builder.table_metadata, document
        )
        write_frontend_builder_action_enumeration(
            builder, builder.table_metadata, document
        )
        write_frontend_form_builder_implementation(
            builder, builder.table_metadata, document
        )
        write_frontend_form_buildable_implementation(builder, document)
        write_frontend_yew_form(builder, builder.table_metadata, document=document)

    document.flush()
    document.close()

    # We verify that the forms generation has been successful by
    # checking that all of the builder names and their relative new
    # variants and update variants are present in the generated file.

    with open(path, "r", encoding="utf8") as document:
        content = document.read()

    for builder in builder_structs:
        assert (
            builder.name in content
        ), f"Builder {builder.name} not found in the generated file."

        flat_variant = builder.get_flat_variant()

        if flat_variant.is_insertable():
            assert (
                builder.get_new_variant().name in content
            ), f"New variant {builder.get_new_variant().name} not found in the generated file."

        if flat_variant.is_updatable():
            assert (
                builder.get_update_variant().name in content
            ), f"Update variant {builder.get_update_variant().name} not found in the generated file."


def generate_table_schema():
    # Read the replacements from the JSON file
    replacements = compress_json.local_load("replacements.json")

    # We make sure the migrations were fully executed
    status = os.system("diesel migration run")

    if status != 0:
        raise Exception("The migrations were not fully executed.")

    # We run the diesel extended CLI command
    status = os.system("diesel_ext --model --add-table-name > src/models.rs")

    if status != 0:
        raise Exception("The diesel_ext command failed.")


def check_for_common_typos_in_migrations():
    for directory in os.listdir("migrations"):
        if not os.path.isdir(f"migrations/{directory}"):
            continue

        if not os.path.exists(f"migrations/{directory}/up.sql") or not os.path.exists(
            f"migrations/{directory}/down.sql"
        ):
            continue

        with open(f"migrations/{directory}/up.sql", "r", encoding="utf8") as up_file:
            up_content = up_file.read()

        with open(f"migrations/{directory}/down.sql", "r", encoding="utf8") as down_file:
            down_content = down_file.read()

        if "CREATE TABLE IF EXISTS" in up_content:
            raise RuntimeError(
                f"Migration `{directory}` contains a typo: `CREATE TABLE IF EXISTS` instead of `CREATE TABLE IF NOT EXISTS`."
            )

        if "DROP TABLE IF NOT EXISTS" in down_content:
            raise RuntimeError(
                f"Migration `{directory}` contains a typo: `DROP TABLE IF NOT EXISTS` instead of `DROP TABLE IF EXISTS`."
            )

        # If there is a creation of a temporary table in a up or down migration, in the same document there
        # must be a deletion of the temporary table.
        for content, content_name in (
            (up_content, "up"),
            (down_content, "down"),
        ):
            if "CREATE TEMPORARY TABLE" in content:
                # We retrieve the name of the temporary table.
                table_name = (
                    content.split("CREATE TEMPORARY TABLE")[1].split("(")[0].strip()
                )
                # We check that the deletion of the temporary table is present in the up migration.
                if f"DROP TABLE {table_name}" not in content:
                    raise RuntimeError(
                        f"Migration `{directory}` contains a `CREATE TEMPORARY TABLE` constraint in the {content_name}.sql file, but does not contain a `DROP TABLE {table_name}` constraint in the {content_name}.sql file."
                    )


def ensures_gluesql_compliance():
    for directory in os.listdir("migrations"):
        if not os.path.isdir(f"migrations/{directory}"):
            continue

        if not os.path.exists(f"migrations/{directory}/up.sql") or not os.path.exists(
            f"migrations/{directory}/down.sql"
        ):
            continue

        with open(f"migrations/{directory}/up.sql", "r", encoding="utf8") as up_file:
            up_content = up_file.read()

        if "SERIAL PRIMARY KEY" in up_content:
            raise RuntimeError(
                f"Migration `{directory}` contains a `SERIAL PRIMARY KEY` constraint, which is not supported by GlueSQL. "
                "Please replace it with `INTEGER PRIMARY KEY`."
            )

        if up_content.count("CREATE TABLE") != up_content.count(
            "CREATE TABLE IF NOT EXISTS"
        ):
            raise RuntimeError(
                f"Migration `{directory}` does not use `CREATE TABLE IF NOT EXISTS` consistently. "
                f"Replace the use of `CREATE TABLE` with `CREATE TABLE IF NOT EXISTS` in the `up.sql` file "
                "so to avoid conflicts when running the migrations within GlueSQL."
            )


def ensures_migrations_simmetry():
    # We check that, if in a migration directory up.sql contains a
    # certain string, down.sql contains the symmetric string.

    opposites = {
        "CREATE TABLE": "DROP TABLE",
        "CREATE TABLE IF NOT EXISTS": "DROP TABLE IF EXISTS",
        "CREATE INDEX": "DROP INDEX",
        "CREATE VIEW": "DROP VIEW",
        # "CREATE FUNCTION": "DROP FUNCTION",
        "CREATE TRIGGER": "DROP TRIGGER",
        "CREATE TYPE": "DROP TYPE",
    }

    for directory in os.listdir("migrations"):
        if not os.path.isdir(f"migrations/{directory}"):
            continue

        if not os.path.exists(f"migrations/{directory}/up.sql") or not os.path.exists(
            f"migrations/{directory}/down.sql"
        ):
            continue

        if directory == "00000000000000_diesel_initial_setup":
            continue

        with open(f"migrations/{directory}/up.sql", "r", encoding="utf8") as up_file:
            up_content = up_file.read()

        with open(
            f"migrations/{directory}/down.sql", "r", encoding="utf8"
        ) as down_file:
            down_content = down_file.read()

        for up_key, down_key in opposites.items():
            if up_key in up_content and down_key not in down_content:
                raise ValueError(
                    f"Migration {directory} is not symmetric: up.sql contains `{up_key}` but down.sql does not contain `{down_key}`."
                )
            if down_key in down_content and up_key not in up_content:
                raise ValueError(
                    f"Migration {directory} is not symmetric: down.sql contains `{down_key}` but up.sql does not contain `{up_key}`."
                )


if __name__ == "__main__":
    # Load dotenv file
    load_dotenv()
    regroup_tables()

    # If there is a "__pycache__" directory, we remove it as Diesel
    # seems to be keen to try and run it as a migration, and it will
    # fail.
    if os.path.exists("__pycache__"):
        shutil.rmtree("__pycache__")

    if os.path.exists("migrations/__pycache__"):
        shutil.rmtree("migrations/__pycache__")

    if not os.path.exists("./db_data/bio_ott_taxons.csv.gz"):
        retrieve_taxons()

    tables_metadata = find_foreign_keys()

    enforce_migration_naming_convention()
    replace_serial_indices()
    check_for_common_typos_in_migrations()
    ensures_migrations_simmetry()
    ensures_gluesql_compliance()
    print("Ensured migrations simmetry & GlueSQL compliance.")

    handle_minimal_revertion()
    generate_table_schema()
    print("Generated models.")

    ensures_all_update_at_trigger_exists(tables_metadata)
    ensure_created_at_columns(tables_metadata)
    ensure_updated_at_columns(tables_metadata)

    generate_view_schema(tables_metadata)
    print("Generated view schema.")
    check_schema_completion()
    print("Checked schema completion.")
    generate_view_structs()
    print("Generated view structs.")

    table_structs: List[StructMetadata] = extract_structs(
        "src/models.rs", table_metadata=tables_metadata
    )
    view_structs: List[StructMetadata] = extract_structs(
        "src/views/views.rs", table_metadata=tables_metadata
    )

    write_backend_structs("src/models.rs", "tables", table_structs, tables_metadata)
    write_backend_structs("src/views/views.rs", "views", view_structs, tables_metadata)
    print(
        f"Generated {len(table_structs)} tables and {len(view_structs)} views implementations for backend."
    )

    write_web_common_structs(table_structs, "tables", "Table", tables_metadata)
    write_web_common_structs(view_structs, "views", "View", tables_metadata)
    print("Generated web common structs.")

    nested_structs: List[StructMetadata] = generate_nested_structs(
        "src/nested_models.rs", table_structs + view_structs, tables_metadata
    )
    print(f"Generated {len(nested_structs)} nested structs for backend.")

    new_model_structs = derive_new_models(table_structs, tables_metadata)
    print(f"Derived {len(new_model_structs)} structs for the New versions")

    update_model_structs = derive_update_models(table_structs)
    print(f"Derived {len(update_model_structs)} structs for the Update versions")

    tables: List[TableStructMetadata] = write_webcommons_table_names_enumeration(
        table_structs + view_structs + nested_structs,
        new_model_structs,
        update_model_structs,
        tables_metadata,
    )
    ensure_updatable_tables_have_roles_tables(tables, tables_metadata)
    ensure_tables_have_creation_notification_trigger(tables, tables_metadata)
    print("Generated table names enumeration for web_common.")

    write_diesel_table_names_enumeration(tables)
    print("Generated table names enumeration for diesel.")

    write_web_common_nested_structs("nested_models.rs", nested_structs)
    print("Generated nested structs for web_common.")

    write_web_common_search_trait_implementations(
        nested_structs + table_structs + view_structs
    )
    print("Generated search trait implementations for web_common.")

    builder_structs = derive_model_builders(new_model_structs + update_model_structs)
    print(f"Derived {len(builder_structs)} builders for the New & Update versions")

    write_web_common_new_structs(new_model_structs)
    write_web_common_update_structs(update_model_structs)
    print("Generated new & update structs for web_common.")

    write_diesel_new_structs(new_model_structs)
    write_diesel_update_structs(
        update_model_structs
        + [
            new_model_struct
            for new_model_struct in new_model_structs
            if new_model_struct.is_update_variant()
        ]
    )
    print("Generated new & update structs for diesel.")

    write_frontend_forms(
        builder_structs,
    )
    print("Generated frontend forms.")

    write_frontend_router_page(
        builder_structs,
    )
    print("Generated frontend router page.")
