"""Submodule to ensure that all updatable tables have roles tables."""

from typing import List
import os
import shutil
from tqdm.auto import tqdm
from userinput import userinput
from insert_migration import insert_migration
from constraint_checkers.table_metadata import TableStructMetadata
from constraint_checkers.find_foreign_keys import TableMetadata
from constraint_checkers.regroup_tables import get_desinences


class MissingRolesTable(Exception):
    """Exception raised when a roles table is missing."""

    def __init__(self, table_name: str, referring_table_name: str):
        self.table_name = table_name
        self.referring_table_name = referring_table_name
        super().__init__(f"Table {table_name}_{referring_table_name}_roles is missing.")

class UnexpectedRolesTable(Exception):
    """Exception raised when a roles table is unexpected."""

    def __init__(self, table_name: str, referring_table_name: str):
        self.table_name = table_name
        self.referring_table_name = referring_table_name
        super().__init__(f"Table {table_name}_{referring_table_name}_roles is unexpected.")

class MissingRoleInvitationsTable(Exception):
    """Exception raised when a role invitations table is missing."""

    def __init__(self, table_name: str, referring_table_name: str):
        self.table_name = table_name
        self.referring_table_name = referring_table_name
        super().__init__(
            f"Table {table_name}_{referring_table_name}_role_invitations is missing."
        )


class UnexpectedRoleInvitationsTable(Exception):
    """Exception raised when a role invitations table is unexpected."""

    def __init__(self, table_name: str, referring_table_name: str):
        self.table_name = table_name
        self.referring_table_name = referring_table_name
        super().__init__(
            f"Table {table_name}_{referring_table_name}_role_invitations is unexpected."
        )

class MissingRoleRequestsTable(Exception):
    """Exception raised when a role requests table is missing."""

    def __init__(self, table_name: str, referring_table_name: str):
        self.table_name = table_name
        self.referring_table_name = referring_table_name
        super().__init__(
            f"Table {table_name}_{referring_table_name}_role_requests is missing."
        )

class UnexpectedRoleRequestsTable(Exception):
    """Exception raised when a role requests table is unexpected."""

    def __init__(self, table_name: str, referring_table_name: str):
        self.table_name = table_name
        self.referring_table_name = referring_table_name
        super().__init__(
            f"Table {table_name}_{referring_table_name}_role_requests is unexpected."
        )

def is_role_table(table_name: str) -> bool:
    """Check if a table is a role table.

    Parameters
    ----------
    table_name : str
        The table name.

    Returns
    -------
    bool
        Whether the table is a role table.
    """
    return table_name.endswith("_role_invitations") or table_name.endswith("_role_requests") or table_name.endswith("_roles")

def handle_unexpected_roles_table(
    table_name: str,
    referring_table_name: str,
    desinence: str = "roles",
):
    """Handle the unexpected user roles table.

    Parameters
    ----------
    table_name : str
        The table name.
    referring_table_name: str
        The referring table name.
    desinence : str
        The desinence to use for the table name.
        Can be either "roles" or "role_requests" or "role_invitations".
    """
    assert isinstance(table_name, str)
    assert isinstance(referring_table_name, str)
    assert isinstance(desinence, str)

    if referring_table_name not in ("users", "teams"):
        raise ValueError(f"Referring table name {referring_table_name} is not valid.")

    if desinence not in ("roles", "role_requests", "role_invitations"):
        raise ValueError(f"Desinence {desinence} is not valid.")

    expected_table_name = f"{table_name}_{referring_table_name}_{desinence}"

    drop_table = userinput(
        f"Table {expected_table_name} is unexpected. Do you want to drop it? (yes/no)",
        label="{name}",
        default=False,
        validator="human_bool",
        sanitizer="human_bool",
    )

    if drop_table:
        # We revert all migrations.
        status = os.system("diesel migration revert --all")

        if status != 0:
            raise RuntimeError("Could not revert all migrations.")

        # We delete all migrations associated to the table.
        desinences = get_desinences(expected_table_name)
        for migration in os.listdir("migrations"):
            if not os.path.isdir(f"migrations/{migration}"):
                continue
            _number, desinence = migration.split("_", 1)
            if desinence in desinences:
                shutil.rmtree(f"migrations/{migration}")

