"""Submodule providing functions to regroup tables.

Implementative details
----------------------
When adding new migrations to the diesel migrations, expecially when multiple
users are adding migrations at once, the migrations may start to result sparse
and harder to logically follow. This method ensures that the migrations are regrouped
in such a way that all the migrations relative to a table are grouped together.
"""

import os
from typing import Dict, List
import shutil
from jaro import jaro_winkler_metric
from userinput import userinput
from userinput.utils import set_validator

ALLOW_LISTED_MIGRATIONS = [
    "00000000000000_diesel_initial_setup",
    "00000000000001_enable_uuid_extension",
    "00000000000002_enable_pg_trgm_extension",
]

ALLOW_LISTED_SINGLETONS = [
    "units"
]

SUPPORT_TABLE_NAMES = [
    "units",
    "user_emails",
    "colors",
    "font_awesome_icons",
    "document_formats",
    "login_providers",
    "materials",
    "notifications",
    "team_states",
    "project_states",
    "sample_container_categories",
    "nameplate_categories",
    "permanence_categories"
]

def get_best_insertion_point(table_name: str, expected_desinence: str) -> int:
    """Get the best insertion point for a new migration related to a table.

    Parameters
    ----------
    table_name : str
        The name of the table for which we want to find the best insertion point.

    Returns
    -------
    int
        The best insertion point for a new migration related to the table.

    Raises
    ------
    Exception
        If the migration that created the table cannot be found.
    """

    assert (
        table_name in expected_desinence
    ), f"Table name {table_name} is not in the expected desinence {expected_desinence}"

    valid_desinences = get_desinences(table_name)

    assert (
        expected_desinence in valid_desinences
    ), f"Desinence {expected_desinence} is not in the desinences of the table {table_name}"

    index_in_desinences = valid_desinences.index(expected_desinence)

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
        if desinence == expected_desinence:
            raise RuntimeError(f"Migration {migration} already exists.")
        if (
            desinence in valid_desinences
            and valid_desinences.index(desinence) < index_in_desinences
        ):
            migration_number = number

    if migration_number is None:
        raise RuntimeError(
            f"Could not find the migration that created the {table_name} table."
        )

    return int(migration_number) + 1


def get_desinences(table_name: str) -> List[str]:
    """Get the possible desinences of a table."""
    return [
        f"create_{table_name}_table",
        f"create_{table_name}_sequential_index",
        f"create_{table_name}_updated_at_trigger",
        f"create_{table_name}_parent_circularity_trigger",
        f"create_{table_name}_can_x_trigger",
        f"populate_{table_name}_table",
        f"create_{table_name}_gin_index",
    ]


def get_tables() -> List[str]:
    """Extracts the table names from the migrations.

    Implementative details
    ----------------------
    The table names are extracted from the migrations by searching for the
    string "CREATE TABLE IF NOT EXISTS {table_name} (" in the up migrations.
    """
    tables = []

    target = "CREATE TABLE IF NOT EXISTS"

    migrations = os.listdir("migrations")
    migrations = sorted(migrations)

    for migration in migrations:
        if not os.path.isdir(f"migrations/{migration}"):
            continue
        with open(f"migrations/{migration}/up.sql", "r", encoding="utf-8") as f:
            lines = f.readlines()
            for line in lines:
                if target in line:
                    table_name = line.split(target)[1].split("(")[0].strip()
                    tables.append(table_name)
    return tables


def table_dependencies() -> Dict[str, List[str]]:
    """Return the table dependencies."""

    tables = get_tables()
    dependencies = {table: [] for table in tables}

    for migration in os.listdir("migrations"):
        if not os.path.isdir(f"migrations/{migration}"):
            continue

        if migration in ALLOW_LISTED_MIGRATIONS:
            continue

        _, migration_desinence = migration.split("_", 1)
        current_table = None

        # We identify the table associated to this migration.
        for table in tables:
            desinences = get_desinences(table)
            if migration_desinence in desinences:
                current_table = table
                break

        if current_table is None:

            matches_and_scores = [
                (desinence, jaro_winkler_metric(migration_desinence, desinence))
                for table in tables
                for desinence in get_desinences(table)
            ]

            sorted_matches_and_scores = sorted(
                matches_and_scores, key=lambda x: x[1], reverse=True
            )

            print(
                f"Could not find the table associated to the migration {migration}. "
                "Maybe it is not a valid migration name or the table name is misspelled. "
                "Here are the closest matches:"
            )
            for number, (match, score) in enumerate(sorted_matches_and_scores[:10]):
                print(f"{number}) {match} - {score}")

            should_delete = userinput(
                name=f"delete_migration_{migration}",
                label="Do you want to delete this migration?",
                default="no",
                validator="human_bool",
                sanitizer="human_bool",
            )

            if should_delete:
                # First, we rollback the migrations
                status = os.system("diesel migration revert --all")
                if status != 0:
                    raise RuntimeError("Could not revert the migrations.")
                shutil.rmtree(f"migrations/{migration}")
                raise RuntimeError("Migration deleted. Please re-run the script.")
            
            should_rename = userinput(
                name=f"should_rename_migration_{migration}",
                label="Do you want to rename this migration?",
                validator="human_bool",
                sanitizer="human_bool",
            )

            if should_rename:
                renaming_target = userinput(
                    name=f"rename_migration_{migration}",
                    label="Which of the top 10 do you want to use?",
                    validator=set_validator([str(i) for i in range(10)]),
                    sanitizer="integer",
                )

                assert isinstance(renaming_target, int)
                assert renaming_target < 10

                new_migration_name = sorted_matches_and_scores[renaming_target][0]
                new_migration_name = f"{migration.split('_')[0]}_{new_migration_name}"

                os.rename(f"migrations/{migration}", f"migrations/{new_migration_name}")

                raise RuntimeError(
                    f"Migration renamed to {new_migration_name}. Please re-run the script."
                )

            raise RuntimeError("Migration not associated to any table.")
            
        # We identify the tables that are being referenced in the migration.
        with open(f"migrations/{migration}/up.sql", "r", encoding="utf-8") as f:
            for line in f.readlines():
                if line.startswith("--"):
                    continue
                for table in tables:
                    needles = (f" {table}(", f" {table} (")
                    if (
                        any(needle in line for needle in needles)
                        and table != current_table
                    ):
                        if table not in dependencies[current_table]:
                            dependencies[current_table].append(table)

    return dependencies


