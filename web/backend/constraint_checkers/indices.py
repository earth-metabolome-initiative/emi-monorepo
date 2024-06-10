"""This module contains the logic to find the indices of a PostgreSQL database."""

from typing import List, Optional, Union
from tqdm.auto import tqdm
from constraint_checkers.cursor import get_cursor
from constraint_checkers.find_foreign_keys import SQLFunction, TableMetadata

NON_SEARCHABLE_THIRD_PARTY_TABLES = ["colors", "users", "font_awesome_icons"]


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

    def get_primary_keys(self) -> List[str]:
        """Returns the primary keys of the table associated with the index."""
        return [
            primary_key
            for primary_key, _type in PGIndices.table_metadata.get_primary_key_names_and_types(
                self.table_name
            )
        ]

    def index_table_name(self) -> str:
        """Returns the name of the table associated with the index."""
        return self.table_name

    # def is_gin(self) -> bool:
    #     """Returns whether the index is of type `gin_trgm_ops`."""
    #     return self.index_type == "gin_trgm_ops"

    # def is_gist(self) -> bool:
    #     """Returns whether the index is of type `gist_trgm_ops`."""
    #     return self.index_type == "gist_trgm_ops"

    def __repr__(self) -> str:
        return f"PGIndex(name={self.name}, table_name={self.table_name}, arguments={self.arguments}, index_type={self.index_type})"

    def _format_function(
        self,
        alias_number: Optional[int] = None,
    ) -> str:
        """Returns the argument portion of the distance or similarity function."""
        if alias_number == -1:
            alias_number = None

        if self.associated_function is not None:
            if alias_number is not None:
                arguments = ", ".join(
                    f"{self.index_table_name()}{alias_number}.field({self.table_name}::dsl::{argument})"
                    for argument in self.associated_function.arguments
                )
            else:
                arguments = ", ".join(
                    f"{self.index_table_name()}::dsl::{argument}"
                    for argument in self.associated_function.arguments
                )

            return f"crate::database::sql_function_bindings::{self.associated_function.name}({arguments})"

        assert " " not in self.arguments, (
            "The arguments of the index contain spaces, which is not expected. "
            f"The current index is {self}."
        )
        if alias_number is not None:
            return f"{self.index_table_name()}{alias_number}.field({self.table_name}::dsl::{self.arguments})"
        return f"{self.index_table_name()}::dsl::{self.arguments}"
        

    def _format_diesel(
        self,
        query: str,
        similarity_method: str,
        desinence: str,
        alias_number: Optional[int] = None,
    ) -> str:
        """Returns the index in Diesel format."""
        assert similarity_method in PGIndices.SIMILARITY_METHODS
        assert desinence in ("commutator_op", "dist", "dist_op")

        formatted_function = self._format_function(alias_number)
        if "dist" in desinence:
            return f"crate::database::sql_function_bindings::{similarity_method}_{desinence}({formatted_function}, {query})"
        return f"{formatted_function}.{similarity_method}_{desinence}({query})"


    def format_diesel_search_filter(
        self, query: str, similarity_method: str, alias_number: Optional[int] = None
    ) -> str:
        """Returns the index in Diesel format.

        Parameters
        ----------
        query : str
            The query to search for.
        similarity_method : str
            The similarity method to use.
        alias_number : Optional[int]
            The alias number of the index.
        """

        formatted_function = self._format_function(alias_number)
        
        return (
            f"{self._format_diesel(query, similarity_method, 'commutator_op', alias_number)}.or(\n"
            f"    {formatted_function}.ilike(format!(\"%{{}}%\", {query}))\n"
            ")\n"
        )

    def format_distance_operator_diesel(
        self, query: str, similarity_method: str, alias_number: Optional[int] = None
    ) -> str:
        """Returns the index in Diesel format.

        Parameters
        ----------
        query : str
            The query to search for.
        similarity_method : str
            The similarity method to use.
        """
        if similarity_method in ("strict_word_similarity", "word_similarity"):
            return self._format_diesel(
                query, similarity_method, "dist_op", alias_number
            )
        return self._format_diesel(query, similarity_method, "dist", alias_number)


