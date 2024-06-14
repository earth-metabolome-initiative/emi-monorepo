"""Submodule containing the enforcement of the migration naming convention."""
import os
from constraint_checkers.migrations_changed import are_migrations_changed
from constraint_checkers.is_file_changed import is_file_changed


def enforce_migration_naming_convention():
    """Check that the migrations are named according to the convention."""
    if not (are_migrations_changed() or is_file_changed(__file__)):
        print("Migrations have not changed. Skipping the check for the migration naming convention.")
        return

    for directory in os.listdir("migrations"):
        if not os.path.isdir(f"migrations/{directory}"):
            continue

        if not os.path.exists(f"migrations/{directory}/up.sql") or not os.path.exists(
            f"migrations/{directory}/down.sql"
        ):
            continue

        if directory == "00000000000000_diesel_initial_setup":
            continue

        with open(f"migrations/{directory}/up.sql", "r", encoding="utf8") as up_file:
            up_content = up_file.read()

        expected_name = directory

        if "CREATE TEMP TABLE" in up_content:
            raise RuntimeError(
                f"Migration {directory} contains a `CREATE TEMP TABLE` constraint. Please standardize the naming convention "
                "and replace it with `CREATE TEMPORARY TABLE`."
            )

        if "CREATE TABLE IF NOT EXISTS" in up_content:
            # This document contains a table creation, and as such
            # we expect its name to conform to {number_of_migration}_create_{table_name}_table
            table_name = (
                up_content.split("CREATE TABLE IF NOT EXISTS")[1].split("(")[0].strip()
            )
            number_of_migration = directory.split("_")[0]

            expected_name = f"{number_of_migration}_create_{table_name}_table"

        if directory != expected_name:
            raise RuntimeError(
                f"Migration {directory} does not conform to the naming convention. "
                f"Expected name: {expected_name}."
            )