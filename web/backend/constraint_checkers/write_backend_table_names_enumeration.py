"""Write the table enumeration in the backend crate."""

from typing import List, Dict, Optional, Tuple
import os
from constraint_checkers.table_metadata import TableStructMetadata
from constraint_checkers.struct_metadata import (
    StructMetadata,
    MethodDefinition,
    AttributeMetadata,
)
from constraint_checkers.is_file_changed import is_file_changed
from constraint_checkers.migrations_changed import are_migrations_changed


def write_backend_table_names_enumeration(
    tables: List[TableStructMetadata],
):
    """Write the table names enumeration in the backend crate."""
    if not (are_migrations_changed() or is_file_changed(__file__) or is_file_changed("./constraint_checkers/write_backend_flat_variants.py")):
        print("No change in migrations or file. Skipping writing backend table enum.")
        return

    assert isinstance(tables, list), "Tables must be a list."
    assert len(tables) > 0, "Tables must not be empty."
    assert all(
        isinstance(table, TableStructMetadata) for table in tables
    ), "All tables must be of type TableStructMetadata."

    # We check that we are currently executing in the `backend` crate
    # so to make sure that the relative path to the `web_common` crate
    # is correct.
    if not os.getcwd().endswith("backend"):
        raise RuntimeError("This script must be executed in the `backend` crate.")

    path = "src/database/table_enumeration.rs"

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
        "use crate::database::*;",
        "use web_common::database::PrimaryKey;",
        "use diesel::r2d2::PooledConnection;",
        "use diesel::r2d2::ConnectionManager;",
    ]

    table_enum_struct = StructMetadata(struct_name="Table", table_name=None)

    document.write("\n".join(imports) + "\n\n")

    # First, we determine the methods that are common across all tables associated
    # richest structs, and we implement them for the Table enum.
    method_counts: Dict[MethodDefinition, Tuple[MethodDefinition, int]] = {}
    for table in tables:
        for method in table.get_richest_struct().backend_methods():
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

        table_method = table_enum_struct.add_backend_method(
            MethodDefinition(name=method.name, summary=method.summary, visibility="")
        )
        table_method.include_self_ref()
        method_return_type = method.get_return_type()
        return_type = AttributeMetadata(
            original_name=method_return_type.original_name,
            name=method_return_type.name,
            data_type=method_return_type.data_type(route="backend")
            .replace("Vec<Self>", "Vec<u8>")
            .replace("Self", "Vec<u8>"),
            optional=method_return_type.optional,
        )
        table_method.set_return_type(return_type)

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
                        original_name=argument.original_name,
                        name=argument.name,
                        data_type="Vec<u8>",
                        optional=argument.optional,
                    ),
                    description=method.get_argument_description(argument),
                )
            elif method.is_primary_key_argument(argument):
                continue
            else:
                table_method.add_argument(
                    argument,
                    description=method.get_argument_description(argument),
                )
        table_methods.append(table_method)

    # We write the trait that provides the backend implementations for the Table enumeration.

    document.write(
        "/// Trait providing the backend implementations for the Table enumeration\n"
        "pub trait BackendTable {\n"
    )
    for method in table_methods:
        method.write_header_to(document)
        document.write(";\n\n")

    document.write("}\n\n")

    # Next, we actually implement the trait for the Table enum.

    document.write("impl BackendTable for web_common::database::Table {\n")

    for method in table_methods:
        method.write_header_to(document)
        document.write(" {\n")
        document.write("        Ok(match self {\n")
        for table in tables:
            richest_variant: StructMetadata = table.get_richest_struct()
            struct_method: Optional[MethodDefinition] = (
                richest_variant.get_backend_method_by_name(method.name)
            )
            if struct_method is not None:
                document.write(
                    f"            web_common::database::Table::{table.camel_cased()} => {{\n"
                )

                if "Self" in struct_method.get_return_type().data_type(route="backend"):
                    document.write("bincode::serialize(&")

                document.write(f"{table.richest_struct_name()}::{method.name}(\n")
                arguments = []
                for argument in method.arguments:
                    if argument.name == "self":
                        continue
                    original_argument = struct_method.get_argument_by_name(
                        argument.name
                    )
                    if (
                        original_argument is not None
                        and original_argument.has_struct_data_type()
                    ):
                        if original_argument.optional:
                            formatted_argument = f"{argument.name}.map(|{argument.name}| bincode::deserialize::<{original_argument.data_type(route='backend')}>(&{argument.name})).transpose()?"
                        else:
                            formatted_argument = f"bincode::deserialize::<{original_argument.data_type(route='backend')}>(&{argument.name})?"

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
                document.write(")")
                if "Result" in struct_method.get_return_type().format_data_type(route="backend"):
                    document.write("?")
                if "Self" in struct_method.get_return_type().data_type(route="backend"):
                    document.write(")?\n")
                document.write("            },\n")
            else:
                document.write(
                    f'            web_common::database::Table::{table.camel_cased()} => unimplemented!("Method {method.name} not implemented for table {table.name}."),\n'
                )
        # We close the match
        document.write("        })\n")
        # And we close the method
        document.write("    }\n\n")
    # And we close the trait implementation
    document.write("    }\n\n")

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
        if not table.is_insertable():
            document.write(
                f'            web_common::database::Table::{table.camel_cased()} => unreachable!("Table `{table.name}` is not insertable as it does not have a known column associated to a creator user id."),\n'
            )
            continue

        document.write(
            f"            web_common::database::Table::{table.camel_cased()} => {{\n"
            f"                let row: {table.get_new_flat_variant().full_path(route='backend')} = bincode::deserialize::<{table.get_new_flat_variant().full_path(route='backend')}>(&row)?;\n"
            f"                let inserted_row: {table.flat_variant.full_path(route='backend')} = <{table.get_new_flat_variant().full_path(route='backend')} as InsertRow>::insert(row, user_id, connection)?;\n"
        )

        # If the table has a richer variant than the flat one, we convert the flat struct
        # to the richest struct using the `from_flat` method.
        richest_struct = table.get_richest_struct()
        if richest_struct.is_nested():
            if richest_struct.has_attribute_that_may_be_hidden():
                document.write(
                    f"                let nested_row = {richest_struct.full_path(route='backend')}::from_flat(inserted_row, Some(user_id), connection)?;\n"
                )
            else:
                document.write(
                    f"                let nested_row = {richest_struct.full_path(route='backend')}::from_flat(inserted_row, connection)?;\n"
                )
            document.write("                 bincode::serialize(&nested_row)?\n")
        else:
            document.write("                 bincode::serialize(&inserted_row)?\n")

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
        if not table.is_updatable():
            document.write(
                f'            web_common::database::Table::{table.camel_cased()} => unreachable!("Table `{table.name}` is not updatable as it does not have a known column associated to an updater user id."),\n'
            )
            continue

        document.write(
            f"            web_common::database::Table::{table.camel_cased()} => {{\n"
            f"                let row: web_common::database::{table.update_flat_variant_name()} = bincode::deserialize::<web_common::database::{table.update_flat_variant_name()}>(&row)?;\n"
            f"                let updated_row: crate::database::flat_variants::{table.flat_variant_name()} = <web_common::database::{table.update_flat_variant_name()} as UpdateRow>::update(row, user_id, connection)?;\n"
        )

        richest_struct = table.get_richest_struct()
        # If the table has a richer variant than the flat one, we convert the flat struct
        # to the richest struct using the `from_flat` method.
        if richest_struct.is_nested():
            if richest_struct.has_attribute_that_may_be_hidden():
                document.write(
                    f"                let nested_row = crate::database::nested_variants::{richest_struct.name}::from_flat(updated_row, Some(user_id), connection)?;\n"
                )
            else:
                document.write(
                    f"                let nested_row = crate::database::nested_variants::{richest_struct.name}::from_flat(updated_row, connection)?;\n"
                )
            document.write("                 bincode::serialize(&nested_row)?\n")
        else:
            document.write("                 bincode::serialize(&updated_row)?\n")

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
        "    /// * `user_id` - The id of the user retrieving the row.\n"
        "    /// * `connection` - The database connection.\n"
        "    ///\n"
        "    /// # Returns\n"
        "    /// The bincode-serialized row of the table.\n"
        "    fn from_flat_str(\n"
        "         &self,\n"
        "         row: &str,\n"
        "         user_id: Option<i32>,\n"
        "         connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>\n"
        ") -> Result<Vec<u8>, web_common::api::ApiError>;\n"
        "}\n\n"
    )

    document.write(
        "impl FromFlatStrTable for web_common::database::Table {\n\n"
        "    fn from_flat_str(\n"
        "        &self,\n"
        "        row: &str,\n"
        "        user_id: Option<i32>,\n"
        "        connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>\n"
        "    ) -> Result<Vec<u8>, web_common::api::ApiError> {\n"
        "        Ok(match self {\n"
    )

    for table in tables:
        flat_variant = table.get_flat_variant()
        richest_variant = table.get_richest_struct()

        if not richest_variant.is_nested():
            document.write(
                f"            web_common::database::Table::{table.camel_cased()} => bincode::serialize(&serde_json::from_str::<crate::database::flat_variants::{table.flat_variant_name()}>(row)?)?,\n"
            )
            continue

        document.write(
            f"            web_common::database::Table::{table.camel_cased()} => {{\n"
            f"                let flat_row: crate::database::flat_variants::{flat_variant.name} = serde_json::from_str::<crate::database::flat_variants::{flat_variant.name}>(row)?;\n"
        )
        if richest_variant.has_attribute_that_may_be_hidden():
            document.write(
                f"                 let richest_row = crate::database::nested_variants::{richest_variant.name}::from_flat(flat_row, user_id, connection)?;\n"
            )
        else:
            document.write(
                f"                 let richest_row = crate::database::nested_variants::{richest_variant.name}::from_flat(flat_row, connection)?;\n"
            )
        document.write(
            "                 bincode::serialize(&richest_row)?\n            },\n"
        )

    document.write("        })\n    }\n}\n")

    document.flush()
    document.close()
