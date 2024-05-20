"""This module contains the functions to generate the backend & frontend models."""

import os
import io
import shutil
from typing import List, Dict
from dotenv import load_dotenv
from retrieve_taxons import retrieve_taxons
from tqdm.auto import tqdm
from constraint_checkers import (
    find_foreign_keys,
    ensures_all_update_at_trigger_exists,
    TableMetadata,
    ViewColumn,
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
from constraint_checkers import generate_view_schema, generate_table_schema
from constraint_checkers import (
    check_parent_circularity_trigger,
    create_filter_structs,
    ensures_migrations_simmetry,
    ensures_gluesql_compliance,
)
from constraint_checkers import (
    check_for_common_typos_in_migrations,
    write_frontend_forms,
)


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
    table_metadata: TableMetadata,
):
    """Write the `From` implementations for the structs in the `src/models.rs` file."""

    if len(struct_metadatas) == 0:
        return

    similarity_indices: PGIndices = find_pg_trgm_indices(table_metadata)

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

    with open(path, "w", encoding="utf8") as file:
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
        file.write("\n".join(imports) + "\n\n")

        # Then, we write the structs.

        for struct in tqdm(
            struct_metadatas,
            desc=f"Writing {table_type} to backend",
            unit="struct",
            leave=False,
        ):

            primary_keys = struct.get_primary_keys()

            editable_variants = [False]

            if struct.has_associated_roles() and struct.table_name != "users":
                editable_variants.append(True)

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

            # For all the tables that have an associated roles table, we implement methods
            # to determine whetheer a provided user id is a Viewer (role_id >= 3), Editor (role_id >= 2), or Admin (role_id == 1),
            # or the user is the creator of the struct (i.e. the created_by field is equal to the user_id).
            if struct.has_associated_roles() and struct.table_name != "users":
                # We start by creating the more general method that checks whether the user has a role
                # with a role_id less than or equal to the provided role_id.
                file.write(
                    "    /// Check whether the user has a role with a role_id less than or equal to the provided role_id.\n"
                    "    ///\n"
                    "    /// # Arguments\n"
                    f"    /// * `{struct.get_formatted_primary_keys(include_prefix=False)}` - The primary key(s) of the struct to delete.\n"
                    "    /// * `author_user_id` - The ID of the user to check.\n"
                    "    /// * `role_id` - The role_id to check against.\n"
                    "    /// * `connection` - The connection to the database.\n"
                    "    ///\n"
                    "    pub fn has_role_by_id(\n"
                    f"        {struct.get_formatted_primary_keys(include_prefix=False)}: {struct.get_formatted_primary_key_data_types()},\n"
                    "        author_user_id: i32,\n"
                    "        role_id: i32,\n"
                    "        connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>\n"
                    "    ) -> Result<bool, diesel::result::Error> {\n"
                    f"        diesel::select(diesel::dsl::exists({struct.table_name}::dsl::{struct.table_name}\n"
                )
                for primary_key in primary_keys:
                    file.write(
                        f"            .filter({struct.table_name}::dsl::{primary_key.name}.eq({primary_key.name}))\n"
                    )
                file.write(
                    f"           .filter({struct.table_name}::dsl::created_by.eq(author_user_id))\n"
                    "            .or_filter(\n"
                    f"               {struct.table_name}::dsl::id.eq(id)\n"
                    f"                   .and({struct.table_name}::dsl::id.eq_any(\n"
                    f"                       {struct.table_name}_users_roles::table\n"
                    f"                           .select({struct.table_name}_users_roles::dsl::table_id)\n"
                    f"                           .filter({struct.table_name}_users_roles::dsl::user_id.eq(author_user_id)\n"
                    f"                           .and({struct.table_name}_users_roles::dsl::role_id.le(role_id)),\n"
                    "                    )),\n"
                    "               )\n"
                    "         )\n"
                )
                if struct.table_name != "teams":
                    file.write(
                        "                    .or_filter(\n"
                        f"                       {struct.table_name}::dsl::id.eq(id)\n"
                        f"                           .and({struct.table_name}::dsl::id.eq_any(\n"
                        f"                               {struct.table_name}_teams_roles::table\n"
                        f"                                   .select({struct.table_name}_teams_roles::dsl::table_id)\n"
                        f"                                   .filter({struct.table_name}_teams_roles::dsl::role_id.le(role_id))\n"
                        "                                   .inner_join(teams_users_roles::table.on(\n"
                        f"                                       {struct.table_name}_teams_roles::dsl::team_id.eq(teams_users_roles::dsl::table_id)\n"
                        "                                           .and(teams_users_roles::dsl::user_id.eq(author_user_id))\n"
                        "                                           .and(teams_users_roles::dsl::role_id.le(role_id)),\n"
                        "                                   )),\n"
                        "                              ))\n"
                        "                       )\n"
                    )
                file.write(
                    "            ))\n"
                    "         .get_result::<bool>(connection)\n"
                    "    }\n"
                )

                # We now create the more specific methods that check whether the user has a role
                # with a role_id less than or equal to the provided role_id. We start with the Viewer role.

                file.write(
                    "    /// Check whether the user is a Viewer (role_id >= 3).\n"
                    "    ///\n"
                    "    /// # Arguments\n"
                    "    /// * `author_user_id` - The ID of the user to check.\n"
                    "    /// * `connection` - The connection to the database.\n"
                    "    ///\n"
                    "    pub fn is_viewer(\n"
                    "        &self,\n"
                    "        author_user_id: i32,\n"
                    "        connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>\n"
                    "    ) -> Result<bool, diesel::result::Error> {\n"
                    "        Self::is_viewer_by_id(\n"
                    f"            {struct.get_formatted_primary_keys(include_prefix=True)},\n"
                    "            author_user_id,\n"
                    "            connection,\n"
                    "        )\n"
                    "    }\n"
                )

                file.write(
                    "    /// Check whether the user is a Viewer (role_id >= 3) for the provided primary key(s).\n"
                    "    ///\n"
                    "    /// # Arguments\n"
                    f"    /// * `{struct.get_formatted_primary_keys(include_prefix=False)}` - The primary key(s) of the struct to delete.\n"
                    "    /// * `author_user_id` - The ID of the user to check.\n"
                    "    /// * `connection` - The connection to the database.\n"
                    "    ///\n"
                    "    pub fn is_viewer_by_id(\n"
                    f"        {struct.get_formatted_primary_keys(include_prefix=False)}: {struct.get_formatted_primary_key_data_types()},\n"
                    "        author_user_id: i32,\n"
                    "        connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>\n"
                    "    ) -> Result<bool, diesel::result::Error> {\n"
                    "        Self::has_role_by_id(\n"
                    f"            {struct.get_formatted_primary_keys(include_prefix=False)},\n"
                    "            author_user_id,\n"
                    "            3,\n"
                    "            connection,\n"
                    "        )\n"
                    "    }\n"
                )

                # We now create the more specific methods that check whether the user has a role
                # with a role_id less than or equal to the provided role_id. We continue with the Editor role.

                file.write(
                    "    /// Check whether the user is an Editor (role_id >= 2).\n"
                    "    ///\n"
                    "    /// # Arguments\n"
                    "    /// * `author_user_id` - The ID of the user to check.\n"
                    "    /// * `connection` - The connection to the database.\n"
                    "    ///\n"
                    "    pub fn is_editor(\n"
                    "        &self,\n"
                    "        author_user_id: i32,\n"
                    "        connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>\n"
                    "    ) -> Result<bool, diesel::result::Error> {\n"
                    "        Self::is_editor_by_id(\n"
                    f"            {struct.get_formatted_primary_keys(include_prefix=True)},\n"
                    "            author_user_id,\n"
                    "            connection,\n"
                    "        )\n"
                    "    }\n"
                )

                file.write(
                    "    /// Check whether the user is an Editor (role_id >= 2).\n"
                    "    ///\n"
                    "    /// # Arguments\n"
                    f"    /// * `{struct.get_formatted_primary_keys(include_prefix=False)}` - The primary key(s) of the struct to delete.\n"
                    "    /// * `author_user_id` - The ID of the user to check.\n"
                    "    /// * `connection` - The connection to the database.\n"
                    "    ///\n"
                    "    pub fn is_editor_by_id(\n"
                    f"        {struct.get_formatted_primary_keys(include_prefix=False)}: {struct.get_formatted_primary_key_data_types()},\n"
                    "        author_user_id: i32,\n"
                    "        connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>\n"
                    "    ) -> Result<bool, diesel::result::Error> {\n"
                    "        Self::has_role_by_id(\n"
                    f"            {struct.get_formatted_primary_keys(include_prefix=False)},\n"
                    "            author_user_id,\n"
                    "            2,\n"
                    "            connection,\n"
                    "        )\n"
                    "    }\n"
                )

                # We now create the more specific methods that check whether the user has a role
                # with a role_id less than or equal to the provided role_id. We finish with the Admin role.

                file.write(
                    "    /// Check whether the user is an Admin (role_id == 1).\n"
                    "    ///\n"
                    "    /// # Arguments\n"
                    "    /// * `author_user_id` - The ID of the user to check.\n"
                    "    /// * `connection` - The connection to the database.\n"
                    "    ///\n"
                    "    pub fn is_admin(\n"
                    "        &self,\n"
                    "        author_user_id: i32,\n"
                    "        connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>\n"
                    "    ) -> Result<bool, diesel::result::Error> {\n"
                    "        Self::is_admin_by_id(\n"
                    f"            {struct.get_formatted_primary_keys(include_prefix=True)},\n"
                    "            author_user_id,\n"
                    "            connection,\n"
                    "        )\n"
                    "    }\n"
                )

                file.write(
                    "    /// Check whether the user is an Admin (role_id == 1).\n"
                    "    ///\n"
                    "    /// # Arguments\n"
                    f"    /// * `{struct.get_formatted_primary_keys(include_prefix=False)}` - The primary key(s) of the struct to delete.\n"
                    "    /// * `author_user_id` - The ID of the user to check.\n"
                    "    /// * `connection` - The connection to the database.\n"
                    "    ///\n"
                    "    pub fn is_admin_by_id(\n"
                    f"        {struct.get_formatted_primary_keys(include_prefix=False)}: {struct.get_formatted_primary_key_data_types()},\n"
                    "        author_user_id: i32,\n"
                    "        connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>\n"
                    "    ) -> Result<bool, diesel::result::Error> {\n"
                    "        Self::has_role_by_id(\n"
                    f"            {struct.get_formatted_primary_keys(include_prefix=False)},\n"
                    "            author_user_id,\n"
                    "            1,\n"
                    "            connection,\n"
                    "        )\n"
                    "    }\n"
                )

            # For all tables we implement a `all` method that retrieves all of
            # the rows in the table structured as a vector of the struct.

            for editable_variant in editable_variants:
                if editable_variant:
                    file.write(
                        "    /// Get all of the editable structs from the database.\n"
                    )
                else:
                    file.write("    /// Get all of the structs from the database.\n")
                file.write("    ///\n" "    /// # Arguments\n")

                if editable_variant:
                    file.write(
                        "    /// * `author_user_id` - The ID of the user who is performing the search.\n"
                    )

                if struct.has_filter_variant():
                    file.write(
                        "    /// * `filter` - The optional filter to apply to the query.\n"
                    )

                file.write(
                    "    /// * `limit` - The maximum number of structs to retrieve. By default, this is 10.\n"
                    "    /// * `offset` - The number of structs to skip. By default, this is 0.\n"
                    "    /// * `connection` - The connection to the database.\n"
                    "    ///\n"
                )
                if editable_variant:
                    file.write(
                        "    pub fn all_editables(\n" "        author_user_id: i32,\n"
                    )
                else:
                    file.write("    pub fn all(\n")

                if struct.has_filter_variant():
                    filter_struct = struct.get_filter_variant()
                    file.write(
                        f"        filter: Option<&web_common::database::{filter_struct.name}>,\n"
                    )
                file.write(
                    "        limit: Option<i64>,\n"
                    "        offset: Option<i64>,\n"
                    "        connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>\n"
                )
                file.write("    ) -> Result<Vec<Self>, diesel::result::Error> {\n")
                if table_type == "tables":
                    file.write(f"        use crate::schema::{struct.table_name};\n")
                else:
                    file.write(
                        f"        use crate::views::schema::{struct.table_name};\n"
                    )
                # If the limit is None, we do not apply any limit to the query.

                if struct.has_filter_variant():
                    file.write(
                        f"        let mut query = {struct.table_name}::dsl::{struct.table_name}\n"
                    )
                else:
                    file.write(
                        f"        {struct.table_name}::dsl::{struct.table_name}\n"
                    )

                if editable_variant:
                    file.write(
                        f"           .filter({struct.table_name}::dsl::created_by.eq(author_user_id))\n"
                        "            .or_filter(\n"
                        f"               {struct.table_name}::dsl::id.eq_any(\n"
                        f"                   {struct.table_name}_users_roles::table\n"
                        f"                       .select({struct.table_name}_users_roles::dsl::table_id)\n"
                        f"                       .filter({struct.table_name}_users_roles::dsl::user_id.eq(author_user_id)\n"
                        f"                       .and({struct.table_name}_users_roles::dsl::role_id.le(2)),\n"
                        "               )),\n"
                        "            )\n"
                    )
                    if struct.table_name != "teams":
                        file.write(
                            "                .or_filter(\n"
                            f"                   {struct.table_name}::dsl::id.eq_any(\n"
                            f"                       {struct.table_name}_teams_roles::table\n"
                            f"                           .select({struct.table_name}_teams_roles::dsl::table_id)\n"
                            f"                           .filter({struct.table_name}_teams_roles::dsl::role_id.le(2))\n"
                            "                           .inner_join(teams_users_roles::table.on(\n"
                            f"                               {struct.table_name}_teams_roles::dsl::team_id.eq(teams_users_roles::dsl::table_id)\n"
                            "                                   .and(teams_users_roles::dsl::user_id.eq(author_user_id))\n"
                            "                                   .and(teams_users_roles::dsl::role_id.le(2)),\n"
                            "                           )),\n"
                            "                   ),\n"
                            "            )\n"
                        )

                if struct.has_filter_variant():
                    # We need to filter the query by the filter struct,
                    # but these filters are composed by several optional fields
                    # and therefore we need to box the query and apply the filters
                    # only if they are not None.
                    file.write("            .into_boxed();\n")

                    filter_struct = struct.get_filter_variant()

                    for filter_attribute in filter_struct.attributes:
                        file.write(
                            f"        if let Some(value) = filter.and_then(|f| f.{filter_attribute.name}) {{\n"
                            f"            query = query.filter({filter_struct.table_name}::dsl::{filter_attribute.name}.eq(value));\n"
                            "        }\n"
                        )

                    file.write("        query\n")

                file.write(
                    "            .offset(offset.unwrap_or(0))\n"
                    "            .limit(limit.unwrap_or(10))\n"
                    "            .load::<Self>(connection)\n"
                    "    }\n"
                )

            # For the tables that have an updated_at column, we implement the
            # `all_by_updated_at` method that retrieves all of the rows in the
            # table structured as a vector of the struct ordered by the updated_at
            # column in descending order.

            if table_metadata.has_updated_at_column(struct.table_name):
                file.write(
                    "    /// Get all of the structs from the database ordered by the updated_at column.\n"
                    "    ///\n"
                    "    /// # Arguments\n"
                )

                if struct.has_filter_variant():
                    file.write(
                        "    /// * `filter` - The optional filter to apply to the query.\n"
                    )

                file.write(
                    "    /// * `limit` - The maximum number of structs to retrieve. By default, this is 10.\n"
                    "    /// * `offset` - The number of structs to skip. By default, this is 0.\n"
                    "    /// * `connection` - The connection to the database.\n"
                    "    ///\n"
                    "    pub fn all_by_updated_at(\n"
                )
                if struct.has_filter_variant():
                    filter_struct = struct.get_filter_variant()
                    file.write(
                        f"        filter: Option<&web_common::database::{filter_struct.name}>,\n"
                    )

                file.write(
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
                if struct.has_filter_variant():
                    filter_struct = struct.get_filter_variant()
                    file.write(
                        f"        let mut query = {struct.table_name}::dsl::{struct.table_name}\n"
                        "            .into_boxed();\n"
                    )

                    for filter_attribute in filter_struct.attributes:
                        file.write(
                            f"        if let Some(value) = filter.and_then(|f| f.{filter_attribute.name}) {{\n"
                            f"            query = query.filter({filter_struct.table_name}::dsl::{filter_attribute.name}.eq(value));\n"
                            "        }\n"
                        )

                    file.write("        query\n")
                else:
                    file.write(
                        f"        {struct.table_name}::dsl::{struct.table_name}\n"
                    )

                file.write(
                    f"            .order_by({struct.table_name}::dsl::updated_at.desc())\n"
                    "            .offset(offset.unwrap_or(0))\n"
                    "            .limit(limit.unwrap_or(10))\n"
                    "            .load::<Self>(connection)\n"
                    "    }\n"
                )

            if struct.has_associated_roles() and struct.table_name != "users":
                file.write(
                    "    /// Delete the struct from the database.\n"
                    "    ///\n"
                    "    /// # Arguments\n"
                    "    /// * `author_user_id` - The ID of the user who is deleting the struct.\n"
                    "    /// * `connection` - The connection to the database.\n"
                    "    ///\n"
                    "    pub fn delete(\n"
                    "        &self,\n"
                    "        author_user_id: i32,\n"
                    "        connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>\n"
                    "    ) -> Result<usize, diesel::result::Error> {\n"
                    f"        Self::delete_by_id({struct.get_formatted_primary_keys(include_prefix=True)}, author_user_id, connection)\n"
                    "    }\n"
                )

                file.write(
                    "    /// Delete the struct from the database by its ID.\n"
                    "    ///\n"
                    "    /// # Arguments\n"
                    f"    /// * `{struct.get_formatted_primary_keys(include_prefix=False)}` - The primary key(s) of the struct to delete.\n"
                    "    /// * `author_user_id` - The ID of the user who is deleting the struct.\n"
                    "    /// * `connection` - The connection to the database.\n"
                    "    ///\n"
                    "    pub fn delete_by_id(\n"
                    f"       {struct.get_formatted_primary_keys(include_prefix=False)}: {struct.get_formatted_primary_key_data_types()},\n"
                    "        author_user_id: i32,\n"
                    "        connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>\n"
                    "    ) -> Result<usize, diesel::result::Error> {\n"
                    f"        if !Self::is_admin_by_id({struct.get_formatted_primary_keys(include_prefix=False)}, author_user_id, connection)? {{\n"
                    "            return Err(diesel::result::Error::NotFound);\n"
                    "        }\n"
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
                file.write(f"        use crate::views::schema::{struct.table_name};\n")
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
                    unique_column.as_ref() for unique_column in unique_columns
                ]

                if len(reference_unique_columns) == 1:
                    human_readable_unique_columns = reference_unique_columns[0].name
                else:
                    human_readable_unique_columns = (
                        ", ".join(
                            [column.name for column in reference_unique_columns[:-1]]
                        )
                        + f" and {reference_unique_columns[-1].name}"
                    )

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
                        file.write(
                            f"    /// * `{unique_column.name}` - The {unique_column.name} of the struct to get.\n"
                        )

                    file.write(
                        "    /// * `connection` - The connection to the database.\n"
                        "    ///\n"
                        f"    pub fn {from_method_name}(\n"
                    )
                    for unique_column in reference_unique_columns:
                        file.write(
                            f"        {unique_column.name}: {unique_column.format_data_type()},\n"
                        )

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
                    file.write("            .first::<Self>(connection)\n" "    }\n")

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

                    for editable_filter in editable_variants:

                        if editable_filter:
                            file.write(
                                f"    /// Search for the editable struct by a given string by Postgres's `{method_name}`.\n"
                            )
                        else:
                            file.write(
                                f"    /// Search for the struct by a given string by Postgres's `{method_name}`.\n"
                            )
                        file.write("    ///\n    /// # Arguments\n")

                        if editable_filter:
                            # In the editable filter version we also receive in input the author_user_id
                            # of the user who is performing the search.
                            file.write(
                                "    /// * `author_user_id` - The ID of the user who is performing the search.\n"
                            )
                        file.write(
                            "    /// * `query` - The string to search for.\n"
                            "    /// * `limit` - The maximum number of results, by default `10`.\n"
                            "    /// * `connection` - The connection to the database.\n"
                            "    ///\n"
                        )
                        if editable_filter:
                            file.write(f"    pub fn {method_name}_search_editables(\n")
                        else:
                            file.write(f"    pub fn {method_name}_search(\n")

                        if editable_filter:
                            file.write("        author_user_id: i32,\n")

                        file.write(
                            "        query: &str,\n"
                            "        limit: Option<i32>,\n"
                            "        connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>\n"
                        )

                        file.write(
                            "    ) -> Result<Vec<Self>, diesel::result::Error> {\n"
                            "        let limit = limit.unwrap_or(10);\n"
                            "        // If the query string is empty, we run an all query with the\n"
                            "        // limit parameter provided instead of a more complex similarity\n"
                            "        // search.\n"
                            "        if query.is_empty() {\n"
                        )
                        if editable_filter:
                            if struct.has_filter_variant():
                                file.write(
                                    "            return Self::all_editables(author_user_id, None, Some(limit as i64), None, connection);\n"
                                )
                            else:
                                file.write(
                                    "            return Self::all_editables(author_user_id, Some(limit as i64), None, connection);\n"
                                )
                        else:
                            if struct.has_filter_variant():
                                file.write(
                                    "            return Self::all(None, Some(limit as i64), None, connection);\n"
                                )
                            else:
                                file.write(
                                    "            return Self::all(Some(limit as i64), None, connection);\n"
                                )
                        file.write("        }\n")

                        joined_field_names = ", ".join(
                            attribute.name for attribute in struct.attributes
                        )

                        editable_where_clause = ""

                        if editable_filter:
                            editable_where_clause = (
                                f'            "AND {struct.table_name}.created_by = $3 ",\n'
                                f'            "OR {struct.table_name}.id IN ",\n'
                                f'            "(SELECT {struct.table_name}_users_roles.table FROM {struct.table_name}_users_roles ",\n'
                                f'            "WHERE {struct.table_name}_users_roles.user_id = $3 AND {struct.table_name}_users_roles.role_id <= 2) ",\n'
                                f'            "OR {struct.table_name}.id IN ",\n'
                                f'            "(SELECT {struct.table_name}_teams_roles.table_id FROM {struct.table_name}_teams_roles ",\n'
                                f'            "WHERE {struct.table_name}_teams_roles.role_id <= 2 AND {struct.table_name}_teams_roles.table_id IN ",\n'
                                '            "(SELECT teams_users_roles.table_id FROM teams_users_roles ",\n'
                                '            "WHERE teams_users_roles.user_id = $3 AND teams_users_roles.role_id <= 2)) ",\n'
                            )

                        if table_type == "views":
                            file.write(
                                "        let similarity_query = concat!(\n"
                                '            "WITH selected_ids AS (",\n'
                                f'            "SELECT {similarity_index.table_name}.{primary_key.name} AS id FROM {similarity_index.table_name} ",\n'
                            )
                            if similarity_index.is_gin():
                                file.write(
                                    f'            "WHERE $1 {similarity_operator} {similarity_index.arguments} ",\n'
                                    f"{editable_where_clause}"
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
                                    f"{editable_where_clause}"
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
                        )
                        if editable_filter:
                            file.write(
                                "            .bind::<diesel::sql_types::Integer, _>(author_user_id)\n"
                            )
                        file.write("            .load(connection)\n" "}\n")

            # Finally, we cluse the struct implementation.
            file.write("}\n")


def extract_structs(path: str) -> List[StructMetadata]:
    """Extract the structs from the Rust file at the given path.

    Parameters
    ----------
    path : str
        The path to the Rust file.
    """
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
                struct_metadata.table_metadata.register_flat_variant(
                    struct_metadata.table_name, struct_metadata.name
                )
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
    table_metadata: TableMetadata,
):
    """Write the structs in the target file in the `web_common` crate.

    Parameters
    ----------
    structs : List[StructMetadata]
        The list of structs to write in the target file.
    target : str
        The path where to write the structs in the `web_common` crate.
    table_metadata : TableMetadata
        The metadata of the tables.
    """
    # The derive statements to include in the `src/database/tables.rs` document
    imports = [
        "use serde::Deserialize;",
        "use serde::Serialize;",
        "use uuid::Uuid;",
        "use chrono::NaiveDateTime;",
        "use crate::database::*;",
    ]

    document = open(f"../web_common/src/database/{target}.rs", "w", encoding="utf8")

    for import_statament in imports:
        document.write(f"{import_statament}\n")

    # First, we define the Tabular & Filtrable traits which we will implement for all of the
    # structs.
    if target == "tables":
        document.write(
            "\npub trait Tabular {\n"
            "    const TABLE: Table;\n"
            "}\n\n"
            "pub trait Filtrable: PartialEq {\n"
            "    type Filter: Serialize + PartialEq + Clone;\n"
            "}\n\n"
        )

    for struct in tqdm(
        structs,
        desc="Writing frontend structs",
        unit="struct",
        leave=False,
    ):
        # We write the struct definition.
        struct.write_to(document)

        # We implement the Tabular trait for the struct. This trait
        # is used to convert the struct into a Table variant.

        document.write(f"impl Tabular for {struct.name} {{\n")
        document.write(
            f"    const TABLE: Table = Table::{struct.capitalized_table_name()};\n"
        )
        document.write("}\n")

        # We implement the Filtrable trait for the struct. This trait
        # is used to provide the informations on the filter struct that
        # can be used to filter the struct in the database.
        if struct.has_filter_variant():
            filter_struct = struct.get_filter_variant()
            document.write(f"\nimpl Filtrable for {struct.name} {{\n")
            document.write(f"    type Filter = {filter_struct.name};\n")
            document.write("}\n")
        elif struct.table_name == "users":
            document.write(f"\nimpl Filtrable for {struct.name} {{\n")
            document.write("    type Filter = EmptyFilter;\n")
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
        )
        if struct.has_filter_variant():
            document.write("    /// * `filter` - The filter to apply to the results.\n")
        document.write(
            "    /// * `limit` - The maximum number of results, by default `10`.\n"
            "    /// * `offset` - The offset of the results, by default `0`.\n"
            "    /// * `connection` - The connection to the database.\n"
            "    ///\n"
            "    pub async fn all<C>(\n"
        )
        if struct.has_filter_variant():
            filter_variant = struct.get_filter_variant()
            document.write(f"        filter: Option<&{filter_variant.name}>,\n")
        document.write(
            "        limit: Option<i64>,\n"
            "        offset: Option<i64>,\n"
            "        connection: &mut gluesql::prelude::Glue<C>,\n"
            "    ) -> Result<Vec<Self>, gluesql::prelude::Error> where\n"
            "        C: gluesql::core::store::GStore + gluesql::core::store::GStoreMut,\n"
            "    {\n"
            "        use gluesql::core::ast_builder::*;\n"
            f'        let select_row = table("{struct.table_name}")\n'
            "            .select()\n"
        )
        if struct.has_filter_variant():
            document.write(
                "            .filter(filter.map_or_else(|| gluesql::core::ast::Expr::Literal(gluesql::core::ast::AstLiteral::Boolean(true)).into(), |filter| filter.as_filter_expression()))\n"
            )
        document.write(
            f'           .project("{columns}")\n'
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
        if table_metadata.has_updated_at_column(struct.table_name):
            document.write(
                f"    /// Get all {struct.name} from the database ordered by the `updated_at` column.\n"
                "    ///\n"
                "    /// # Arguments\n"
            )
            if struct.has_filter_variant():
                document.write(
                    "    /// * `filter` - The filter to apply to the results.\n"
                )
            document.write(
                "    /// * `limit` - The maximum number of results, by default `10`.\n"
                "    /// * `offset` - The offset of the results, by default `0`.\n"
                "    /// * `connection` - The connection to the database.\n"
                "    ///\n"
                "    pub async fn all_by_updated_at<C>(\n"
            )
            if struct.has_filter_variant():
                filter_variant = struct.get_filter_variant()
                document.write(f"        filter: Option<&{filter_variant.name}>,\n")
            document.write(
                "        limit: Option<i64>,\n"
                "        offset: Option<i64>,\n"
                "        connection: &mut gluesql::prelude::Glue<C>,\n"
                "    ) -> Result<Vec<Self>, gluesql::prelude::Error> where\n"
                "        C: gluesql::core::store::GStore + gluesql::core::store::GStoreMut,\n"
                "    {\n"
                "        use gluesql::core::ast_builder::*;\n"
                f'        let select_row = table("{struct.table_name}")\n'
                "            .select()\n"
            )
            if struct.has_filter_variant():
                document.write(
                    "            .filter(filter.map_or_else(|| gluesql::core::ast::Expr::Literal(gluesql::core::ast::AstLiteral::Boolean(true)).into(), |filter| filter.as_filter_expression()))\n"
                )
            document.write(
                f'           .project("{columns}")\n'
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

    last_line_was_table = False
    brackets_count = 0
    view_structs = []

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
    path: str, struct_metadatas: List[StructMetadata]
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
        # "use crate::views::views::*;",
    ]

    document.write("\n".join(imports) + "\n\n")

    def get_struct_by_table_name(table_name: str) -> StructMetadata:
        for struct in struct_metadatas:
            if struct.table_name == table_name:
                return struct
        raise ValueError(f"Table name {table_name} not found in the struct metadata.")

    nested_structs = []

    # For each of the struct, we generated the Nested{struct_name} version
    # if the struct contains a reference to another struct.
    for struct in tqdm(
        struct_metadatas, desc="Generating nested structs", leave=False, unit="struct"
    ):
        foreign_keys = struct.get_foreign_keys()

        if len(foreign_keys) == 0:
            continue

        nested_struct = StructMetadata(f"Nested{struct.name}", struct.table_name)
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
            foreign_key_table_name = struct.table_metadata.get_foreign_key_table_name(
                struct.table_name, attribute.name
            )

            normalized_attribute_name = attribute.normalized_name()
            foreign_struct = get_struct_by_table_name(foreign_key_table_name)

            if (
                struct.name == foreign_struct.name
                or not foreign_struct.has_foreign_keys()
            ):
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
                nested_struct.attributes = []
                for new_attribute in new_attributes:
                    nested_struct.add_attribute(new_attribute)
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
        flat_variant = nested_struct.get_flat_variant()

        # We implement the all for the nested structs

        # First, we implement a method that will be reused by several of the following methods,
        # including the all, get and search ones: a method that given the flat struct and a connection
        # to the database returns a result containing the nested struct.
        document.write(
            f"impl {nested_struct.name} {{\n"
            "    /// Convert the flat struct to the nested struct.\n"
            "    ///\n"
            "    /// # Arguments\n"
            "    /// * `flat_variant` - The flat struct.\n"
            "    /// * `connection` - The database connection.\n"
            "    pub fn from_flat(\n"
            f"        flat_variant: {flat_variant.name},\n"
            "        connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>,\n"
            "    ) -> Result<Self, diesel::result::Error> {\n"
            "        Ok(Self {\n"
        )
        for attribute in nested_struct.attributes:
            if attribute.name == "inner":
                continue
            if (
                attribute.data_type() == nested_struct.name
                or flat_variant.has_attribute(attribute)
            ):
                document.write(
                    f"            {attribute.name}: flat_variant.{attribute.name},\n"
                )
                continue
            if attribute.optional:
                document.write(
                    f"            {attribute.name}: flat_variant.{attribute.original_name}.map(|flat_variant| {attribute.data_type()}::get(flat_variant, connection)).transpose()?,\n"
                )
            else:
                document.write(
                    f"            {attribute.name}: {attribute.data_type()}::get(flat_variant.{attribute.original_name}, connection)?,\n"
                )

        document.write(
            "                inner: flat_variant,\n" "        })\n" "    }\n" "}\n"
        )

        # Then we implement the all query.

        document.write(
            f"impl {nested_struct.name} {{\n"
            "    /// Get all the nested structs from the database.\n"
            "    ///\n"
            "    /// # Arguments\n"
        )
        if nested_struct.has_filter_variant():
            document.write("    /// * `filter` - The filter to apply to the results.\n")
        document.write(
            "    /// * `limit` - The maximum number of rows to return. By default `10`.\n"
            "    /// * `offset` - The offset of the rows to return. By default `0`.\n"
            "    /// * `connection` - The database connection.\n"
            "    pub fn all(\n"
        )
        if nested_struct.has_filter_variant():
            filter_variant = nested_struct.get_filter_variant()
            document.write(
                f"        filter: Option<&web_common::database::{filter_variant.name}>,\n"
            )
        document.write(
            "        limit: Option<i64>,\n"
            "        offset: Option<i64>,\n"
            "        connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>,\n"
            "    ) -> Result<Vec<Self>, diesel::result::Error> {\n"
        )
        if nested_struct.has_filter_variant():
            document.write(
                f"        {flat_variant.name}::all(filter, limit, offset, connection)?.into_iter().map(|flat_variant| Self::from_flat(flat_variant, connection)).collect()\n"
            )
        else:
            document.write(
                f"        {flat_variant.name}::all(limit, offset, connection)?.into_iter().map(|flat_variant| Self::from_flat(flat_variant, connection)).collect()\n"
            )
        document.write("    }\n}\n")

        if nested_struct.has_associated_roles() and nested_struct.table_name != "users":
            document.write(
                f"impl {nested_struct.name} {{\n"
                "    /// Get all the editables nested structs from the database.\n"
                "    ///\n"
                "    /// # Arguments\n"
                "    /// * `author_user_id` - The user id.\n"
            )
            if nested_struct.has_filter_variant():
                document.write(
                    "    /// * `filter` - The filter to apply to the results.\n"
                )
            document.write(
                "    /// * `limit` - The maximum number of rows to return. By default `10`.\n"
                "    /// * `offset` - The offset of the rows to return. By default `0`.\n"
                "    /// * `connection` - The database connection.\n"
                "    pub fn all_editables(\n"
                "        author_user_id: i32,\n"
            )
            if nested_struct.has_filter_variant():
                filter_variant = nested_struct.get_filter_variant()
                document.write(
                    f"        filter: Option<&web_common::database::{filter_variant.name}>,\n"
                )
            document.write(
                "        limit: Option<i64>,\n"
                "        offset: Option<i64>,\n"
                "        connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>,\n"
                "    ) -> Result<Vec<Self>, diesel::result::Error> {\n"
            )
            if nested_struct.has_filter_variant():
                document.write(
                    f"        {flat_variant.name}::all_editables(author_user_id, filter, limit, offset, connection)?.into_iter().map(|flat_variant| Self::from_flat(flat_variant, connection)).collect()\n"
                )
            else:
                document.write(
                    f"        {flat_variant.name}::all_editables(author_user_id, limit, offset, connection)?.into_iter().map(|flat_variant| Self::from_flat(flat_variant, connection)).collect()\n"
                )
            document.write("    }\n}\n")

        # Then, for all the tables that have an updated_at column, we implement the
        # `all_by_updated_at` method, which returns all of the nested structs ordered
        # by the `updated_at` column.
        if flat_variant.table_metadata.has_updated_at_column(flat_variant.table_name):
            document.write(
                f"impl {nested_struct.name} {{\n"
                "    /// Get all the nested structs from the database ordered by the `updated_at` column.\n"
                "    ///\n"
                "    /// # Arguments\n"
            )
            if nested_struct.has_filter_variant():
                document.write(
                    "    /// * `filter` - The filter to apply to the results.\n"
                )
            document.write(
                "    /// * `limit` - The maximum number of rows to return. By default `10`.\n"
                "    /// * `offset` - The offset of the rows to return. By default `0`.\n"
                "    /// * `connection` - The database connection.\n"
                "    pub fn all_by_updated_at(\n"
            )
            if nested_struct.has_filter_variant():
                filter_variant = nested_struct.get_filter_variant()
                document.write(
                    f"        filter: Option<&web_common::database::{filter_variant.name}>,\n"
                )
            document.write(
                "        limit: Option<i64>,\n"
                "        offset: Option<i64>,\n"
                "        connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>,\n"
                "    ) -> Result<Vec<Self>, diesel::result::Error> {\n"
            )
            if nested_struct.has_filter_variant():
                document.write(
                    f"        {flat_variant.name}::all_by_updated_at(filter, limit, offset, connection)?.into_iter().map(|flat_variant| Self::from_flat(flat_variant, connection)).collect()\n"
                )
            else:
                document.write(
                    f"        {flat_variant.name}::all_by_updated_at(limit, offset, connection)?.into_iter().map(|flat_variant| Self::from_flat(flat_variant, connection)).collect()\n"
                )
            document.write("    }\n}\n")

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
            f"       {flat_variant.name}::get({nested_struct.get_formatted_primary_keys(include_prefix=False)}, connection).and_then(|flat_variant| Self::from_flat(flat_variant, connection))\n"
            "    }\n"
            "}\n"
        )

        # For each of the columns in the struct that have a UNIQUE constraint,
        # we implement the methods `from_{column_name}` by employing the method
        # of the same name available for the main struct associated to this struct
        for unique_columns in flat_variant.get_unique_constraints():

            unique_column_references = [
                unique_column.as_ref() for unique_column in unique_columns
            ]

            joined = "_and_".join(
                [unique_column.name for unique_column in unique_column_references]
            )
            from_method_name = f"from_{joined}"

            if len(unique_column_references) == 1:
                human_readable_column_names = unique_column_references[0].name
            else:
                human_readable_column_names = (
                    ", ".join(
                        [
                            unique_column.name
                            for unique_column in unique_column_references[:-1]
                        ]
                    )
                    + f" and {unique_column_references[-1].name}"
                )

            comma_separated_column_names = ", ".join(
                [unique_column.name for unique_column in unique_column_references]
            )

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
                f"        {flat_variant.name}::{from_method_name}({comma_separated_column_names}, connection).and_then(|flat_variant| Self::from_flat(flat_variant, connection))\n"
                "    }\n"
                "}\n"
            )

        # If there is an index on the table, we implement the search method that
        # calls search on the flat version of the struct and then iterates on the
        # primary keys of the results and constructs the nested structs by calling
        # the `get` method several times.
        if flat_variant.is_searchable():
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
                    f"       {flat_variant.name}::{method_name}_search(query, limit, connection)?.into_iter().map(|flat_variant| Self::from_flat(flat_variant, connection)).collect()\n"
                    "    }\n"
                    "}\n"
                )

                if (
                    nested_struct.has_associated_roles()
                    and nested_struct.table_name != "users"
                ):
                    document.write(
                        f"impl {nested_struct.name} {{\n"
                        "    /// Search the table by the query.\n"
                        "    ///\n"
                        "    /// # Arguments\n"
                        "    /// * `author_user_id` - The user id.\n"
                        "    /// * `query` - The string to search for.\n"
                        "    /// * `limit` - The maximum number of results, by default `10`.\n"
                        f"    pub fn {method_name}_search_editables(\n"
                        "        author_user_id: i32,\n"
                        "        query: &str,\n"
                        "        limit: Option<i32>,\n"
                        "        connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>,\n"
                        "    ) -> Result<Vec<Self>, diesel::result::Error> {\n"
                        f"       {flat_variant.name}::{method_name}_search_editables(author_user_id, query, limit, connection)?.into_iter().map(|flat_variant| Self::from_flat(flat_variant, connection)).collect()\n"
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


def write_web_common_nested_structs(
    path: str, nested_structs: List[StructMetadata]
):
    """Writes the nested structs to the web_common crate."""

    # We open the file to write the nested structs
    document = open(f"../web_common/src/database/{path}", "w", encoding="utf8")

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
        "use super::*;",
    ]

    document.write("\n".join(imports) + "\n\n")

    for nested_struct in nested_structs:
        nested_struct.write_to(document)

        document.write(
            f"impl Tabular for {nested_struct.name} {{\n"
            f"    const TABLE: Table = Table::{nested_struct.capitalized_table_name()};\n"
            "}\n"
        )

        if nested_struct.has_filter_variant():
            filter_variant = nested_struct.get_filter_variant()

            document.write(
                f"impl Filtrable for {nested_struct.name} {{\n"
                f"    type Filter = {filter_variant.name};\n"
                "}\n"
            )

        # We implement the `get` method for the struct when the frontend feature
        # is enabled using GlueSQL. This method will be extremely similar to the
        # `get` method for the Diesel-based approach of the backend.

        flat_variant = nested_struct.get_flat_variant()

        document.write(
            f"#[cfg(feature = \"frontend\")]\nimpl {nested_struct.name} {{\n"
        )

        # First, we implement the `from_flat` method that will be used to convert
        # the flat struct to the nested struct. This method receives the flat struct
        # and the connection to the database and returns the nested struct.
        document.write(
            "    /// Convert the flat struct to the nested struct.\n"
            "    ///\n"
            "    /// # Arguments\n"
            "    /// * `flat_variant` - The flat struct.\n"
            "    /// * `connection` - The database connection.\n"
            "    pub async fn from_flat(\n"
            f"        flat_variant: {flat_variant.name},\n"
            "        connection: &mut gluesql::prelude::Glue<impl gluesql::core::store::GStore + gluesql::core::store::GStoreMut>,\n"
            "    ) -> Result<Self, gluesql::prelude::Error> {\n"
            "        Ok(Self {\n"
        )
        for attribute in nested_struct.attributes:
            if attribute.name == "inner":
                continue
            if (
                attribute.data_type() == nested_struct.name
                or flat_variant.has_attribute(attribute)
            ):
                document.write(
                    f"            {attribute.name}: flat_variant.{attribute.name},\n"
                )
                continue
            if attribute.optional:
                document.write(
                    f"            {attribute.name}: if let Some({attribute.original_name}) = flat_variant.{attribute.original_name} {{ {attribute.data_type()}::get({attribute.original_name}, connection).await? }} else {{ None }},\n"
                )
            else:
                document.write(
                    f"            {attribute.name}: {attribute.data_type()}::get(flat_variant.{attribute.original_name}, connection).await?.unwrap(),\n"
                )

        if any(attribute.name == "inner" for attribute in nested_struct.attributes):
            document.write("            inner: flat_variant,\n")

        document.write("        })\n    }\n")

        document.write(
            "    /// Get the nested struct from the provided primary key.\n"
            "    ///\n"
            "    /// # Arguments\n"
            f"    /// * `{flat_variant.get_formatted_primary_keys(include_prefix=False)}` - The primary key(s) of the row.\n"
            "    /// * `connection` - The database connection.\n"
            "    pub async fn get<C>(\n"
            f"        {flat_variant.get_formatted_primary_keys(include_prefix=False)}: {flat_variant.get_formatted_primary_key_data_types()},\n"
            "        connection: &mut gluesql::prelude::Glue<C>,\n"
            "    ) -> Result<Option<Self>, gluesql::prelude::Error> where\n"
            "        C: gluesql::core::store::GStore + gluesql::core::store::GStoreMut,\n"
            "    {\n"
            f"       let flat_variant = {flat_variant.name}::get({flat_variant.get_formatted_primary_keys(include_prefix=False)}, connection).await?;"
            "        match flat_variant {\n"
            "            Some(flat_variant) => Ok(Some(Self::from_flat(flat_variant, connection).await?)),\n"
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
        )
        if nested_struct.has_filter_variant():
            document.write("    /// * `filter` - The filter to apply to the results.\n")
        document.write(
            "    /// * `limit` - The maximum number of rows to return.\n"
            "    /// * `offset` - The number of rows to skip.\n"
            "    /// * `connection` - The database connection.\n"
            "    pub async fn all<C>(\n"
        )
        if nested_struct.has_filter_variant():
            filter_variant = nested_struct.get_filter_variant()
            document.write(f"        filter: Option<&{filter_variant.name}>,\n")
        document.write(
            "        limit: Option<i64>,\n"
            "        offset: Option<i64>,\n"
            "        connection: &mut gluesql::prelude::Glue<C>,\n"
            "    ) -> Result<Vec<Self>, gluesql::prelude::Error> where\n"
            "        C: gluesql::core::store::GStore + gluesql::core::store::GStoreMut,\n"
            "    {\n"
        )
        if nested_struct.has_filter_variant():
            document.write(
                f"        let flat_variants = {flat_variant.name}::all(filter, limit, offset, connection).await?;\n"
            )
        else:
            document.write(
                f"        let flat_variants = {flat_variant.name}::all(None, limit, offset, connection).await?;\n"
            )
        document.write(
            "         let mut nested_structs = Vec::with_capacity(flat_variants.len());\n"
            "         for flat_variant in flat_variants {\n"
            "             nested_structs.push(Self::from_flat(flat_variant, connection).await?);\n"
            "         }\n"
            "         Ok(nested_structs)\n"
            "    }\n"
        )

        # We implement the all_by_updated_at method for the struct when the frontend feature
        # is enabled using GlueSQL. This method will be extremely similar to the `all_by_updated_at`
        # method for the Diesel-based approach of the backend.

        if flat_variant.table_metadata.has_updated_at_column(flat_variant.table_name):
            document.write(
                "    /// Get all the nested structs from the database ordered by the `updated_at` column.\n"
                "    ///\n"
                "    /// # Arguments\n"
            )
            if nested_struct.has_filter_variant():
                document.write(
                    "    /// * `filter` - The filter to apply to the results.\n"
                )
            document.write(
                "    /// * `limit` - The maximum number of rows to return.\n"
                "    /// * `offset` - The number of rows to skip.\n"
                "    /// * `connection` - The database connection.\n"
                "    pub async fn all_by_updated_at<C>(\n"
            )
            if nested_struct.has_filter_variant():
                filter_variant = nested_struct.get_filter_variant()
                document.write(f"        filter: Option<&{filter_variant.name}>,\n")
            document.write(
                "        limit: Option<i64>,\n"
                "        offset: Option<i64>,\n"
                "        connection: &mut gluesql::prelude::Glue<C>,\n"
                "    ) -> Result<Vec<Self>, gluesql::prelude::Error> where\n"
                "        C: gluesql::core::store::GStore + gluesql::core::store::GStoreMut,\n"
                "    {\n"
            )
            if nested_struct.has_filter_variant():
                document.write(
                    f"        let flat_variants = {flat_variant.name}::all_by_updated_at(filter, limit, offset, connection).await?;\n"
                )
            else:
                document.write(
                    f"        let flat_variants = {flat_variant.name}::all_by_updated_at(None, limit, offset, connection).await?;\n"
                )
            document.write(
                "         let mut nested_structs = Vec::with_capacity(flat_variants.len());\n"
                "         for flat_variant in flat_variants {\n"
                "             nested_structs.push(Self::from_flat(flat_variant, connection).await?);\n"
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
        for attribute in nested_struct.attributes:
            assert attribute.has_struct_data_type()

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
) -> List[TableStructMetadata]:
    imports = ["use serde::Deserialize;", "use serde::Serialize;", "use super::*;"]

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
        struct.set_richest_variant(tables[struct.table_name].get_richest_struct())

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
                f"                let filter: Option<{filter_variant.name}> = filter.map(|filter| bincode::deserialize(&filter).map_err(crate::api::ApiError::from)).transpose()?;\n"
                f"                crate::database::{table.richest_struct_name()}::all(filter.as_ref(), limit, offset, connection).await?.into_iter().map(|row| bincode::serialize(&row).map_err(crate::api::ApiError::from)).collect()\n"
            )
        else:
            document.write(
                '                 assert!(filter.is_none(), "Filter not implemented for this table.");\n'
                f"                crate::database::{table.richest_struct_name()}::all(limit, offset, connection).await?.into_iter().map(|row| bincode::serialize(&row).map_err(crate::api::ApiError::from)).collect()\n"
            )

        document.write("            },\n")

    document.write("        }\n    }\n")

    # Next, for all the tables that have an updated_at column, we implement the
    # `all_by_updated_at` method, which returns all of the rows ordered by the
    # `updated_at` column. When the table does not have an `updated_at` column,
    # we panic with an unimplemented!() macro.

    document.write(
        "    /// Get all the rows from the table ordered by the `updated_at` column.\n"
        "    ///\n"
        "    /// # Arguments\n"
        "    /// * `filter` - The filter to apply to the rows.\n"
        "    /// * `limit` - The maximum number of rows to return.\n"
        "    /// * `offset` - The number of rows to skip. By default `0`.\n"
        "    /// * `connection` - The database connection.\n"
        "    ///\n"
        "    /// # Returns\n"
        "    /// A vector of the rows of the table.\n"
        "    pub async fn all_by_updated_at<C>(\n"
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
        if table.has_updated_at_column():
            document.write(f"            Table::{table.camel_cased()} => {{\n")
            if table.has_filter_variant():
                filter_variant = table.get_filter_variant()
                document.write(
                    f"                let filter: Option<{filter_variant.name}> = filter.map(|filter| bincode::deserialize(&filter).map_err(crate::api::ApiError::from)).transpose()?;\n"
                    f"                crate::database::{table.richest_struct_name()}::all_by_updated_at(filter.as_ref(), limit, offset, connection).await?.into_iter().map(|row| bincode::serialize(&row).map_err(crate::api::ApiError::from)).collect()\n"
                )
            else:
                document.write(
                    '                 assert!(filter.is_none(), "Filter not implemented for this table.");\n'
                    f"                crate::database::{table.richest_struct_name()}::all_by_updated_at(limit, offset, connection).await?.into_iter().map(|row| bincode::serialize(&row).map_err(crate::api::ApiError::from)).collect()\n"
                )
            document.write("            },\n")
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
    tables_metadata: TableMetadata,
):
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

    search_indices: PGIndices = find_pg_trgm_indices(tables_metadata)

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
            if search_indices.has_table(table.name):
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
            if search_indices.has_table(table.name):
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

    document.write("        })\n" "    }\n" "}\n")

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

    document.write("        })\n" "    }\n" "}\n")

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

    document.write("        })\n" "    }\n")

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

    document.write("        }\n" "    }\n" "}\n")

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

    document.write("        })\n" "    }\n" "}\n")

    document.flush()
    document.close()


