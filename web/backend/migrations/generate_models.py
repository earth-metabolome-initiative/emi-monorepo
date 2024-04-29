"""Python script to run a patched version of the diesel extended CLI to generate models.

Implementation details
----------------------
The diesel extended CLI can be used to generated the structs associated to the database tables. However, the
generated structs are not complete most commonly, as it does not come equipped with some of the postgres types.
Fortunately, this can be easily patched with some replace statements.

We start by running the extended CLI command:

```bash
diesel_ext --model --add-table-name > src/models.rs
```

Then, we need to handle the following replacements, plus adding on the top of the file the associated imports.
The imports need to be added solely when the replacements are effective, i.e. there is actually a change in the file,
otherwise we would add unnecessary imports and cause compilation warnings.

The replacements are defined in the file `replacements.json` and are applied to the generated file `src/models.rs`.
The structure of the JSON file is as follows:

```json
[
    {
        "search": "search_string",
        "replace": "replace_string",
        "imports": ["import1", "import2"]
    }
]
```

With a more concrete example:

```json
[
    {
        "search": "Option</* TODO: unknown type Nullable<Numrange> */>",
        "replace": "Option<Range<Numeric>>",
        "imports": [
            "use diesel::sql_types::Numeric;",
            "use diesel::sql_types::Range;"
        ]
    }
]
```

Since we also need to interface with the Frontend database which are NOT based
on Postgres, we also need to duplicate the code in the web commons and generate
the From implementations for the structs in the `src/models.rs` file. The `web_common`
structs will be generated in the `src/database/tables.rs` file in the `web_common` crate.
Since these structs are field-by-field identical, we can simply copy the structs while
ignoring the `#[derive(...)]` and `#[table_name = "..."]` attributes which would not be
applicable in the `web_common` crate. The `From` implementations will be generated in the
`src/models.rs` file in the `backend` crate, below each of the diesel-generated structs and
will make use of the full path to the struct in the `web_common` crate so to avoid conflicts.

"""

import os
import re
import shutil
from typing import List, Optional, Tuple, Union

import compress_json
import pandas as pd
import psycopg2
from densify_directory_counter import densify_directory_counter
from dotenv import load_dotenv
from retrieve_taxons import retrieve_taxons
from tqdm.auto import tqdm
from functools import cache
import glob


def struct_name_from_table_name(table_name: str) -> str:
    """Convert the table name to the struct name."""
    if table_name.endswith("s"):
        table_name = table_name[:-1]
    return "".join(word.capitalize() for word in table_name.split("_"))


class PGIndex:

    def __init__(
        self, name: str,
        table_name: str,
        arguments: str,
        index_type: str
    ):
        self.name = name
        self.table_name = table_name
        self.arguments = arguments
        self.index_type = index_type

    def is_gin(self) -> bool:
        return self.index_type == "gin_trgm_ops"

class PGIndices:

    SIMILARITY_METHODS = (
        ("similarity", "%", "<->"),
        ("word_similarity", "<%", "<<->"),
        ("strict_word_similarity", "<<%", "<<<->"),
    )

    def __init__(self, indices: List[PGIndex]):
        self.indices = indices
        self.foreign_keys_information = find_foreign_keys()

    def has_table(self, table_name: str) -> bool:
        if self.foreign_keys_information.is_view(table_name):
            view_columns = self.foreign_keys_information.extract_view_columns(
                table_name
            )
            # We seek an "id" column in the view columns
            for column in view_columns:
                if column.alias_name == "id":
                    return self.has_table(column.table_name)

        for index in self.indices:
            if index.table_name == table_name:
                return True
        return False

    def get_table(self, table_name: str) -> PGIndex:
        if self.foreign_keys_information.is_view(table_name):
            view_columns = self.foreign_keys_information.extract_view_columns(
                table_name
            )
            # We seek an "id" column in the view columns
            for column in view_columns:
                if column.alias_name == "id":
                    return self.get_table(column.table_name)

        for index in self.indices:
            if index.table_name == table_name:
                return index
        return None


class AttributeMetadata:

    def __init__(
        self,
        original_name: str,
        name: str,
        data_type: Union[str, "StructMetadata"],
        optional: bool,
    ):
        self.original_name = original_name
        self.name = name
        self._data_type = data_type
        self.optional = optional

    def is_undefined_nested_dependencies(self) -> bool:
        return not self.has_struct_data_type() and self.data_type().startswith("Nested")

    def has_struct_data_type(self) -> bool:
        return isinstance(self._data_type, StructMetadata)

    def format_data_type(self) -> str:
        data_type = self.data_type()

        if self.optional:
            return f"Option<{data_type}>"
        return data_type

    def data_type(self) -> str:
        if isinstance(self._data_type, StructMetadata):
            return self._data_type.name
        elif isinstance(self._data_type, str):
            return self._data_type

        raise ValueError("The data type must be either a string or a StructMetadata.")

    def implements_serialize(self) -> bool:
        return (
            not isinstance(self._data_type, StructMetadata)
            or self._data_type.can_implement_serialize()
        )

    def implements_deserialize(self) -> bool:
        return (
            not isinstance(self._data_type, StructMetadata)
            or self._data_type.can_implement_deserialize()
        )

    def implements_eq(self) -> bool:
        return (
            self._data_type not in ["f32", "f64"]
            or isinstance(self._data_type, StructMetadata)
            and self._data_type.can_implement_eq()
        )

    def __eq__(self, other: "AttributeMetadata") -> bool:
        return (
            self.name == other.name
            and self._data_type == other._data_type
            and self.optional == other.optional
        )

    def implements_clone(self) -> bool:
        return (
            self._data_type
            in [
                "bool",
                "i8",
                "i16",
                "i32",
                "i64",
                "i128",
                "u8",
                "u16",
                "u32",
                "u64",
                "u128",
                "f32",
                "f64",
                "Uuid",
                "String",
                "NaiveDateTime",
            ]
            or isinstance(self._data_type, StructMetadata)
            and self._data_type.can_implement_clone()
        )


class StructMetadata:

    def __init__(self, struct_name: str, table_name: str):
        self.name = struct_name
        self.table_name = table_name
        self.attributes: List[AttributeMetadata] = []
        self._derives: List[str] = []

    def write_to(self, file: "File", diesel: Optional[str] = None):
        if diesel is not None:
            if diesel not in ["tables", "views"]:
                raise ValueError("The table type must be either 'tables' or 'views'.")

        file.write(f"#[derive({', '.join(self.derives(diesel=diesel))})]\n")
        if diesel is not None:
            file.write(f"#[diesel(table_name = {self.table_name})]\n")
        file.write(f"pub struct {self.name} {{\n")
        for attribute in self.attributes:
            file.write(f"    pub {attribute.name}: {attribute.format_data_type()},\n")
        file.write("}\n\n")

    def has_undefined_nested_dependencies(self) -> bool:
        """Returns whether the struct has undefined nested dependencies.

        Implementative details
        -----------------------
        This method checks if any of the attributes of the struct
        is not a struct and starts with the word `Nested`.
        """
        return any(
            attribute.is_undefined_nested_dependencies()
            for attribute in self.attributes
        )

    def get_attribute_by_name(self, attribute_name: str) -> Optional[AttributeMetadata]:
        for attribute in self.attributes:
            if attribute.name == attribute_name:
                return attribute
        return None

    def capitalized_table_name(self) -> str:
        return "".join(word.capitalize() for word in self.table_name.split("_"))

    def is_nested(self) -> bool:
        return any(
            isinstance(attribute._data_type, StructMetadata)
            for attribute in self.attributes
        )

    def add_attribute(self, attribute_metadata: AttributeMetadata):
        self.attributes.append(attribute_metadata)

    def add_derive(self, derive: str):
        self._derives.append(derive)

    def contains_optional_fields(self) -> bool:
        return any(attribute.optional for attribute in self.attributes)

    def contains_only_optional_fields(self) -> bool:
        return all(attribute.optional for attribute in self.attributes)

    def only_primary_key(self) -> bool:
        """Returns whether the struct contains only the primary key."""
        if len(self.attributes) != 1:
            return False
        
        table_metadata = find_foreign_keys()
        return self.attributes[0].name == table_metadata.get_primary_key_name_and_type(self.table_name)[0]
 
    def derives(self, diesel: Optional[str] = None) -> List[str]:
        """Returns the list of derives for the struct.
        
        Parameters
        ----------
        diesel : bool
            Whether to include the derives for the diesel crate.
        """
        derives = self._derives.copy()
        if self.can_implement_eq() and "Eq" not in self._derives:
            derives.append("Eq")
        if "PartialEq" not in self._derives:
            derives.append("PartialEq")
        if "Debug" not in self._derives:
            derives.append("Debug")
        if self.can_implement_clone() and "Clone" not in self._derives:
            derives.append("Clone")
        if self.can_implement_serialize() and "Serialize" not in self._derives:
            derives.append("Serialize")
        if self.can_implement_deserialize() and "Deserialize" not in self._derives:
            derives.append("Deserialize")

        diesel_derives = [
            "Identifiable",
            "QueryableByName",
            "Queryable",
        ]

        if diesel == "tables":
            diesel_derives.extend(["Insertable", "Selectable"])

        if diesel:
            for derive in diesel_derives:
                if derive not in derives:
                    derives.append(derive)

            # If the attributes are not strictly limited to the
            # primary key, we add the AsChangeset derive.
            if not self.only_primary_key() and diesel == "tables":
                derives.append("AsChangeset")
        else:
            derives = [derive for derive in derives if derive not in diesel_derives]

        return derives

    def can_implement_clone(self) -> bool:
        return all(attribute.implements_clone() for attribute in self.attributes)

    def can_implement_eq(self) -> bool:
        return all(attribute.implements_eq() for attribute in self.attributes)

    def can_implement_serialize(self) -> bool:
        return all(
            attribute.implements_serialize() for attribute in self.attributes
        )

    def can_implement_deserialize(self) -> bool:
        return all(
            attribute.implements_deserialize() for attribute in self.attributes
        )

    def has_attribute(self, attribute: AttributeMetadata) -> bool:
        """Returns the type of the attribute"""
        return any(
            attribute == existing_attribute for existing_attribute in self.attributes
        )

    def is_nested(self) -> bool:
        return any(
            isinstance(attribute._data_type, StructMetadata)
            for attribute in self.attributes
        )


