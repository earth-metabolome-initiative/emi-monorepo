"""Insert a migration with the given name at the specified counter.

Implementative details
----------------------
This script will insert a migration with the given name at the specified counter. It will
first revert all of the migrations with a counter greater than or equal to the specified
counter, then increase the counter of all migrations with a counter greater than or equal
to the specified counter, and finally create a new migration with the specified name at the
specified counter.

The migrations must have names of the form "00000000000045_some_name", where the number is
the counter. The script will automatically pad the counter with zeroes to the appropriate
length.
"""

import os
import sys
import re
import shutil
import datetime

from increase_directory_counter import increase_directory_counter

def insert_migration(counter: int, name: str):
    """Insert a migration with the given name at the specified counter.
    
    Parameters
    ----------
    counter : int
        The counter at which to insert the migration.
    name : str
        The name of the migration.
    """
    # We check that the current working directory is the root of the backend
    # project, i.e. the current directory is called "backend" and there is a
    # "migrations" directory in it.
    if not os.path.isdir(os.path.join(os.getcwd(), "migrations")):
        print("The current directory is not the root of the backend project")
        sys.exit(1)

    # We check that the provided name is not empty
    if not name:
        print("The name of the migration cannot be empty")
        sys.exit(1)

    # If there is a directory with the name "__pycache__" in the migrations
    # directory, we remove it.
    if os.path.isdir(os.path.join(os.getcwd(), "migrations", "__pycache__")):
        shutil.rmtree(os.path.join(os.getcwd(), "migrations", "__pycache__"))

    # We check that all directories in the migrations directory have the up.sql
    # and the down.sql files.
    for directory in os.listdir(os.path.join(os.getcwd(), "migrations")):
        if os.path.isfile(os.path.join(os.getcwd(), "migrations", directory)):
            continue

        if not os.path.isfile(os.path.join(os.getcwd(), "migrations", directory, "up.sql")):
            print(f"The directory {directory} does not contain an up.sql file")
            sys.exit(1)

        if not os.path.isfile(os.path.join(os.getcwd(), "migrations", directory, "down.sql")):
            print(f"The directory {directory} does not contain a down.sql file")
            sys.exit(1)

    # We check that there is a directory with the name starting with
    # the padded version of the counter. If there is not, we raise an error.
    padded_counter = str(counter).zfill(14)

    if not any(
        directory.startswith(f"{padded_counter}_")
        for directory in os.listdir(os.path.join(os.getcwd(), "migrations"))
    ):
        print(f"No migration with counter {counter} exists")
        sys.exit(1)

    # We revert all migrations with a counter greater than or equal to the
    # specified counter using diesel.
    print("Reverting migrations")

    # We determine the number of migrations to revert, which are the number
    # of directories with a counter greater than or equal to the specified
    # counter.
    migrations_dir = os.path.join(os.getcwd(), "migrations")
    number_of_migrations_to_revert = sum(
        1
        for directory in os.listdir(migrations_dir)
        if re.match(r"\d{14}_", directory) and int(directory.split("_")[0]) >= counter
    )

    status = os.system(f"diesel migration --migration-dir migrations revert --number {number_of_migrations_to_revert}")

    if status != 0:
        print("Reverting migrations failed")
        sys.exit(1)

    # We increase the counter of all migrations with a counter greater than or
    # equal to the specified counter.
    print("Increasing migration counters")
    increase_directory_counter(counter)

    # We now create a new directory with the specified name at the specified
    # counter, and add therein the up.sql and down.sql files. In both files,
    # in order to make the migrations executable, we add a no-op SQL statement.
    print("Creating new migration")
    new_migration_dir = os.path.join(migrations_dir, f"{padded_counter}_{name}")
    os.mkdir(new_migration_dir)

    with open(os.path.join(new_migration_dir, "up.sql"), "w") as up_file:
        up_file.write("-- This is a no-op SQL statement\n")
        up_file.write("SELECT 1;")

    with open(os.path.join(new_migration_dir, "down.sql"), "w") as down_file:
        down_file.write("-- This is a no-op SQL statement\n")
        down_file.write("SELECT 1;")

    print("Done")

if __name__ == "__main__":
    if len(sys.argv) != 3:
        print("Usage: python insert_migration.py <counter> <name>")
        sys.exit(1)

    try:
        counter = int(sys.argv[1])
    except ValueError:
        print("The counter must be an integer")
        sys.exit(1)

    insert_migration(counter, sys.argv[2])