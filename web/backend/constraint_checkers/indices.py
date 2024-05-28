"""This module contains the logic to find the indices of a PostgreSQL database."""

from typing import List, Dict, Optional
from functools import cache
from tqdm.auto import tqdm
from constraint_checkers.cursor import get_cursor
from constraint_checkers.find_foreign_keys import SQLFunction, TableMetadata


class PGIndex:
    """Represents a PostgreSQL index.
    
    Parameters
    ----------
    name : str
        The name of the index.
    table_name : str
        The name of the table that the index is associated with.
    arguments : str
        The arguments of the index.
    index_type : str
        The type of the index.
    associated_function : Optional[SQLFunction]
        The function associated with the index.
    foreign_table_names : Optional[List[str]]
        List of foreign tables that have a UNIQUE foreign key constraint with the table
        but do not have a search index themselves, so the search index of the table can be
        used to search in them as well.
    """

    def __init__(
        self,
        name: str,
        table_name: str,
        arguments: str,
        index_type: str,
        associated_function: Optional[SQLFunction],
    ):
        self.name = name
        self.table_name = table_name
        self.arguments = arguments
        self.index_type = index_type
        self.associated_function = associated_function
        self.foreign_table_names: Dict[str, str] = dict()

    def get_primary_keys(self) -> List[str]:
        """Returns the primary keys of the table associated with the index."""
        return [
            primary_key
            for primary_key, _type in PGIndices.table_metadata.get_primary_key_names_and_types(self.table_name)
        ]

    def add_foreign_table_name(self, foreign_table_name: str, foreign_key_id: str) -> None:
        """Adds a foreign table name to the list of foreign tables of the index.

        Parameters
        ----------
        foreign_table_name : str
            The name of the foreign table.
        foreign_key_id : str
            The id of the foreign key.
        """
        if foreign_table_name in self.foreign_table_names:
            raise ValueError(
                f"The foreign table {foreign_table_name} is already in the list of foreign tables "
                f"of the index {self.name} with id {self.foreign_table_names[foreign_table_name]}."
            )
        self.foreign_table_names[foreign_table_name] = foreign_key_id

    def is_foreign_table(self, table_name: str) -> bool:
        """Returns whether the table with the given name is a foreign table."""
        return table_name in self.foreign_table_names

    def is_gin(self) -> bool:
        """Returns whether the index is of type `gin_trgm_ops`."""
        return self.index_type == "gin_trgm_ops"

    def is_gist(self) -> bool:
        """Returns whether the index is of type `gist_trgm_ops`."""
        return self.index_type == "gist_trgm_ops"

    def __repr__(self) -> str:
        return f"PGIndex(name={self.name}, table_name={self.table_name}, arguments={self.arguments}, index_type={self.index_type})"

    def _format_diesel(self, query: str, similarity_method: str, desinence: str) -> str:
        """Returns the index in Diesel format."""
        assert similarity_method in PGIndices.SIMILARITY_METHODS
        assert desinence in ("op", "dist", "dist_op")

        if self.associated_function is not None:
            arguments = ", ".join(
                f'{self.table_name}::dsl::{argument}'
                for argument in self.associated_function.arguments
            )

            return f"{similarity_method}_{desinence}({self.associated_function.name}({arguments}), {query})"

        assert ' ' not in self.arguments, (
            "We are not expecting any spaces in the arguments of the index, as this "
            f"should be a single argument. The arguments are: '{self.arguments}'."
        )

        return f"{similarity_method}_{desinence}({self.table_name}::dsl::{self.arguments}, {query})"

    def format_diesel_inner_join(self, table_name: str) -> str:
        """Formats the inner join Diesel statement for the foreign table with the given name."""
        if table_name not in self.foreign_table_names:
            raise ValueError(
                f"The table {table_name} is not a foreign table of the index {self.name}."
            )

        primary_keys = self.get_primary_keys()
        assert len(primary_keys) == 1, (
            "We are expecting the table to have a single primary key, but it has "
            f"{len(primary_keys)} primary keys: {primary_keys}. "
            "We do not support tables with multiple primary keys in a inner join "
            "statement at the moment."
        )
        primary_key = primary_keys[0]
        foreign_key_id = self.foreign_table_names[table_name]

        return f".inner_join({self.table_name}::dsl::{self.table_name}.on({table_name}::dsl::{foreign_key_id}.eq({self.table_name}::dsl::{primary_key})))"

    def format_operator_diesel(self, query: str, similarity_method: str) -> str:
        """Returns the index in Diesel format.
        
        Parameters
        ----------
        query : str
            The query to search for.
        similarity_method : str
            The similarity method to use.
        """
        return self._format_diesel(query, similarity_method, "op")

    def format_distance_operator_diesel(self, query: str, similarity_method: str) -> str:
        """Returns the index in Diesel format.
        
        Parameters
        ----------
        query : str
            The query to search for.
        similarity_method : str
            The similarity method to use.
        """
        if similarity_method in ("strict_word_similarity", "word_similarity"):
            return self._format_diesel(query, similarity_method, "dist_op")
        return self._format_diesel(query, similarity_method, "dist")


