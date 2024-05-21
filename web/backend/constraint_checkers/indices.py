"""This module contains the logic to find the indices of a PostgreSQL database."""

from typing import List, Dict
from functools import cache
from constraint_checkers.cursor import get_cursor
from constraint_checkers.find_foreign_keys import TableMetadata


class PGIndex:
    """Represents a PostgreSQL index."""

    def __init__(self, name: str, table_name: str, arguments: str, index_type: str):
        self.name = name
        self.table_name = table_name
        self.arguments = arguments
        self.index_type = index_type

    def is_gin(self) -> bool:
        """Returns whether the index is of type `gin_trgm_ops`."""
        return self.index_type == "gin_trgm_ops"

    def is_gist(self) -> bool:
        """Returns whether the index is of type `gist_trgm_ops`."""
        return self.index_type == "gist_trgm_ops"

    def is_btree(self) -> bool:
        """Returns whether the index is of type `btree`."""
        return self.index_type == "btree"

    def __repr__(self) -> str:
        return f"PGIndex(name={self.name}, table_name={self.table_name}, arguments={self.arguments}, index_type={self.index_type})"


class PGIndices:
    """Represents a list of PostgreSQL indices."""

    SIMILARITY_METHODS = (
        ("similarity", "%", "<->"),
        ("word_similarity", "<%", "<<->"),
        ("strict_word_similarity", "<<%", "<<<->"),
    )

    def __init__(self, indices: List[PGIndex], tables_metadata: TableMetadata):
        assert all(
            isinstance(index, PGIndex)
            for index in indices
        )
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
            indexdef ILIKE '%using gist%'
            OR
            indexdef ILIKE '%using btree%';
        """
    )
    indices = cursor.fetchall()
    pg_indices: Dict[str, PGIndex] = {}
    for index in indices:
        arguments = index[3]

        index_type = None
        for possible_index_type in (
            "gin_trgm_ops",
            "gist_trgm_ops",
            "btree",
        ):
            if possible_index_type in index[2]:
                index_type = possible_index_type
                break
        
        arguments = arguments.replace(f" {index_type}", "")

        # If there is already an index for the table, we only replace it if it is
        # a btree index, since it is the most restrictive one
        if index[1] in pg_indices:
            if pg_indices[index[1]].is_btree():
                pg_indices[index[1]] = PGIndex(
                    index[0], index[1], arguments, index_type
                )
        else:
            pg_indices[index[1]] = PGIndex(
                index[0], index[1], arguments, index_type
            )

    cursor.close()
    conn.close()

    return PGIndices(list(pg_indices.values()), tables_metadata)
