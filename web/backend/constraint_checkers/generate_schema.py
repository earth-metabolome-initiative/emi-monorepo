"""Submodule containing the generation of the view schema."""

from typing import List
from tqdm.auto import tqdm
from constraint_checkers.cursor import get_cursor
from constraint_checkers.is_file_changed import is_file_changed
from constraint_checkers.find_foreign_keys import (
    TableMetadata,
    SQLColumn,
    postgres_type_to_diesel_type,
)
from constraint_checkers.migrations_changed import are_migrations_changed

TABLE_DENY_LIST = ["__diesel_schema_migrations", "spatial_ref_sys"]
VIEW_DENY_LIST = ["geometry_columns", "geography_columns"]


def get_views(cursor) -> List[str]:
    """Return list with the view names"""
    cursor.execute(
        "SELECT table_name FROM information_schema.views WHERE table_schema = 'public';"
    )
    views = cursor.fetchall()
    return views


def generate_diesel_schema(
    table_name: str,
    columns: List[SQLColumn],
    table_metadata: TableMetadata,
    schema_file: "TextIO",
) -> str:
    """Generate Diesel schema code for a view."""

    schema_code = "diesel::table! {\n"

    primary_key_ids = ", ".join(
        column
        for column, _ in table_metadata.get_primary_key_names_and_types(table_name)
    )
    schema_code += f"    {table_name} ({primary_key_ids}) {{\n"

    for column in columns:
        if column.nullable:
            schema_code += f"        {column.alias_name} -> diesel::sql_types::Nullable<{postgres_type_to_diesel_type(column.data_type)}>,\n"
        else:
            schema_code += f"        {column.alias_name} -> {postgres_type_to_diesel_type(column.data_type)},\n"
    schema_code += "    }\n"
    schema_code += "}\n"

    schema_file.write(f"{schema_code}\n")


def generate_foreign_keys(
    table_name: str,
    table_metadata: TableMetadata,
    schema_file: "TextIO",
):
    """Generate Diesel schema code for foreign keys."""
    covered_tables = set()
    for foreign_key in table_metadata.get_foreign_keys(table_name):
        foreign_key_table_name = table_metadata.get_foreign_key_table_name(
            table_name, foreign_key
        )
        if foreign_key_table_name in covered_tables or foreign_key_table_name == table_name:
            continue
        covered_tables.add(foreign_key_table_name)
        schema_file.write(
            f"diesel::joinable!({table_name} -> {foreign_key_table_name} ({foreign_key}));\n"
        )


def generate_schema(
    table_metadata: TableMetadata,
):
    """Generate the schema"""
    if not (are_migrations_changed() or is_file_changed(__file__)):
        print("No migrations changed. Skipping the generation of the view schema.")
        return

    # We load the data from the environment variables from the `.env` file
    # at `../.env`.
    conn, cursor = get_cursor()

    table_names: List[str] = []

    # We open the file to write the schema
    with open("src/database/schema.rs", "w", encoding="utf8") as schema_file:
        for table_name in tqdm(
            table_metadata.all_tables(),
            desc="Generating Diesel schema for tables",
            unit="table",
            leave=False,
        ):
            if table_name in TABLE_DENY_LIST:
                continue

            table_names.append(table_name)
            try:
                generate_diesel_schema(
                    table_name,
                    table_metadata.extract_table_columns(table_name),
                    table_metadata,
                    schema_file,
                )
            except NotImplementedError as e:
                raise NotImplementedError(
                    f"Error while generating schema for table {table_name}: {e}"
                ) from e

        for table_name in tqdm(
            table_metadata.all_tables(),
            desc="Generating Diesel schema for foreign keys",
            unit="table",
            leave=False,
        ):
            if table_name in TABLE_DENY_LIST:
                continue

            generate_foreign_keys(table_name, table_metadata, schema_file)

        # Generating Diesel schema for each view
        for view in tqdm(
            get_views(cursor),
            desc="Generating Diesel schema for views",
            unit="view",
            leave=False,
        ):
            view_name = view[0]
            if view_name in VIEW_DENY_LIST:
                continue

            table_names.append(view_name)
            generate_diesel_schema(
                view_name,
                table_metadata.extract_view_columns(view_name),
                table_metadata,
                schema_file,
            )

        joined_table_names = ",\n".join(table_names)
        schema_file.write(
            f"diesel::allow_tables_to_appear_in_same_query!({joined_table_names});"
        )
        print(f"Generated {len(table_names)} Diesel schema entries.")

    # Closing the cursor and connection
    cursor.close()
    conn.close()
