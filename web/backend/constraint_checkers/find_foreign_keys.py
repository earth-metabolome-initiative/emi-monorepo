from typing import List, Optional, Tuple, Union, Dict
import pandas as pd
from functools import cache
import os
from constraint_checkers.cursor import get_cursor
import re
from constraint_checkers.regroup_tables import get_desinences


def is_role_table(table_name: str) -> bool:
    """Check if a table is a role table.

    Parameters
    ----------
    table_name : str
        The table name.

    Returns
    -------
    bool
        Whether the table is a role table.
    """
    return (
        table_name.endswith("_roles")
        or is_role_invitation_table(table_name)
        or is_role_request_table(table_name)
    )


def is_role_invitation_table(
    table_name: str,
) -> bool:
    """Check if a table is a role invitation table.

    Parameters
    ----------
    table_name : str
        The table name.

    Returns
    -------
    bool
        Whether the table is a role invitation table.
    """
    return table_name.endswith("_role_invitations")


def is_role_request_table(
    table_name: str,
) -> bool:
    """Check if a table is a role request table.

    Parameters
    ----------
    table_name : str
        The table name.

    Returns
    -------
    bool
        Whether the table is a role request table.
    """
    return table_name.endswith("_role_requests")


class ViewColumn:

    def __init__(
        self,
        column_name: str,
        data_type: str,
        alias_name: str,
        table_name: str,
        nullable: bool,
    ):
        self.column_name = column_name
        self.data_type = data_type
        self.alias_name = alias_name
        self.table_name = table_name
        self.nullable = nullable


def postgres_type_to_diesel_type(postgres_type: str) -> str:
    """Converts a Postgres type to a Diesel type."""
    if postgres_type == "integer":
        return "diesel::sql_types::Integer"
    if postgres_type == "text":
        return "diesel::sql_types::Text"
    if postgres_type == "timestamp with time zone":
        return "diesel::sql_types::Timestamp"
    if postgres_type == "uuid":
        return "diesel::sql_types::Uuid"
    if postgres_type == "boolean":
        return "diesel::sql_types::Bool"
    if postgres_type == "real":
        return "diesel::sql_types::Float"

    raise ValueError(f"Unknown Postgres type: {postgres_type}")


class SQLFunction:
    """Class providing metadata for a SQL function."""

    def __init__(self, function_name: str):
        self.name = function_name
        self.flat_variant: Optional["StructMetadata"] = None
        self.arguments: List[str] = []
        self.argument_types: List[str] = []
        self.return_type: Optional[str] = None

    def add_argument(self, argument: str, argument_type: str):
        """Adds an argument to the SQL function."""
        self.arguments.append(argument)
        self.argument_types.append(argument_type)

    def set_return_type(self, return_type: str):
        """Sets the return type of the SQL function."""
        self.return_type = return_type

    def set_flat_variant(self, flat_variant: "StructMetadata"):
        """Sets the table name of the SQL function."""
        self.flat_variant = flat_variant

    def __repr__(self):
        return f"SQLFunction(name={self.name}, arguments={self.arguments}, argument_types={self.argument_types}, return_type={self.return_type})"

    def write_diesel_binding_to_file(self, f):
        """Writes the Diesel binding for the SQL function to a file."""
        f.write("diesel::expression::functions::sql_function! {\n")
        f.write(f"   fn {self.name}(\n")
        for argument, argument_type in zip(self.arguments, self.argument_types):
            if self.flat_variant is not None:
                optional = self.flat_variant.get_attribute_by_name(argument).optional
            elif "can_view" in self.name and argument == "author_user_id":
                optional = True
            else:
                optional = False

            if optional:
                f.write(
                    f"        {argument}: diesel::sql_types::Nullable<{postgres_type_to_diesel_type(argument_type)}>,\n"
                )
            else:
                f.write(
                    f"        {argument}: {postgres_type_to_diesel_type(argument_type)},\n"
                )
        f.write(f"    ) -> {postgres_type_to_diesel_type(self.return_type)};\n")
        f.write("}\n\n")


