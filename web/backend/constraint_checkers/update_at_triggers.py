"""This module contains the function that ensures that all tables that have an `updated_at` column
have a trigger that updates the column upon each row update.
"""
import os
from constraint_checkers.find_foreign_keys import TableMetadata
from userinput import userinput
from insert_migration import insert_migration

def handle_update_at_trigger_creation(
    table_name: str,
):
    trigger_name = f"{table_name}_updated_at_trigger"
    print(
        f"The table {table_name} has an `updated_at` column, but it does not have an `{trigger_name}` trigger. "
        "We can create the trigger for you."
    )
    proceed = userinput(
        name="Proceed with the creation of the trigger?",
        default="no",
        validator="human_bool",
        sanitizer="human_bool",
    )

    if proceed:
        # First, we identify the position of the migration that has created the current
        # table by finding the one with desinence `_create_{table_name}_table`.
        migrations = [
            directory
            for directory in os.listdir("migrations")
            if os.path.isdir(f"migrations/{directory}")
            and os.path.exists(f"migrations/{directory}/up.sql")
        ]

        migration_number = None

        for migration in migrations:
            number, desinence = migration.split("_", maxsplit=1)
            if desinence == f"create_{table_name}_table":
                migration_number = number
                break

        if migration_number is None:
            raise Exception(
                f"Could not find the migration that created the {table_name} table."
            )

        # We create the trigger migration.
        trigger_migration_name = f"create_{trigger_name}"
        migration_number = int(migration_number) + 1
        padded_migration_number = str(migration_number).zfill(14)
        full_migration_name = f"{padded_migration_number}_{trigger_migration_name}"

        insert_migration(counter=migration_number, name=trigger_migration_name)

        with open(
            f"./migrations/{full_migration_name}/up.sql", "w", encoding="utf8"
        ) as up_index_migration:
            up_index_migration.write(
                f"-- Create the `{trigger_name}` trigger on the {table_name} table.\n\n"
                f"CREATE OR REPLACE FUNCTION {trigger_name}()\n"
                "RETURNS TRIGGER AS $$\n"
                "BEGIN\n"
                "    NEW.updated_at = NOW();\n"
                "    RETURN NEW;\n"
                "END;\n"
                "$$ LANGUAGE plpgsql;\n\n"
                f"CREATE TRIGGER {trigger_name}\n"
                f"BEFORE UPDATE ON {table_name}\n"
                f"FOR EACH ROW\n"
                f"EXECUTE FUNCTION {trigger_name}();\n"
            )

        with open(
            f"./migrations/{full_migration_name}/down.sql", "w", encoding="utf8"
        ) as down_index_migration:
            down_index_migration.write(
                f"-- Drop the `{trigger_name}` trigger on the {table_name} table.\n\n"
                f"DROP TRIGGER {trigger_name} ON {table_name};\n"
                f"DROP FUNCTION {trigger_name};\n"
            )

        print(
            f"Created the `{trigger_name}` trigger for the {table_name} table in the {full_migration_name} migration."
        )


def ensures_all_update_at_trigger_exists(
    tables_metadata: TableMetadata
):
    """Check that for all tables that have an updated_at column, there exists an update_at trigger.

    Implementation details
    ----------------------
    While in other database engine there are ways to specify that a column needs to be updated
    upon each row update, in PostgreSQL, this is not possible. For this reason, we need to create
    a trigger that updates the updated_at column upon each row update. This function checks that
    for each table that has an updated_at column, there exists a trigger that updates the column
    upon each row update. If it does not, the function guides the user to create the trigger and
    afterwards raises an exception to stop the pipeline as it will need to be rerun.
    """
    for table_name in tables_metadata.tables():
        trigger_name = f"{table_name}_updated_at_trigger"
        if tables_metadata.has_updated_at_column(table_name):
            if not tables_metadata.has_updated_at_trigger(table_name):
                handle_update_at_trigger_creation(table_name)
                raise Exception(
                    f"The table {table_name} has an `updated_at` column, but it does not have an `{trigger_name}` trigger. "
                    "Please create the trigger and rerun the pipeline."
                )
        else:
            # If the table does not have an updated_at column, we check
            # that it DOES NOT have the trigger. If it does, this is most
            # likely an error.
            if tables_metadata.has_updated_at_trigger(table_name):
                raise Exception(
                    f"The table {table_name} does not have an `updated_at` column, but it has an `{trigger_name}` trigger. "
                    "Please remove the trigger and rerun the pipeline."
                )