def get_cursor():
    """Get the cursor to the database."""
    dbname = os.getenv("POSTGRES_DB")
    user = os.getenv("POSTGRES_USER")
    password = os.getenv("POSTGRES_PASSWORD")
    port = os.getenv("PGPORT")
    # url = os.getenv("POSTGRES_URL")

    # Establishing a connection to the PostgreSQL database
    conn = psycopg2.connect(
        dbname=dbname,
        user=user,
        password=password,
        host="localhost",
        port=port,
    )

    return conn, conn.cursor()

def find_pg_trgm_indices() -> PGIndices:
    """Returns the list of indices that are of type `pg_trgm`."""
    tables_metadata = find_foreign_keys()
    conn, cursor = get_cursor()
    cursor.execute(
        """
        SELECT
            indexname AS index_name,
            tablename AS table_name,
            substring(indexdef from '\((.*)\)') AS arguments
        FROM
            pg_indexes
        WHERE
            indexdef ILIKE '%using gin%'
            AND indexdef ILIKE '%gin_trgm_ops%'
            OR
            indexdef ILIKE '%using gist%'
            AND indexdef ILIKE '%gist_trgm_ops%';
        """
    )
    indices = cursor.fetchall()
    pg_indices = []
    for index in indices:
        sanitized_coumn_names = []
        table_name = index[1]
        table_columns = tables_metadata.get_columns(table_name)
        arguments = index[2]

        index_type = None
        for possible_index_type in (
            "gin_trgm_ops",
            "gist_trgm_ops",
        ):
            if possible_index_type in arguments:
                index_type = possible_index_type
                break

        # At this time, we only support indices with a single {index_type}.
        # We check that in the definition of the index there appears
        # only a single {index_type} call.
        if arguments.count(index_type) != 1:
            raise ValueError(
                f"The index {index[0]} has more than one {index_type} call."
            )

        assert arguments.endswith(f" {index_type}")

        arguments = arguments.replace(f" {index_type}", "")

        pg_indices.append(
            PGIndex(
                name=index[0],
                table_name=index[1],
                arguments=arguments,
                index_type=index_type,
            )
        )

    cursor.close()
    conn.close()

    return PGIndices(pg_indices)