class DerivedPGIndex:
    """Class representing a derived index, meaning a research index defined on a foreign key table.

    We reuse these foreign key table indices to search in the primary table.

    Parameters
    ----------
    table_name : str
        The name of the table that the index is associated with.
    foreign_key_id : str
        The id of the foreign key.
    index : PGIndex
        The index of the foreign key table.
    optional : bool
        Whether the foreign key is optional.
    unique : bool
        Whether the foreign key is unique.
    """

    def __init__(
        self,
        table_name: str,
        foreign_key_id: str,
        index: Union[PGIndex, "DerivedPGIndex"],
        optional: bool = False,
        unique: bool = False,
    ):
        self.table_name = table_name
        self.foreign_key_id = foreign_key_id
        self.index = index
        self.optional = optional
        self.unique = unique
        self._alias_number: Optional[int] = None
        self._inner_alias_number: Optional[int] = None

    def set_alias_number(self, alias_number: int):
        """Sets the alias number of the index."""
        self._alias_number = alias_number

    def set_inner_alias_number(self, alias_number: int):
        """Sets the inner alias number of the index."""
        self._inner_alias_number = alias_number

    def is_first_order(self) -> bool:
        """Returns whether the index is first order."""
        return isinstance(self.index, PGIndex)

    def is_second_order(self) -> bool:
        """Returns whether the index is second order."""
        return isinstance(self.index, DerivedPGIndex)

    def index_table_name(self) -> str:
        """Returns the name of the table associated with the index."""
        return self.index.table_name

    def has_alias_number(self) -> bool:
        """Returns whether the index has an alias number."""
        return self._alias_number is not None

    def has_inner_alias_number(self) -> bool:
        """Returns whether the index has an inner alias number."""
        return self._inner_alias_number is not None

    def get_alias_number(self) -> int:
        """Returns the alias number of the index."""
        assert self.has_alias_number(), (
            "The index does not have an alias number, which is not expected. "
            f"The current index is {self}."
        )
        return self._alias_number

    def get_inner_alias_number(self) -> int:
        """Returns the inner alias number of the index."""
        assert self.has_inner_alias_number(), (
            "The index does not have an inner alias number, which is not expected. "
            f"The current index is {self}."
        )
        return self._inner_alias_number

    def format_diesel_search_join(self) -> str:
        """Formats the inner join Diesel statement for the foreign table with the given name."""
        if self.is_first_order():
            comment = f"This operation is defined by a first order index linking {self.table_name}.{self.foreign_key_id} to {self.index.table_name}."
        elif self.is_second_order():
            comment = f"This operation is defined by a second order index linking {self.table_name}.{self.foreign_key_id} to {self.index.table_name} and {self.index.table_name}.{self.index.foreign_key_id} to {self.index.index.table_name}."

        nullable = ""
        join_operation = "inner_join"
        if self.optional:
            nullable = ".nullable()"
            if self.unique:
                join_operation = "left_join"

        if self._alias_number is not None:
            first_order_query = (
                f"// {comment}\n"
                f".{join_operation}(\n"
                f"   {self.index_table_name()}{self._alias_number}.on(\n"
                f"       {self.table_name}::dsl::{self.foreign_key_id}.eq(\n"
                f"           {self.index_table_name()}{self._alias_number}.field({self.index_table_name()}::dsl::id){nullable}\n"
                "        )\n"
                "    )\n"
                ")\n"
            )
        else:
            first_order_query = (
                f"// {comment}\n"
                f".{join_operation}(\n"
                f"   {self.index_table_name()}::dsl::{self.index_table_name()}.on(\n"
                f"       {self.table_name}::dsl::{self.foreign_key_id}.eq(\n"
                f"           {self.index_table_name()}::dsl::id{nullable}\n"
                "        )\n"
                "    )\n"
                ")\n"
            )

        if self.is_first_order():
            return first_order_query

        if self.is_second_order():

            nullable = ""
            join_operation = "inner_join"
            if self.index.optional:
                nullable = ".nullable()"
                if not self.index.unique:
                    join_operation = "left_join"

            if self._alias_number is not None:
                index_table_eq = f"{self.index.table_name}{self._alias_number}.field({self.index.table_name}::dsl::{self.index.foreign_key_id})"
            else:
                index_table_eq = (
                    f"{self.index.table_name}::dsl::{self.index.foreign_key_id}"
                )

            if self._inner_alias_number is not None:
                second_order_query = (
                    f"// {comment}\n"
                    f".{join_operation}(\n"
                    f"   {self.index.index_table_name()}{self._inner_alias_number}.on(\n"
                    f"       {index_table_eq}.eq(\n"
                    f"           {self.index.index_table_name()}{self._inner_alias_number}.field({self.index.index_table_name()}::dsl::id){nullable}\n"
                    "        )\n"
                    "    )\n"
                    ")\n"
                )
            else:
                second_order_query = (
                    f"// {comment}\n"
                    f".{join_operation}(\n"
                    f"   {self.index.index_table_name()}::dsl::{self.index.index_table_name()}.on(\n"
                    f"       {index_table_eq}.eq(\n"
                    f"           {self.index.index_table_name()}::dsl::id{nullable}\n"
                    "        )\n"
                    "    )\n"
                    ")\n"
                )
            return first_order_query + second_order_query

        assert False, (
            "The index is neither first order nor second order, which is not expected. "
            f"The current index is {self}."
        )

    def format_diesel_search_filter(
        self, query: str, similarity_method: str, alias_number: Optional[int] = None
    ) -> str:
        """Returns the index in Diesel format.

        Parameters
        ----------
        query : str
            The query to search for.
        similarity_method : str
            The similarity method to use.
        alias_number : Optional[int]
            The alias number of the index.
        """
        if alias_number is None:
            if self.is_second_order():
                alias_number = self._inner_alias_number
            elif self.is_first_order():
                alias_number = self._alias_number
            
            if alias_number is None:
                alias_number = -1
        return self.index.format_diesel_search_filter(
            query, similarity_method, alias_number
        )

    def format_distance_operator_diesel(
        self, query: str, similarity_method: str, alias_number: Optional[int] = None
    ) -> str:
        """Returns the index in Diesel format.

        Parameters
        ----------
        query : str
            The query to search for.
        similarity_method : str
            The similarity method to use.
        alias_number : Optional[int]
            The alias number of the index.
        """
        if alias_number is None:
            if self.is_second_order():
                alias_number = self._inner_alias_number
            elif self.is_first_order():
                alias_number = self._alias_number

            if alias_number is None:
                alias_number = -1
        return self.index.format_distance_operator_diesel(
            query, similarity_method, alias_number
        )


