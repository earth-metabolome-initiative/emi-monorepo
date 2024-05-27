"""Ensures that there exists an SQL function usable to check whether a user can {operation} a row in a table."""

from typing import List, Optional, Dict
from tqdm.auto import tqdm
from insert_migration import insert_migration
from userinput import userinput
from constraint_checkers.regroup_tables import get_best_insertion_point
from constraint_checkers.find_foreign_keys import is_role_table
from constraint_checkers.table_metadata import TableStructMetadata
from constraint_checkers.struct_metadata import AttributeMetadata


def handle_missing_can_x_function(
    table: TableStructMetadata,
):
    """Creates the can_update function if it does not exist."""
    assert isinstance(table, TableStructMetadata)
    operations = ("update", "delete", "view")
    user_column_names = ["created_by", "updated_by"]

    if table.name == "users":
        user_column_names = ["id"]

    associated_roles_tables = []
    flat_variant = table.get_flat_variant()

    for possible_associated_roles_table in (
        f"{table.name}_users_roles",
        f"{table.name}_teams_users",
    ):
        if flat_variant.table_metadata.is_table(possible_associated_roles_table):
            associated_roles_tables.append(possible_associated_roles_table)

    if len(associated_roles_tables) > 0:
        assert (
            len(flat_variant.get_primary_keys()) == 1
        ), f"The table {table.name} has associated roles tables, but it does not have exactly one primary key column."

    roles: Dict[str, int] = {
        "update": 2,
        "view": 3,
        "delete": 1,
    }

    method_names: Dict[str, str] = {
        "update": table.get_can_update_function_name(),
        "view": table.get_can_view_function_name(),
        "delete": table.get_can_admin_function_name(),
    }

    trigger_desinence_name = f"create_{table.name}_can_x_trigger"

    print(
        f"The table {table.name} is insertable or updatable, but it does not have a can_update/can_admin/can_view function or trigger. "
        "This function is used to determine whether a user can perform the operation on a row. "
        "We can create the trigger for you."
    )

    proceed = userinput(
        name=trigger_desinence_name,
        label="Proceed with the creation of the trigger?",
        default="no",
        validator="human_bool",
        sanitizer="human_bool",
    )

    if not proceed:
        print(f"Skipping the creation of the trigger for the table {table.name}.")
        return

    parent_columns: Optional[List[AttributeMetadata]] = (
        flat_variant.get_editability_determinant_columns()
    )

    if parent_columns is None:
        parent_columns = []

    migration_number = get_best_insertion_point(
        table_name=table.name, expected_desinence=trigger_desinence_name
    )

    # We create the trigger migration.
    full_migration_name = f"{str(migration_number).zfill(14)}_{trigger_desinence_name}"
    insert_migration(counter=migration_number, name=trigger_desinence_name)

    up_index_migration = open(
        f"./migrations/{full_migration_name}/up.sql", "w", encoding="utf8"
    )

    if len(parent_columns) > 0 and all(
        parent_column.optional for parent_column in parent_columns
    ):
        assert len(associated_roles_tables) > 0, (
            f"The table {table.name} is updatable and has either no parent columns or all of them are optional, "
            "but it does not have associated roles tables. This is not supported. Specifically, the parent columns "
            f"are {parent_columns}."
        )

    primary_keys = ", ".join(
        f"{column.name} {column.sql_data_type()}" for column in table.get_primary_keys()
    )

    for operation in operations:
        can_x_function = method_names[operation]
        max_role_id = roles[operation]

        if operation == "view" and not table.may_be_hidden():
            continue

        up_index_migration.write(
            f"-- The function `{can_x_function}` takes a user ID (INTEGER) and the primary keys\n"
            "-- and returns a BOOLEAN indicating whether the user can {operation} the row. Since this table's editability\n"
            "-- may depend on the parent column, this function retrieves the value of the parent column from the row\n"
            f"-- and calls the parent column's can_{operation} function if the parent column is not NULL. Otherwise, the function\n"
            f"-- checks if the row was created by the user or if the user is found in either the {table.name}_users_roles table or\n"
            f"-- the {table.name}_teams_users table with an appropriate role id.\n"
            f"CREATE FUNCTION {can_x_function}(author_user_id INTEGER, {primary_keys})\n"
            "RETURNS BOOLEAN AS $$\n"
            "DECLARE\n"
            f"    canary INTEGER; -- Value used to check whether the row we are queering for actually exists, so to distinguish when the parent column is NULL and when the row is missing.\n"
        )
        columns_to_query = []
        for parent_column in parent_columns:
            up_index_migration.write(
                f"    {parent_column.name} {parent_column.sql_data_type()};\n"
            )
            columns_to_query.append(parent_column.name)

        for user_column_name in user_column_names:
            if table.has_column(user_column_name):
                up_index_migration.write(f"    {user_column_name} INTEGER;\n")
                columns_to_query.append(user_column_name)

        if operation == "view" and table.has_public_column():
            up_index_migration.write(f"    {table.get_public_column().name} BOOLEAN;\n")
            columns_to_query.append(table.get_public_column().name)

        up_index_migration.write(
            "BEGIN\n"
            f"-- We retrieve the value of the parent column from the row, as identified by the provided primary key(s).\n"
            f"    SELECT {', '.join(columns_to_query + ['1'])} INTO {', '.join(columns_to_query + ['canary'])} FROM {table.name} WHERE {table.get_function_primary_key_where_clause()};\n"
            "-- If the row does not exist, we return FALSE.\n"
            "    IF canary IS NULL THEN\n"
            "        RETURN TRUE;\n"
            "    END IF;\n"
        )

        # If the operation if either an edit or a delete, we check that the provided
        # author_user_id is not NULL.
        if operation in ("update", "delete"):
            up_index_migration.write(
                "-- If the author_user_id is NULL, we return FALSE.\n"
                "    IF author_user_id IS NULL THEN\n"
                "        RETURN FALSE;\n"
                "    END IF;\n"
            )

        if table.has_public_column() and operation == "view":
            up_index_migration.write(
                "-- If the row is public, we return TRUE.\n"
                f"    IF {table.get_public_column().name} THEN\n"
                "        RETURN TRUE;\n"
                "    END IF;\n"
            )
        for user_column_name in user_column_names:
            if table.has_column(user_column_name):
                up_index_migration.write(
                    f"-- We check whether the user is the {user_column_name} of the row.\n"
                    f"    IF author_user_id = {user_column_name} THEN\n"
                    "        RETURN TRUE;\n"
                    "    END IF;\n"
                )

        # Next, we check whether the user is in either of the associated roles tables.
        for associated_roles_table in associated_roles_tables:
            if associated_roles_table == f"{table.name}_users_roles":
                up_index_migration.write(
                    f"-- We check whether the user is in the {associated_roles_table} table with an appropriate role id.\n"
                    f"    IF EXISTS (SELECT 1 FROM {associated_roles_table} WHERE {associated_roles_table}.user_id = author_user_id AND {associated_roles_table}.role_id <= {max_role_id} AND {associated_roles_table}.table_id == {flat_variant.get_primary_keys()[0].name}) THEN\n"
                    "        RETURN TRUE;\n"
                    "    END IF;\n"
                )
            elif associated_roles_table == f"{table.name}_teams_roles":
                up_index_migration.write(
                    f"-- We check whether the user is in one of the teams in the {associated_roles_table} table with a role id,\n"
                    f"-- and that the team is associated with the row with role. We do this by joining the teams_users_roles table\n"
                    f"-- with the {table.name}_teams_roles table and checking whether the user is in the team and has the correct role.\n"
                    f"   IF EXISTS (SELECT 1 FROM {table.name}_teams_roles \n"
                    f"       JOIN teams_users_roles ON {table.name}_teams_roles.team_id = teams_users_roles.team_id\n"
                    f"       WHERE teams_users_roles.user_id = author_user_id AND {table.name}_teams_roles.role_id <= {max_role_id}) AND teams_users_roles.role_id <= {max_role_id}\n"
                    f"             AND {table.name}_teams_roles.table_id = {flat_variant.get_primary_keys()[0].name}) THEN\n"
                    "        RETURN TRUE;\n"
                    "    END IF;\n"
                )

        if is_role_table(table.name):
            for parent_column in parent_columns:
                foreign_key_flat_variant = table.get_foreign_key_flat_variant(
                    parent_column
                )

                if parent_column.optional:
                    up_index_migration.write(
                        f"-- If the parent column is not NULL, we call the can_{operation} function of the parent column to determine whether the user can {operation} the row.\n"
                        f"    IF {parent_column.name} IS NOT NULL THEN\n"
                    )

                up_index_migration.write(
                    f"        IF can_{operation}_{foreign_key_flat_variant.table_name}(author_user_id, {parent_column.name}) THEN\n"
                    "            RETURN TRUE;\n"
                    "        END IF;\n"
                )
                if parent_column.optional:
                    up_index_migration.write("    END IF;\n")
        else:
            for parent_column in parent_columns:
                foreign_key_flat_variant = table.get_foreign_key_flat_variant(
                    parent_column
                )

                if not foreign_key_flat_variant.may_be_hidden():
                    continue

                if parent_column.optional:
                    up_index_migration.write(
                        f"-- If the parent column is not NULL, we call the can_{operation} function of the parent column to determine whether the user can edit the row.\n"
                        f"    IF {parent_column.name} IS NOT NULL THEN\n"
                    )

                up_index_migration.write(
                    f"        IF NOT can_{operation}_{foreign_key_flat_variant.table_name}(author_user_id, {parent_column.name}) THEN\n"
                    "            RETURN FALSE;\n"
                    "        END IF;\n"
                )
                if parent_column.optional:
                    up_index_migration.write("    END IF;\n")

        if len(parent_columns) > 0 and not is_role_table(table.name):
            up_index_migration.write("    RETURN TRUE;\n")
        else:
            up_index_migration.write("    RETURN FALSE;\n")

        # Otherwise, we return FALSE.
        up_index_migration.write("END;\n" "$$\n" "LANGUAGE plpgsql;\n\n")

        if operation == "update":
            # We then implement the trigger function, which does not receive any arguments as trigger functions
            # cannot receive arguments. This method checks that the created_by and updated_by columns values are
            # indeed autorized to run this edit operation as defined by the can_update function of the parent column,
            # when it is not None. This method is only called when the row is being inserted or updated. Another method
            # is used to check whether the row can be deleted.
            if table.is_updatable():
                up_index_migration.write(
                    f"-- The function `{can_x_function}_trigger` is a trigger function that checks whether the user can {operation} the row.\n"
                    f"CREATE FUNCTION {can_x_function}_trigger()\n"
                    "RETURNS TRIGGER AS $$\n"
                    "BEGIN\n"
                )
                if table.has_column("updated_by"):
                    up_index_migration.write(
                        "    IF TG_OP = 'UPDATE' THEN\n"
                        f"        IF NOT {can_x_function}(NEW.updated_by, {flat_variant.get_formatted_primary_keys(include_prefix=True, prefix='NEW')}) THEN\n"
                        f"            RAISE EXCEPTION 'The user does not have the permission to {operation} this row.';\n"
                        "        END IF;\n"
                        "    END IF;\n"
                    )
                # Otherwise, if the operation is an insert and the parent column is not NULL, we check whether the user can edit
                # the parent column.
                for parent_column in parent_columns:
                    if parent_column.optional:
                        null_check = f" AND NEW.{parent_column.name} IS NOT NULL"
                    else:
                        null_check = ""
                    foreign_table = table.get_foreign_key_table_name(parent_column.name)

                    up_index_migration.write(
                        "-- We check whether the user can {operation} the row.\n"
                        f"    IF TG_OP = 'INSERT'{null_check} THEN\n"
                        f"        IF NOT can_{operation}_{foreign_table}(NEW.created_by, NEW.{parent_column.name}) THEN\n"
                        f"            RAISE EXCEPTION 'The user does not have the permission to {operation} this row.';\n"
                        "        END IF;\n"
                        "    END IF;\n"
                    )
                up_index_migration.write(
                    "    RETURN NEW;\n" "END;\n" "$$\n" "LANGUAGE plpgsql;\n\n"
                )

        # We then create the trigger that calls the trigger function.
        if operation == "update" and table.is_updatable():
            up_index_migration.write(
                f"-- We create a trigger that calls the `{can_x_function}` function before each INSERT or UPDATE.\n"
                f"CREATE TRIGGER {can_x_function}\n"
                f"BEFORE INSERT OR UPDATE ON {table.name}\n"
                f"FOR EACH ROW\n"
                f"EXECUTE FUNCTION {can_x_function}_trigger();\n"
            )

    up_index_migration.flush()
    up_index_migration.close()

    with open(
        f"./migrations/{full_migration_name}/down.sql", "w", encoding="utf8"
    ) as down_index_migration:
        for operation in operations:
            can_x_function = method_names[operation]

            if operation == "view" and not table.may_be_hidden():
                continue

            down_index_migration.write(
                f"-- Drop the `{can_x_function}` function and trigger on the {table.name} table.\n\n"
            )
            argument_types = ", ".join(
                ["INTEGER"]
                + [column.sql_data_type() for column in table.get_primary_keys()]
            )
            if operation == "update" and table.is_updatable():
                down_index_migration.write(
                    f"DROP TRIGGER {can_x_function} ON {table.name};\n"
                    f"DROP FUNCTION IF EXISTS {can_x_function}_trigger();\n"
                )
            down_index_migration.write(
                f"DROP FUNCTION IF EXISTS {can_x_function}({argument_types});\n"
            )

    raise RuntimeError(
        f"We have created the can_x function and trigger for the table {table.name}. "
        "Please run the migration to apply the changes."
    )


