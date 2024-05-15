from typing import List, Optional, Tuple, Union
import pandas as pd
from functools import cache
from constraint_checkers.cursor import get_cursor
import re


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


class TableMetadata:

    def __init__(self, table_metadata: pd.DataFrame):
        self.table_metadata = table_metadata
        self._view_names: List[str] = []
        self._table_names: List[str] = []

    def tables(self) -> List[str]:
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

        if self.is_primary_key(table_name, column_name):
            return table_name

        table_columns = self.table_metadata[
            (self.table_metadata["referencing_table"] == table_name)
            & (self.table_metadata["referencing_column"] == column_name)
        ]
        return table_columns["referenced_table"].values[0]

    @cache
    def foreign_key_table_has_foreign_keys(
        self, table_name: str, foreign_key_column: str
    ) -> bool:
        """Returns whether the foreign key table has foreign keys.

        Parameters
        ----------
        table_name : str
            The name of the table.
        foreign_key_column : str
            The name of the foreign key column.

        Implementation details
        ----------------------
        This method checks if the foreign key table has foreign keys.
        """
        return self.has_foreign_keys(
            self.get_foreign_key_table_name(table_name, foreign_key_column)
        )

    @cache
    def has_primary_key(self, table_name: str) -> bool:
        """Returns whether the table has a primary key.

        Parameters
        ----------
        table_name : str
            The name of the table.

        Implementation details
        ----------------------
        This method returns whether the table metadata associated with
        the table name has a non-null value in the `column_key` column.
        """
        return self.get_primary_key_names_and_types(table_name) is not None

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
                    return self.get_primary_key_name_and_type(column.table_name)
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
    ) -> Union[List[str], List[ViewColumn]]:
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
            SELECT min(column_name)
            FROM information_schema.table_constraints AS c
            JOIN information_schema.constraint_column_usage AS cc
                USING (table_schema, table_name, constraint_name)
            WHERE c.constraint_type = 'UNIQUE' AND c.table_name = '{table_name}'
            GROUP BY table_schema, table_name
            HAVING count(*) = 1;
            """
        )

        columns = cursor.fetchall()
        cursor.close()

        return [column[0] for column in columns]

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
                    return self.get_default_value(view_column.table_name, column_name)

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

    def is_edge_table(self) -> bool:
        """Returns whether the table is an edge table.

        Implementation details
        -----------------------
        An edge table is a table whose primary key is defined as a
        tuple of two or more foreign keys.
        """
        return len(self.get_primary_key_name_and_type()) > 1


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
