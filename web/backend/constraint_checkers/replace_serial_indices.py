"""Replaces SERIAL indices with INTEGER indices in the migrations."""
import os
from constraint_checkers.migrations_changed import are_migrations_changed
from insert_migration import insert_migration
from constraint_checkers.is_file_changed import is_file_changed

def replace_serial_indices():
    """Replaces SERIAL indices with INTEGER indices in the migrations.

    Implementation details
    ----------------------
    GlueSQL, the database engine used in the frontend, does not support several
    SQL features, including the SERIAL type. This function replaces the SERIAL
    type with the INTEGER type in the migrations, and creates a new migration
    inserted immediately after the one that contained the SERIAL type, to replace
    the integer primary key with a SERIAL primary key. By splitting the migration
    in two parts, we ensure that the first migration can be used as-is in the
    frontend as well, while the second migration can be used in the backend.
    """
    if not (are_migrations_changed() or is_file_changed(__file__)):
        print("Migrations have not changed. Skipping the replacement of SERIAL indices.")
        return

    serial_index_updated = True

    while serial_index_updated:
        serial_index_updated = False

        for directory in os.listdir("migrations"):
            if not os.path.isdir(f"migrations/{directory}"):
                continue

            if not os.path.exists(
                f"migrations/{directory}/up.sql"
            ) or not os.path.exists(f"migrations/{directory}/down.sql"):
                continue

            if directory == "00000000000000_diesel_initial_setup":
                continue

            with open(f"migrations/{directory}/up.sql", "r", encoding="utf8") as up_file:
                up_content = up_file.read()

            if "SERIAL PRIMARY KEY" in up_content:
                print(f"Replacing SERIAL in {directory} for GlueSQL compliance.")

                serial_index_updated = True
                current_migration_code_str, directory_name = directory.split(
                    "_", maxsplit=1
                )

                assert directory_name.startswith("create_") and directory_name.endswith(
                    "_table"
                )

                table_name = directory_name[7:-6]

                current_migration_code = int(current_migration_code_str)

                new_migration_name = f"create_{table_name}_default_index"

                insert_migration(
                    counter=current_migration_code + 1, name=new_migration_name
                )

                # We replace the SERIAL PRIMARY KEY with INTEGER PRIMARY KEY
                up_content = up_content.replace(
                    "SERIAL PRIMARY KEY", "INTEGER PRIMARY KEY"
                )

                with open(f"migrations/{directory}/up.sql", "w", encoding="utf8") as up_file:
                    up_file.write(up_content)

                padded_migration_code = str(current_migration_code + 1).zfill(14)
                full_new_migration_name = (
                    f"{padded_migration_code}_{new_migration_name}"
                )

                # We write the up and down SQL files for the new migration
                # that will replace the INTEGER PRIMARY KEY with a SERIAL
                # PRIMARY KEY.

                with open(
                    f"migrations/{full_new_migration_name}/up.sql", "w"
                ) as up_file:
                    up_file.write(
                        "-- This up migration replaces the INTEGER primary key with a SERIAL primary key.\n"
                        "-- This migration is intended to be used in the backend.\n"
                        "-- This migration is automatically generated.\n\n"
                        f"CREATE SEQUENCE {table_name}_id_seq;\n"
                        f"ALTER TABLE {table_name} ALTER COLUMN id SET DEFAULT nextval('{table_name}_id_seq');\n"
                        f"ALTER TABLE {table_name} ALTER COLUMN id SET NOT NULL;\n"
                        f"ALTER SEQUENCE {table_name}_id_seq OWNED BY {table_name}.id;\n"
                    )

                with open(
                    f"migrations/{full_new_migration_name}/down.sql", "w"
                ) as down_file:
                    down_file.write(
                        "-- This down migration drops what was created in the up migration.\n"
                        "-- This migration is intended to be used in the backend.\n"
                        "-- This migration is automatically generated.\n\n"
                        f"ALTER SEQUENCE {table_name}_id_seq OWNED BY NONE;\n"
                        f"ALTER TABLE {table_name} ALTER COLUMN id DROP DEFAULT;\n"
                        f"DROP SEQUENCE {table_name}_id_seq;\n"
                    )

                # We break as the list of directories has now changed
                break