def write_web_common_search_trait_implementations(
    struct_metadatas: List[StructMetadata],
):
    """Writes the Searchable trait implementations for the structs.

    Parameters
    ----------
    struct_metadatas : List[StructMetadata]
        The list of the StructMetadata objects.

    """
    # We check that we are currently executing in the `backend` crate
    # so to make sure that the relative path to the `web_common` crate
    # is correct.
    if not os.getcwd().endswith("backend"):
        raise RuntimeError("This script must be executed in the `backend` crate.")

    document = open("../web_common/src/database/search_tables.rs", "w", encoding="utf8")

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
        "pub trait Searchable<const EDIT: bool> {\n"
        "    fn search_task(query: String, limit: u32) -> super::Select;\n"
        "}\n"
    )

    for struct in struct_metadatas:
        if struct.is_searchable():
            document.write(
                f"impl Searchable<false> for {struct.name} {{\n"
                "    fn search_task(query: String, limit: u32) -> super::Select {\n"
                "        super::Select::search(\n"
                f"             Table::{struct.capitalized_table_name()},\n"
                "              query,\n"
                "              limit,\n"
                "        )\n"
                "    }\n"
                "}\n"
            )

        if struct.has_associated_roles() and struct.table_name != "users":
            document.write(
                f"impl Searchable<true> for {struct.name} {{\n"
                "    fn search_task(query: String, limit: u32) -> super::Select {\n"
                f"        super::Select::search_editables(\n"
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

    new_structs = []

    for struct in tqdm(
        struct_metadatas,
        desc="Deriving new structs",
        unit="struct",
        leave=False,
    ):
        if not struct.is_insertable():
            continue

        assert not struct.is_nested()

        new_struct = StructMetadata(
            struct_name=f"New{struct.name}",
            table_name=struct.table_name,
        )

        new_struct.set_flat_variant(struct)

        primary_keys = struct.get_primary_keys()

        for derive in struct.derives():
            new_struct.add_derive(derive)

        for attribute in struct.attributes:
            if attribute.is_automatically_determined_column():
                continue

            if attribute in primary_keys:
                if attribute.is_uuid():
                    new_struct.add_attribute(attribute)
                    continue
                if len(primary_keys) == 1:
                    continue
                new_struct.add_attribute(attribute)
                continue
            new_struct.add_attribute(attribute)

        assert (
            len(new_struct.attributes) > 0
        ), f"New struct {new_struct.name} has no attributes."

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
        )

        update_struct.set_flat_variant(struct)

        for derive in struct.derives():
            update_struct.add_derive(derive)

        for attribute in struct.attributes:
            if attribute.is_automatically_determined_column():
                continue
            update_struct.add_attribute(attribute)

        assert (
            len(update_struct.attributes) > 0
        ), f"Update struct {update_struct.name} has no attributes."

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
            struct_name=f"{flat_variant.name}Builder", table_name=struct.table_name
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

        for primary_key in primary_keys:
            if primary_key not in foreign_keys:
                builder.add_attribute(
                    AttributeMetadata(
                        original_name=primary_key.original_name,
                        name=primary_key.name,
                        data_type=primary_key.raw_data_type(),
                        optional=True,
                    )
                )

        for attribute in flat_variant.attributes:
            if attribute in foreign_keys or attribute in primary_keys:
                continue

            if attribute.is_automatically_determined_column():
                continue

            builder.add_attribute(attribute.as_option())

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

                builder.add_attribute(
                    AttributeMetadata(
                        original_name=attribute.original_name,
                        name=attribute.name,
                        data_type=attribute.raw_data_type(),
                        optional=True,
                    )
                )

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
            raise RuntimeError(
                f"Could not find the builder for the struct {struct.name}."
            )

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
        "use super::*;",
    ]

    for import_statement in imports:
        document.write(f"{import_statement}\n")

    document.write("\n")

    for struct in tqdm(
        new_struct_metadatas,
        desc="Writing new structs",
        unit="struct",
        leave=False,
    ):
        assert struct.is_new_variant()

        struct.write_to(document)

        # We implement the Tabular trait for the new struct. This trait
        # is used to display the new struct in a table format in the frontend.

        document.write(
            f"impl Tabular for {struct.name} {{\n"
            f"    const TABLE: Table = Table::{struct.capitalized_table_name()};\n"
            "}\n"
        )

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

        attributes = [
            creator_user_id_attribute,
        ] + struct.attributes

        if struct.is_updatable():
            updator_user_id_attribute: AttributeMetadata = (
                struct.get_updator_user_id_attribute()
            )
            attributes.append(updator_user_id_attribute)
        else:
            updator_user_id_attribute = None

        columns = []

        document.write(f'#[cfg(feature = "frontend")]\n' f"impl {struct.name} {{\n")

        document.write(
            f"    pub fn into_row(self, {creator_user_id_attribute.name}: {creator_user_id_attribute.format_data_type()}) -> Vec<gluesql::core::ast_builder::ExprNode<'static>> {{\n"
        )

        document.write("        vec![\n")
        for attribute in attributes:
            columns.append(attribute.name)

            if attribute in (
                creator_user_id_attribute,
                updator_user_id_attribute,
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
        "use uuid::Uuid;",
        "use super::*;",
    ]

    for import_statement in imports:
        document.write(f"{import_statement}\n")

    document.write("\n")

    for struct in tqdm(
        update_struct_metadatas,
        desc="Writing update structs",
        unit="struct",
        leave=False,
    ):
        assert struct.is_update_variant()

        struct.write_to(document)

        # We implement the Tabular trait for the new struct. This trait
        # is used to display the new struct in a table format in the frontend.

        document.write(
            f"impl Tabular for {struct.name} {{\n"
            f"    const TABLE: Table = Table::{struct.capitalized_table_name()};\n"
            "}\n"
        )

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

    document.write("\n".join(imports) + "\n")

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

            assert not creator_user_id_attribute.has_struct_data_type()

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

        # First, we write the intermediate struct that is used to update the row in the database.
        document.write(
            f"/// Intermediate representation of the update variant {struct.name}.\n"
            "#[derive(Identifiable, AsChangeset)]\n"
            f"#[diesel(table_name = {struct.table_name})]\n"
            "#[diesel(treat_none_as_null = true)]\n"
            f"#[diesel(primary_key({struct.get_formatted_primary_keys(include_prefix=False, include_parenthesis=False)}))]\n"
            f"pub(super) struct {intermediate_struct_name} {{\n"
        )

        all_attributes: List[AttributeMetadata] = struct.attributes

        if struct.table_name != "users":
            all_attributes = [updator_user_id_attribute] + all_attributes

        for attribute in all_attributes:
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
            "        self.to_intermediate(user_id)\n"
            "            .save_changes(connection)\n"
            "    }\n"
            "}\n\n"
        )

    document.flush()
    document.close()


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
    check_parent_circularity_trigger(tables_metadata)

    generate_view_schema(tables_metadata)
    print("Generated view schema.")
    check_schema_completion()
    print("Checked schema completion.")
    generate_view_structs()
    print("Generated view structs.")

    table_structs: List[StructMetadata] = extract_structs("src/models.rs")
    view_structs: List[StructMetadata] = extract_structs("src/views/views.rs")

    print(
        f"Extracted {len(table_structs)} tables and {len(view_structs)} views from the backend."
    )

    filter_structs: List[StructMetadata] = create_filter_structs(
        table_structs + view_structs
    )
    print(f"Generated {len(filter_structs)} filter structs.")

    write_backend_structs("src/models.rs", "tables", table_structs, tables_metadata)
    write_backend_structs("src/views/views.rs", "views", view_structs, tables_metadata)
    print(
        f"Generated {len(table_structs)} tables and {len(view_structs)} views implementations for backend."
    )

    write_web_common_structs(table_structs, "tables", tables_metadata)
    write_web_common_structs(view_structs, "views", tables_metadata)
    print("Generated web common structs.")

    nested_structs: List[StructMetadata] = generate_nested_structs(
        "src/nested_models.rs", table_structs + view_structs
    )
    print(f"Generated {len(nested_structs)} nested structs for backend.")

    new_model_structs = derive_new_models(table_structs)
    print(f"Derived {len(new_model_structs)} structs for the New versions")

    update_model_structs = derive_update_models(table_structs)
    print(f"Derived {len(update_model_structs)} structs for the Update versions")

    tables: List[TableStructMetadata] = write_webcommons_table_names_enumeration(
        table_structs + view_structs + nested_structs,
        new_model_structs,
        update_model_structs,
    )
    ensure_updatable_tables_have_roles_tables(tables, tables_metadata)
    ensure_tables_have_creation_notification_trigger(tables, tables_metadata)
    print("Generated table names enumeration for web_common.")

    write_diesel_table_names_enumeration(tables, tables_metadata)
    print("Generated table names enumeration for diesel.")

    write_web_common_nested_structs("nested_models.rs", nested_structs)
    print("Generated nested structs for web_common.")

    write_web_common_search_trait_implementations(
        nested_structs + table_structs + view_structs,
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

    write_frontend_pages(
        builder_structs,
    )
    print("Generated frontend pages.")

    write_frontend_router_page(
        builder_structs,
    )
    print("Generated frontend router page.")
