"""Submodule ensuring that all tables with a created_by column also have an created_at column."""

from constraint_checkers.find_foreign_keys import TableMetadata


def ensure_created_at_columns(
    table_metadata: TableMetadata
):
    """Ensure that all tables with a created_by column also have an created_at column."""

    for table_name in table_metadata.tables():
        
        if table_name == 'users':
            continue

        columns = table_metadata.get_columns(table_name)

        if "created_by" in columns and "created_at" not in columns:
            raise ValueError(
                f"Table {table_name} has a created_by column but no created_at column."
            )

        if "created_by" not in columns and "created_at" in columns:
            raise ValueError(
                f"Table {table_name} has a created_at column but no created_by column."
            )