def handle_missing_roles_table(
    table_name: str,
    referring_table_name: str,
    table_metadata: TableMetadata,
    desinence: str = "roles",
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
    desinence : str
        The desinence to use for the table name.
        Can be either "roles" or "role_requests" or "role_invitations".
    """

    if referring_table_name not in ("users", "teams"):
        raise ValueError(f"Referring table name {referring_table_name} is not valid.")

    if desinence not in ("roles", "role_requests", "role_invitations"):
        raise ValueError(f"Desinence {desinence} is not valid.")

    expected_table_name = f"{table_name}_{referring_table_name}_{desinence}"

    create_table = userinput(
        f"Table {expected_table_name} is missing. Do you want to create it? (yes/no)",
        label="{name}",
        default=False,
        validator="human_bool",
        sanitizer="human_bool",
    )

    if create_table:
        # First, we identify the number of the migration where to create the table.
        # The migration must be position after the last migration relative to the currently
        # considered table.
        desinences = get_desinences(table_name)

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

        primary_keys = table_metadata.get_primary_key_names_and_types(table_name)

        assert len(primary_keys) == 1

        primary_key_name, primary_key_type = primary_keys[0]

        column_name = referring_table_name[:-1] + "_id"

        # First, we write the up migration.
        with open(
            f"migrations/{full_migration_name}/up.sql", "w", encoding="utf-8"
        ) as f:
            f.write(
                f"-- Create the {expected_table_name} table.\n"
                f"CREATE TABLE IF NOT EXISTS {expected_table_name} (\n"
                f"    table_id {primary_key_type} NOT NULL REFERENCES {table_name}({primary_key_name}) ON DELETE CASCADE,\n"
                f"    {column_name} INTEGER NOT NULL REFERENCES {referring_table_name}(id) ON DELETE CASCADE,\n"
                "    role_id INTEGER NOT NULL REFERENCES roles(id) ON DELETE CASCADE,\n"
                "    created_by INTEGER NOT NULL REFERENCES users(id) ON DELETE CASCADE,\n"
                "    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,\n"
                f"    PRIMARY KEY (table_id, {column_name})\n"
                ");\n"
            )

        # Then, we write the down migration.
        with open(
            f"migrations/{full_migration_name}/down.sql", "w", encoding="utf-8"
        ) as f:
            f.write(
                f"-- Drop the {expected_table_name} table.\n"
                f"DROP TABLE IF EXISTS {expected_table_name};\n"
            )


def ensure_updatable_tables_have_role_requests_tables(
    tables: List[TableStructMetadata], table_metadata: TableMetadata
):
    """Ensure that all updatable tables have role requests tables.

    Parameters
    ----------
    new_model_structs : List[TableStructMetadata]
        The new model structs.
    table_metadata : TableMetadata
        The table metadata.

    Implementation details
    ----------------------
    All tables that contain a column such as `requested_by`, must have a corresponding
    `{table_name}_role_requests` table.
    When this table is missing, the function raises an exception and we
    try to handle the creation process asking the user to confirm the creation of the
    missing role requests table.
    """

    for table in tqdm(
        tables,
        desc="Ensuring updatable tables have role requests tables",
        unit="table",
        leave=False,
    ):
        if table.is_updatable() and not table.editability_always_depend_on_parent_column():
            if not table_metadata.is_table(f"{table.name}_users_role_requests"):
                handle_missing_roles_table(
                    table.name, "users", table_metadata, "role_requests"
                )
                raise MissingRoleRequestsTable(table.name, "users")
            if not table_metadata.is_table(f"{table.name}_teams_role_requests") and table.name not in ("users", "teams"):
                handle_missing_roles_table(
                    table.name, "teams", table_metadata, "role_requests"
                )
                raise MissingRoleRequestsTable(table.name, "teams")
        else:
            if table_metadata.is_table(f"{table.name}_users_role_requests"):
                handle_unexpected_roles_table(
                    table.name, "users", "role_requests"
                )
                raise UnexpectedRoleRequestsTable(table.name, "users")
            if table_metadata.is_table(f"{table.name}_teams_role_requests") and table.name not in ("users", "teams"):
                handle_unexpected_roles_table(
                    table.name, "teams", "role_requests"
                )
                raise UnexpectedRoleRequestsTable(table.name, "teams")


def ensure_updatable_tables_have_role_invitations_tables(
    tables: List[TableStructMetadata], table_metadata: TableMetadata
):
    """Ensure that all updatable tables have role invitations tables.

    Parameters
    ----------
    new_model_structs : List[TableStructMetadata]
        The new model structs.
    table_metadata : TableMetadata
        The table metadata.

    Implementation details
    ----------------------
    All tables that contain a column such as `invited_by`, must have a corresponding
    `{table_name}_role_invitations` table.
    When this table is missing, the function raises an exception and we
    try to handle the creation process asking the user to confirm the creation of the
    missing role invitations table.
    """

    for table in tqdm(
        tables,
        desc="Ensuring updatable tables have role invitations tables",
        unit="table",
        leave=False,
    ):
        if table.is_updatable() and not table.editability_always_depend_on_parent_column():
            if not table_metadata.is_table(f"{table.name}_users_role_invitations"):
                handle_missing_roles_table(
                    table.name, "users", table_metadata, "role_invitations"
                )
                raise MissingRoleInvitationsTable(table.name, "users")
            if not table_metadata.is_table(f"{table.name}_teams_role_invitations") and table.name not in ("users", "teams"):
                handle_missing_roles_table(
                    table.name, "teams", table_metadata, "role_invitations"
                )
                raise MissingRoleInvitationsTable(table.name, "teams")
        else:
            if table_metadata.is_table(f"{table.name}_users_role_invitations"):
                handle_unexpected_roles_table(
                    table.name, "users", "role_invitations"
                )
                raise UnexpectedRoleInvitationsTable(table.name, "users")
            if table_metadata.is_table(f"{table.name}_teams_role_invitations") and table.name not in ("users", "teams"):
                handle_unexpected_roles_table(
                    table.name, "teams", "role_invitations"
                )
                raise UnexpectedRoleInvitationsTable(table.name, "teams")
        


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
    ensure_updatable_tables_have_role_invitations_tables(tables, table_metadata)
    ensure_updatable_tables_have_role_requests_tables(tables, table_metadata)

    for table in tqdm(
        tables,
        desc="Ensuring updatable tables have roles tables",
        unit="table",
        leave=False,
    ):
        if table.is_updatable() and not table.editability_always_depend_on_parent_column():
            if not table_metadata.is_table(f"{table.name}_users_roles"):
                handle_missing_roles_table(table.name, "users", table_metadata)
                raise MissingRolesTable(table.name, "users")
            if not table_metadata.is_table(
                f"{table.name}_teams_roles"
            ) and table.name not in ("users", "teams"):
                handle_missing_roles_table(table.name, "teams", table_metadata)
                raise MissingRolesTable(table.name, "teams")
        else:
            if table_metadata.is_table(f"{table.name}_users_roles"):
                handle_unexpected_roles_table(table.name, "users", "roles")
                raise UnexpectedRolesTable(table.name, "users")
            if table_metadata.is_table(f"{table.name}_teams_roles") and table.name not in ("users", "teams"):
                handle_unexpected_roles_table(table.name, "teams", "roles")
                raise UnexpectedRolesTable(table.name, "teams")