class PGIndices:
    """Represents a list of PostgreSQL indices."""

    SIMILARITY_METHODS = (
        "similarity",
        "word_similarity",
        "strict_word_similarity",
    )
    table_metadata: TableMetadata = None

    def __init__(self, indices: List[PGIndex]):
        assert all(isinstance(index, PGIndex) for index in indices)
        self.indices = indices

    def tables(self) -> List[str]:
        """Returns the list of table names that have an index."""
        return list(set(index.table_name for index in self.indices))

    @cache
    def has_table(self, table_name: str) -> bool:
        """Returns whether the table with the given name has an index."""
        if PGIndices.table_metadata.is_view(table_name):
            view_columns = PGIndices.table_metadata.extract_view_columns(
                table_name
            )
            # We seek an "id" column in the view columns
            for column in view_columns:
                if column.alias_name == "id":
                    return self.has_table(column.table_name)

        for index in self.indices:
            if index.table_name == table_name:
                return True
            if table_name in index.foreign_table_names:
                return True
        return False

    @cache
    def get_table(self, table_name: str) -> PGIndex:
        """Returns the index of the table with the given name."""
        if PGIndices.table_metadata.is_view(table_name):
            view_columns = PGIndices.table_metadata.extract_view_columns(
                table_name
            )
            # We seek an "id" column in the view columns
            for column in view_columns:
                if column.alias_name == "id":
                    return self.get_table(column.table_name)

        for index in self.indices:
            if index.table_name == table_name:
                return index
            if table_name in index.foreign_table_names:
                return index
        raise ValueError(f"The table {table_name} does not have an index.")


def find_search_indices(
    table_metadata: TableMetadata,
) -> PGIndices:
    """Returns the list of indices that can be used for search purposes."""
    conn, cursor = get_cursor()
    cursor.execute(
        r"""
        SELECT
            indexname AS index_name,
            tablename AS table_name,
            indexdef,
            substring(indexdef from '\((.*)\)') AS arguments
        FROM
            pg_indexes
        WHERE
            indexdef ILIKE '%using gin%'
            OR
            indexdef ILIKE '%using gist%';
        """
    )
    indices = cursor.fetchall()
    sql_functions: List[SQLFunction] = table_metadata.get_all_postgres_functions()
    pg_indices: Dict[str, PGIndex] = {}
    for index in tqdm(
        indices,
        desc="Finding search indices",
        unit="index",
        leave=False,
    ):
        arguments = index[3]

        index_type = None
        for possible_index_type in (
            "gin_trgm_ops",
            "gist_trgm_ops",
        ):
            if possible_index_type in index[2]:
                index_type = possible_index_type
                break

        arguments = arguments.replace(f" {index_type}", "")

        # We find the associated function by searching the one with
        # the longest name that is a substring of the arguments. Also,
        # all of the arguments in the associated function must be present
        # in the arguments of the index.
        associated_function = None
        for sql_function in sql_functions:
            if sql_function.name in arguments and all(
                argument in arguments for argument in sql_function.arguments
            ):
                if associated_function is None:
                    associated_function = sql_function
                elif len(sql_function.name) > len(associated_function.name):
                    associated_function = sql_function

        if index[1] in pg_indices:
            pg_indices[index[1]] = PGIndex(
                index[0], index[1], arguments, index_type, associated_function
            )
        else:
            pg_indices[index[1]] = PGIndex(
                index[0], index[1], arguments, index_type, associated_function
            )

    # We find the foreign tables that have a UNIQUE foreign key constraint with the table
    # but do not have a search index themselves, so the search index of the table can be
    # used to search in them as well.
    for table_name in table_metadata.tables():
        if table_name in pg_indices:
            continue
        for unique_foreign_keys in table_metadata.get_unique_constraint_columns(table_name):
            if len(unique_foreign_keys) != 1:
                continue
            unique_foreign_key = unique_foreign_keys[0]
            if unique_foreign_key not in table_metadata.get_foreign_keys(table_name):
                continue

            foreign_key_table_name = table_metadata.get_foreign_key_table_name(table_name, unique_foreign_key)
            if foreign_key_table_name in pg_indices:
                pg_indices[foreign_key_table_name].add_foreign_table_name(table_name, unique_foreign_key)

    cursor.close()
    conn.close()

    PGIndices.table_metadata = table_metadata

    return PGIndices(list(pg_indices.values()))
