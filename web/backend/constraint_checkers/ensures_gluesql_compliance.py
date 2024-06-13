"""Ensures that the migrations are GlueSQL compliant."""

import os

from constraint_checkers.is_file_changed import is_file_changed
from constraint_checkers.migrations_changed import are_migrations_changed
from constraint_checkers.regroup_tables import (
    get_create_table_desinence,
    is_create_table_migration,
)

TABLES_IGNORED_FOR_GLUESQL = ["user_emails"]


def check_absence_of_serial_primary_key_constraint(
    migration: str, up_content: str
) -> None:
    """Check the absence of a `SERIAL PRIMARY KEY` constraint in a migration.

    Parameters
    ----------
    migration : str
        The name of the migration.
    up_content : str
        The content of the `up.sql` file of the migration.
    """
    if "SERIAL PRIMARY KEY" in up_content:
        raise RuntimeError(
            f"Migration `{migration}` contains a `SERIAL PRIMARY KEY` constraint, which is not supported by GlueSQL. "
            "Please replace it with `INTEGER PRIMARY KEY`."
        )


def check_foreign_key_constraints(migration: str, up_content: str) -> None:
    """Check the foreign key constraints in a migration.

    Parameters
    ----------
    migration : str
        The name of the migration.
    up_content : str
        The content of the `up.sql` file of the migration.
    """
    up_content = up_content.lower()

    new_up_content = ""
    # First, we remove all commented lines
    for line in up_content.split("\n"):
        line = line.strip()
        line = line.split("--")[0]
        new_up_content += line + "\n"

    up_content = new_up_content

    for line in up_content.split(","):
        if "references" in line and not "foreign key" in line:
            raise RuntimeError(
                f"Migration `{migration}` at line :\n{line}\n\nContains a reference to another table, but it is not written as 'FOREIGN KEY'.\n"
                "For example the line `domain_id INTEGER REFERENCES bio_ott_taxon_items(id) ON DELETE CASCADE,`\nshould be writtent as :\n"
                "domain_id INTEGER, FOREIGN KEY (domain_id) REFERENCES bio_ott_taxon_items(id) ON DELETE CASCADE \n"
                "This is required by GlueSQL according to this : "
                "https://docs.rs/gluesql/0.15.0/gluesql/core/sqlparser/ast/enum.TableConstraint.html#variant.ForeignKey.field.referred_columns"
            )


def check_absence_of_current_timestamp(migration: str, up_content: str):
    """Check the absence of a `CURRENT_TIMESTAMP` value in a migration.

    Parameters
    ----------
    migration : str
        The name of the migration.
    up_content : str
        The content of the `up.sql` file of the migration.
    """

    up_content = up_content.lower()
    if "current_timestamp" in up_content:
        raise RuntimeError(
            f"Migration `{migration}` contains a `CURRENT_TIMESTAMP` default value, which is not supported by GlueSQL. Please change it to `NOW()`."
        )


def ensures_gluesql_compliance():
    """Ensures that the migrations are GlueSQL compliant."""
    if not (are_migrations_changed() or is_file_changed(__file__)):
        print("Migrations have not changed. Skipping the check for GlueSQL compliance.")
        return

    for migration in sorted(os.listdir("migrations")):
        if not os.path.isdir(f"migrations/{migration}"):
            continue
        if not is_create_table_migration(migration):
            continue

        if any(
            migration.endswith(get_create_table_desinence(create_table))
            for create_table in TABLES_IGNORED_FOR_GLUESQL
        ):
            continue

        with open(f"migrations/{migration}/up.sql", "r", encoding="utf8") as up_file:
            up_content = up_file.read()

        check_absence_of_serial_primary_key_constraint(migration, up_content)
        check_foreign_key_constraints(migration, up_content)
        check_absence_of_current_timestamp(migration, up_content)
