"""Ensures that the migrations are GlueSQL compliant."""

import os
from constraint_checkers.migrations_changed import are_migrations_changed

def ensures_gluesql_compliance():
    """Ensures that the migrations are GlueSQL compliant."""
    if not are_migrations_changed():
        print("Migrations have not changed. Skipping the check for GlueSQL compliance.")
        return

    for directory in os.listdir("migrations"):
        if not os.path.isdir(f"migrations/{directory}"):
            continue

        if not os.path.exists(f"migrations/{directory}/up.sql") or not os.path.exists(
            f"migrations/{directory}/down.sql"
        ):
            continue

        with open(f"migrations/{directory}/up.sql", "r", encoding="utf8") as up_file:
            up_content = up_file.read()

        if "SERIAL PRIMARY KEY" in up_content:
            raise RuntimeError(
                f"Migration `{directory}` contains a `SERIAL PRIMARY KEY` constraint, which is not supported by GlueSQL. "
                "Please replace it with `INTEGER PRIMARY KEY`."
            )

        if up_content.count("CREATE TABLE") != up_content.count(
            "CREATE TABLE IF NOT EXISTS"
        ):
            raise RuntimeError(
                f"Migration `{directory}` does not use `CREATE TABLE IF NOT EXISTS` consistently. "
                f"Replace the use of `CREATE TABLE` with `CREATE TABLE IF NOT EXISTS` in the `up.sql` file "
                "so to avoid conflicts when running the migrations within GlueSQL."
            )
