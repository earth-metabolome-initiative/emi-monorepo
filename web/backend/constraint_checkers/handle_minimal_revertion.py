"""Keeps track with hashes of changes of migrations and reverts to the last edited migration if needed
so to avoid reverting every time all of the migrations."""

import os
import sys
from dict_hash import sha256
import compress_json
from constraint_checkers.migrations_changed import are_migrations_changed
from constraint_checkers.is_file_changed import is_file_changed


def hash_migration(migration: str) -> str:
    """Hashes the migration."""
    migration_metadata = {}
    migration_metadata["migration"] = migration
    with open(f"migrations/{migration}/up.sql", "r", encoding="utf-8") as file:
        migration_metadata["up"] = file.read()
    with open(f"migrations/{migration}/down.sql", "r", encoding="utf-8") as file:
        migration_metadata["down"] = file.read()
    return sha256(migration_metadata)


def handle_minimal_revertion():
    """Keeps track with hashes of changes of migrations and reverts to the last edited migration if needed
    so to avoid reverting every time all of the migrations."""
    if not (are_migrations_changed() or is_file_changed(__file__)):
        print("Migrations have not changed. Minimal revertion is not needed.")
        return

    try:
        # We try to load the migration hashes.
        stored_migration_hashes = compress_json.load("migration_hashes.json")
    except FileNotFoundError:
        # If the file does not exist, we create it.
        stored_migration_hashes = []

    # We get the list of all the migrations.
    migrations = os.listdir("migrations")

    # We sort the migrations by their number.
    migrations = sorted(migrations)

    updated_hashes = []

    first_divergent_migration = None

    number_of_migrations_to_revert = 0

    # We compute the migration for each migration, and
    # when we find a migration that does not appear in the
    # migration hashes, we revert to the last migration.
    for migration in migrations:
        if not os.path.isdir(f"migrations/{migration}"):
            continue

        migration_hash = hash_migration(migration)
        updated_hashes.append(migration_hash)
        if (
            migration_hash not in stored_migration_hashes
            and first_divergent_migration is None
        ):
            # The migration is not in the hashes, so we
            # need to revert up until this migration.
            first_divergent_migration = migration

        if first_divergent_migration is not None:
            number_of_migrations_to_revert += 1

    compress_json.dump(updated_hashes, "migration_hashes.json")

    if first_divergent_migration is not None:
        status = os.system(
            f"diesel migration --migration-dir migrations revert --number {number_of_migrations_to_revert}"
        )

        if status != 0:
            print("Reverting migrations failed")
            sys.exit(1)
