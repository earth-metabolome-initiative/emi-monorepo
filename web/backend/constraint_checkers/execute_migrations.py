"""Generate the table schema from the database."""
import os
from constraint_checkers.migrations_changed import are_migrations_changed
from constraint_checkers.is_file_changed import is_file_changed

def execute_migrations():
    """Generate the table schema from the database."""
    if not (are_migrations_changed() or is_file_changed(__file__)):
        print("No migrations changed. Skipping the generation of the table schema.")
        return
    
    # We make sure the migrations were fully executed
    status = os.system("diesel migration run")

    if status != 0:
        raise RuntimeError("The migrations were not fully executed.")

    print("Executed the migrations.")