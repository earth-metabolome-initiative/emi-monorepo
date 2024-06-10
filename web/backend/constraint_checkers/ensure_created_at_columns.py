"""Submodule ensuring that all tables with a created_by column also have an created_at column."""

from constraint_checkers.find_foreign_keys import TableMetadata
from tqdm.auto import tqdm
from constraint_checkers.is_file_changed import is_file_changed
from constraint_checkers.migrations_changed import are_migrations_changed


def ensure_created_at_columns(
    table_metadata: TableMetadata
):
    """Ensure that all tables with a created_by column also have an created_at column."""
    if not (are_migrations_changed() or is_file_changed(__file__)):
        print("No change detected in the migrations or the file. Skipping the created-at function check.")
        return

    for table_name in tqdm(
        table_metadata.all_tables(),
        unit="table",
        desc="Ensuring created-at constraint",
        leave=False
    ):

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
