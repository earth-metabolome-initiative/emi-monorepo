"""Submodule to ensure that all tables have a creation notification trigger."""

from typing import List
from constraint_checkers.table_metadata import TableStructMetadata
from constraint_checkers.find_foreign_keys import TableMetadata


def ensure_tables_have_creation_notification_trigger(
    tables: List[TableStructMetadata], table_metadata: TableMetadata
):
    """Ensure that all tables have a creation notification trigger.

    Parameters
    ----------
    new_model_structs : List[TableStructMetadata]
        The new model structs.
    table_metadata : TableMetadata
        The table metadata.

    Implementation details
    ----------------------
    This function ensures that all tables associated to structs that are a new variant
    have a creation notification trigger. If a table does not have a creation
    notification trigger, an attempt at creating one is made and the user is asked
    to confirm the creation of the trigger.
    The name of this trigger is of the form "notify_{table_name}_creation".

    When the table contains foreign keys which refer to tables that have updated_by
    and updated_at columns, it is also necessary to check that the foreign key table
    has a creation notification trigger for the current table.
    The name of this foreign trigger is of the form "notify_{foreign_table_name}_for_{table_name}_creation".

    """
