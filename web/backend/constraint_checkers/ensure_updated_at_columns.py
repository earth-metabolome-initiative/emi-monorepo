"""Submodule ensuring that all tables with a updated_by column also have an updated_at column."""

from constraint_checkers.find_foreign_keys import find_foreign_keys


def ensure_updated_at_columns():
    """Ensure that all tables with a updated_by column also have an updated_at column."""
    table_metadata = find_foreign_keys()

    for table_name in table_metadata.tables():
        columns = table_metadata.get_columns(table_name)

        if "updated_by" in columns and "updated_at" not in columns:
            raise ValueError(
                f"Table {table_name} has a updated_by column but no updated_at column."
            )

        if "updated_by" not in columns and "updated_at" in columns:
            raise ValueError(
                f"Table {table_name} has a updated_at column but no updated_by column."
            )
