"""Writes the table names enumeration to the web_common crate."""

import os
from typing import List, Dict, Tuple
from constraint_checkers.struct_metadata import (
    StructMetadata,
    MethodDefinition,
    AttributeMetadata,
)
from constraint_checkers.table_metadata import TableStructMetadata


def derive_web_common_table_methods(
    table_enum_struct: StructMetadata,
    tables: List[TableStructMetadata],
):
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
                        name=("row" if argument.name == "self" else argument.name),
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


def derive_tables_enumeration(
    struct_metadatas: List[StructMetadata],
    new_model_structs: List[StructMetadata],
    update_model_structs: List[StructMetadata],
) -> Tuple[StructMetadata, List[TableStructMetadata]]:
    """Writes the table names enumeration to the web_common crate."""
    # The derives to apply to the structs in the `src/database/tables.rs` document
    derives = [
        "serde::Deserialize",
        "serde::Serialize",
        "Clone",
        "Debug",
        "PartialEq",
        "Eq",
        "Copy",
    ]

    # We check that we are currently executing in the `backend` crate
    # so to make sure that the relative path to the `web_common` crate
    # is correct.
    if not os.getcwd().endswith("backend"):
        raise RuntimeError("This script must be executed in the `backend` crate.")

    table_enum_struct = StructMetadata(struct_name="Table", table_name=None)
    for derive in derives:
        table_enum_struct.add_derive(derive)

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

    return (
        table_enum_struct,
        tables,
    )
