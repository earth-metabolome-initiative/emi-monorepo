"""Submodule to ensure that all updatable tables have roles tables."""

from typing import List
import os
from constraint_checkers.table_metadata import TableStructMetadata
from constraint_checkers.find_foreign_keys import TableMetadata
from tqdm.auto import tqdm
from userinput import userinput
from insert_migration import insert_migration

class MissingRolesTable(Exception):
    """Exception raised when a roles table is missing."""

    def __init__(self, table_name: str, referring_table_name: str):
        self.table_name = table_name
        self.referring_table_name = referring_table_name
        super().__init__(
            f"Table {table_name}_{referring_table_name}_roles is missing."
        )


def handle_missing_roles_table(
    table_name: str, referring_table_name: str, table_metadata: TableMetadata
):
    """Handle the missing user roles table.

    Parameters
    ----------
    table_name : str
        The table name.
    referring_table_name: str
        The referring table name.
    table_metadata : TableMetadata
        The table metadata.
    """

    if referring_table_name not in ("users", "teams"):
        raise ValueError(f"Referring table name {referring_table_name} is not valid.")

    expected_table_name = f"{table_name}_{referring_table_name}_roles"

    create_table = userinput(
        f"Table {expected_table_name} is missing. Do you want to create it? (yes/no)",
        default=False,
        validator="human_bool",
        sanitizer="human_bool",
    )

    if create_table:
        # First, we identify the number of the migration where to create the table.
        # The migration must be position after the last migration relative to the currently
        # considered table.
        desinences = [
            f"create_{table_name}_table",
            f"create_{table_name}_sequential_index",
            f"create_{table_name}_updated_at_trigger",
            f"create_{table_name}_gin_index",
            f"populate_{table_name}_table",
        ]

        migration_number = 0

        for migration in os.listdir("migrations"):
            if not os.path.isdir(f"migrations/{migration}"):
                continue
            number, desinence = migration.split("_", 1)
            if desinence in desinences:
                migration_number = max(migration_number, int(number))

        migration_number += 1

        expected_migration = f"create_{expected_table_name}_table"

        insert_migration(migration_number, expected_migration)

        padded_migration_number = str(migration_number).zfill(14)
        full_migration_name = f"{padded_migration_number}_{expected_migration}"

        primary_keys = (
            table_metadata.get_primary_key_names_and_types(table_name)
        )

        assert len(primary_keys) == 1

        primary_key_name, primary_key_type = primary_keys[0]

        column_name = referring_table_name[:-1] + "_id"

        # First, we write the up migration.
        with open(f"migrations/{full_migration_name}/up.sql", "w", encoding="utf-8") as f:
            f.write(
                f"""
                -- Create the {expected_table_name} table.
                CREATE TABLE IF NOT EXISTS {expected_table_name} (
                    table_id {primary_key_type} NOT NULL REFERENCES {table_name}({primary_key_name}),
                    {column_name} INTEGER NOT NULL REFERENCES {referring_table_name}(id),
                    role_id INTEGER NOT NULL REFERENCES roles(id),
                    created_by INTEGER NOT NULL REFERENCES users(id),
                    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
                    updated_by INTEGER NOT NULL REFERENCES users(id),
                    updated_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
                    PRIMARY KEY (table_id, {column_name}, role_id)
                );
            """
            )

        # Then, we write the down migration.
        with open(f"migrations/{full_migration_name}/down.sql", "w", encoding="utf-8") as f:
            f.write(
                f"""
                -- Drop the {expected_table_name} table.
                DROP TABLE IF EXISTS {expected_table_name};
            """
            )


def ensure_updatable_tables_have_roles_tables(
    tables: List[TableStructMetadata], table_metadata: TableMetadata
):
    """Ensure that all updatable tables have roles tables.

    Parameters
    ----------
    new_model_structs : List[TableStructMetadata]
        The new model structs.
    table_metadata : TableMetadata
        The table metadata.

    Implementation details
    ----------------------
    All tables that contain a column such as `updated_by`, must have a corresponding
    `{table_name}_users_roles` table, alongside the `{table_name}_teams_roles` table.
    When either of these tables are missing, the function raises an exception and we
    try to handle the creation process asking the user to confirm the creation of the
    missing roles table.
    """

    for table in tqdm(
        tables, desc="Ensuring updatable tables have roles tables", unit="table"
    ):
        if table.is_updatable() and not table.is_junktion_table():
            if not table_metadata.is_table(f"{table.name}_users_roles"):
                handle_missing_roles_table(table.name, "users", table_metadata)
                raise MissingRolesTable(table.name, "users")
            if not table_metadata.is_table(f"{table.name}_teams_roles") and table.name != "teams":
                handle_missing_roles_table(table.name, "teams", table_metadata)
                raise MissingRolesTable(table.name, "teams")