class PGIndices:
    """Represents a list of PostgreSQL indices."""

    # Since in practice we are actually only using the strict word similarity method,
    # we can remove the other methods so to avoid generating dead code and reducing
    # the size of the overall code base.
    SIMILARITY_METHODS = (
        # "similarity",
        # "word_similarity",
        "strict_word_similarity",
    )
    table_metadata: TableMetadata = None

    def __init__(self, indices: List[PGIndex]):
        assert all(isinstance(index, PGIndex) for index in indices)
        self.indices = indices

    def get_table_index(self, table_name: str) -> Optional[PGIndex]:
        """Returns the indices of the table with the given name."""
        if PGIndices.table_metadata.is_view(table_name):
            view_columns = PGIndices.table_metadata.extract_view_columns(table_name)
            # We seek an "id" column in the view columns
            for column in view_columns:
                if column.alias_name == "id":
                    return self.get_table_index(column.table_name)

        for index in self.indices:
            if index.table_name == table_name:
                return index
        return None


def find_primary_search_indices(
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
    pg_indices: List[PGIndex] = []
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

        pg_indices.append(
            PGIndex(index[0], index[1], arguments, index_type, associated_function)
        )

    cursor.close()
    conn.close()

    PGIndices.table_metadata = table_metadata

    return PGIndices(pg_indices)