class TableMetadata:
    """Class for table metadata."""

    def __init__(self, table_metadata: pd.DataFrame):
        self.table_metadata = table_metadata
        self._view_names: List[str] = []
        self._table_names: List[str] = []
        self._flat_variants: Dict[str, "StructMetadata"] = {}

    def tables(self) -> List[str]:
        """Returns the list of tables."""
        return self.table_metadata["referencing_table"].unique().tolist()

    def is_table(self, table_name: str) -> bool:
        """Returns whether the table is a table."""
        if table_name in self._view_names:
            return False
        if table_name in self._table_names:
            return True
        conn, cursor = get_cursor()
        cursor.execute(
            f"""
            SELECT
                table_name
            FROM
                information_schema.tables
            WHERE
                table_name = '{table_name}'
                AND table_type = 'BASE TABLE';
            """
        )
        is_table = cursor.fetchone() is not None
        cursor.close()
        conn.close()

        if is_table:
            self._table_names.append(table_name)

        return is_table

    def register_flat_variant(self, table_name: str, flat_variant: "StructMetadata"):
        """Registers a flat variant for a table.

        Parameters
        ----------
        table_name : str
            The name of the table.
        flat_variant : str
            The name of the flat variant.
        """
        if table_name in self._flat_variants:
            raise ValueError(f"The table {table_name} already has a flat variant.")
        assert not isinstance(flat_variant, str)
        assert not flat_variant.is_nested()

        self._flat_variants[table_name] = flat_variant

    def get_flat_variant(self, table_name: str) -> "StructMetadata":
        """Returns the flat variant of the table.

        Parameters
        ----------
        table_name : str
            The name of the table.
        """
        return self._flat_variants.get(table_name)

    def is_view(self, table_name: str) -> bool:
        """Returns whether the table is a view."""
        if table_name in self._view_names:
            return True
        if table_name in self._table_names:
            return False
        conn, cursor = get_cursor()
        cursor.execute(
            f"""
            SELECT
                table_name
            FROM
                information_schema.tables
            WHERE
                table_name = '{table_name}'
                AND table_type = 'VIEW';
            """
        )
        is_view = cursor.fetchone() is not None
        cursor.close()
        conn.close()

        if is_view:
            self._view_names.append(table_name)

        return is_view

    @cache
    def extract_view_columns(self, view_name: str) -> List[ViewColumn]:
        """Returns list of columns, their alias and the original table name from a provided view name.

        # Example
        Suppose you have a simple view creation statement like this:

        ```sql
        CREATE VIEW view_name AS
        SELECT
            table_name.column_name AS alias_name
        FROM
            table_name;
        ```

        This function will return a list of tuples like this:

            ```python
            [("column_name", "alias_name", "table_name")]
            ```
        """
        conn, cursor = get_cursor()
        cursor.execute(
            f"""
            SELECT
                view_definition
            FROM
                information_schema.views
            WHERE
                table_name = '{view_name}';
            """
        )
        view_definition = cursor.fetchall()
        cursor.close()
        conn.close()

        if len(view_definition) == 0:
            raise ValueError(f"The view {view_name} does not exist.")

        view_definition = view_definition[0][0]
        view_definition = view_definition.replace("\n", " ")

        # Extract the columns from the view definition
        columns = re.findall(r"SELECT (.*?) FROM", view_definition)[0]
        columns = columns.split(", ")

        # For each column, we need to identify the original table name.
        table_name_mappings = {
            "thumbnail_documents": "documents",
            "profile_picture_documents": "documents",
        }

        extracted_columns: List[ViewColumn] = []
        for column in columns:
            if " AS " in column:
                original_column_name, alias_column_name = column.split(" AS ")
                if "." in original_column_name:
                    original_table_name, original_column_name = (
                        original_column_name.split(".")
                    )
                    original_table_name = original_table_name.strip()
                else:
                    continue

                remapped = False

                if not self.is_table(original_table_name) and not self.is_view(
                    original_table_name
                ):
                    remapped_table_name = table_name_mappings.get(original_table_name)
                    remapped = True
                    if remapped_table_name is None:
                        raise ValueError(
                            f"The table {original_table_name} does not exist."
                        )
                    original_table_name = remapped_table_name

                extracted_columns.append(
                    ViewColumn(
                        column_name=original_column_name.strip(),
                        data_type=self.get_column_data_type(
                            original_table_name, original_column_name.strip()
                        ),
                        alias_name=alias_column_name.strip(),
                        table_name=original_table_name.strip(),
                        nullable=remapped
                        or self.is_nullable(
                            original_table_name, original_column_name.strip()
                        ),
                    )
                )
            else:
                if "." in column:
                    original_table_name, original_column_name = column.split(".")
                    original_table_name = original_table_name.strip()
                else:
                    continue

                remapped = False

                if not self.is_table(original_table_name) and not self.is_view(
                    original_table_name
                ):
                    remapped_table_name = table_name_mappings.get(original_table_name)
                    remapped = True
                    if remapped_table_name is None:
                        raise ValueError(
                            f"The table {original_table_name} does not exist."
                        )
                    original_table_name = remapped_table_name

                extracted_columns.append(
                    ViewColumn(
                        column_name=original_column_name.strip(),
                        data_type=self.get_column_data_type(
                            original_table_name, original_column_name.strip()
                        ),
                        alias_name=original_column_name.strip(),
                        table_name=original_table_name.strip(),
                        nullable=remapped
                        or self.is_nullable(
                            original_table_name, original_column_name.strip()
                        ),
                    )
                )

        return extracted_columns

    @cache
    def has_foreign_keys(self, table_name: str) -> bool:
        """Returns whether the table has foreign keys.

        Parameters
        ----------
        table_name : str
            The name of the table.

        Implementation details
        ----------------------
        This method checks if any of the columns in the table metadata
        associated with the table name have a non-null value in the
        `referenced_table` column. If any of the columns have a non-null
        value, then the table has foreign keys.
        """
        if self.is_view(table_name):
            for column in self.extract_view_columns(table_name):
                if column.column_name in self.get_foreign_keys(column.table_name):
                    return True
            return False

        primary_keys = self.get_primary_key_names_and_types(table_name)

        primary_key_names = [primary_key[0] for primary_key in primary_keys]

        foreign_keys = self.get_foreign_keys(table_name)
        return any(foreign_key not in primary_key_names for foreign_key in foreign_keys)

    @cache
    def get_foreign_keys(self, table_name: str) -> List[str]:
        """Returns the foreign keys of the table.

        Parameters
        ----------
        table_name : str
            The name of the table.

        Implementation details
        ----------------------
        This method returns the list of columns in the table metadata
        associated with the table name that have a non-null value in the
        `referenced_table` column. These columns are the foreign keys
        of the table.
        """
        if self.is_view(table_name):
            foreign_keys = []
            for column in self.extract_view_columns(table_name):
                if column.column_name in self.get_foreign_keys(
                    column.table_name
                ) or self.is_primary_key(column.table_name, column.column_name):
                    foreign_keys.append(column.alias_name)
            return foreign_keys

        table_columns = self.table_metadata[
            self.table_metadata["referencing_table"] == table_name
        ]
        return table_columns[table_columns["referenced_table"].notnull()][
            "referencing_column"
        ].tolist()

    @cache
    def get_foreign_key_table_name(self, table_name: str, column_name: str) -> str:
        """Returns the table that the foreign key references.

        Parameters
        ----------
        table_name : str
            The name of the table.
        column_name : str
            The name of the column.

        Implementation details
        ----------------------
        This method returns the value in the `referenced_table` column
        in the table metadata associated with the table name and column
        name. This value is the table that the foreign key references.
        """
        if self.is_view(table_name):
            for column in self.extract_view_columns(table_name):
                if column.alias_name == column_name:
                    return self.get_foreign_key_table_name(
                        column.table_name, column.column_name
                    )
            raise ValueError(
                f"The column {column_name} does not exist in the view {table_name}."
            )

        if column_name not in self.get_foreign_keys(table_name):
            raise ValueError(
                f"The column {column_name} is not a foreign key in the table {table_name}."
            )

        table_columns = self.table_metadata[
            (self.table_metadata["referencing_table"] == table_name)
            & (self.table_metadata["referencing_column"] == column_name)
        ]

        if table_columns.empty:
            if self.is_primary_key(table_name, column_name):
                return table_name
            else:
                raise ValueError(
                    f"The column {column_name} does not exist in the table {table_name}."
                )

        return table_columns["referenced_table"].values[0]

    def is_primary_key(self, table_name: str, candidate_key: str) -> bool:
        """Returns whether the candidate key is the primary key of the table.

        Parameters
        ----------
        table_name : str
            The name of the table.
        candidate_key : str
            The candidate key.

        Implementation details
        ----------------------
        This method returns whether the candidate key is the primary key
        of the table metadata associated with the table name.
        """
        return any(
            primary_key[0] == candidate_key
            for primary_key in self.get_primary_key_names_and_types(table_name)
        )

    @cache
    def get_column_data_type(self, table_name: str, column_name: str) -> str:
        """Returns the data type of the column.

        Parameters
        ----------
        table_name : str
            The name of the table.
        column_name : str
            The name of the column.
        """
        if self.is_view(table_name):
            for view_column in self.extract_view_columns(table_name):
                if view_column.alias_name == column_name:
                    return view_column.data_type

        _conn, cursor = get_cursor()

        cursor.execute(
            f"""
            SELECT
                data_type
            FROM
                information_schema.columns
            WHERE
                table_name = '{table_name}'
                AND column_name = '{column_name}';
            """
        )

        data_type = cursor.fetchone()[0]

        cursor.close()

        return data_type

    @cache
    def is_nullable(self, table_name: str, column_name: str) -> bool:
        """Returns whether the column is nullable.

        Parameters
        ----------
        table_name : str
            The name of the table.
        column_name : str
            The name of the column.
        """
        if self.is_view(table_name):
            for view_column in self.extract_view_columns(table_name):
                if view_column.alias_name == column_name:
                    return view_column.nullable

        _conn, cursor = get_cursor()

        cursor.execute(
            f"""
            SELECT
                is_nullable
            FROM
                information_schema.columns
            WHERE
                table_name = '{table_name}'
                AND column_name = '{column_name}';
            """
        )

        assert cursor.rowcount == 1, f"Column {column_name} does not exist in table {table_name}."

        is_nullable = cursor.fetchone()[0]

        cursor.close()

        return is_nullable == "YES"

    @cache
    def get_primary_key_names_and_types(
        self, table_name: str
    ) -> Optional[List[Tuple[str, str]]]:
        """Returns the names and types of the components of the primary key of the table.

        Parameters
        ----------
        table_name : str
            The name of the table.

        Implementation details
        ----------------------
        This method returns the name and data type of the column in the
        table metadata associated with the table name that has the value
        `PRI` in the `column_key` column. This column is the primary key
        of the table. When the primary key is a composite key, this method
        returns the names and data types of the components of the primary
        key.
        """
        if self.is_view(table_name):
            # We check if the view has an "id" column and if it does,
            # we return the primary key of the associated table.
            view_columns = self.extract_view_columns(table_name)
            for column in view_columns:
                if column.alias_name == "id":
                    return self.get_primary_key_names_and_types(column.table_name)
            return None
        _conn, cursor = get_cursor()

        cursor.execute(
            f"""
            SELECT
                kcu.column_name,
                data_type
            FROM
                information_schema.table_constraints AS tc
            JOIN
                information_schema.key_column_usage AS kcu
            ON
                tc.constraint_name = kcu.constraint_name
            JOIN
                information_schema.columns AS cols
            ON
                kcu.table_name = cols.table_name
                AND kcu.column_name = cols.column_name
            WHERE
                tc.table_name = '{table_name}'
                AND tc.constraint_type = 'PRIMARY KEY';
            """
        )

        primary_key = cursor.fetchall()

        cursor.close()

        return primary_key

    @cache
    def get_unique_constraint_columns(
        self, table_name: str
    ) -> Union[List[List[str]], List[ViewColumn]]:
        """Returns the columns of the unique constraint."""
        if self.is_view(table_name):
            # In a view, we return as set of unique columns the columns
            # that appear in the view and are unique in the table of
            # reference of the view, i.e. the one associated to the ID
            # column of the view.
            view_columns = self.extract_view_columns(table_name)
            base_table_name = None
            for column in view_columns:
                if column.alias_name == "id":
                    base_table_name = column.table_name
                    break
            if base_table_name is None:
                raise ValueError(f"The view {table_name} does not have an ID column.")
            base_unique_columns = self.get_unique_constraint_columns(base_table_name)

            view_unique_columns: List[ViewColumn] = []
            for column in view_columns:
                if column.column_name in base_unique_columns:
                    view_unique_columns.append(
                        ViewColumn(
                            column_name=column.column_name,
                            data_type=column.data_type,
                            alias_name=column.alias_name,
                            table_name=column.table_name,
                            nullable=column.nullable,
                        )
                    )

            return view_unique_columns

        _conn, cursor = get_cursor()
        cursor.execute(
            f"""
            SELECT array_agg(attname ORDER BY attnum) AS columns
            FROM pg_constraint
            JOIN pg_attribute ON pg_constraint.conrelid = pg_attribute.attrelid
            AND pg_attribute.attnum = ANY(pg_constraint.conkey)
            JOIN pg_class ON pg_class.oid = pg_constraint.conrelid
            JOIN pg_namespace ON pg_namespace.oid = pg_class.relnamespace
            WHERE contype = 'u' -- 'u' for unique constraint
            AND pg_namespace.nspname = 'public'
            AND pg_class.relname = '{table_name}'
            GROUP BY conname, contype;
            """
        )

        columns = cursor.fetchall()
        cursor.close()

        # We flatten the list of lists of lists

        unique_constraints = []

        for column in columns:
            unique_constraints.append(column[0])

        return unique_constraints

    @cache
    def has_trigger_by_name(self, table_name: str, trigger_name: str) -> bool:
        """Returns whether the table has a trigger by name.

        Parameters
        ----------
        table_name : str
            The name of the table.
        trigger_name : str
            The name of the trigger.

        Implementation details
        ----------------------
        This method returns whether the table metadata associated with
        the table name has a non-null value in the `trigger_name` column.
        """
        _conn, cursor = get_cursor()
        cursor.execute(
            f"""
            SELECT
                trigger_name
            FROM
                information_schema.triggers
            WHERE
                event_object_table = '{table_name}'
                AND trigger_name = '{trigger_name}';
            """
        )

        has_trigger = cursor.fetchone() is not None
        cursor.close()

        return has_trigger

    @cache
    def get_columns(self, table_name: str) -> List[str]:
        """Returns the columns of the table."""
        if self.is_view(table_name):
            return [column[1] for column in self.extract_view_columns(table_name)]

        _conn, cursor = get_cursor()
        cursor.execute(
            f"""
            SELECT
                column_name
            FROM
                information_schema.columns
            WHERE
                table_name = '{table_name}';
            """
        )

        columns = cursor.fetchall()
        cursor.close()

        return [column[0] for column in columns]

    @cache
    def has_updated_at_trigger(self, table_name: str) -> bool:
        """Returns whether the table has an `updated_at` trigger.

        Parameters
        ----------
        table_name : str
            The name of the table.

        Implementation details
        ----------------------
        This method returns whether the table metadata associated with
        the table name has a trigger named `updated_at`.
        """
        return self.has_trigger_by_name(table_name, f"{table_name}_updated_at_trigger")

    @cache
    def has_updated_at_column(self, table_name: str) -> bool:
        """Returns whether the table has an `updated_at` column.

        Parameters
        ----------
        table_name : str
            The name of the table.

        Implementation details
        ----------------------
        This method returns whether the table metadata associated with
        the table name has a column named `updated_at`.
        """
        return "updated_at" in self.get_columns(table_name)

    @cache
    def get_default_column_value(
        self, table_name: str, column_name: str
    ) -> Optional[str]:
        """Returns the default value of the column.

        Parameters
        ----------
        table_name : str
            The name of the table.
        column_name : str
            The name of the column.

        Implementation details
        ----------------------
        This method returns the value in the `column_default` column in
        the table metadata associated with the table name and column name.
        This value is the default value of the column.
        """
        if self.is_view(table_name):
            for view_column in self.extract_view_columns(table_name):
                if view_column.alias_name == column_name:
                    return self.get_default_column_value(
                        view_column.table_name, column_name
                    )

        _conn, cursor = get_cursor()
        cursor.execute(
            f"""
            SELECT
                column_default
            FROM
                information_schema.columns
            WHERE
                table_name = '{table_name}'
                AND column_name = '{column_name}';
            """
        )

        default_value = cursor.fetchone()[0]
        cursor.close()

        return default_value

    def get_circular_foreign_keys(self, table_name: str) -> List[str]:
        """Returns the circular foreign keys of the table.

        Parameters
        ----------
        table_name : str
            The name of the table.

        Implementation details
        ----------------------
        This method returns the list of columns in the table metadata
        associated with the table name that have a non-null value in the
        `referenced_table` column and have a foreign key that references
        the table name.
        """
        primary_keys = self.get_primary_key_names_and_types(table_name)
        primary_key_names = [primary_key[0] for primary_key in primary_keys]

        return [
            column
            for column in self.get_foreign_keys(table_name)
            if self.get_foreign_key_table_name(table_name, column) == table_name
            and column not in primary_key_names
        ]

    def has_circular_parent_column(self, table_name: str) -> bool:
        """Returns whether the table has a circular parent column.

        Parameters
        ----------
        table_name : str
            The name of the table.

        Implementation details
        ----------------------
        This method returns whether the table has a column that is a
        foreign key to the table itself.
        """
        return len(self.get_circular_foreign_keys(table_name)) > 0

    def has_parent_circularity_trigger(self, table_name: str) -> bool:
        """Returns whether the table has a parent circularity trigger.

        Parameters
        ----------
        table_name : str
            The name of the table.

        Implementation details
        ----------------------
        This method returns whether the table has a trigger named
        `parent_circularity_trigger`.
        """
        return self.has_trigger_by_name(
            table_name, f"{table_name}_parent_circularity_trigger"
        )

    def has_associated_roles(self, table_name: str) -> bool:
        """Returns whether the table has associated roles.

        Parameters
        ----------
        table_name : str
            The name of the table.

        Implementation details
        ----------------------
        This method returns whether there exists a table in the database
        names `{table_name}_{referece}_roles`.
        """
        return any(
            self.is_table(f"{table_name}_{reference}_roles")
            for reference in ("users", "teams")
        )

    def _register_flat_variant_associated_with_function(
        self, sql_function: SQLFunction
    ):
        """Searches for a flat struct strictly associated with the function.

        Parameters
        ----------
        sql_function : SQLFunction
            The SQL function.

        Implementation details
        ----------------------
        In order to find the flat struct associated with the function, we
        first search the migration that contains the definition of the function.
        All functions are defined as "CREATE FUNCTION" in the migration.
        If we do not find any such migration, the function may be defined automatically
        by things such as a GIN index (e.g. similarity), in which case there are no associated tables.
        If we find the migration, we then search the table name that completes a desinence
        that fully matches the migration name. At this point, we retrieve the associated
        flat variant and we check that all of the arguments in the sql function are columns
        of the flat variant. If this is the case, we register the flat variant with the function,
        otherwise we raise an error.
        """
        migration_name = None
        for migration in os.listdir("migrations"):
            if not os.path.exists(f"migrations/{migration}/up.sql"):
                continue

            with open(f"migrations/{migration}/up.sql", "r", encoding="utf8") as f:
                migration_contents = f.read()

            if not f"CREATE FUNCTION {sql_function.name}" in migration_contents:
                continue

            migration_name = migration
            break

        if migration_name is None:
            return

        assert (
            len(self._flat_variants) > 0
        ), "We expect the table metadata to contain at least one flat variant. "

        # We have found the migration that contains the function definition.
        # We now search for the table name that completes the desinence.
        desinence = migration_name.split("_", maxsplit=1)[-1]
        found_table = False
        for table_name, flat_variant in self._flat_variants.items():
            if desinence in get_desinences(table_name):
                found_table = True
                if all(
                    flat_variant.get_attribute_by_name(argument) is not None
                    for argument in sql_function.arguments
                ):
                    sql_function.set_flat_variant(flat_variant)
                    return
                if "_can_x_" in migration_name:
                    continue
                raise ValueError(
                    f"The function {sql_function.name} is not associated with the table {table_name}, and "
                    f"yet we found it in the migration {migration_name}, which is associated with the table {table_name}."
                )

        if not found_table:
            raise ValueError(
                f"Could not find a table associated with the function {sql_function.name}, "
                f"even though we found the migration {migration_name}."
            )

    def get_all_postgres_functions(self) -> List[SQLFunction]:
        """Returns the list of all Postgres functions."""
        _conn, cursor = get_cursor()
        cursor.execute(
            """
            SELECT
                p.proname AS function_name,
                pg_catalog.pg_get_function_result(p.oid) AS return_type,
                pg_catalog.pg_get_function_arguments(p.oid) AS arguments
            FROM pg_catalog.pg_proc p
            LEFT JOIN pg_catalog.pg_namespace n ON n.oid = p.pronamespace
            WHERE pg_catalog.pg_function_is_visible(p.oid)
            AND n.nspname NOT IN ('pg_catalog', 'information_schema')
            AND p.proname NOT LIKE 'diesel_%'
            AND p.proname NOT LIKE 'uuid_%'
            AND p.proname NOT LIKE 'set_%'
            AND p.proname NOT LIKE 'show_%'
            AND p.proname NOT LIKE 'gtrgm_%'
            AND p.proname NOT LIKE 'gin_%'
            AND pg_catalog.pg_get_function_result(p.oid) != 'trigger';
            """
        )

        postgres_functions = cursor.fetchall()
        cursor.close()

        assert len(postgres_functions) > 0, (
            "There are no Postgres functions in the database. "
            "We expect the database to contain several functions, such "
            "as the can_view and can_update functions."
        )

        sql_functions = []

        for function in postgres_functions:
            function_name = function[0]
            sql_function = SQLFunction(function_name)
            arguments = function[2].split(", ")
            for i, argument in enumerate(arguments):
                if " " in argument:
                    argument_name, argument_type = argument.split(" ")
                else:
                    argument_name = f"arg_{i}"
                    argument_type = argument
                sql_function.add_argument(argument_name, argument_type)
            sql_function.set_return_type(function[1])
            sql_functions.append(sql_function)

        for function in sql_functions:
            self._register_flat_variant_associated_with_function(function)

        return sql_functions

    @cache
    def has_postgres_function(self, function_name: str) -> bool:
        """Returns whether the table has a Postgres function.

        Parameters
        ----------
        function_name : str
            The name of the function.

        Implementation details
        ----------------------
        This method returns whether the table metadata associated with
        the table name has a function named `function_name`.
        """
        _conn, cursor = get_cursor()
        cursor.execute(
            f"""
            SELECT
                proname
            FROM
                pg_proc
            WHERE
                proname = '{function_name}';
            """
        )

        has_function = cursor.fetchone() is not None
        cursor.close()

        return has_function


def find_foreign_keys() -> TableMetadata:
    """Returns the list of indices that are of type `pg_trgm`."""
    conn, cursor = get_cursor()
    cursor.execute(
        """
        SELECT
            tc.constraint_name AS constraint_name,
            tc.table_name AS referencing_table,
            kcu.column_name AS referencing_column,
            ccu.table_name AS referenced_table,
            ccu.column_name AS referenced_column,
            CASE
                WHEN tc.constraint_type = 'FOREIGN KEY' THEN 'NO'
                ELSE 'YES'
            END AS is_optional
        FROM
            information_schema.table_constraints AS tc
        JOIN information_schema.key_column_usage AS kcu
        ON
            tc.constraint_name = kcu.constraint_name
        JOIN information_schema.constraint_column_usage AS ccu
        ON
            tc.constraint_name = ccu.constraint_name
        WHERE
            tc.constraint_type = 'FOREIGN KEY';
        """
    )
    table_metadata = cursor.fetchall()

    columns = [desc[0] for desc in cursor.description]
    table_metadata = pd.DataFrame(table_metadata, columns=columns)

    cursor.close()
    conn.close()

    return TableMetadata(table_metadata)
