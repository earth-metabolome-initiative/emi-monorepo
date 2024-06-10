"""Submodule ensuring that all tables with a updated_by column also have an updated_at column."""
from tqdm.auto import tqdm
from constraint_checkers.find_foreign_keys import TableMetadata
from constraint_checkers.is_file_changed import is_file_changed
from constraint_checkers.migrations_changed import are_migrations_changed


def ensure_updated_at_columns(
    table_metadata: TableMetadata,
):
    """Ensure that all tables with a updated_by column also have an updated_at column.
    
    Parameters
    ----------
    table_metadata : TableMetadata
        The table metadata.
    """
    if not (are_migrations_changed() or is_file_changed(__file__)):
        print("No change detected in the migrations or the file. Skipping the roles table check.")
        return

    for table_name in tqdm(
        table_metadata.all_tables(),
        unit="table",
        desc="Ensuring updated-at constraint",
        leave=False
    ):

        if table_name == 'users':
            continue

        columns = table_metadata.get_columns(table_name)

        if "updated_by" in columns and "updated_at" not in columns:
            raise ValueError(
                f"Table {table_name} has a updated_by column but no updated_at column."
            )

        if "updated_by" not in columns and "updated_at" in columns:
            raise ValueError(
                f"Table {table_name} has a updated_at column but no updated_by column."
            )