def sql_type_to_rust_type(sql_type: str) -> str:
    """Convert the SQL type to the Rust type."""
    if sql_type == "uuid":
        return "uuid::Uuid"
    if sql_type == "integer":
        return "i32"
    raise NotImplementedError(f"The SQL type {sql_type} is not supported.")


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

        primary_key_name, _ = self.get_primary_key_name_and_type(table_name)
        foreign_keys = self.get_foreign_keys(table_name)
        return any(foreign_key != primary_key_name for foreign_key in foreign_keys)

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
        return self.get_primary_key_name_and_type(table_name) is not None

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
        primary_key_name, _ = self.get_primary_key_name_and_type(table_name)
        return primary_key_name == candidate_key

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
    def get_primary_key_name_and_type(
        self, table_name: str
    ) -> Optional[Tuple[str, str]]:
        """Returns the name and type of the primary key of the table.

        Parameters
        ----------
        table_name : str
            The name of the table.

        Implementation details
        ----------------------
        This method returns the name and data type of the column in the
        table metadata associated with the table name that has the value
        `PRI` in the `column_key` column. This column is the primary key
        of the table.
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

        primary_key = cursor.fetchone()

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


def write_backend_structs(
    path: str,
    table_type: str,
    struct_metadatas: List[StructMetadata]
):
    """Write the `From` implementations for the structs in the `src/models.rs` file."""

    if len(struct_metadatas) == 0:
        return

    similarity_indices: PGIndices = find_pg_trgm_indices()
    table_metadatas = find_foreign_keys()

    # After each struct ends, as defined by the `}` character, after
    # we have found a `struct` keyword, we write the `From` implementation
    # for the struct where we implement the conversion to the struct in the
    # `web_common` crate.

    impl_from_line = "impl From<{struct_name}> for web_common::database::{table_type}::{struct_name} {{\n"
    reverse_from = "impl From<web_common::database::{table_type}::{struct_name}> for {struct_name} {{\n"

    imports = [
        "use diesel::Queryable;",
        "use diesel::QueryableByName;",
        "use diesel::Identifiable;",
        "use diesel::Insertable;",
        "use crate::schema::*;",
        "use diesel::Selectable;",
        "use serde::Deserialize;",
        "use serde::Serialize;",
        "use diesel::r2d2::ConnectionManager;",
        "use diesel::r2d2::PooledConnection;",
        "use diesel::prelude::*;",
        "use uuid::Uuid;",
        "use chrono::NaiveDateTime;",
    ]

    if table_type == "views":
        imports.append("use crate::views::schema::*;")

    with open(path, "w") as file:
        # First, we write out the macros for clippy.
        file.write(
            "#![allow(unused)]\n"
            "#![allow(clippy::all)]\n\n"
        )

        # Then, we write the import statements.
        for import_statement in imports:
            file.write(f"{import_statement}\n")

        # Then, we write the structs.
        file.write("\n")

        for struct in tqdm(
            struct_metadatas,
            desc=f"Writing {table_type} to backend",
            unit="struct",
            leave=False,
        ):


            # First of all, we write out the struct.
            struct.write_to(file, diesel=table_type)

            file.write(impl_from_line.format(
                struct_name=struct.name, table_type=table_type
            ))
            file.write(
                f"    fn from(item: {struct.name}) -> Self {{\n"
                "        Self {\n"
            )
            for attribute in struct.attributes:
                file.write(f"            {attribute.name}: item.{attribute.name},\n")
            file.write(
                "        }\n"
                "    }\n"
                "}\n\n"
            )

            file.write(reverse_from.format(
                struct_name=struct.name, table_type=table_type
            ))
            file.write(
                f"    fn from(item: web_common::database::{table_type}::{struct.name}) -> Self {{\n"
                "        Self {\n"
            )
            for attribute in struct.attributes:
                file.write(f"            {attribute.name}: item.{attribute.name},\n")
            file.write(
                "        }\n"
                "    }\n"
                "}\n\n"
            )

            # We now generate the `get` method for the diesel struct.
            # This method receives the ID of the struct and returns the
            # struct from the database.
            #
            # ```rust
            # pub fn get(
            #     id: Uuid,
            #     connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>
            # ) -> Result<Self, diesel::Error> {
            #     login_providers::dsl::login_providers
            #         .filter(login_providers::dsl::id.eq(provider_id))
            #         .first::<Self>(&mut conn)
            # }
            # ```

            file.write(
                f"impl {struct.name} {{\n"
            )

            # For all tables we implement a `all` method that retrieves all of
            # the rows in the table structured as a vector of the struct.

            file.write(
                "    /// Get all of the structs from the database.\n"
                "    ///\n"
                "    /// # Arguments\n"
                "    /// * `limit` - The maximum number of structs to retrieve.\n"
                "    /// * `connection` - The connection to the database.\n"
                "    ///\n"
                f"    pub fn all(\n"
                "        limit: Option<i64>,\n"
                "        connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>\n"
                "    ) -> Result<Vec<Self>, diesel::result::Error> {\n"
            )
            if table_type == "tables":
                file.write(f"        use crate::schema::{struct.table_name};\n")
            else:
                file.write(
                    f"        use crate::views::schema::{struct.table_name};\n"
                )
            # If the limit is None, we do not apply any limit to the query.
            file.write(
                f"        let query = {struct.table_name}::dsl::{struct.table_name};\n"
                "        if let Some(limit) = limit {\n"
                "            query.limit(limit).load::<Self>(connection)\n"
                "        } else {\n"
                "            query.load::<Self>(connection)\n"
                "        }\n"
                "    }\n"
            )

            if table_metadatas.has_primary_key(struct.table_name):
                primary_key_name, _ = table_metadatas.get_primary_key_name_and_type(
                    struct.table_name
                )
                primary_key = struct.get_attribute_by_name(primary_key_name)
            elif table_metadatas.is_view(struct.table_name):
                primary_key = struct.get_attribute_by_name("id")

            # If the current struct type we are processing is a table, we also provide
            # the insert and insert_or_update methods. The insert method inserts a new
            # row in the table with the values of the struct. The insert_or_update method
            # inserts a new row in the table with the values of the struct if the row does
            # not exist, otherwise it updates the row with the values of the struct.
            # Additionally, we also implement the delete method.
            if table_type == "tables":
                file.write(
                    "    /// Insert the struct into the database.\n"
                    "    ///\n"
                    "    /// # Arguments\n"
                    "    /// * `connection` - The connection to the database.\n"
                    "    ///\n"
                    "    pub fn insert(\n"
                    "        &self,\n"
                    "        connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>\n"
                    "    ) -> Result<Self, diesel::result::Error> {\n"
                    f"        diesel::insert_into({struct.table_name}::table)\n"
                    f"            .values(self)\n"
                    "            .get_result(connection)\n"
                    "    }\n"
                )

                if not struct.only_primary_key():
                    file.write(
                        "    /// Insert the struct into the database or update it if it already exists.\n"
                        "    ///\n"
                        "    /// # Arguments\n"
                        "    /// * `connection` - The connection to the database.\n"
                        "    ///\n"
                        "    pub fn insert_or_update(\n"
                        "        &self,\n"
                        "        connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>\n"
                        "    ) -> Result<Self, diesel::result::Error> {\n"
                        f"        diesel::insert_into({struct.table_name}::table)\n"
                        f"            .values(self)\n"
                        f"            .on_conflict({struct.table_name}::dsl::{primary_key.name})\n"
                        f"            .do_update()\n"
                        f"            .set(self)\n"
                        f"            .get_result(connection)\n"
                        "    }\n"
                    )

                file.write(
                    "    /// Delete the struct from the database.\n"
                    "    ///\n"
                    "    /// # Arguments\n"
                    "    /// * `connection` - The connection to the database.\n"
                    "    ///\n"
                    "    pub fn delete(\n"
                    "        &self,\n"
                    "        connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>\n"
                    "    ) -> Result<usize, diesel::result::Error> {\n"
                    f"        diesel::delete({struct.table_name}::dsl::{struct.table_name}\n"
                    f"            .filter({struct.table_name}::dsl::{primary_key.name}.eq(self.{primary_key.name}))\n"
                    "        )\n"
                    "        .execute(connection)\n"
                    "    }\n"
                )

            if primary_key is not None:
                file.write(
                    "    /// Get the struct from the database by its ID.\n"
                    "    ///\n"
                    "    /// # Arguments\n"
                    f"    /// * `{primary_key.name}` - The ID of the struct to get.\n"
                    "    /// * `connection` - The connection to the database.\n"
                    "    ///\n"
                    "    pub fn get(\n"
                    f"        {primary_key.name}: {primary_key.data_type()},\n"
                    "        connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>\n"
                    "    ) -> Result<Self, diesel::result::Error> {\n"
                )
                if table_type == "tables":
                    file.write(
                        f"        use crate::schema::{struct.table_name};\n"
                    )
                else:
                    file.write(
                        f"        use crate::views::schema::{struct.table_name};\n"
                    )
                file.write(
                    f"        {struct.table_name}::dsl::{struct.table_name}\n"
                    f"            .filter({struct.table_name}::dsl::{primary_key.name}.eq({primary_key.name}))\n"
                    "            .first::<Self>(connection)\n"
                    "    }\n"
                )

            # For each of the columns in the struct that are required to be UNIQUE
            # in the SQL, as defined by the get_unique_constraint_columns method, we
            # implement methods of the form `from_{column_name}` that retrieves the
            # struct by the unique column. In the case of a view, we first retrieve
            # the associated base table and secondarily we execute with a get the
            # struct by the id column from the obtained base table.
            for unique_column in table_metadatas.get_unique_constraint_columns(
                struct.table_name
            ):
                if table_type == "views":
                    # In the case of view, we do not receive simple string as unique column
                    # but instances of ViewColumn.
                    attribute = struct.get_attribute_by_name(
                        unique_column.alias_name
                    )
                    attribute_data_type = attribute.data_type()
                    if attribute_data_type == "String":
                        attribute_data_type = "&str"

                    struct_name = struct_name_from_table_name(
                        unique_column.table_name
                    )

                    file.write(
                        f"    /// Get the struct from the database by its {unique_column.alias_name}.\n"
                        "    ///\n"
                        "    /// # Arguments\n"
                        f"    /// * `{unique_column.alias_name}` - The {unique_column.alias_name} of the struct to get.\n"
                        "    /// * `connection` - The connection to the database.\n"
                        "    ///\n"
                        f"    pub fn from_{unique_column.alias_name}(\n"
                        f"        {unique_column.alias_name}: {attribute_data_type},\n"
                        "        connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>\n"
                        "    ) -> Result<Self, diesel::result::Error> {\n"
                        f"        {struct_name}::from_{unique_column.column_name}({unique_column.alias_name}, connection)\n"
                        f"            .map(|entry| {struct.name}::get(entry.{primary_key.name}, connection))\n"
                        "    }\n"
                    )

                else:
                    attribute = struct.get_attribute_by_name(unique_column)
                    attribute_data_type = attribute.data_type()
                    if attribute_data_type == "String":
                        attribute_data_type = "&str"
                    file.write(
                        f"    /// Get the struct from the database by its {unique_column}.\n"
                        "    ///\n"
                        "    /// # Arguments\n"
                        f"    /// * `{unique_column}` - The {unique_column} of the struct to get.\n"
                        "    /// * `connection` - The connection to the database.\n"
                        "    ///\n"
                        f"    pub fn from_{unique_column}(\n"
                        f"        {unique_column}: {attribute_data_type},\n"
                        "        connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>\n"
                        "    ) -> Result<Self, diesel::result::Error> {\n"
                        f"        use crate::schema::{struct.table_name};\n"
                        f"        {struct.table_name}::dsl::{struct.table_name}\n"
                        f"            .filter({struct.table_name}::dsl::{unique_column}.eq({unique_column}))\n"
                        "            .first::<Self>(connection)\n"
                        "    }\n"
                    )

            # If this table implements the `pg_trgm` index, we also
            # provide the `search` method to search for the struct
            # by a given string. The method also receives a limit
            # parameter to limit the number of results.
            if similarity_indices.has_table(struct.table_name):
                similarity_index: PGIndex = similarity_indices.get_table(
                    struct.table_name
                )

                for method_name, similarity_operator, distance_operator in PGIndices.SIMILARITY_METHODS:

                    file.write(
                        f"    /// Search for the struct by a given string by Postgres's `{method_name}`.\n"
                        "    ///\n"
                        "    /// # Arguments\n"
                        "    /// * `query` - The string to search for.\n"
                        "    /// * `limit` - The maximum number of results, by default `10`.\n"
                        "    /// * `connection` - The connection to the database.\n"
                        "    ///\n"
                        f"    pub fn {method_name}_search(\n"
                        "        query: &str,\n"
                        "        limit: Option<i32>,\n"
                        "        connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>\n"
                        "    ) -> Result<Vec<Self>, diesel::result::Error> {\n"
                        "        let limit = limit.unwrap_or(10);\n"
                        "        // If the query string is empty, we run an all query with the\n"
                        "        // limit parameter provided instead of a more complex similarity\n"
                        "        // search.\n"
                        "        if query.is_empty() {\n"
                        "            return Self::all(Some(limit as i64), connection);\n"
                        "        }\n"
                    )

                    joined_field_names = ", ".join(
                        attribute.name
                        for attribute in struct.attributes
                    )

                    if table_type == "views":
                        file.write(
                            "        let similarity_query = concat!(\n"
                            f'            "WITH selected_ids AS (",\n'
                            f'            "SELECT {similarity_index.table_name}.{primary_key.name} AS id FROM {similarity_index.table_name} ",\n'
                        )
                        if similarity_index.is_gin():
                            file.write(
                                f'            "WHERE $1 {similarity_operator} {similarity_index.arguments}  ",\n'
                                f'            "ORDER BY {method_name}($1, {similarity_index.arguments}) DESC LIMIT $2",\n'
                            )
                        else:
                            file.write(
                                f'            "ORDER BY $1 {distance_operator} {similarity_index.arguments} LIMIT $2",\n'
                            )
                        file.write(
                            '         ")",\n'
                            f'            "SELECT {joined_field_names} FROM {struct.table_name} ",\n'
                            f'            "JOIN selected_ids ON selected_ids.id = {struct.table_name}.id"\n'
                            "        );\n"
                        )
                    else:
                        file.write(
                            "        let similarity_query = concat!(\n"
                            f'            "SELECT {joined_field_names} FROM {struct.table_name} ",\n'
                        )
                        if similarity_index.is_gin():
                            file.write(
                                f'            "WHERE $1 {similarity_operator} {similarity_index.arguments} ",\n'
                                f'            "ORDER BY {method_name}($1, {similarity_index.arguments}) DESC LIMIT $2",\n'
                            )
                        else:
                            file.write(
                                f'            "ORDER BY $1 {distance_operator} {similarity_index.arguments} LIMIT $2;"\n'
                            )
                        file.write("        );\n")

                    file.write(
                        "        diesel::sql_query(similarity_query)\n"
                        "            .bind::<diesel::sql_types::Text, _>(query)\n"
                        "            .bind::<diesel::sql_types::Integer, _>(limit)\n"
                        "            .load(connection)\n"
                        "}\n"
                    )

            # Finally, we cluse the struct implementation.
            file.write("}\n")

def extract_structs(
    path: str
) -> List[StructMetadata]: 
    # A dictionary to store the table names and their
    # respective structs.
    struct_metadatas: List[StructMetadata] = []
    derives = []
    last_table_name = None
    inside_struct = False

    with open(path, "r", encoding="utf8") as file:
        document = file.read()

    for line in document.split("\n"):
        # We skip all lines beginning with `//!` as they are comments
        if line.startswith("//!"):
            continue

        # We find the table name by searching lines like
        # #[diesel(table_name = item_continuous_quantities)]
        if "table_name =" in line:
            last_table_name = line.split("=")[1].strip(" )]").split(":")[-1]

        # If we are in a derive line, we extract the derives:
        if line.startswith("#[derive("):
            derives = line.split("(")[1].strip(")]").split(", ")

        # We determine whether a new struct has started
        # by checking if the `struct` keyword is present
        # in the line.
        if "struct" in line:
            struct_name = line.split(" ")[2]

            struct_metadata = StructMetadata(
                table_name=last_table_name,
                struct_name=struct_name,
            )

            for derive in derives:
                struct_metadata.add_derive(derive)

            inside_struct = True

        if inside_struct:
            # If the current line contains the id field,
            # we store the type of the id field.
            if "pub" in line and ":" in line:
                field_name = line.strip().split(" ")[1].strip(":")
                field_type = line.split(":")[1].strip(", ")
                option = False
                if field_type.startswith("Option<"):
                    option = True
                    field_type = field_type[7:-1]
                struct_metadata.add_attribute(
                    AttributeMetadata(
                        original_name=field_name,
                        name=field_name,
                        data_type=field_type,
                        optional=option,
                    )
                )

            # We determine whether the struct has ended
            # by checking if the `}` keyword is present
            # in the line.
            if "}" in line:
                inside_struct = False
                struct_metadatas.append(struct_metadata)

    return struct_metadatas


def write_web_common_structs(
    structs: List[StructMetadata], target: str, enumeration: str
):
    """Write the structs in the target file in the `web_common` crate.

    Parameters
    ----------
    structs : List[StructMetadata]
        The list of structs to write in the target file.
    target : str
        The path where to write the structs in the `web_common` crate.
    enumeration : str
        The name of the enumeration to write in the target file.
    """
    # The derive statements to include in the `src/database/tables.rs` document
    imports = [
        "use serde::Deserialize;",
        "use serde::Serialize;",
        "use uuid::Uuid;",
        "use chrono::NaiveDateTime;",
    ]

    table_metadatas = find_foreign_keys()

    # We check that we are currently executing in the `backend` crate
    # so to make sure that the relative path to the `web_common` crate
    # is correct.
    if not os.getcwd().endswith("backend"):
        raise Exception("This script must be executed in the `backend` crate.")

    tables = open(f"../web_common/src/database/{target}.rs", "w", encoding="utf8")

    for import_statament in imports:
        tables.write(f"{import_statament}\n")

    similarity_indices: PGIndices = find_pg_trgm_indices()

    for struct in tqdm(
        structs,
        desc="Writing frontend structs",
        unit="struct",
        leave=False,
    ):
        struct_has_just_finished = False

        tables.write("#[derive(")
        tables.write(", ".join(struct.derives()))
        tables.write(")]\n")
        # We also write conditional derives for the frontend feature
        # that ask for the `frontend` feature to be enabled and derive
        # the yew::html::Properties trait for the struct.
        tables.write(
            '#[cfg_attr(feature = "frontend", derive(yew::html::Properties))]\n'
        )

        tables.write(f"pub struct {struct.name} {{\n")
        for attribute in struct.attributes:
            tables.write(
                f"    pub {attribute.name}: {attribute.format_data_type()},\n"
            )
        tables.write("}\n")

        # This variant of the struct implementation is only
        # available when in the web_common is enabled the frontend
        # feature. It provides several methods including the use
        # of GlueSQL. Fortunately, it does not force us like Diesel
        # to create yet again another duplicate of the struct.
        tables.write('#[cfg(feature = "frontend")]\n')
        tables.write(f"impl {struct.name} {{\n")
        columns = ", ".join(
            [attribute.name for attribute in struct.attributes]
        )

        # As first thing, we implement the `into_row` method for the struct. This method
        # converts the struct into a vector of `gluesql::core::ast_builder::ExprList`
        # variants, which are used to insert the struct into the GlueSQL database.
        types_and_methods = {
            "i8": "gluesql::core::ast_builder::num({})",
            "i16": "gluesql::core::ast_builder::num({})",
            "i32": "gluesql::core::ast_builder::num({})",
            "i64": "gluesql::core::ast_builder::num({})",
            "i128": "gluesql::core::ast_builder::num({})",
            "u8": "gluesql::core::ast_builder::num({})",
            "u16": "gluesql::core::ast_builder::num({})",
            "u32": "gluesql::core::ast_builder::num({})",
            "u64": "gluesql::core::ast_builder::num({})",
            "u128": "gluesql::core::ast_builder::num({})",
            "f32": "gluesql::core::ast_builder::num({})",
            "f64": "gluesql::core::ast_builder::num({})",
            "String": "gluesql::core::ast_builder::text({})",
            "Uuid": "gluesql::core::ast_builder::uuid({}.to_string())",
            "bool": "({}.into())",
            "NaiveDateTime": "gluesql::core::ast_builder::timestamp({}.to_string())",
            "DateTime<Utc>": "gluesql::core::ast_builder::timestamp({}.to_string())",
        }

        update_types_and_methods = types_and_methods.copy()
        update_types_and_methods["bool"] = "{}"

        tables.write(
            "    pub fn into_row(self) -> Vec<gluesql::core::ast_builder::ExprNode<'static>> {\n"
        )

        tables.write("        vec![\n")
        for attribute in struct.attributes:

            if attribute.optional:
                if attribute.data_type() in types_and_methods:
                    tables.write(f"            match self.{attribute.name} {{\n")
                    tables.write(
                        f"                Some({attribute.name}) => {types_and_methods[attribute.data_type()].format(attribute.name)},\n"
                    )
                    tables.write(
                        "                None => gluesql::core::ast_builder::null(),\n"
                    )
                    tables.write("            },\n")
                else:
                    raise NotImplementedError(
                        f"The type {attribute.data_type()} is not supported. "
                        f"The struct {struct.name} contains an {attribute.data_type()}. "
                    )
            elif attribute.data_type() in types_and_methods:
                tables.write(
                    f"            {types_and_methods[attribute.data_type()].format(f'self.{attribute.name}')},\n"
                )
            else:
                raise NotImplementedError(
                    f"The type {attribute.data_type()} is not supported."
                )

        tables.write("        ]\n")

        tables.write("    }\n\n")

        # We implement the `insert` method for the struct. This method
        # receives a connection to the GlueSQL database and inserts the
        # struct into the database.
        tables.write(
            f"    /// Insert the {struct.name} into the database.\n"
        )
        tables.write("    ///\n")
        tables.write("    /// # Arguments\n")
        tables.write("    /// * `connection` - The connection to the database.\n")
        tables.write("    ///\n")
        tables.write("    /// # Returns\n")
        tables.write(
            f"    /// The number of rows inserted in table {struct.name}\n"
        )
        tables.write("    pub async fn insert<C>(\n")
        tables.write("        self,\n")
        tables.write("        connection: &mut gluesql::prelude::Glue<C>,\n")
        tables.write("    ) -> Result<usize, gluesql::prelude::Error> where\n")
        tables.write(
            "        C: gluesql::core::store::GStore + gluesql::core::store::GStoreMut,\n"
        )
        tables.write("    {\n")
        tables.write("        use gluesql::core::ast_builder::*;\n")
        # We use the AST builder as much as possible so to avoid SQL injection attacks.
        tables.write(f'        table("{struct.table_name}")\n')
        tables.write("            .insert()\n")
        tables.write(f'            .columns("{columns}")\n')
        tables.write("            .values(vec![self.into_row()])\n")
        tables.write("            .execute(connection)\n")
        tables.write("            .await\n")
        tables.write("             .map(|payload| match payload {\n")
        tables.write(
            "                 gluesql::prelude::Payload::Insert ( number_of_inserted_rows ) => number_of_inserted_rows,\n"
        )
        tables.write(
            '                 _ => unreachable!("Payload must be an Insert"),\n'
        )
        tables.write("             })\n")
        tables.write("    }\n\n")

        # We implement the `get` method for the struct. This method
        # receives the ID of the struct and a connection to the GlueSQL
        # database. The method returns the struct from the database.
        tables.write(
            f"    /// Get {struct.name} from the database by its ID.\n"
        )
        tables.write("    ///\n")
        tables.write("    /// # Arguments\n")
        primary_key_name, primary_key_type = (
            table_metadatas.get_primary_key_name_and_type(
                struct.table_name
            )
        )
        rust_primary_key_type = sql_type_to_rust_type(primary_key_type)

        tables.write(
            f"    /// * `{primary_key_name}` - The ID of {struct.name} to get.\n"
        )
        tables.write("    /// * `connection` - The connection to the database.\n")
        tables.write("    ///\n")
        tables.write("    pub async fn get<C>(\n")
        tables.write(f"        {primary_key_name}: {rust_primary_key_type},\n")
        tables.write("        connection: &mut gluesql::prelude::Glue<C>,\n")
        tables.write(
            "    ) -> Result<Option<Self>, gluesql::prelude::Error> where\n"
        )
        tables.write(
            "        C: gluesql::core::store::GStore + gluesql::core::store::GStoreMut,\n"
        )
        tables.write("    {\n")
        tables.write("        use gluesql::core::ast_builder::*;\n")
        # We use the AST builder as much as possible so to avoid SQL injection attacks.
        tables.write(
            f'        let select_row = table("{struct.table_name}")\n'
        )
        tables.write("            .select()\n")
        tables.write(
            f'            .filter(col("id").eq({primary_key_name}.to_string()))\n'
        )
        tables.write(f'            .project("{columns}")\n')
        tables.write("            .limit(1)\n")
        tables.write("            .execute(connection)\n")
        tables.write("            .await?;\n")
        tables.write("         Ok(select_row.select()\n")
        tables.write("            .unwrap()\n")
        tables.write("            .map(Self::from_row)\n")
        tables.write("            .collect::<Vec<_>>()\n")
        tables.write("            .pop())\n")
        tables.write("    }\n\n")

        # We implement the `delete` method for the struct. This method deletes
        # the struct from the GlueSQL database.
        tables.write(f"    /// Delete {struct.name} from the database.\n")
        tables.write("    ///\n")
        tables.write("    /// # Arguments\n")
        tables.write(
            f"    /// * `{primary_key_name}` - The ID of the struct to delete.\n"
        )
        tables.write("    /// * `connection` - The connection to the database.\n")
        tables.write("    ///\n")
        tables.write("    /// # Returns\n")
        tables.write("    /// The number of rows deleted.\n")
        tables.write("    pub async fn delete_from_id<C>(\n")
        tables.write(f"        {primary_key_name}: {rust_primary_key_type},\n")
        tables.write("        connection: &mut gluesql::prelude::Glue<C>,\n")
        tables.write("    ) -> Result<usize, gluesql::prelude::Error> where\n")
        tables.write(
            "        C: gluesql::core::store::GStore + gluesql::core::store::GStoreMut,\n"
        )
        tables.write("    {\n")
        tables.write("        use gluesql::core::ast_builder::*;\n")
        # We use the AST builder as much as possible so to avoid SQL injection attacks.
        tables.write(f'        table("{struct.table_name}")\n')
        tables.write("            .delete()\n")
        tables.write('            .filter(col("id").eq(id.to_string()))\n')
        tables.write("            .execute(connection)\n")
        tables.write("            .await\n")
        tables.write("             .map(|payload| match payload {\n")
        tables.write(
            "                 gluesql::prelude::Payload::Delete(number_of_deleted_rows) => number_of_deleted_rows,\n"
        )
        tables.write(
            '                 _ => unreachable!("Payload must be a Delete"),\n'
        )
        tables.write("             })\n")
        tables.write("    }\n\n")

        # We implement the `delete` method for the struct. This method deletes
        # the current instance of the struct from the GlueSQL database.
        tables.write(
            f"    /// Delete the current instance of {struct.name} from the database.\n"
        )
        tables.write("    ///\n")
        tables.write("    /// # Arguments\n")
        tables.write("    /// * `connection` - The connection to the database.\n")
        tables.write("    ///\n")
        tables.write("    /// # Returns\n")
        tables.write("    /// The number of rows deleted.\n")
        tables.write("    pub async fn delete<C>(\n")
        tables.write("        self,\n")
        tables.write("        connection: &mut gluesql::prelude::Glue<C>,\n")
        tables.write("    ) -> Result<usize, gluesql::prelude::Error> where\n")
        tables.write(
            "        C: gluesql::core::store::GStore + gluesql::core::store::GStoreMut,\n"
        )
        tables.write("    {\n")
        tables.write("        Self::delete_from_id(self.id, connection).await\n")
        tables.write("    }\n")

        # We implement the `update` method for the struct. This method updates
        # the struct in the GlueSQL database.
        tables.write("    /// Update the struct in the database.\n")
        tables.write("    ///\n")
        tables.write("    /// # Arguments\n")
        tables.write("    /// * `connection` - The connection to the database.\n")
        tables.write("    ///\n")
        tables.write("    /// # Returns\n")
        tables.write("    /// The number of rows updated.\n")
        tables.write("    pub async fn update<C>(\n")
        tables.write("        self,\n")
        tables.write("        connection: &mut gluesql::prelude::Glue<C>,\n")
        tables.write("    ) -> Result<usize, gluesql::prelude::Error> where\n")
        tables.write(
            "        C: gluesql::core::store::GStore + gluesql::core::store::GStoreMut,\n"
        )
        tables.write("    {\n")
        tables.write("        use gluesql::core::ast_builder::*;\n")
        # We use the AST builder as much as possible so to avoid SQL injection attacks.

        # First, we determine whether the current struct has at least an optional field.

        if struct.contains_optional_fields():
            tables.write(
                f'        let mut update_row = table("{struct.table_name}")\n'
            )
        else:
            tables.write(f'        table("{struct.table_name}")\n')
        tables.write("            .update()")

        if struct.contains_only_optional_fields():
            raise NotImplementedError(
                f"The struct {struct.name} does not contain any non-optional attributes. "
                "It is not well defined how to update a struct without any attributes."
            )

        for attribute in struct.attributes:
            if attribute.optional:
                # We handle this in the next loop
                continue
            if attribute.data_type() in update_types_and_methods:
                conversion = update_types_and_methods[attribute.data_type()].format(
                    f"self.{attribute.name}"
                )
                tables.write(f'        \n.set("{attribute.name}", {conversion})')
            else:
                raise NotImplementedError(
                    f"The type {attribute.data_type()} is not supported."
                    f"The struct {struct.name} contains an {attribute.data_type()}."
                )

        if struct.contains_optional_fields():
            tables.write(";\n")

        # After all of the non-optional fields, we handle the optional fields.
        for attribute in struct.attributes:
            if not attribute.optional:
                continue
            conversion = update_types_and_methods[attribute.data_type()].format(
                f"self.{attribute.name}"
            )
            if attribute.data_type() in update_types_and_methods:
                tables.write(
                    f"        if let Some({attribute.name}) = self.{attribute.name} {{\n"
                )
                tables.write(
                    f'            update_row = update_row.set("{attribute.name}", {update_types_and_methods[attribute.data_type()].format(attribute.name)});\n'
                )
                tables.write("        }\n")
            else:
                raise NotImplementedError(
                    f"The type {attribute.data_type()} is not supported. "
                    f"The struct {attribute.name} contains an {attribute.data_type()}. "
                )

        if struct.contains_optional_fields():
            tables.write("            update_row.execute(connection)\n")
        else:
            tables.write("            .execute(connection)\n")
        tables.write("            .await\n")
        tables.write("             .map(|payload| match payload {\n")
        tables.write(
            "                 gluesql::prelude::Payload::Update(number_of_updated_rows) => number_of_updated_rows,\n"
        )
        tables.write(
            '                 _ => unreachable!("Expected Payload::Update")\n'
        )
        tables.write("})\n")
        tables.write("    }\n\n")

        # Next, we implement the `update_or_insert` method for the struct. This method
        # inserts the struct into the GlueSQL database if it does not exist, otherwise
        # it updates the struct in the database.
        tables.write(
            "    /// Update the struct in the database if it exists, otherwise insert it.\n"
        )
        tables.write("    ///\n")
        tables.write("    /// # Arguments\n")
        tables.write("    /// * `connection` - The connection to the database.\n")
        tables.write("    ///\n")
        tables.write("    /// # Returns\n")
        tables.write("    /// The number of rows updated or inserted.\n")
        tables.write("    pub async fn update_or_insert<C>(\n")
        tables.write("        self,\n")
        tables.write("        connection: &mut gluesql::prelude::Glue<C>,\n")
        tables.write("    ) -> Result<usize, gluesql::prelude::Error> where\n")
        tables.write(
            "        C: gluesql::core::store::GStore + gluesql::core::store::GStoreMut,\n"
        )
        tables.write("    {\n")
        tables.write(
            "        let number_of_rows = self.clone().update(connection).await?;\n"
        )
        tables.write("        if number_of_rows == 0 {\n")
        tables.write("            self.insert(connection).await\n")
        tables.write("        } else {\n")
        tables.write("            Ok(number_of_rows)\n")
        tables.write("        }\n")
        tables.write("    }\n")

        # We implement the `all` method for the struct. This method returns all of the
        # structs in the GlueSQL database.
        tables.write(f"    /// Get all {struct.name} from the database.\n")
        tables.write("    ///\n")
        tables.write("    /// # Arguments\n")
        tables.write("    /// * `connection` - The connection to the database.\n")
        tables.write("    ///\n")
        tables.write("    pub async fn all<C>(\n")
        tables.write("        connection: &mut gluesql::prelude::Glue<C>,\n")
        tables.write("    ) -> Result<Vec<Self>, gluesql::prelude::Error> where\n")
        tables.write(
            "        C: gluesql::core::store::GStore + gluesql::core::store::GStoreMut,\n"
        )
        tables.write("    {\n")
        tables.write("        use gluesql::core::ast_builder::*;\n")
        tables.write(
            f'        let select_row = table("{struct.table_name}")\n'
        )
        tables.write("            .select()\n")
        tables.write(f'            .project("{columns}")\n')
        tables.write("            .execute(connection)\n")
        tables.write("            .await?;\n")
        tables.write("        Ok(select_row.select()\n")
        tables.write("            .unwrap()\n")
        tables.write("            .map(Self::from_row)\n")
        tables.write("            .collect::<Vec<_>>())\n")
        tables.write("    }\n")

        # We implement the `from_row` method for the struct. This method
        # receives a row from the GlueSQL database, which is a `HashMap<&str, &&Value>`.
        # The method returns the struct from the row.
        tables.write(
            "    pub fn from_row(row: std::collections::HashMap<&str, &gluesql::prelude::Value>) -> Self {\n"
        )
        tables.write("        Self {\n")

        clonables = {
            "bool": "Bool",
            "i8": "I8",
            "i16": "I16",
            "i32": "I32",
            "i64": "I64",
            "i128": "I128",
            "u8": "U8",
            "u16": "U16",
            "u32": "U32",
            "u64": "U64",
            "u128": "U128",
            "f32": "F32",
            "f64": "F64",
            "String": "Str",
            "NaiveDateTime": "Timestamp",
        }

        for attribute in struct.attributes:
            if attribute.format_data_type() == "Uuid":
                tables.write(
                    f'            {attribute.name}: match row.get("{attribute.name}").unwrap() {{\n'
                )
                tables.write(
                    f"                gluesql::prelude::Value::Uuid({attribute.name}) => Uuid::from_u128(*{attribute.name}),\n"
                )
                tables.write(
                    '                _ => unreachable!("Expected Uuid"),\n'
                )
                tables.write("            },\n")
            elif attribute.format_data_type() == "Option<Uuid>":
                tables.write(
                    f'            {attribute.name}: match row.get("{attribute.name}").unwrap() {{\n'
                )
                tables.write(
                    "                gluesql::prelude::Value::Null => None,\n"
                )
                tables.write(
                    f"                gluesql::prelude::Value::Uuid({attribute.name}) => Some(Uuid::from_u128(*{attribute.name})),\n"
                )
                tables.write(
                    '                _ => unreachable!("Expected Uuid"),\n'
                )
                tables.write("            },\n")
            elif attribute.implements_clone():
                if attribute.optional:
                    tables.write(
                        f'            {attribute.name}: match row.get("{attribute.name}").unwrap() {{\n'
                    )
                    tables.write(
                        "                gluesql::prelude::Value::Null => None,\n"
                    )
                    tables.write(
                        f"                gluesql::prelude::Value::{clonables[attribute.data_type()]}({attribute.name}) => Some({attribute.name}.clone()),\n"
                    )
                    tables.write(
                        f'                _ => unreachable!("Expected {clonables[attribute.data_type()]}")\n'
                    )
                    tables.write("            },\n")
                else:
                    tables.write(
                        f'            {attribute.name}: match row.get("{attribute.name}").unwrap() {{\n'
                    )
                    tables.write(
                        f"                gluesql::prelude::Value::{clonables[attribute.data_type()]}({attribute.name}) => {attribute.name}.clone(),\n"
                    )
                    tables.write(
                        f'                _ => unreachable!("Expected {clonables[attribute.data_type()]}")\n'
                    )
                    tables.write("            },\n")
            else:
                raise NotImplementedError(
                    f"Found an unsupported attribute type for the struct {struct_name}: {attribute.data_type()} "
                    f"for the attribute {attribute.name}."
                )
        tables.write("        }\n")
        tables.write("    }\n")

        # And finally we close the struct implementation
        tables.write("}\n")

    tables.close()


def get_view_names() -> List[str]:
    """Returns list of view names.

    Implementative details
    -------------------------
    The view names are extracted from the `migrations` directory. The view names are extracted
    from the `up.sql` file in each of the directories. The view names are extracted by searching
    for the `CREATE VIEW` statements in the SQL file.
    """
    view_names = []
    for directory in os.listdir("migrations"):
        if not os.path.isdir(f"migrations/{directory}"):
            continue
        with open(f"migrations/{directory}/up.sql", "r", encoding="utf8") as file:
            content = file.read()
        for line in content.split("\n"):
            if "CREATE VIEW" in line or "CREATE MATERIALIZED VIEW" in line:
                view_name = line.rsplit(" ", maxsplit=2)[1]
                view_names.append(view_name)
    return view_names


def get_views(cursor) -> List[str]:
    """Return list with the view names"""
    cursor.execute(
        "SELECT table_name FROM information_schema.views WHERE table_schema = 'public';"
    )
    views = cursor.fetchall()
    return views


def map_postgres_to_rust_type(pg_type):
    pg_to_rust_types = {
        "uuid": "diesel::sql_types::Uuid",
        "text": "diesel::sql_types::Text",
        "timestamp without time zone": "diesel::sql_types::Timestamp",
        "character varying": "diesel::sql_types::Text",
        "integer": "diesel::sql_types::Integer",
    }

    if pg_type in pg_to_rust_types:
        return pg_to_rust_types[pg_type]

    raise NotImplementedError(f'Postgres type "{pg_type}" is not supported.')


def generate_diesel_schema(view_name: str, columns: List[ViewColumn]) -> str:
    schema_code = "diesel::table! {\n"
    schema_code += f"    {view_name} (id) {{\n"
    for column in columns:
        if column.nullable:
            schema_code += f"        {column.alias_name} -> diesel::sql_types::Nullable<{map_postgres_to_rust_type(column.data_type)}>,\n"
        else:
            schema_code += f"        {column.alias_name} -> {map_postgres_to_rust_type(column.data_type)},\n"
    schema_code += "    }\n"
    schema_code += "}\n"
    return schema_code


def generate_view_schema():
    """Generate the view schema.

    Implementative details
    -------------------------
    We generate the views by connecting to the database and querying the `information_schema`
    tables. We then write the views to the file `src/views/schema.rs`. The database is a postgres
    database, and the connection string is read from the environment variable `DATABASE_URL`.
    """
    # We load the data from the environment variables from the `.env` file
    # at `../.env`.
    conn, cursor = get_cursor()

    # Getting the list of views
    views = get_views(cursor)
    table_metadata = find_foreign_keys()

    # We open the file to write the schema
    schema_file = open("src/views/schema.rs", "w", encoding="utf8")

    # Generating Diesel schema for each view
    for view in views:
        view_name = view[0]
        columns = table_metadata.extract_view_columns(view_name)
        schema_code = generate_diesel_schema(view_name, columns)
        schema_file.write(schema_code + "\n")

    # Closing the cursor and connection
    cursor.close()
    conn.close()


def check_schema_completion():
    """Check the view schema completion.

    Implementative details
    -------------------------
    Diesel does not support the `CREATE VIEW` statements, and as such we need to manually
    create the views in the schema file `src/views/schema.rs`. This script check that all
    of the view names are present in the schema file.
    """
    view_names = get_view_names()
    with open("src/views/schema.rs", "r", encoding="utf8") as file:
        content = file.read()
    for view_name in view_names:
        if view_name not in content:
            raise NotImplementedError(
                f"View {view_name} is not present in the schema file."
            )


def generate_view_structs():
    """Generate the view structs.

    Implementative details
    -------------------------
    Since Diesel solely supports the generation of the table structs, we need to use
    this custom script to generate the view structs. The view structs are generated
    starting from the schema file `src/views/schema.rs` and are written to the file
    `src/views/views.rs`. The view structs are generated by copying the views structs
    defined in the views schema, with the data types appropriately changed to match the
    view schema.

    An example of a schema entry is:

    ```rust
    diesel::table! {
        users_view (id) {
            id -> Uuid,
            first_name -> Nullable<Text>,
            middle_name -> Nullable<Text>,
            last_name -> Nullable<Text>,
            created_at -> Timestamp,
            updated_at -> Timestamp,
        }
    }
    ```
    """

    with open("src/views/schema.rs", "r", encoding="utf8") as file:
        schema = file.read()

    views = open("src/views/views.rs", "w", encoding="utf8")

    if len(schema) == 0:
        views.close()
        return

    data_types = {
        "diesel::sql_types::Uuid": "Uuid",
        "diesel::sql_types::Text": "String",
        "diesel::sql_types::Timestamp": "NaiveDateTime",
        "diesel::sql_types::Integer": "i32",
    }

    imports = [
        "use serde::Deserialize;",
        "use serde::Serialize;",
        "use diesel::Queryable;",
        "use diesel::QueryableByName;",
        "use uuid::Uuid;",
        "use chrono::NaiveDateTime;",
        "use diesel::r2d2::PooledConnection;",
        "use diesel::r2d2::ConnectionManager;",
        "use diesel::prelude::*;",
    ]

    derives = [
        "Deserialize",
        "Serialize",
        "Debug",
        "PartialEq",
        "Queryable",
        "QueryableByName",
    ]

    for import_statement in imports:
        views.write(f"{import_statement}\n")

    view_structs = []
    last_line_was_table = False
    brackets_count = 0

    for line in schema.split("\n"):
        if "{" in line:
            brackets_count += 1
        if "}" in line:
            brackets_count -= 1

        if last_line_was_table:
            view_name = line.split("(")[0].strip(" ")
            view_struct = StructMetadata(
                struct_name=struct_name_from_table_name(view_name),
                table_name=view_name,
            )
            view_structs.append(view_struct)

        if "diesel::table! {" in line:
            last_line_was_table = True
            continue
        else:
            last_line_was_table = False

        if "->" in line:
            (attribute, data_type) = line.strip(" ,").split(" -> ")
            optional = False
            if "Nullable<" in data_type:
                optional = True
                data_type = data_type.split("Nullable<", maxsplit=1)[1].strip(">")
            view_struct.add_attribute(
                AttributeMetadata(
                    original_name=attribute,
                    name=attribute,
                    data_type=data_types[data_type],
                    optional=optional,
                )
            )

        # If we have found the end of the struct, we write the struct
        # to the file.
        if brackets_count == 0 and view_name is not None:
            # First, we generate the derives.
            for derive in derives:
                view_struct.add_derive(derive)
            
            views.write("#[derive(")
            views.write(", ".join(view_struct.derives()))
            views.write(")]\n")

            # Then, we write the table_name attribute to link
            # the struct to the view.
            views.write(
                f"#[diesel(table_name = crate::views::schema::{view_struct.table_name})]\n"
            )

            # We write the struct definition
            views.write(f"pub struct {view_struct.name} {{\n")
            for attribute in view_struct.attributes:
                views.write(
                    f"    pub {attribute.name}: {attribute.format_data_type()},\n"
                )
            views.write("}\n\n")

        if brackets_count == 0:
            attributes = {}
            view_name = None

    view_names_from_sql = get_view_names()
    for view_struct in view_structs:
        assert (
            view_struct.table_name in view_names_from_sql
        ), f'View "{view_struct.table_name}" is not present in the "schema.rs" file.'

    views.close()


def generate_nested_structs(
    path: str, struct_metadatas: List[StructMetadata]
) -> List[StructMetadata]:
    """Generate the nested structs.

    Implementative details
    -------------------------
    Normally, a table struct is generated from a row in the database. However, in some cases,
    a table row may contain a reference id to another table. In this case, we generate a nested
    struct for the referenced table. Depending on whether this referenced row contains also a
    reference to another table, we may generate the nested struct version of the referenced row
    or the flat version, i.e. the row itself.

    For each table, we query the postgres to get the foreign keys. We then generate the nested
    structs for the referenced tables. The nested structs are written to the file `src/models.rs`.
    """
    tables_metadata = find_foreign_keys()
    similarity_indices: PGIndices = find_pg_trgm_indices()

    # We open the file to write the nested structs
    tables = open(path, "w", encoding="utf8")

    # Preliminarly, we write a docstring at the very head
    # of this submodule to explain what it does and warn the
    # reader not to write anything in this file as it is
    # automatically generated.
    tables.write(
        "//! This module contains the nested structs for the database tables.\n"
    )
    tables.write("//!\n")
    tables.write(
        "//! This file is automatically generated. Do not write anything here.\n"
    )
    tables.write("\n")

    # We start with the necessary imports.
    imports = [
        "use serde::Deserialize;",
        "use serde::Serialize;",
        "use diesel::r2d2::ConnectionManager;",
        "use diesel::r2d2::PooledConnection;",
        "use crate::models::*;",
        "use crate::views::views::*;",
    ]

    for import_statement in imports:
        tables.write(f"{import_statement}\n")

    def get_struct_by_table_name(table_name: str) -> StructMetadata:
        for struct in struct_metadatas:
            if struct.table_name == table_name:
                return struct
        raise ValueError(f"Table name {table_name} not found in the struct metadata.")

    new_struct_metadatas = []

    # For each of the struct, we generated the Nested{struct_name} version
    # if the struct contains a reference to another struct.
    for struct in tqdm(struct_metadatas, desc="Generating nested structs", leave=False, unit="struct"):
        # If the struct does not have any foreign keys, we skip it
        if not tables_metadata.has_foreign_keys(
            struct.table_name
        ) and not tables_metadata.is_view(struct.table_name):
            continue

        foreign_keys = tables_metadata.get_foreign_keys(struct.table_name)

        primary_key_attribute = None
        if tables_metadata.has_primary_key(struct.table_name):
            # We implement the `get` method, which returns the nested struct
            # from a provided row primary key.
            primary_key_attribute, primary_key_type = (
                tables_metadata.get_primary_key_name_and_type(struct.table_name)
            )
            rust_primary_key_type = sql_type_to_rust_type(primary_key_type)

            # If all of the foreign keys are equal to the primary key, we skip
            # the struct as it is a self-referencing struct.
            if all(fk == primary_key_attribute for fk in foreign_keys):
                continue

        else:
            # If the table does not have a primary key, as may happen in the context
            # of a view, we use the attribyte `id` as the primary key.
            for attribute in struct.attributes:
                if attribute.name == "id":
                    primary_key_attribute = "id"
                    primary_key_type = attribute.data_type()
                    rust_primary_key_type = primary_key_type
                    break
            if primary_key_attribute is None:
                raise ValueError(
                    f"Table {struct.table_name} does not have a primary key nor an `id` attribute. "
                    f"It has the following attributes: {struct.attributes}"
                )

        if primary_key_attribute not in foreign_keys:
            foreign_keys.append(primary_key_attribute)

        nested_struct_name = f"Nested{struct.name}"
        new_struct_metadata = StructMetadata(nested_struct_name, struct.table_name)

        # We write the Nested{struct_name} struct
        tables.write("#[derive(")
        new_struct_metadata.add_derive("Debug")
        new_struct_metadata.add_derive("Serialize")
        new_struct_metadata.add_derive("Deserialize")
        new_struct_metadata.add_derive("PartialEq")
        tables.write(", ".join(new_struct_metadata.derives()))
        tables.write(")]\n")

        tables.write(f"pub struct {nested_struct_name} {{\n")
        for attribute in struct.attributes:

            # If the current attribute is among the foreign keys, we replace it
            # with the foreign struct. This struct may be also nested if the foreign
            # table has foreign keys, which we check by using the `has_foreign_keys`
            # method of the `tables_metadata` object.
            if attribute.name in foreign_keys:
                if attribute.name == primary_key_attribute:
                    foreign_struct = struct
                    normalized_attribute_name = "inner"
                else:
                    foreign_key_table_name = tables_metadata.get_foreign_key_table_name(
                        struct.table_name, attribute.name
                    )
                    normalized_attribute_name = attribute.name
                    foreign_struct = get_struct_by_table_name(foreign_key_table_name)

                if normalized_attribute_name.endswith("_id"):
                    normalized_attribute_name = normalized_attribute_name[:-3]
                if attribute.name != primary_key_attribute and (
                    tables_metadata.foreign_key_table_has_foreign_keys(
                        struct.table_name, attribute.name
                    )
                    or tables_metadata.is_view(struct.table_name)
                    and primary_key_attribute == "id"
                ):
                    # If the nested version of the foreign struct is already present,
                    # we cannot use it save risking the struct be extremely nested.
                    # Think for instance a leaf taxon struct containing its parent taxon
                    # and the parent taxon containing its parent taxon and so on.
                    if struct.name == foreign_struct.name:
                        new_attribute = AttributeMetadata(
                            original_name=attribute.name,
                            name=normalized_attribute_name,
                            data_type=foreign_struct,
                            optional=attribute.optional,
                        )
                    else:
                        new_attribute = AttributeMetadata(
                            original_name=attribute.name,
                            name=normalized_attribute_name,
                            data_type=f"Nested{foreign_struct.name}",
                            optional=attribute.optional,
                        )
                    tables.write(
                        f"    pub {new_attribute.name}: {new_attribute.format_data_type()},\n"
                    )
                    new_struct_metadata.add_attribute(new_attribute)
                else:
                    new_attribute = AttributeMetadata(
                        original_name=attribute.name,
                        name=normalized_attribute_name,
                        data_type=foreign_struct,
                        optional=attribute.optional,
                    )
                    tables.write(
                        f"    pub {new_attribute.name}: {new_attribute.format_data_type()},\n"
                    )
                    new_struct_metadata.add_attribute(new_attribute)
            else:
                continue
        tables.write("}\n\n")

        # We implement the all for the nested structs

        # First, we implement a method that will be reused by several of the following methods,
        # including the all, get and search ones: a method that given the flat struct and a connection
        # to the database returns a result containing the nested struct.
        tables.write(
            f"impl {nested_struct_name} {{\n"
            "    /// Convert the flat struct to the nested struct.\n"
            "    ///\n"
            "    /// # Arguments\n"
            "    /// * `flat_struct` - The flat struct.\n"
            "    /// * `connection` - The database connection.\n"
            "    pub fn from_flat(\n"
            f"        flat_struct: {struct.name},\n"
            "        connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>,\n"
            "    ) -> Result<Self, diesel::result::Error> {\n"
            "        Ok(Self {\n"
        )
        for attribute in new_struct_metadata.attributes:
            if attribute.name == "inner":
                continue
            if (
                attribute.data_type() == new_struct_metadata.name
                or struct.has_attribute(attribute)
            ):
                tables.write(
                    f"            {attribute.name}: flat_struct.{attribute.name},\n"
                )
                continue
            if attribute.optional:
                tables.write(
                    f"            {attribute.name}: flat_struct.{attribute.original_name}.map(|flat_struct| {attribute.data_type()}::get(flat_struct, connection)).transpose()?,\n"
                )
            else:
                tables.write(
                    f"            {attribute.name}: {attribute.data_type()}::get(flat_struct.{attribute.original_name}, connection)?,\n"
                )
        if any(
            attribute.name == primary_key_attribute for attribute in struct.attributes
        ):
            tables.write(f"                inner: flat_struct,\n")
        tables.write("        })\n" "    }\n" "}\n")

        # Then we implement the all query.

        tables.write(
            f"impl {nested_struct_name} {{\n"
            "    /// Get all the nested structs from the database.\n"
            "    ///\n"
            "    /// # Arguments\n"
            "    /// * `limit` - The maximum number of rows to return.\n"
            "    /// * `connection` - The database connection.\n"
            "    pub fn all(\n"
            "        limit: Option<i64>,\n"
            "        connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>,\n"
            "    ) -> Result<Vec<Self>, diesel::result::Error> {\n"
            f"        {struct.name}::all(limit, connection)?.into_iter().map(|flat_struct| Self::from_flat(flat_struct, connection)).collect()\n"
            "    }\n"
            "}\n"
        )

        tables.write(
            f"impl {new_struct_metadata.name} {{\n"
            "    /// Get the nested struct from the provided primary key.\n"
            "    ///\n"
            "    /// # Arguments\n"
            f"    /// * `{primary_key_attribute}` - The primary key of the row.\n"
            "    /// * `connection` - The database connection.\n"
            "    pub fn get(\n"
            f"        {primary_key_attribute}: {rust_primary_key_type},\n"
            "        connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>,\n"
            "    ) -> Result<Self, diesel::result::Error>\n"
            "    {\n"
            f"       {struct.name}::get({primary_key_attribute}, connection).and_then(|flat_struct| Self::from_flat(flat_struct, connection))\n"
            "    }\n"
            "}\n"
        )

        # For each of the columns in the struct that have a UNIQUE constraint,
        # we implement the methods `from_{column_name}` by employing the method
        # of the same name available for the main struct associated to this struct
        for unique_column in tables_metadata.get_unique_constraint_columns(
            struct.table_name
        ):
            if isinstance(unique_column, ViewColumn):
                unique_column = unique_column.alias_name

            attribute = struct.get_attribute_by_name(unique_column)
            attribute_data_type = attribute.data_type()
            if attribute_data_type == "String":
                attribute_data_type = "&str"

            tables.write(
                f"impl {nested_struct_name} {{\n"
                f"    /// Get the nested struct from the provided {unique_column}.\n"
                "    ///\n"
                f"    /// # Arguments\n"
                f"    /// * `{attribute.name}` - The {attribute.name} of the row.\n"
                "    /// * `connection` - The database connection.\n"
                f"    pub fn from_{attribute.name}(\n"
                f"        {attribute.name}: {attribute_data_type},\n"
                "        connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>,\n"
                "    ) -> Result<Self, diesel::result::Error>\n"
                "    {\n"
                f"        {struct.name}::from_{attribute.name}({attribute.name}, connection).and_then(|flat_struct| Self::from_flat(flat_struct, connection))\n"
                "    }\n"
                "}\n"
            )

        # If there is an index on the table, we implement the search method that
        # calls search on the flat version of the struct and then iterates on the
        # primary keys of the results and constructs the nested structs by calling
        # the `get` method several times.
        if similarity_indices.has_table(struct.table_name):
            for method_name, _, _ in PGIndices.SIMILARITY_METHODS:
                tables.write(
                    f"impl Nested{struct.name} {{\n"
                    "    /// Search the table by the query.\n"
                    "    ///\n"
                    "    /// # Arguments\n"
                    "    /// * `query` - The string to search for.\n"
                    "    /// * `limit` - The maximum number of results, by default `10`.\n"
                    f"    pub fn {method_name}_search(\n"
                    "        query: &str,\n"
                    "        limit: Option<i32>,\n"
                    "        connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>,\n"
                    "    ) -> Result<Vec<Self>, diesel::result::Error> {\n"
                    f"       {struct.name}::{method_name}_search(query, limit, connection)?.into_iter().map(|flat_struct| Self::from_flat(flat_struct, connection)).collect()\n"
                    "    }\n"
                    "}\n"
                )

        # We implement the bidirectional From methods for the nested struct
        # present in the web_common crate, which does not use Diesel or its
        # structs, but the web_common version of the structs.
        tables.write(
            f"impl From<web_common::database::nested_models::Nested{struct.name}> for Nested{struct.name} {{\n"
            f"    fn from(item: web_common::database::nested_models::Nested{struct.name}) -> Self {{\n"
            "        Self {\n"
        )
        for attribute in new_struct_metadata.attributes:
            if attribute.optional:
                tables.write(
                    f"            {attribute.name}: item.{attribute.name}.map(|item| item.into()),\n"
                )
            else:
                tables.write(
                    f"            {attribute.name}: item.{attribute.name}.into(),\n"
                )
        tables.write("        }\n" "    }\n" "}\n")

        tables.write(
            f"impl From<Nested{struct.name}> for web_common::database::nested_models::Nested{struct.name} {{\n"
            f"    fn from(item: Nested{struct.name}) -> Self {{\n"
            "        Self {\n"
        )
        for attribute in new_struct_metadata.attributes:
            if attribute.optional:
                tables.write(
                    f"            {attribute.name}: item.{attribute.name}.map(|item| item.into()),\n"
                )
            else:
                tables.write(
                    f"            {attribute.name}: item.{attribute.name}.into(),\n"
                )
        tables.write("        }\n" "    }\n" "}\n")

        new_struct_metadatas.append(new_struct_metadata)

    tables.close()

    # We replace until convergence the data type of the structs with the structs themselves.
    # This is necessary as the nested structs may contain references to other structs, which
    # in turn may contain references to other structs and so on.

    changed = True

    while changed:
        changed = False
        updated_struct_metadatas = []
        for new_struct in new_struct_metadatas:
            if not new_struct.has_undefined_nested_dependencies():
                updated_struct_metadatas.append(new_struct)
                continue
            new_attributes = []
            converged = True
            for attribute in new_struct.attributes:
                if attribute.is_undefined_nested_dependencies():
                    for struct in new_struct_metadatas + struct_metadatas:
                        if struct.name == attribute.data_type():
                            if struct.has_undefined_nested_dependencies():
                                converged = False
                                continue
                            new_attributes.append(
                                AttributeMetadata(
                                    original_name=attribute.original_name,
                                    name=attribute.name,
                                    data_type=struct,
                                    optional=attribute.optional,
                                )
                            )
                            changed = True
                            break
                else:
                    new_attributes.append(attribute)
            if converged:
                new_struct.attributes = new_attributes
            updated_struct_metadatas.append(new_struct)
        new_struct_metadatas = updated_struct_metadatas

    return new_struct_metadatas


def write_web_common_nested_structs(path: str, nested_structs: List[StructMetadata]):
    """Writes the nested structs to the web_common crate."""

    # We open the file to write the nested structs
    tables = open(f"../web_common/src/database/{path}", "w", encoding="utf8")
    table_metadatas = find_foreign_keys()

    # Preliminarly, we write a docstring at the very head
    # of this submodule to explain what it does and warn the
    # reader not to write anything in this file as it is
    # automatically generated.
    tables.write(
        "//! This module contains the nested structs for the database tables.\n"
    )
    tables.write("//!\n")
    tables.write(
        "//! This file is automatically generated. Do not write anything here.\n"
    )
    tables.write("\n")

    # We start with the necessary imports.
    imports = [
        "use serde::Deserialize;",
        "use serde::Serialize;",
        "use super::tables::*;",
        "use super::views::*;",
    ]

    for import_statement in imports:
        tables.write(f"{import_statement}\n")

    for struct_metadata in nested_structs:
        if not struct_metadata.can_implement_clone():
            print(f"# {struct_metadata.name}")
            for attribute in struct_metadata.attributes:
                print(f"* {attribute.name} {attribute.implements_clone()}")

        tables.write("#[derive(" + ", ".join(struct_metadata.derives()) + ")]\n")
        tables.write(f"pub struct {struct_metadata.name} {{\n")
        for attribute in struct_metadata.attributes:
            tables.write(f"    pub {attribute.name}: {attribute.format_data_type()},\n")
        tables.write("}\n")

        # We implement the `get` method for the struct when the frontend feature
        # is enabled using GlueSQL. This method will be extremely similar to the
        # `get` method for the Diesel-based approach of the backend.

        primary_key_name, primary_key_type = (
            table_metadatas.get_primary_key_name_and_type(struct_metadata.table_name)
        )

        rust_primary_key_type = sql_type_to_rust_type(primary_key_type)
        flat_struct = struct_metadata.get_attribute_by_name("inner")._data_type

        tables.write(
            f'#[cfg(feature = "frontend")]\n'
            f"impl {struct_metadata.name} {{\n"
            "    /// Get the nested struct from the provided primary key.\n"
            "    ///\n"
            "    /// # Arguments\n"
            f"    /// * `{primary_key_name}` - The primary key of the row.\n"
            "    /// * `connection` - The database connection.\n"
            "    pub async fn get<C>(\n"
            f"        {primary_key_name}: {rust_primary_key_type},\n"
            "        connection: &mut gluesql::prelude::Glue<C>,\n"
            "    ) -> Result<Option<Self>, gluesql::prelude::Error> where\n"
            "        C: gluesql::core::store::GStore + gluesql::core::store::GStoreMut,\n"
            "    {\n"
            f"       if let Some(flat_struct) = {flat_struct.name}::get({primary_key_name}, connection).await? {{\n"
            "        Ok(Some(Self {\n"
        )
        for attribute in struct_metadata.attributes:
            if (
                attribute.data_type() == struct_metadata.name
                or flat_struct.has_attribute(attribute)
            ):
                tables.write(
                    f"            {attribute.name}: flat_struct.{attribute.name},\n"
                )
                continue
            if attribute.optional:
                tables.write(
                    f"            {attribute.name}: if let Some({attribute.original_name}) = flat_struct.{attribute.original_name} {{ {attribute.data_type()}::get({attribute.original_name}, connection).await? }} else {{ return Ok(None) }},\n"
                )
            else:
                tables.write(
                    f"            {attribute.name}: if let Some({attribute.name}) = {attribute.data_type()}::get(flat_struct.{attribute.original_name}, connection).await? {{ {attribute.name} }} else {{return Ok(None)}},\n"
                )
        tables.write(
            "        }))\n"
            "       } else {\n"
            "           Ok(None)\n"
            "       }\n"
            "    }\n"
            "}\n"
        )

    tables.close()


def write_table_names_enumeration(struct_metadatas: List[StructMetadata]):
    imports = [
        "use serde::Deserialize;",
        "use serde::Serialize;",
    ]

    # The derives to apply to the structs in the `src/database/tables.rs` document
    derives = ["Deserialize", "Serialize", "Clone", "Debug", "PartialEq", "Eq", "Copy"]

    # We check that we are currently executing in the `backend` crate
    # so to make sure that the relative path to the `web_common` crate
    # is correct.
    if not os.getcwd().endswith("backend"):
        raise Exception("This script must be executed in the `backend` crate.")

    tables = open(f"../web_common/src/database/table_names.rs", "w", encoding="utf8")

    # Preliminarly, we write a docstring at the very head
    # of this submodule to explain what it does and warn the
    # reader not to write anything in this file as it is
    # automatically generated.

    tables.write(
        "//! This module contains the table names enumeration.\n"
        "//!\n"
        "//! This module is automatically generated. Do not write anything here.\n\n"
    )

    for import_statement in imports:
        tables.write(f"{import_statement}\n")

    unique_table_names = {
        (struct_metadata.table_name, struct_metadata.capitalized_table_name())
        for struct_metadata in struct_metadatas
    }

    unique_table_names = list(unique_table_names)

    unique_table_names = sorted(unique_table_names, key=lambda x: x[0])

    tables.write("#[derive(" + ", ".join(derives) + ")]\n")
    tables.write("pub enum Table {\n")
    for _, table_name in unique_table_names:
        tables.write(f"    {table_name},\n")
    tables.write("}\n\n")

    # We implement the `AsRef` trait for the `Table` enum
    # to convert it into &str.
    tables.write("impl AsRef<str> for Table {\n")
    tables.write("    fn as_ref(&self) -> &str {\n")
    tables.write("        match self {\n")
    for table_name, capitalized_table_name in unique_table_names:
        tables.write(
            f'            Table::{capitalized_table_name} => "{table_name}",\n'
        )
    tables.write("        }\n")
    tables.write("    }\n")
    tables.write("}\n")

    # We implement display

    tables.write("impl std::fmt::Display for Table {\n")
    tables.write(
        "    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {\n"
    )
    tables.write('        write!(f, "{}", self.as_ref())\n')
    tables.write("    }\n")
    tables.write("}\n")

    # We implement the conversion From<Table> for String

    tables.write("impl From<Table> for String {\n")
    tables.write("    fn from(table: Table) -> Self {\n")
    tables.write("        table.to_string()\n")
    tables.write("    }\n")
    tables.write("}\n")

    # We implement the TryFrom trait from String to Table

    tables.write("impl std::convert::TryFrom<&str> for Table {\n")
    tables.write("    type Error = String;\n")
    tables.write("    fn try_from(value: &str) -> Result<Self, Self::Error> {\n")
    tables.write("        match value {\n")
    for table_name, capitalized_table_name in unique_table_names:
        tables.write(
            f'            "{table_name}" => Ok(Table::{capitalized_table_name}),\n'
        )
    tables.write(
        '            table_name => Err(format!("Unknown table name: {}", table_name)),\n'
    )
    tables.write("        }\n")
    tables.write("    }\n")
    tables.write("}\n")

    tables.close()


def write_web_common_search_trait_implementations(
    struct_metadatas: List[StructMetadata],
):
    # We check that we are currently executing in the `backend` crate
    # so to make sure that the relative path to the `web_common` crate
    # is correct.
    if not os.getcwd().endswith("backend"):
        raise Exception("This script must be executed in the `backend` crate.")

    tables = open(f"../web_common/src/database/search_tables.rs", "w", encoding="utf8")
    similarity_indices: PGIndices = find_pg_trgm_indices()

    imports = [
        "use crate::database::*;",
    ]

    # Preliminarly, we write a docstring at the very head
    # of this submodule to explain what it does and warn the
    # reader not to write anything in this file as it is
    # automatically generated.

    tables.write(
        "//! This module contains the table names enumeration.\n"
        "//!\n"
        "//! This module is automatically generated. Do not write anything here.\n\n"
    )

    for import_statement in imports:
        tables.write(f"{import_statement}\n")

    # First, we create the Searchable trait that will be implemented by all the structs
    # that are searchable.

    tables.write("pub trait Searchable {\n")
    tables.write("    fn search_task(query: String, limit: u32) -> super::Select;\n")
    tables.write("}\n")

    for struct in struct_metadatas:
        if similarity_indices.has_table(struct.table_name):
            tables.write(f"impl Searchable for {struct.name} {{\n")
            tables.write(
                "    fn search_task(query: String, limit: u32) -> super::Select {\n"
            )
            tables.write(f"        super::Select::search(\n")
            tables.write(f"             Table::{struct.capitalized_table_name()},\n")
            tables.write("              query,\n")
            tables.write("              limit,\n")
            tables.write("        )\n")
            tables.write("    }\n")
            tables.write("}\n")

    tables.close()


def generate_table_schema():
    # Read the replacements from the JSON file
    replacements = compress_json.local_load("replacements.json")

    # We make sure the migrations were fully executed
    status = os.system("diesel migration run")

    if status != 0:
        raise Exception("The migrations were not fully executed.")

    # We run the diesel extended CLI command
    status = os.system("diesel_ext --model --add-table-name > src/models.rs")

    if status != 0:
        raise Exception("The diesel_ext command failed.")

if __name__ == "__main__":
    # Load dotenv file
    load_dotenv()

    # Due to git collisions, it may happen that a migration has been renamed
    # but the old directory is still there. For each directory in the migrations
    # that is composed of the [0-9]+_[a-z_]+ pattern, we check if the directory
    # as a twin with a different code but the same name. If so, we remove the
    # empty version of these directories.
    for directory in os.listdir("migrations"):
        if not os.path.isdir(f"migrations/{directory}"):
            continue
        if re.match(r"^[0-9]+_[a-z_]+$", directory):
            # We check whether the current directory DOES NOT
            # contain either a down.sql or an up.sql file.
            if not os.path.exists(
                f"migrations/{directory}/down.sql"
            ) or not os.path.exists(f"migrations/{directory}/up.sql"):
                code, migration_name = directory.split("_", maxsplit=1)
                twin_found = False
                for directory2 in os.listdir("migrations"):
                    if not os.path.isdir(f"migrations/{directory2}"):
                        continue
                    if re.match(r"^[0-9]+_[a-z_]+$", directory2):
                        code2, migration_name2 = directory2.split("_", maxsplit=1)
                        if code != code2 and migration_name == migration_name2:
                            print(f"Removing {directory}")
                            shutil.rmtree(f"migrations/{directory}")
                            twin_found = True
                            break
                if not twin_found:
                    raise Exception(
                        f"Directory {directory} does not contain either a `down.sql` or an `up.sql` file "
                        "and there is no twin directory with a different code."
                    )

    densify_directory_counter()

    # If there is a "__pycache__" directory, we remove it as Diesel
    # seems to be keen to try and run it as a migration, and it will
    # fail.
    if os.path.exists("__pycache__"):
        shutil.rmtree("__pycache__")

    if os.path.exists("migrations/__pycache__"):
        shutil.rmtree("migrations/__pycache__")

    if not os.path.exists("./db_data/bio_ott_taxons.csv.gz"):
        retrieve_taxons()

    generate_table_schema()
    print("Generated models.")
    generate_view_schema()
    print("Generated view schema.")
    check_schema_completion()
    print("Checked schema completion.")
    generate_view_structs()
    print("Generated view structs.")

    table_structs: List[StructMetadata] = extract_structs("src/models.rs")
    view_structs: List[StructMetadata] = extract_structs("src/views/views.rs")

    write_backend_structs("src/models.rs", "tables", table_structs)
    write_backend_structs("src/views/views.rs", "views", view_structs)
    print("Generated From implementations for backend.")

    write_web_common_structs(
        table_structs, "tables", "Table"
    )
    write_web_common_structs(
        view_structs, "views", "View"
    )
    print("Generated web common structs.")
    
    write_table_names_enumeration(table_structs + view_structs)
    print("Generated table names enumeration for web_common.")

    nested_structs: List[StructMetadata] = generate_nested_structs(
        "src/nested_models.rs", table_structs + view_structs
    )
    print("Generated nested structs for backend.")
    write_web_common_nested_structs("nested_models.rs", nested_structs)
    print("Generated nested structs for web_common.")

    write_web_common_search_trait_implementations(
        nested_structs + table_structs + view_structs
    )
    print("Generated search trait implementations for web_common.")
