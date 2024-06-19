"""Writes the table names enumeration to the web_common crate."""

import os
from typing import List, Dict, Optional, Tuple
from constraint_checkers.struct_metadata import (
    StructMetadata,
    MethodDefinition,
    AttributeMetadata,
)
from constraint_checkers.table_metadata import TableStructMetadata


def write_web_common_table_names_enumeration(
    struct_metadatas: List[StructMetadata],
    new_model_structs: List[StructMetadata],
    update_model_structs: List[StructMetadata],
) -> List[TableStructMetadata]:
    """Writes the table names enumeration to the web_common crate."""
    imports = [
        "use serde::Deserialize;",
        "use serde::Serialize;",
        "use crate::database::*;",
    ]

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

    table_enum_struct = StructMetadata(struct_name="Table", table_name=None)

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

    # First, we determine the methods that are common across all tables associated
    # richest structs, and we implement them for the Table enum.
    method_counts: Dict[MethodDefinition, Tuple[MethodDefinition, int]] = {}
    for table in tables:
        methods = table.get_richest_struct().webcommon_methods()
        assert len(methods) > 0, f"No methods found for table {table.name}."
        for method in methods:
            if method.is_private():
                continue
            if method.has_self_reference():
                continue
            if method.name.startswith("from_"):
                continue

            # For the time being, we skip the search methods that return a score.
            if "with_score" in method.name:
                continue
            if method not in method_counts:
                method_counts[method] = (0, method)

            (count, registered_method) = method_counts[method]

            if len(method.arguments) > len(registered_method.arguments):
                registered_method = method
            method_counts[method] = (method_counts[method][0] + 1, registered_method)

    # We implement the table dispatching for all methods that have at least 4 implementations.
    table_methods: List[MethodDefinition] = []
    for count, method in method_counts.values():
        if count < 3:
            print(f"Skipping method {method.name} with {count} implementations.")
            continue

        assert (
            not method.has_self_reference()
        ), f"Method {method.name} has a self reference."

        table_method = table_enum_struct.add_webcommon_method(
            MethodDefinition(name=method.name, summary=method.summary)
        )
        table_method.include_self_ref()
        table_method.is_async = method.is_async
        method_return_type = method.get_return_type()
        return_type = AttributeMetadata(
            original_name=method_return_type.name,
            name=method_return_type.name,
            data_type=method_return_type.data_type(route="web_common")
            .replace("Vec<Self>", "Vec<u8>")
            .replace("Self", "Vec<u8>"),
            optional=method_return_type.optional,
        )
        table_method.set_return_type(return_type)

        for generic in method.generics:
            table_method.add_generic(generic)

        for where_clause in method.where_clauses:
            table_method.add_where_clause(where_clause)

        if method.has_primary_key_as_argument():
            argument = method.get_primary_key_argument()
            table_method.add_argument(
                AttributeMetadata(
                    original_name="primary_key",
                    name="primary_key",
                    data_type="PrimaryKey",
                    optional=argument.optional,
                ),
                description=method.get_argument_description(argument),
            )

        for argument in method.arguments:
            # If the argument is a struct, we need to convert it to a Vec<u8>
            if argument.has_struct_data_type():
                table_method.add_argument(
                    AttributeMetadata(
                        original_name=argument.name,
                        name=(
                            "row" if argument.name == "self" else argument.name
                        ),
                        data_type="Vec<u8>",
                        optional=argument.optional,
                    ),
                    description=(
                        "Row to be processed"
                        if argument.name == "self"
                        else method.get_argument_description(argument)
                    ),
                )
            elif method.is_primary_key_argument(argument):
                continue
            else:
                table_method.add_argument(
                    argument,
                    description=method.get_argument_description(argument),
                )
        table_methods.append(table_method)

    assert len(table_methods) > 0, "No table methods found."

    # Next, we actually implement the trait for the Table enum.
    document.write('#[cfg(feature = "frontend")]\nimpl crate::database::Table {\n')

    for method in table_methods:
        method.write_header_to(document)
        document.write(" {\n        Ok(match self {\n")
        for table in tables:
            richest_variant: StructMetadata = table.get_richest_struct()
            struct_method: Optional[MethodDefinition] = (
                richest_variant.get_webcommon_method_by_name(method.name)
            )
            if struct_method is not None:
                document.write(
                    f"            crate::database::Table::{table.camel_cased()} => {{\n"
                )

                return_type = struct_method.get_return_type()
                if "Self" in return_type.data_type(route="web_common"):
                    document.write("let result = ")

                awaits = ".await" if struct_method.is_async else ""

                document.write(f"{table.richest_struct_name()}::{method.name}(\n")
                arguments = []
                for argument in method.arguments:
                    if argument.name == "self":
                        continue
                    original_argument = struct_method.get_argument_by_name(
                        argument.original_name
                    )
                    if (
                        original_argument is not None
                        and original_argument.has_struct_data_type()
                    ):
                        if original_argument.optional:
                            formatted_argument = f"{argument.name}.map(|{argument.name}| bincode::deserialize::<{original_argument.data_type(route='web_common')}>(&{argument.name})).transpose()?"
                        else:
                            formatted_argument = f"bincode::deserialize::<{original_argument.data_type(route='web_common')}>(&{argument.name})?"

                        if original_argument.reference:
                            formatted_argument = f"{formatted_argument}.as_ref()"

                        arguments.append(formatted_argument)
                    elif (
                        argument.name == "primary_key"
                        and struct_method.has_primary_key_as_argument()
                    ):
                        if argument.optional:
                            arguments.append("primary_key.map(|pk| pk.into())")
                        else:
                            arguments.append("primary_key.into()")
                    elif original_argument:
                        arguments.append(argument.name)
                document.write(",\n".join(arguments))
                document.write(f"){awaits}")
                if "Result" in return_type.format_data_type(
                    route="web_common"
                ):
                    document.write("?")
                if "Self" in return_type.data_type(
                    route="web_common"
                ):
                    document.write(
                        ";\n"
                    )
                    if return_type.optional or "Option" in return_type.data_type(route="web_common"):
                        document.write("result.map(|result| bincode::serialize(&result)).transpose()?")
                    else:
                        document.write("bincode::serialize(&result)?")

                document.write("            },\n")
            else:
                document.write(
                    f'            crate::database::Table::{table.camel_cased()} => unimplemented!("Method {method.name} not implemented for table {table.name}."),\n'
                )
        # We close the match
        document.write("        })\n")
        # And we close the method
        document.write("    }\n\n")

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

    document.write("        }\n    Ok(())}\n")

    document.write("}\n")

    document.flush()
    document.close()

    # Next, we implement for the frontend the search method for the Table enum, which receives a search
    # query, a filter, a limit, an offset, and a connection to the database. The method returns a Result,
    # where the Ok variant is a vector of bincode-serialized rows of the richest struct of the table variant,
    # while the Err variant contains an ApiError.

    # /// Searches for records in the database.
    # ///
    # /// # Arguments
    # /// * `lowercase_query` - The search query.
    # /// * `filter` - The filter to apply to the search.
    # /// * `limit` - The maximum number of records to return, defaults to 10.
    # /// * `offset` - The number of records to skip, defaults to 0.
    # /// * `connection` - The database connection.
    # pub(crate) async fn search<C, T>(
    #     lowercase_query: &str,
    #     filter: Option<&T::Filter>,
    #     limit: Option<i64>,
    #     offset: Option<i64>,
    #     connection: &mut gluesql::prelude::Glue<C>,
    # ) -> Result<Vec<T>, web_common::api::ApiError>
    # where
    #     C: gluesql::core::store::GStore + gluesql::core::store::GStoreMut,
    #     T: Filtrable + Tabular + SimilarityScore + AllRecords,
    # {

    document = open("../frontend/src/search_dispatch.rs", "w", encoding="utf8")

    document.write(
        "//! This module contains the search method for the Table enum.\n"
        "//!\n"
        "//! This module is automatically generated. Do not write anything here.\n\n"
    )

    document.write(
        "use web_common::database::*;\n"
        "use crate::search::*;\n"
    )

    document.write(
        "pub(crate) trait SearchableTable {\n"
        "    fn search<C>(\n"
        "        &self,\n"
        "        lowercase_query: &str,\n"
        "        filter: Option<Vec<u8>>,\n"
        "        limit: Option<i64>,\n"
        "        offset: Option<i64>,\n"
        "        connection: &mut gluesql::prelude::Glue<C>,\n"
        "    ) -> impl std::future::Future<Output = Result<Vec<u8>, web_common::api::ApiError>>\n"
        "    where\n"
        "        C: gluesql::core::store::GStore + gluesql::core::store::GStoreMut;\n"
        "}\n\n"
    )

    document.write(
        "impl SearchableTable for Table {\n"
        "    async fn search<C>(\n"
        "        &self,\n"
        "        lowercase_query: &str,\n"
        "        filter: Option<Vec<u8>>,\n"
        "        limit: Option<i64>,\n"
        "        offset: Option<i64>,\n"
        "        connection: &mut gluesql::prelude::Glue<C>,\n"
        "    ) -> Result<Vec<u8>, web_common::api::ApiError>\n"
        "    where\n"
        "        C: gluesql::core::store::GStore + gluesql::core::store::GStoreMut,\n"
        "    {\n"
        "        Ok(match self {\n"
    )
    for table in tables:
        document.write(
            f"Table::{table.camel_cased()} => {{\n"
            "     let filter = filter.map(|filter| bincode::deserialize(&filter)).transpose()?;\n"
            f"    bincode::serialize(&search::<C, {table.get_richest_struct().full_path('frontend')}>(\n"
            "         lowercase_query,\n"
            "         filter.as_ref(),\n"
            "         limit,\n"
            "         offset,\n"
            "         connection\n"
            "     ).await?)?\n"
            "}\n"
        )

    document.write("        })\n    }\n}\n")

    document.flush()
    document.close()

    return tables
