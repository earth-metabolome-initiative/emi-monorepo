"""Submodule providing functions to regroup tables.

Implementative details
----------------------
When adding new migrations to the diesel migrations, expecially when multiple
users are adding migrations at once, the migrations may start to result sparse
and harder to logically follow. This method ensures that the migrations are regrouped
in such a way that all the migrations relative to a table are grouped together.
"""
from typing import List
import os

WHITE_LISTED_MIGRATIONS = [
    "00000000000000_diesel_initial_setup",
    "00000000000001_enable_uuid_extension",
    "00000000000002_enable_pg_trgm_extension"
]

def get_desinences(table_name: str) -> List[str]:
    """Get the possible desinences of a table."""
    return [
        f"create_{table_name}_table",
        f"create_{table_name}_sequential_index",
        f"create_{table_name}_updated_at_trigger",
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

def regroup_tables():
    """Regroup the tables."""

    associated_tables = {}
    orphan_migrations = []
    mapped_migrations = []
    
    table_names = get_tables()

    for table_name in table_names:
        desinences = get_desinences(table_name)
        associated_tables[table_name] = []
        for migration in os.listdir("migrations"):
            if not os.path.isdir(f"migrations/{migration}"):
                continue

            if migration in WHITE_LISTED_MIGRATIONS:
                continue

            if migration in mapped_migrations:
                continue

            if table_name not in migration:
                continue

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

    starting_number = len(WHITE_LISTED_MIGRATIONS)

    for table_name in table_names:
        migrations = associated_tables[table_name]
        desinences = get_desinences(table_name)

        # We sort the migrations by the index of the desinence in the desinences list.
        migrations = sorted(migrations, key=lambda x: desinences.index(x.split("_", 1)[1]))

        for i, migration in enumerate(migrations):
            _number, desinence = migration.split("_", 1)
            padded_migration_number = str(i + starting_number).zfill(14)
            full_migration_name = f"{padded_migration_number}_{desinence}"
            if migration != full_migration_name:
                os.rename(f"migrations/{migration}", f"migrations/{full_migration_name}")
        
        starting_number += len(migrations)
