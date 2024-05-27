"""This module contains the logic to find the indices of a PostgreSQL database."""

from typing import List, Dict, Optional
from functools import cache
from tqdm.auto import tqdm
from constraint_checkers.cursor import get_cursor
from constraint_checkers.find_foreign_keys import TableMetadata, SQLFunction


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
        foreign_table_names: Optional[List[str]] = None,
    ):
        self.name = name
        self.table_name = table_name
        self.arguments = arguments
        self.index_type = index_type
        self.associated_function = associated_function
        self.foreign_table_names = foreign_table_names if foreign_table_names is not None else []

    def add_foreign_table_name(self, foreign_table_name: str) -> None:
        """Adds a foreign table name to the list of foreign tables."""
        self.foreign_table_names.append(foreign_table_name)

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

    def __init__(self, indices: List[PGIndex], tables_metadata: TableMetadata):
        assert all(isinstance(index, PGIndex) for index in indices)
        self.indices = indices
        self.foreign_keys_information = tables_metadata

    def tables(self) -> List[str]:
        """Returns the list of table names that have an index."""
        return list(set(index.table_name for index in self.indices))

    @cache
    def has_table(self, table_name: str) -> bool:
        """Returns whether the table with the given name has an index."""
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
            if table_name in index.foreign_table_names:
                return True
        return False

    @cache
    def get_table(self, table_name: str) -> PGIndex:
        """Returns the index of the table with the given name."""
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
            if table_name in index.foreign_table_names:
                return index
        raise ValueError(f"The table {table_name} does not have an index.")


def find_search_indices(
    tables_metadata: TableMetadata,
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
    sql_functions: List[SQLFunction] = tables_metadata.get_all_postgres_functions()
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
    for table_name in tables_metadata.tables():
        if table_name in pg_indices:
            continue
        for unique_foreign_keys in tables_metadata.get_unique_constraint_columns(table_name):
            if len(unique_foreign_keys) != 1:
                continue
            unique_foreign_key = unique_foreign_keys[0]
            if unique_foreign_key not in tables_metadata.get_foreign_keys(table_name):
                continue

            # TODO!
            if True:
                continue

            foreign_key_table_name = tables_metadata.get_foreign_key_table_name(table_name, unique_foreign_key)
            if foreign_key_table_name in pg_indices:
                pg_indices[foreign_key_table_name].add_foreign_table_name(table_name)

    cursor.close()
    conn.close()

    return PGIndices(list(pg_indices.values()), tables_metadata)