def get_sort_tables_by_dependencies() -> List[str]:
    """Returns list of tables sorted by dependencies.

    Implementative details
    ----------------------
    The tables are sorted by dependencies using a topological sort algorithm.
    """
    dependencies = table_dependencies()
    tables = list(dependencies.keys())
    sorted_tables = []

    while len(tables) > 0:
        for table in tables:
            if len(dependencies[table]) == 0:
                sorted_tables.append(table)
                tables.remove(table)
                for other_table in tables:
                    if table in dependencies[other_table]:
                        dependencies[other_table].remove(table)
                break
        else:
            raise RuntimeError(f"Circular dependency detected in the tables {tables}")

    return sorted_tables


def detect_singleton_tables():
    """Detect singleton tables.

    Implementative details
    ----------------------
    A singleton table is a table that is not referenced by any other table.
    """
    dependencies = table_dependencies()

    # If a table does not have neither dependencies nor
    # it appears in the dependencies of other tables, it is a singleton table.
    singleton_tables = []
    for table, deps in dependencies.items():
        if len(deps) == 0 and not any(table in dep for dep in dependencies.values()):
            if table not in ALLOW_LISTED_SINGLETONS:
                singleton_tables.append(table)
    
    if len(singleton_tables) > 0:
        raise RuntimeError(
            f"We found {len(singleton_tables)} singleton tables. "
            "Please either add them to the allow list or remove them. "
            "A sigleton table is a table that is not referenced by any other table nor "
            "it references any other table. The following tables are singleton tables: "
            f"{singleton_tables}"
        )


def regroup_tables():
    """Regroup the tables."""

    detect_singleton_tables()

    associated_tables = {}
    orphan_migrations = []
    mapped_migrations = []

    table_names = get_sort_tables_by_dependencies()

    for table_name in table_names:
        desinences = get_desinences(table_name)
        associated_tables[table_name] = []
        for migration in os.listdir("migrations"):
            if not os.path.isdir(f"migrations/{migration}"):
                continue

            if migration in ALLOW_LISTED_MIGRATIONS:
                continue

            if migration in mapped_migrations:
                continue

            if table_name not in migration:
                continue

            # We check if the directory contains an up.sql and a down.sql file.
            # If not, we ask the user what to do with it.
            if not os.path.exists(f"migrations/{migration}/up.sql") or not os.path.exists(
                f"migrations/{migration}/down.sql"
            ):
                print(
                    f"Migration {migration} does not contain both up.sql and down.sql files."
                )

                should_delete = userinput(
                    name=f"delete_migration_{migration}",
                    label="Do you want to delete this migration?",
                    default="no",
                    validator="human_bool",
                    sanitizer="human_bool",
                )

                if should_delete:
                    # First, we rollback the migrations
                    status = os.system("diesel migration revert --all")
                    if status != 0:
                        raise RuntimeError("Could not revert the migrations.")
                    shutil.rmtree(f"migrations/{migration}")
                    continue

                raise RuntimeError(
                    f"Migration {migration} does not contain both up.sql and down.sql files."
                )

            _number, desinence = migration.split("_", 1)

            if desinence in desinences:
                associated_tables[table_name].append(migration)
                mapped_migrations.append(migration)
                if migration in orphan_migrations:
                    orphan_migrations.remove(migration)
                continue

            if migration not in orphan_migrations:
                orphan_migrations.append(migration)

    if len(orphan_migrations) > 0:
        raise RuntimeError(f"Orphaned migrations found {orphan_migrations}")

    starting_number = len(ALLOW_LISTED_MIGRATIONS)

    for table_name in table_names:
        migrations = associated_tables[table_name]
        desinences = get_desinences(table_name)

        # We sort the migrations by the index of the desinence in the desinences list.
        migrations = sorted(
            migrations, key=lambda x: desinences.index(x.split("_", 1)[1])
        )

        for i, migration in enumerate(migrations):
            _number, desinence = migration.split("_", 1)
            padded_migration_number = str(i + starting_number).zfill(14)
            full_migration_name = f"{padded_migration_number}_{desinence}"
            if migration != full_migration_name:
                os.rename(
                    f"migrations/{migration}", f"migrations/{full_migration_name}"
                )

        starting_number += len(migrations)
