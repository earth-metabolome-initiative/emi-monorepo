"""This module contains the logic to find the indices of a PostgreSQL database."""
from typing import List
from constraint_checkers.cursor import get_cursor
from constraint_checkers.find_foreign_keys import TableMetadata

class PGIndex:

    def __init__(self, name: str, table_name: str, arguments: str, index_type: str):
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

    def __init__(self, indices: List[PGIndex], tables_metadata: TableMetadata):
        self.indices = indices
        self.foreign_keys_information = tables_metadata

    def tables(self) -> List[str]:
        return list(set(index.table_name for index in self.indices))

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

def find_pg_trgm_indices(
    tables_metadata: TableMetadata,
) -> PGIndices:
    """Returns the list of indices that are of type `pg_trgm`."""
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

    return PGIndices(pg_indices, tables_metadata)