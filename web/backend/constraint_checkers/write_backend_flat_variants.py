"""This module contains the functions to write the `From` implementations for the structs in the `src/models.rs` struct_document."""

import os
from typing import List
from tqdm.auto import tqdm
from constraint_checkers.struct_metadata import (
    StructMetadata,
    AttributeMetadata,
    MethodDefinition,
)
from constraint_checkers.indices import PGIndices
from constraint_checkers.regroup_tables import SUPPORT_TABLE_NAMES


def write_backend_flat_variants(
    path: str,
    table_type: str,
    flat_variants: List[StructMetadata],
):
    """Write the `From` implementations for the structs in the `src/models.rs` struct_document."""

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
        "use crate::sql_function_bindings::*;",
        "use diesel::Selectable;",
        "use serde::Deserialize;",
        "use serde::Serialize;",
        "use diesel::r2d2::ConnectionManager;",
        "use diesel::r2d2::PooledConnection;",
        "use diesel::prelude::*;",
        "use web_common::database::filter_structs::*;",
        "use uuid::Uuid;",
        "use chrono::NaiveDateTime;",
    ]

    if table_type == "views":
        imports.append("use crate::views::schema::*;")

    path_directory = path.split(".")[0]
    os.makedirs(path_directory, exist_ok=True)

    main_document = open(path, "w", encoding="utf8")

    warning_header = (
        "//! This file is generated automatically and should not be modified.\n"
        "//!\n"
        "//! Any edits you may apply to this document will be overwritten next time the\n"
        "//! backend is generated. Refrain from making any changes to this file.\n\n"
        "//! If you need to make changes to the backend, please modify the `generate_models`\n"
        "//! document in the `migrations` folder.\n\n"
    )

    # Preliminarly, we write out a few docstrings explaining that
    # the file is generated and should not be modified.
    main_document.write(warning_header)

    # Then, we write the structs.

    connection_argument_and_description = (
        AttributeMetadata(
            original_name="connection",
            name="connection",
            data_type="PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>",
            optional=False,
            reference=True,
            mutable=True,
        ),
        "The connection to the database.",
    )

    limit_and_offset_arguments_and_description = [
        (
            AttributeMetadata(
                original_name="limit",
                name="limit",
                data_type="i64",
                optional=True,
                reference=False,
            ),
            "The maximum number of results to return.",
        ),
        (
            AttributeMetadata(
                original_name="offset",
                name="offset",
                data_type="i64",
                optional=True,
                reference=False,
            ),
            "The number of results to skip.",
        ),
    ]

    for struct in tqdm(
        flat_variants,
        desc=f"Writing {table_type} to backend",
        unit="struct",
        leave=False,
    ):

        struct_document = open(f"{path_directory}/{struct.table_name}.rs", "w", encoding="utf8")
        struct_document.write(warning_header)
        # Then, we write the import statements.
        struct_document.write("\n".join(imports) + "\n\n")

        main_document.write(
            f"mod {struct.table_name};\n"
            f"pub use {struct.table_name}::*;\n"
        )

        primary_keys = struct.get_primary_keys()

        # First of all, we write out the struct.
        struct.write_to(struct_document, diesel=table_type)

        struct_document.write(
            impl_from_line.format(struct_name=struct.name, table_type=table_type)
        )
        struct_document.write(f"    fn from(item: {struct.name}) -> Self {{\n        Self {{\n")
        for attribute in struct.attributes:
            struct_document.write(f"            {attribute.name}: item.{attribute.name},\n")
        struct_document.write("        }\n    }\n}\n\n")

        struct_document.write(reverse_from.format(struct_name=struct.name, table_type=table_type))
        struct_document.write(
            f"    fn from(item: web_common::database::{table_type}::{struct.name}) -> Self {{\n"
            "        Self {\n"
        )
        for attribute in struct.attributes:
            struct_document.write(f"            {attribute.name}: item.{attribute.name},\n")
        struct_document.write("        }\n    }\n}\n\n")

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

        struct_document.write(f"impl {struct.name} {{\n")

        if (
            struct.has_can_update_function()
            and not struct.is_junktion_table()
            and struct.table_name not in SUPPORT_TABLE_NAMES
        ):
            assert struct.is_updatable(), (
                f"The {struct.name} struct has the can_update function, but it is not updatable. "
                "Please remove the can_update function from the struct or set the updatable attribute to True."
            )

        # For all the tables that have an associated roles table, we implement methods
        # to determine whetheer a provided user id is a Viewer (role_id >= 3), Editor (role_id >= 2), or Admin (role_id == 1),
        # or the user is the creator of the struct (i.e. the created_by field is equal to the user_id).
        for operation, operation_able, can_x_function_name in (
            ("view", "viewable", struct.get_can_view_function_name()),
            ("update", "updatable", struct.get_can_update_function_name()),
            ("admin", "administrable", struct.get_can_admin_function_name()),
        ):
            if struct.is_immutable() and operation in ["update", "admin"]:
                continue

            if operation == "view":
                requires_author = struct.may_be_hidden()
            else:
                requires_author = not struct.is_immutable()

            author_user_id = AttributeMetadata(
                original_name="author_user_id",
                name="author_user_id",
                data_type="i32",
                optional=operation == "view",
            )

            if operation == "view" and struct.may_be_hidden():
                assert struct.has_can_view_function(), (
                    f"The {struct.name} struct has the may_be_hidden attribute set to True, but it does not have the can_view function. "
                    "Please add the can_view function to the struct."
                )

            # We now create the more specific methods that check whether the user has a role
            # with a role_id less than or equal to the provided role_id. We start with the Viewer role.
            method: MethodDefinition = struct.add_backend_method(
                MethodDefinition(
                    name=f"can_{operation}",
                    summary=f"Check whether the user can {operation} the struct.",
                )
            )
            method.include_self_ref()
            if struct.table_metadata.has_postgres_function(can_x_function_name):
                method.add_argument(
                    author_user_id, description="The ID of the user to check."
                )
                method.add_argument(*connection_argument_and_description)

            method.set_return_type(
                AttributeMetadata(
                    original_name="_",
                    name="_",
                    data_type="Result<bool, web_common::api::ApiError>",
                    optional=False,
                )
            )

            method.write_header_to(struct_document)

            rust_boolean = "true" if operation == "view" else "false"

            if struct.table_metadata.has_postgres_function(can_x_function_name):
                struct_document.write(
                    " {\n"
                    f"        Self::can_{operation}_by_id(\n"
                    f"            {struct.get_formatted_primary_keys(include_prefix=True)},\n"
                    f"            {author_user_id.name},\n"
                    "            connection,\n"
                    "        )\n"
                    "    }\n"
                )
            else:
                struct_document.write(f"{{\n        Ok({rust_boolean})\n}}\n")

            method: MethodDefinition = struct.add_backend_method(
                MethodDefinition(
                    name=f"can_{operation}_by_id",
                    summary=f"Check whether the user can {operation} the struct associated to the provided ids.",
                )
            )

            if struct.table_metadata.has_postgres_function(can_x_function_name):
                method.add_argument(
                    AttributeMetadata(
                        original_name=struct.get_formatted_primary_keys(
                            include_prefix=False
                        ),
                        name=struct.get_formatted_primary_keys(include_prefix=False),
                        data_type=struct.get_formatted_primary_key_data_types(),
                        optional=False,
                        reference=False,
                        mutable=False,
                    ),
                    description="The primary key(s) of the struct to check.",
                )
                method.add_argument(
                    author_user_id, description="The ID of the user to check."
                )

                method.add_argument(*connection_argument_and_description)

            method.set_return_type(
                AttributeMetadata(
                    original_name="_",
                    name="_",
                    data_type="Result<bool, web_common::api::ApiError>",
                    optional=False,
                )
            )

            method.write_header_to(struct_document)

            if struct.table_metadata.has_postgres_function(can_x_function_name):
                struct_document.write(
                    "{\n"
                    f"       diesel::select({can_x_function_name}({author_user_id.name}, {struct.get_formatted_primary_keys(include_prefix=False, include_parenthesis=False)}))\n"
                    "            .get_result(connection).map_err(web_common::api::ApiError::from)\n"
                    "}\n"
                )
            else:
                struct_document.write(f"{{\n        Ok({rust_boolean})\n}}\n")

            sorted_variants = [False, True]

            for sorted_variant in sorted_variants:

                sorted_desinence = " sorted" if sorted_variant else ""
                sorted_desinence_underscored = sorted_desinence.replace(" ", "_")

                method = struct.add_backend_method(
                    MethodDefinition(
                        name=f"all_{operation_able}{sorted_desinence_underscored}",
                        summary=f"Get all of the{sorted_desinence} {operation_able} structs from the database.",
                    )
                )

                if struct.has_filter_variant():
                    method.add_argument(
                        AttributeMetadata(
                            original_name="filter",
                            name="filter",
                            data_type=struct.get_filter_variant(),
                            optional=True,
                            reference=True,
                        ),
                        description="The optional filter to apply to the query.",
                    )

                if requires_author:
                    method.add_argument(
                        author_user_id,
                        description="The ID of the user who is performing the search.",
                    )

                for arg_and_desc in limit_and_offset_arguments_and_description:
                    method.add_argument(*arg_and_desc)

                method.add_argument(*connection_argument_and_description)

                method.set_return_type(
                    AttributeMetadata(
                        original_name="_",
                        name="_",
                        data_type="Result<Vec<Self>, web_common::api::ApiError>",
                        optional=False,
                    )
                )

                method.write_header_to(struct_document)

                struct_document.write("{\n")
                if table_type == "tables":
                    struct_document.write(f"        use crate::schema::{struct.table_name};\n")
                else:
                    struct_document.write(
                        f"        use crate::views::schema::{struct.table_name};\n"
                    )
                # If the limit is None, we do not apply any limit to the query.

                if struct.has_filter_variant():
                    struct_document.write(
                        f"        let mut query = {struct.table_name}::dsl::{struct.table_name}\n"
                    )
                else:
                    struct_document.write(
                        f"        {struct.table_name}::dsl::{struct.table_name}\n"
                    )

                if struct.has_filter_variant():
                    # We need to filter the query by the filter struct,
                    # but these filters are composed by several optional fields
                    # and therefore we need to box the query and apply the filters
                    # only if they are not None.
                    struct_document.write("            .into_boxed();\n")

                    filter_struct = struct.get_filter_variant()

                    for filter_attribute in filter_struct.attributes:
                        struct_document.write(
                            f"        if let Some({filter_attribute.name}) = filter.and_then(|f| f.{filter_attribute.name}) {{\n"
                            f"            query = query.filter({filter_struct.table_name}::dsl::{filter_attribute.name}.eq({filter_attribute.name}));\n"
                            "        }\n"
                        )

                    struct_document.write("        query\n")

                if struct.table_metadata.has_postgres_function(can_x_function_name):
                    diesel_primary_keys = ", ".join(
                        f"{struct.table_name}::dsl::{primary_key.name}"
                        for primary_key in struct.get_primary_keys()
                    )
                    struct_document.write(
                        f"            .filter({can_x_function_name}({author_user_id.name}, {diesel_primary_keys}))\n"
                    )

                if sorted_variant:
                    if struct.has_updated_at():
                        struct_document.write(
                            f"            .order_by({struct.table_name}::dsl::updated_at.desc())\n"
                        )
                    elif struct.has_created_at():
                        struct_document.write(
                            f"            .order_by({struct.table_name}::dsl::created_at.desc())\n"
                        )

                struct_document.write(
                    "            .offset(offset.unwrap_or(0))\n"
                    "            .limit(limit.unwrap_or(10))\n"
                    "            .load::<Self>(connection).map_err(web_common::api::ApiError::from)\n"
                    "    }\n"
                )

            if operation == "view":

                get_method = struct.add_backend_method(
                    MethodDefinition(
                        name="get",
                        summary="Get the struct from the database by its ID.",
                    )
                )

                get_method.add_argument(
                    AttributeMetadata(
                        original_name=struct.get_formatted_primary_keys(
                            include_prefix=False
                        ),
                        name=struct.get_formatted_primary_keys(include_prefix=False),
                        data_type=struct.get_formatted_primary_key_data_types(),
                        optional=False,
                        reference=False,
                        mutable=False,
                    ),
                    description="The primary key(s) of the struct to get.",
                )

                if requires_author:
                    get_method.add_argument(
                        author_user_id,
                        description="The ID of the user who is performing the search.",
                    )

                get_method.add_argument(*connection_argument_and_description)

                get_method.set_return_type(
                    AttributeMetadata(
                        original_name="_",
                        name="_",
                        data_type="Result<Self, web_common::api::ApiError>",
                        optional=False,
                    )
                )

                get_method.write_header_to(struct_document)

                struct_document.write("{\n")
                if struct.may_be_hidden():
                    struct_document.write(
                        f"        if !Self::can_view_by_id({struct.get_formatted_primary_keys(include_prefix=False)}, {author_user_id.name}, connection)? {{\n"
                        "            return Err(web_common::api::ApiError::Unauthorized);\n"
                        "        }\n"
                    )
                if table_type == "tables":
                    struct_document.write(f"        use crate::schema::{struct.table_name};\n")
                else:
                    struct_document.write(
                        f"        use crate::views::schema::{struct.table_name};\n"
                    )
                struct_document.write(f"        {struct.table_name}::dsl::{struct.table_name}\n")
                for primary_key in primary_keys:
                    struct_document.write(
                        f"            .filter({struct.table_name}::dsl::{primary_key.name}.eq({primary_key.name}))\n"
                    )
                struct_document.write(
                    "            .first::<Self>(connection).map_err(web_common::api::ApiError::from)\n    }\n"
                )

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
                                [
                                    column.name
                                    for column in reference_unique_columns[:-1]
                                ]
                            )
                            + f" and {reference_unique_columns[-1].name}"
                        )

                    from_method_name = f"from_{'_and_'.join([column.name for column in reference_unique_columns])}"

                    if table_type == "views":
                        raise NotImplementedError(
                            f"The `{from_method_name}` method is not implemented for views."
                        )

                    from_method = struct.add_backend_method(
                        MethodDefinition(
                            name=from_method_name,
                            summary=f"Get the struct from the database by its {human_readable_unique_columns}.",
                        )
                    )

                    for unique_column in reference_unique_columns:
                        from_method.add_argument(
                            unique_column,
                            description=f"The {unique_column.name} of the struct to get.",
                        )

                    if requires_author:
                        from_method.add_argument(
                            author_user_id,
                            description="The ID of the user who is performing the search.",
                        )

                    from_method.add_argument(*connection_argument_and_description)

                    from_method.set_return_type(
                        AttributeMetadata(
                            original_name="_",
                            name="_",
                            data_type="Result<Self, web_common::api::ApiError>",
                            optional=False,
                        )
                    )

                    from_method.write_header_to(struct_document)

                    struct_document.write(
                        "{\n"
                        f"        use crate::schema::{struct.table_name};\n"
                        f"        let flat_variant = {struct.table_name}::dsl::{struct.table_name}\n"
                    )
                    for unique_column in reference_unique_columns:
                        struct_document.write(
                            f"            .filter({struct.table_name}::dsl::{unique_column.name}.eq({unique_column.name}))\n"
                        )
                    struct_document.write("            .first::<Self>(connection)?;\n")
                    if struct.may_be_hidden():
                        struct_document.write(
                            f"        if !flat_variant.can_view({author_user_id.name}, connection)? {{\n"
                            "            return Err(web_common::api::ApiError::Unauthorized);\n"
                            "        }\n"
                        )
                    struct_document.write("        Ok(flat_variant)\n    }\n")

            # If this table implements the `pg_trgm` index, we also
            # provide the `search` method to search for the struct
            # by a given string. The method also receives a limit
            # parameter to limit the number of results.
            if struct.is_searchable():
                for similarity_method_name in PGIndices.SIMILARITY_METHODS:

                    similarity_method = struct.add_backend_method(
                        MethodDefinition(
                            name=f"{similarity_method_name}_search_{operation_able}",
                            summary=f"Search for the {operation_able} structs by a given string by Postgres's `{similarity_method_name}`.",
                        )
                    )

                    if struct.has_filter_variant():
                        similarity_method.add_argument(
                            AttributeMetadata(
                                original_name="filter",
                                name="filter",
                                data_type=struct.get_filter_variant(),
                                optional=True,
                                reference=True,
                            ),
                            description="The optional filter to apply to the query.",
                        )

                    if requires_author:
                        similarity_method.add_argument(
                            author_user_id,
                            description="The ID of the user who is performing the search.",
                        )

                    similarity_method.add_argument(
                        AttributeMetadata(
                            original_name="query",
                            name="query",
                            data_type="str",
                            optional=False,
                            reference=True,
                        ),
                        description="The string to search for.",
                    )

                    for arg_and_desc in limit_and_offset_arguments_and_description:
                        similarity_method.add_argument(*arg_and_desc)

                    similarity_method.add_argument(*connection_argument_and_description)

                    similarity_method.set_return_type(
                        AttributeMetadata(
                            original_name="_",
                            name="_",
                            data_type="Result<Vec<Self>, web_common::api::ApiError>",
                            optional=False,
                        )
                    )

                    similarity_method.write_header_to(struct_document)

                    struct_document.write(
                        "{\n"
                        "        // If the query string is empty, we run an all query with the\n"
                        "        // limit parameter provided instead of a more complex similarity\n"
                        "        // search.\n"
                        "        if query.is_empty() {\n"
                        f"            return Self::all_{operation_able}("
                    )
                    if struct.has_filter_variant():
                        struct_document.write("filter, ")

                    if requires_author:
                        struct_document.write(f"{author_user_id.name}, ")

                    struct_document.write("limit, offset, connection);\n        }\n")

                    # We start a query using the diesel query builder.
                    if table_type == "tables":
                        struct_document.write(f"        use crate::schema::{struct.table_name};\n")
                    else:
                        raise NotImplementedError(
                            "The search method is not implemented for views."
                        )

                    optional_filter_attributes = []
                    non_optional_filter_attributes = []
                    filter_method_content = ""

                    # If the current struct has derived search indices, we will need to
                    # execute joins. As such, we also need to add the select method to the
                    # query builder.

                    struct_document.write(struct.format_diesel_search_aliases())

                    if struct.has_derived_search_indices():

                        filter_method_content += (
                            f"            .select({struct.name}::as_select())\n"
                            f"            {struct.format_diesel_search_join()}\n"
                        )

                    if struct.has_filter_variant():
                        optional_filter_attributes = [
                            filter_attribute
                            for filter_attribute in struct.get_filter_variant().attributes
                            if struct.get_attribute_by_name(
                                filter_attribute.name
                            ).optional
                        ]
                        non_optional_filter_attributes = [
                            filter_attribute
                            for filter_attribute in struct.get_filter_variant().attributes
                            if not struct.get_attribute_by_name(
                                filter_attribute.name
                            ).optional
                        ]

                    # If more than one of the non-optional filter is Some in the filter struct,
                    # we raise an unimplemented! macro panic, as I have no idea how to cleanly
                    # handle this case in Diesel and for a while it should cover all the cases.
                    if len(non_optional_filter_attributes) > 1:
                        struct_document.write(
                            f" if filter.map(|f| {'&&'.join(f'f.{filter_attribute.name}.is_some()' for filter_attribute in non_optional_filter_attributes)}).unwrap_or(false) {{\n"
                            "       unimplemented!();\n"
                            " }\n"
                        )

                    for filter_attribute in optional_filter_attributes:
                        filter_method_content += f"            .filter({struct.table_name}::dsl::{filter_attribute.name}.eq(filter.and_then(|f| f.{filter_attribute.name})))\n"

                    if struct.table_metadata.has_postgres_function(can_x_function_name):
                        primary_keys = ", ".join(
                            f"{struct.table_name}::dsl::{primary_key.name}"
                            for primary_key in struct.get_primary_keys()
                        )
                        filter_method_content += f"            .filter({can_x_function_name}({author_user_id.name}, {primary_keys}))\n"

                    filter_method_content += f"            {struct.format_diesel_search_filter('query', similarity_method_name)}\n"

                    filter_method_content += f"            {struct.format_diesel_search_order('query', similarity_method_name)}\n"

                    filter_method_content += (
                        "            .limit(limit.unwrap_or(10))\n"
                        "            .offset(offset.unwrap_or(0))\n"
                        "            .load::<Self>(connection).map_err(web_common::api::ApiError::from)"
                    )

                    for filter_attribute in non_optional_filter_attributes:
                        struct_document.write(
                            f"if let Some({filter_attribute.name}) = filter.and_then(|f| f.{filter_attribute.name}) {{\n"
                            f"        return {struct.table_name}::dsl::{struct.table_name}\n"
                            f"            .filter({struct.table_name}::dsl::{filter_attribute.name}.eq({filter_attribute.name}))\n"
                            f"{filter_method_content};\n}}\n"
                        )

                    struct_document.write(
                        f"        {struct.table_name}::dsl::{struct.table_name}\n"
                        f"{filter_method_content}\n}}\n"
                    )

        if struct.is_insertable():
            delete_method = struct.add_backend_method(
                MethodDefinition(
                    name="delete",
                    summary="Delete the struct from the database.",
                )
            )

            delete_method.include_self_ref()
            delete_method.add_argument(
                author_user_id,
                description="The ID of the user who is deleting the struct.",
            )
            delete_method.add_argument(*connection_argument_and_description)

            delete_method.set_return_type(
                AttributeMetadata(
                    original_name="_",
                    name="_",
                    data_type="Result<usize, web_common::api::ApiError>",
                    optional=False,
                )
            )

            delete_method.write_header_to(struct_document)

            struct_document.write(
                "{\n"
                f"        Self::delete_by_id({struct.get_formatted_primary_keys(include_prefix=True)}, {author_user_id.name}, connection)\n"
                "}\n"
            )

            delete_by_id_method = struct.add_backend_method(
                MethodDefinition(
                    name="delete_by_id",
                    summary="Delete the struct from the database by its ID.",
                )
            )

            delete_by_id_method.add_argument(
                AttributeMetadata(
                    original_name=struct.get_formatted_primary_keys(
                        include_prefix=False
                    ),
                    name=struct.get_formatted_primary_keys(include_prefix=False),
                    data_type=struct.get_formatted_primary_key_data_types(),
                    optional=False,
                    reference=False,
                    mutable=False,
                ),
                description="The primary key(s) of the struct to delete.",
            )

            delete_by_id_method.add_argument(
                author_user_id,
                description="The ID of the user who is deleting the struct.",
            )

            delete_by_id_method.add_argument(*connection_argument_and_description)
            delete_by_id_method.set_return_type(delete_method.get_return_type())

            delete_by_id_method.write_header_to(struct_document)

            struct_document.write(
                "{\n"
                f"        if !Self::can_admin_by_id({struct.get_formatted_primary_keys(include_prefix=False)}, author_user_id, connection)? {{\n"
                "            return Err(web_common::api::ApiError::Unauthorized);\n"
                "        }\n"
                f"        diesel::delete({struct.table_name}::dsl::{struct.table_name}\n"
            )
            for primary_key in struct.get_primary_keys():
                struct_document.write(
                    f"            .filter({struct.table_name}::dsl::{primary_key.name}.eq({primary_key.name}))\n"
                )
            struct_document.write(
                "        ).execute(connection).map_err(web_common::api::ApiError::from)\n    }\n"
            )

        # Finally, we cluse the struct implementation.
        struct_document.write("}\n")