def ensure_can_x_function_existance(tables: List[TableStructMetadata]):
    """Returns whether the function exists."""
    assert isinstance(tables, list)
    assert all(isinstance(table, TableStructMetadata) for table in tables)

    for table in tqdm(
        tables,
        desc="Ensuring that the can_x functions exist",
        unit="table",
        leave=False,
    ):

        if not table.is_insertable() and not table.is_updatable() and not table.may_be_hidden():
            continue

        if not table.may_be_hidden() and table.has_can_view_function():

            visibility_message = ""

            if table.editability_always_depend_on_parent_column():
                editability_columns = table.get_editability_determinant_columns()
                visibility_message = (
                    "The columns that determine the editability of the table are "
                    f"{', '.join(column.name for column in editability_columns)}. "
                )

            raise RuntimeError(
                f"The table {table.name} is insertable or updatable, but it has a can_view function. "
                f"This function is not needed for tables that are not hidden.{visibility_message}"
            )

        if (
            table.is_insertable()
            and not table.has_can_admin_function()
            or table.is_updatable()
            and (not table.has_can_update_function() or not table.has_can_update_trigger())
            or table.may_be_hidden()
            and not table.has_can_view_function()
        ):
            handle_missing_can_x_function(table)
            raise RuntimeError(
                f"The table {table.name} has to be updated. Please update it and re-run the script."
            )
