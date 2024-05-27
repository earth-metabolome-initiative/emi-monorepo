"""Submodule checking for the existance of the parent circularity trigger in tables containing parent-child relationships."""
from tqdm.auto import tqdm
from constraint_checkers.find_foreign_keys import TableMetadata
from insert_migration import insert_migration
from constraint_checkers.regroup_tables import get_best_insertion_point
from userinput import userinput

def handle_missing_parent_circularity_trigger(
    table_name: str,
    table_metadata: TableMetadata,
):
    """Handle the missing parent circularity trigger in tables containing parent-child relationships."""

    trigger_name = f"{table_name}_parent_circularity_trigger"
    print(
        f"The table {table_name} has a parent-child relationship, but it does not have a `{trigger_name}` trigger. "
        "We can create the trigger for you."
    )
    proceed = userinput(
        name="Proceed with the creation of the trigger?",
        default="no",
        validator="human_bool",
        sanitizer="human_bool",
    )

    if proceed:
        migration_trigger_name = f"create_{trigger_name}"
        
        migration_number = get_best_insertion_point(
            table_name=table_name,
            expected_desinence=migration_trigger_name,
        )

        # We create the trigger migration.
        insert_migration(
            counter=migration_number,
            name=migration_trigger_name,
        )

        full_migration_name = f"{str(migration_number).zfill(14)}_{migration_trigger_name}"

        circular_foreign_keys = table_metadata.get_circular_foreign_keys(table_name)
        primary_keys = table_metadata.get_primary_key_names_and_types(table_name)

        assert len(primary_keys) == 1, (
            f"Table {table_name} has more than one primary key and is likely a junktion table."
        )
        
        primary_key_name = primary_keys[0][0]

        with open(
            f"./migrations/{full_migration_name}/up.sql",
            "w",
            encoding="utf8",
        ) as up_migration:
            up_migration.write(
                f"-- Create the `{trigger_name}` trigger on the {table_name} table.\n\n"
                f"CREATE FUNCTION {trigger_name}()\n"
                "RETURNS TRIGGER AS $$\n"
                "BEGIN\n"
            )
            for foreign_key in circular_foreign_keys:
                up_migration.write(
                    f"    IF NEW.{foreign_key} = OLD.{primary_key_name} THEN\n"
                    "        RAISE EXCEPTION 'Circular reference detected.';\n"
                    "    END IF;\n"
                )
            up_migration.write(
                "    RETURN NEW;\n"
                "END;\n"
                "$$ LANGUAGE plpgsql;\n\n"
                f"CREATE TRIGGER {trigger_name}\n"
                f"BEFORE UPDATE ON {table_name}\n"
                f"FOR EACH ROW\n"
                f"EXECUTE FUNCTION {trigger_name}();\n"
            )

        with open(
            f"./migrations/{full_migration_name}/down.sql",
            "w",
            encoding="utf8",
        ) as down_migration:
            down_migration.write(
                f"-- Drop the `{trigger_name}` trigger on the {table_name} table.\n\n"
                f"DROP TRIGGER {trigger_name} ON {table_name};\n"
                f"DROP FUNCTION {trigger_name};\n"
            )

        print(
            f"Created the `{trigger_name}` trigger for the {table_name} table in the {full_migration_name} migration."
        )            

    raise NotImplementedError(
        f"Parent circularity trigger is missing in table {table_name}."
    )


def check_parent_circularity_trigger(tables_metadata: TableMetadata):
    """Check if the parent circularity trigger exists in tables containing parent-child relationships."""
    for table_name in tqdm(
        tables_metadata.tables(),
        desc="Checking parent circularity trigger",
        unit="table",
        leave=False,
    ):
        if tables_metadata.has_circular_parent_column(table_name):
            if not tables_metadata.has_parent_circularity_trigger(table_name):
                handle_missing_parent_circularity_trigger(
                    table_name=table_name,
                    table_metadata=tables_metadata,
                )