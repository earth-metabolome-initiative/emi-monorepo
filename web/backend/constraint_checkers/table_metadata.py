"""This module contains the TableMetadata class."""
from typing import List
from constraint_checkers.struct_metadata import StructMetadata, AttributeMetadata


class TableStructMetadata:
    """Contains metadata about the struct associated to a table."""

    def __init__(self, table_name: str):
        self.name = table_name
        self.richest_struct: StructMetadata = None
        self.flat_struct: StructMetadata = None
        self.new_flat_struct: StructMetadata = None
        self.update_flat_struct: StructMetadata = None

    def requires_authentication(self) -> bool:
        """Returns whether the table requires authentication.

        Implementation details
        -----------------------
        A struct requires authentication to operate on it (either
        insert or update) if it contains a field that is either the
        creator user ID or the updater user ID.
        """
        return any(
            attribute.requires_authentication()
            for attribute in self.flat_struct.attributes
        )

    def is_updatable(self) -> bool:
        """Returns whether the table is updatable.

        Implementation details
        -----------------------
        A table is updatable if the associated struct has at least a
        field containing the updater user ID.
        """
        return self.flat_struct.is_updatable()

    def is_insertable(self) -> bool:
        """Returns whether the table is insertable.

        Implementation details
        -----------------------
        A table is insertable if the associated struct has at least a
        field containing the creator user ID.
        """
        return self.flat_struct.is_insertable()

    def has_uuid_primary_key(self) -> bool:
        """Returns whether the table has a UUID primary key.

        Implementation details
        -----------------------
        This method returns whether the primary key of the table is
        a UUID.
        """
        return self.richest_struct.has_uuid_primary_key()

    def set_new_flat_struct(self, struct: StructMetadata):
        assert struct.table_name == self.name
        assert not struct.is_nested()
        assert struct.is_new_variant()
        if self.new_flat_struct is not None:
            raise ValueError(
                "The new flat struct has already been set. This is unexpected. Please check the table and the structs associated to it."
            )
        self.new_flat_struct = struct

    def get_new_flat_struct(self) -> StructMetadata:
        if self.new_flat_struct is None:
            raise ValueError(
                f"The new flat struct has not been set for the table {self.name}."
            )
        return self.new_flat_struct

    def set_update_flat_struct(self, struct: StructMetadata):
        assert struct.table_name == self.name
        assert not struct.is_nested()
        assert not struct.is_new_variant()
        assert not self.has_uuid_primary_key(), (
            f"The table {self.name} has a UUID primary key. "
            "This means that the update flat struct cannot be set, "
            "as it would be identical to the new flat struct."
        )
        if self.update_flat_struct is not None:
            raise ValueError(
                "The update flat struct has already been set. This is unexpected. Please check the table and the structs associated to it."
            )
        self.update_flat_struct = struct

    def get_update_flat_struct(self) -> StructMetadata:
        if self.has_uuid_primary_key():
            return self.get_new_flat_struct()
        if self.update_flat_struct is None:
            raise ValueError(
                f"The update flat struct has not been set for the table {self.name}."
            )
        return self.update_flat_struct

    def set_richest_struct(self, struct: StructMetadata):
        assert struct.table_name == self.name
        if self.richest_struct is not None:
            if self.richest_struct.is_nested() and not struct.is_nested():
                return
            if self.richest_struct.is_nested() and struct.is_nested():
                raise ValueError(
                    "This table has several nested structs. This is not supported and "
                    "is unexpected. Please check the table and the structs associated to it. "
                    f"The table name is {self.name} and the provided structs are {struct.name} "
                    f"and {self.richest_struct.name}."
                )
        self.richest_struct = struct

    def get_richest_struct(self) -> StructMetadata:
        return self.richest_struct

    def set_flat_struct(self, struct: StructMetadata):
        assert struct.table_name == self.name
        if struct.is_nested():
            raise ValueError(
                "The flat struct cannot be a nested struct. Please provide a struct that is not nested."
            )
        if self.flat_struct is not None:
            raise ValueError(
                "The flat struct has already been set. This is unexpected. Please check the table and the structs associated to it."
            )
        self.flat_struct = struct

    def get_flat_variant(self) -> StructMetadata:
        """Returns the flat struct of the table."""
        if self.flat_struct is None:
            raise ValueError(
                f"The flat struct has not been set for the table {self.name}."
            )
        return self.flat_struct

    def flat_struct_name(self) -> str:
        return self.flat_struct.name

    def new_flat_struct_name(self) -> str:
        return self.get_new_flat_struct().name

    def update_flat_struct_name(self) -> str:
        return self.get_update_flat_struct().name

    def richest_struct_name(self) -> str:
        return self.richest_struct.name

    def camel_cased(self) -> str:
        return "".join(word.capitalize() for word in self.name.split("_"))

    def has_updated_at_column(self) -> bool:
        return self.flat_struct.is_updatable()

    def get_primary_keys(self) -> List[AttributeMetadata]:
        return self.flat_struct.get_primary_keys()

    def is_junktion_table(self) -> bool:
        """Returns whether the table is a junktion table.
        
        Implementation details
        -----------------------
        A table is a junktion table if it has a primary key that is
        a combination of foreign keys.
        """
        return self.flat_struct.is_junktion_table()

    def has_associated_roles(self) -> bool:
        """Returns whether the table has an associated roles table.
        
        Implementation details
        -----------------------
        A table has an associated roles table if it has a column
        that is a foreign key to the roles table.
        """
        return self.flat_struct.has_associated_roles()

    def has_public_column(self) -> bool:
        """Returns whether the table has a public column.
        
        Implementation details
        -----------------------
        A table has a public column if it has a column that is
        a boolean and that is named "public".
        """
        return self.flat_struct.has_public_column()