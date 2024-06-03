"""Ensures that migrations are symmetric."""
import os
from constraint_checkers.migrations_changed import are_migrations_changed


def ensures_migrations_simmetry():
    """Ensures that migrations are symmetric."""
    # We check that, if in a migration directory up.sql contains a
    # certain string, down.sql contains the symmetric string.
    if not are_migrations_changed():
        print("Migrations have not changed. Skipping the check for the symmetry of migrations.")
        return

    opposites = {
        "CREATE TABLE": "DROP TABLE",
        "CREATE TABLE IF NOT EXISTS": "DROP TABLE IF EXISTS",
        "CREATE INDEX": "DROP INDEX",
        "CREATE VIEW": "DROP VIEW",
        # "CREATE FUNCTION": "DROP FUNCTION",
        "CREATE TRIGGER": "DROP TRIGGER",
        "CREATE TYPE": "DROP TYPE",
    }

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

        with open(
            f"migrations/{directory}/down.sql", "r", encoding="utf8"
        ) as down_file:
            down_content = down_file.read()

        for up_key, down_key in opposites.items():
            if up_key in up_content and down_key not in down_content:
                raise ValueError(
                    f"Migration {directory} is not symmetric: up.sql contains `{up_key}` but down.sql does not contain `{down_key}`."
                )
            if down_key in down_content and up_key not in up_content:
                raise ValueError(
                    f"Migration {directory} is not symmetric: down.sql contains `{down_key}` but up.sql does not contain `{up_key}`."
                )