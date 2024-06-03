"""Generate the table schema from the database."""
import os
from constraint_checkers.migrations_changed import are_migrations_changed

def generate_table_schema():
    """Generate the table schema from the database."""
    if not are_migrations_changed():
        print("Migrations have not changed. Skipping the generation of the table schema.")
        return

    # We make sure the migrations were fully executed
    status = os.system("diesel migration run")

    if status != 0:
        raise RuntimeError("The migrations were not fully executed.")

    # We run the diesel extended CLI command
    status = os.system("diesel_ext --model --add-table-name > src/models.rs")

    if status != 0:
        raise RuntimeError("The diesel_ext command failed.")