"""Submodule containing the generation of the view schema."""

from typing import List
from tqdm.auto import tqdm
from constraint_checkers.cursor import get_cursor
from constraint_checkers.find_foreign_keys import TableMetadata, ViewColumn


VIEW_DENY_LIST = [
    "geometry_columns",
    "geography_columns"
]

def get_views(cursor) -> List[str]:
    """Return list with the view names"""
    cursor.execute(
        "SELECT table_name FROM information_schema.views WHERE table_schema = 'public';"
    )
    views = cursor.fetchall()
    return views

def map_postgres_to_rust_type(pg_type):
    """Map the Postgres type to the Rust type."""
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
    """Generate Diesel schema code for a view."""

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


def generate_view_schema(
    table_metadata: TableMetadata,
):
    """Generate the view schema.

    Implementative details
    -------------------------
    We generate the views by connecting to the database and querying the `information_schema`
    tables. We then write the views to the file `src/views/schema.rs`. The database is a postgres
    database, and the connection string is read from the environment variable `DATABASE_URL`.
    """
    # We load the data from the environment variables from the `.env` file
    # at `../.env`.
    conn, cursor = get_cursor()

    # Getting the list of views
    views = get_views(cursor)
    assert isinstance(views, list)
    assert all(isinstance(view, tuple) for view in views)
    assert all(len(view) == 1 for view in views)
    assert all(isinstance(view[0], str) for view in views)

    # We open the file to write the schema
    schema_file = open("src/views/schema.rs", "w", encoding="utf8")

    # Generating Diesel schema for each view
    for view in tqdm(
        views,
        desc="Generating Diesel schema for views",
        unit="view",
        leave=False,
    ):
        view_name = view[0]
        if view_name in VIEW_DENY_LIST:
            continue
        columns = table_metadata.extract_view_columns(view_name)
        schema_code = generate_diesel_schema(view_name, columns)
        schema_file.write(schema_code + "\n")

    # Closing the cursor and connection
    cursor.close()
    conn.close()
