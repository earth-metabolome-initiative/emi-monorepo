"""Submodule containing the enforcement of the migration naming convention."""
import os



def enforce_migration_naming_convention():
    """Check that the migrations are named according to the convention."""

    # We check that if a migration directory contains a population of a given table,
    # we verify that if there is also another migration that creates a search index
    # as indicated by the suffix `_index`, the migration that populates the table must
    # have a lower number than the migration that creates the search index.
    migrations = [
        directory
        for directory in os.listdir("migrations")
        if os.path.isdir(f"migrations/{directory}")
        and os.path.exists(f"migrations/{directory}/up.sql")
    ]

    # We check that all directories are named in one of the following patterns:
    # {number_of_migration}_create_{table_name}_table
    # {number_of_migration}_create_{table_name}_view
    # {number_of_migration}_populate_{table_name}_table
    # {number_of_migration}_create_{table_name}_gin_index
    # {number_of_migration}_create_{table_name}_sequential_index
    # {number_of_migration}_create_{table_name}_notification_trigger
    # {number_of_migration}_create_{table_name}_updated_at_trigger
    # {number_of_migration}_enable_{extension_name}_extension

    # We iterate over the migrations and check that the naming convention is respected.
    for migration in migrations:
        if migration == "00000000000000_diesel_initial_setup":
            continue

        migration_number, migration_name = migration.split("_", maxsplit=1)

        if not any(
            [
                migration_name.startswith("create_")
                and migration_name.endswith("_table"),
                migration_name.startswith("create_")
                and migration_name.endswith("_view"),
                migration_name.startswith("populate_")
                and migration_name.endswith("_table"),
                migration_name.startswith("create_")
                and migration_name.endswith("_gin_index"),
                migration_name.startswith("create_")
                and migration_name.endswith("_sequential_index"),
                migration_name.startswith("create_")
                and migration_name.endswith("_notification_trigger"),
                migration_name.startswith("create_")
                and migration_name.endswith("_updated_at_trigger"),
                migration_name.startswith("enable_")
                and migration_name.endswith("_extension"),
            ]
        ):
            raise Exception(
                f"Migration {migration} does not conform to the naming convention. "
                "Please rename it to match the naming convention."
            )

    for migration in migrations:
        str_number, migration_name = migration.split("_", maxsplit=1)
        number = int(str_number)
        if migration_name.startswith("populate_") and migration_name.endswith("_table"):
            trimmed_migration_name = migration_name[9:-6]
            search_index_migration_name = f"create_{trimmed_migration_name}_gin_index"

            # We search if there exist a migration with a name ending with the
            # migration name we just determined.

            for other_migration in migrations:
                if other_migration.endswith(search_index_migration_name):
                    other_str_number, other_migration_name = other_migration.split(
                        "_", maxsplit=1
                    )
                    other_number = int(other_str_number)

                    if other_number < number:
                        raise Exception(
                            f"Migration {migration} populates a table, "
                            f"but there exists a migration {other_migration} that creates "
                            "a search index for the same table, and it has a lower number "
                            "than the migration that populates the table."
                        )

    for directory in os.listdir("migrations"):
        if not os.path.isdir(f"migrations/{directory}"):
            continue

        if not os.path.exists(f"migrations/{directory}/up.sql") or not os.path.exists(
            f"migrations/{directory}/down.sql"
        ):
            continue

        if directory == "00000000000000_diesel_initial_setup":
            continue

        with open(f"migrations/{directory}/up.sql", "r") as up_file:
            up_content = up_file.read()

        expected_name = directory

        if "CREATE TEMP TABLE" in up_content:
            raise Exception(
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
            raise Exception(
                f"Migration {directory} does not conform to the naming convention. "
                f"Expected name: {expected_name}."
            )