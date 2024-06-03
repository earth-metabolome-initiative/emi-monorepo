"""Ensures that there are not migrations that are equal one another by quadratically comparing them."""

from typing import List, Tuple
import shutil
import os
from userinput import userinput
from constraint_checkers.migrations_changed import are_migrations_changed


def ensures_no_duplicated_migrations():
    """Ensures that there are not migrations that are equal one another by quadratically comparing them."""
    if not are_migrations_changed():
        print("Migrations have not changed. Skipping the check for duplicated migrations.")
        return
    # List containing the names of the migration files and their content
    migration_files: List[Tuple[str, str]] = []

    for migration in os.listdir("migrations"):
        for variant in ("up", "down"):
            path = f"migrations/{migration}/{variant}.sql"
            if os.path.exists(path):
                with open(path, "r", encoding="utf8") as f:
                    migration_files.append((migration, f.read()))

    # Check for duplicated migrations
    for i, (migration1, content1) in enumerate(migration_files):
        for j, (migration2, content2) in enumerate(migration_files):
            if i >= j:
                continue

            if not os.path.exists(f"migrations/{migration1}") or not os.path.exists(
                f"migrations/{migration2}"
            ):
                continue

            desinence1 = migration1.split("_", maxsplit=1)[-1]
            desinence2 = migration2.split("_", maxsplit=1)[-1]

            if desinence1 == desinence2 and migration1 != migration2:
                shutil.rmtree(f"migrations/{migration1}")
                print(f"Deleted duplicated {migration1}.")
                continue

            if content1 == content2:
                print(
                    f"Warning: The migrations {migration1} and {migration2} are equal to one another."
                )
                should_delete = userinput(
                    name=f"delete_{migration1}",
                    label=f"Delete {migration1}?",
                    default="no",
                    sanitizer="human_bool",
                    validator="human_bool",
                )

                if should_delete:
                    shutil.rmtree(f"migrations/{migration1}")
                    print(f"Deleted {migration1}.")
                else:
                    raise RuntimeError(
                        f"The migrations {migration1} and {migration2} are equal to one another."
                    )
