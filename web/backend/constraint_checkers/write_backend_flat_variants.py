"""This module contains the functions to write the `From` implementations for the structs in the `src/models.rs` file."""

from typing import List
from tqdm.auto import tqdm
from constraint_checkers.struct_metadata import StructMetadata, AttributeMetadata
from constraint_checkers.indices import PGIndex, PGIndices


def write_backend_flat_variants(
    path: str,
    table_type: str,
    struct_metadatas: List[StructMetadata],
):
    """Write the `From` implementations for the structs in the `src/models.rs` file."""

    if len(struct_metadatas) == 0:
        return

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
        file.write("#![allow(unused)]\n#![allow(clippy::all)]\n\n")

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
                f"    fn from(item: {struct.name}) -> Self {{\n        Self {{\n"
            )
            for attribute in struct.attributes:
                file.write(f"            {attribute.name}: item.{attribute.name},\n")
            file.write("        }\n    }\n}\n\n")

            file.write(
                reverse_from.format(struct_name=struct.name, table_type=table_type)
            )
            file.write(
                f"    fn from(item: web_common::database::{table_type}::{struct.name}) -> Self {{\n"
                "        Self {\n"
            )
            for attribute in struct.attributes:
                file.write(f"            {attribute.name}: item.{attribute.name},\n")
            file.write("        }\n    }\n}\n\n")

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
                assert (
                    len(primary_keys) == 1
                ), "The has_role_by_id method is only implemented for tables with a single primary key."
                primary_key = primary_keys[0]
                file.write(
                    f"            .filter({struct.table_name}::dsl::{primary_key.name}.eq({primary_key.name}))\n"
                    f"           .filter({struct.table_name}::dsl::created_by.eq(author_user_id))\n"
                    "            .or_filter(\n"
                    f"               {struct.table_name}::dsl::{primary_key.name}.eq({primary_key.name})\n"
                    f"                   .and({struct.table_name}::dsl::{primary_key.name}.eq_any(\n"
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
                        f"                       {struct.table_name}::dsl::{primary_key.name}.eq({primary_key.name})\n"
                        f"                           .and({struct.table_name}::dsl::{primary_key.name}.eq_any(\n"
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

                author_user_id = AttributeMetadata(
                    original_name="author_user_id",
                    name="author_user_id", data_type="i32", optional=not editable_variant
                )

                if editable_variant:
                    file.write(
                        "    /// Get all of the editable structs from the database.\n"
                    )
                else:
                    file.write(
                        "    /// Get all of the viewable structs from the database.\n"
                    )
                file.write(
                    "    ///\n    /// # Arguments\n"
                )

                if struct.has_filter_variant():
                    file.write(
                        "    /// * `filter` - The optional filter to apply to the query.\n"
                    )

                file.write(
                    f"    /// * `{author_user_id.name}` - The ID of the user who is performing the search.\n"
                    "    /// * `limit` - The maximum number of structs to retrieve. By default, this is 10.\n"
                    "    /// * `offset` - The number of structs to skip. By default, this is 0.\n"
                    "    /// * `connection` - The connection to the database.\n"
                    "    ///\n"
                )
                if editable_variant:
                    file.write(
                        "    pub fn all_editables(\n"
                    )
                else:
                    file.write("    pub fn all_viewables(\n")

                if struct.has_filter_variant():
                    filter_struct = struct.get_filter_variant()
                    file.write(
                        f"        filter: Option<&web_common::database::{filter_struct.name}>,\n"
                    )
                file.write(
                    f"        {author_user_id.name}: {author_user_id.format_data_type()},\n"
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
                    primary_keys = struct.get_primary_keys()
                    assert (
                        len(primary_keys) == 1
                    ), "The all_editables method is only implemented for tables with a single primary key."
                    primary_key = primary_keys[0]

                    file.write(
                        f"           .filter({struct.table_name}::dsl::created_by.eq({author_user_id.name}))\n"
                        "            .or_filter(\n"
                        f"               {struct.table_name}::dsl::{primary_key.name}.eq_any(\n"
                        f"                   {struct.table_name}_users_roles::table\n"
                        f"                       .select({struct.table_name}_users_roles::dsl::table_id)\n"
                        f"                       .filter({struct.table_name}_users_roles::dsl::user_id.eq({author_user_id.name})\n"
                        f"                       .and({struct.table_name}_users_roles::dsl::role_id.le(2)),\n"
                        "               )),\n"
                        "            )\n"
                    )
                    if struct.table_name != "teams":
                        file.write(
                            "                .or_filter(\n"
                            f"                   {struct.table_name}::dsl::{primary_key.name}.eq_any(\n"
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
                            f"        if let Some({filter_attribute.name}) = filter.and_then(|f| f.{filter_attribute.name}) {{\n"
                            f"            query = query.filter({filter_struct.table_name}::dsl::{filter_attribute.name}.eq({filter_attribute.name}));\n"
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

            if struct.has_updated_at() or struct.has_created_at():
                file.write(
                    "    /// Get all of the structs from the database ordered by update time.\n"
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
                    "    pub fn all_by_update(\n"
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

                if struct.has_updated_at():
                    file.write(
                        f"            .order_by({struct.table_name}::dsl::updated_at.desc())\n"
                    )
                elif struct.has_created_at():
                    file.write(
                        f"            .order_by({struct.table_name}::dsl::created_at.desc())\n"
                    )
                else:
                    assert False, "The all_by_update method is only implemented for tables with an updated_at column."
                file.write(
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
                file.write("        ).execute(connection)\n    }\n")

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
            file.write("            .first::<Self>(connection)\n    }\n")

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
                    file.write("            .first::<Self>(connection)\n    }\n")

            # If this table implements the `pg_trgm` index, we also
            # provide the `search` method to search for the struct
            # by a given string. The method also receives a limit
            # parameter to limit the number of results.
            if struct.is_searchable():
                similarity_index: PGIndex = struct.pg_indices.get_table(
                    struct.table_name
                )

                for (
                    method_name,
                    similarity_operator,
                    distance_operator,
                ) in PGIndices.SIMILARITY_METHODS:

                    for editable_filter in editable_variants:

                        author_user_id = AttributeMetadata(
                            original_name="author_user_id",
                            name="author_user_id",
                            data_type="i32",
                            optional=not editable_filter
                        )

                        if editable_filter:
                            file.write(
                                f"    /// Search for the editable structs by a given string by Postgres's `{method_name}`.\n"
                            )
                            assert struct.has_can_edit_function(), "The struct must have a can_edit function to implement the search editables method."
                            where_clause = f"{struct.get_can_edit_function_name()}($3, {struct.get_formatted_primary_keys(include_prefix=True, prefix=struct.table_name)})"
                        else:
                            file.write(
                                f"    /// Search for the viewable structs by a given string by Postgres's `{method_name}`.\n"
                            )
                            if struct.is_insertable():
                                assert struct.has_can_view_function(), f"The struct {struct.name} must have a can_view function to implement the search viewables method."
                                where_clause = f"{struct.get_can_view_function_name()}($3, {struct.get_formatted_primary_keys(include_prefix=True, prefix=struct.table_name)})"
                            else:
                                where_clause = None

                        if where_clause is not None:
                            if similarity_index.is_gin():
                                where_clause = f"AND {where_clause} "
                            elif similarity_index.is_gist():
                                where_clause = f'            "WHERE {where_clause} ",\n'
                            else:
                                assert False, "The similarity index must be either GIN or GIST."
                        else:
                            where_clause = ""

                        file.write(
                            "    ///\n    /// # Arguments\n"
                            f"    /// * `{author_user_id.name}` - The ID of the user who is performing the search.\n"
                            "    /// * `query` - The string to search for.\n"
                            "    /// * `limit` - The maximum number of results, by default `10`.\n"
                            "    /// * `connection` - The connection to the database.\n"
                            "    ///\n"
                        )
                        if editable_filter:
                            file.write(f"    pub fn {method_name}_search_editables(\n")
                        else:
                            file.write(f"    pub fn {method_name}_search_viewables(\n")

                        file.write(
                            f"       {author_user_id.name}: {author_user_id.format_data_type()},\n"
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
                                    f"            return Self::all_editables({author_user_id.name}, None, Some(limit as i64), None, connection);\n"
                                )
                            else:
                                file.write(
                                    f"            return Self::all_editables({author_user_id.name}, Some(limit as i64), None, connection);\n"
                                )
                        else:
                            if struct.has_filter_variant():
                                file.write(
                                    f"            return Self::all_viewables({author_user_id.name}, None, Some(limit as i64), None, connection);\n"
                                )
                            else:
                                file.write(
                                    f"            return Self::all_viewables({author_user_id.name}, Some(limit as i64), None, connection);\n"
                                )
                        file.write("        }\n")

                        joined_field_names = ", ".join(
                            attribute.name for attribute in struct.attributes
                        )

                        if table_type == "views":
                            file.write(
                                "        let similarity_query = concat!(\n"
                                '            "WITH selected_ids AS (",\n'
                                f'            "SELECT {similarity_index.table_name}.{primary_key.name} AS id FROM {similarity_index.table_name} ",\n'
                            )
                            if similarity_index.is_gin():
                                file.write(
                                    f'            "WHERE $1 {similarity_operator} {similarity_index.arguments} {where_clause}",\n'
                                    f'            "ORDER BY {method_name}($1, {similarity_index.arguments}) DESC LIMIT $2",\n'
                                )
                            elif similarity_index.is_gist():
                                file.write(
                                    f'{where_clause}            "ORDER BY $1 {distance_operator} {similarity_index.arguments} LIMIT $2",\n'
                                )
                            else:
                                raise RuntimeError(
                                    "The similarity index must be either GIN or GIST."
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
                                    f'            "WHERE $1 {similarity_operator} {similarity_index.arguments} {where_clause}",\n'
                                    f'            "ORDER BY {method_name}($1, {similarity_index.arguments}) DESC LIMIT $2",\n'
                                )
                            elif similarity_index.is_gist():
                                file.write(
                                    f'{where_clause}            "ORDER BY $1 {distance_operator} {similarity_index.arguments} LIMIT $2;"\n'
                                )
                            elif similarity_index.is_btree():
                                # In the case of a btree, we simply search the prefix of the string.
                                file.write(
                                    f'            "WHERE {similarity_index.arguments} LIKE $1%{where_clause}",\n'
                                    '            "LIMIT $2;"\n')
                            else:
                                raise RuntimeError(
                                    "The similarity index must be either GIN or GIST."
                                )
                            file.write("        );\n")

                        file.write(
                            "        diesel::sql_query(similarity_query)\n"
                            "            .bind::<diesel::sql_types::Text, _>(query)\n"
                            "            .bind::<diesel::sql_types::Integer, _>(limit)\n"
                        )
                        file.write(
                            f"            .bind::<{author_user_id.format_data_type(diesel=True)}, _>({author_user_id.name})\n"
                        )
                        file.write("            .load(connection)\n}\n")

            # Finally, we cluse the struct implementation.
            file.write("}\n")
