"""This module contains the TableMetadata class."""
from typing import List
from constraint_checkers.struct_metadata import StructMetadata, AttributeMetadata


class TableStructMetadata:
    """Contains metadata about the struct associated to a table."""

    def __init__(self, table_name: str):
        self.name = table_name
        self.richest_struct: StructMetadata = None
        self.flat_variant: StructMetadata = None
        self.new_flat_variant: StructMetadata = None
        self.update_flat_variant: StructMetadata = None

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
            for attribute in self.flat_variant.attributes
        )

    def is_updatable(self) -> bool:
        """Returns whether the table is updatable.

        Implementation details
        -----------------------
        A table is updatable if the associated struct has at least a
        field containing the updater user ID.
        """
        return self.flat_variant.is_updatable()

    def is_insertable(self) -> bool:
        """Returns whether the table is insertable.

        Implementation details
        -----------------------
        A table is insertable if the associated struct has at least a
        field containing the creator user ID.
        """
        return self.flat_variant.is_insertable()

    def has_uuid_primary_key(self) -> bool:
        """Returns whether the table has a UUID primary key.

        Implementation details
        -----------------------
        This method returns whether the primary key of the table is
        a UUID.
        """
        return self.richest_struct.has_uuid_primary_key()

    def set_new_flat_variant(self, struct: StructMetadata):
        assert struct.table_name == self.name
        assert not struct.is_nested()
        assert struct.is_new_variant()
        if self.new_flat_variant is not None:
            raise ValueError(
                "The new flat struct has already been set. This is unexpected. Please check the table and the structs associated to it."
            )
        self.new_flat_variant = struct

    def get_new_flat_variant(self) -> StructMetadata:
        if self.new_flat_variant is None:
            raise ValueError(
                f"The new flat struct has not been set for the table {self.name}."
            )
        return self.new_flat_variant

    def set_update_flat_variant(self, struct: StructMetadata):
        assert struct.table_name == self.name
        assert not struct.is_nested()
        assert not struct.is_new_variant()
        assert not self.has_uuid_primary_key(), (
            f"The table {self.name} has a UUID primary key. "
            "This means that the update flat struct cannot be set, "
            "as it would be identical to the new flat struct."
        )
        if self.update_flat_variant is not None:
            raise ValueError(
                "The update flat struct has already been set. This is unexpected. Please check the table and the structs associated to it."
            )
        self.update_flat_variant = struct

    def get_update_flat_variant(self) -> StructMetadata:
        if self.has_uuid_primary_key():
            return self.get_new_flat_variant()
        if self.update_flat_variant is None:
            raise ValueError(
                f"The update flat struct has not been set for the table {self.name}."
            )
        return self.update_flat_variant

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

    def set_flat_variant(self, struct: StructMetadata):
        assert struct.table_name == self.name
        if struct.is_nested():
            raise ValueError(
                "The flat struct cannot be a nested struct. Please provide a struct that is not nested."
            )
        if self.flat_variant is not None:
            raise ValueError(
                "The flat struct has already been set. This is unexpected. Please check the table and the structs associated to it."
            )
        self.flat_variant = struct

    def get_flat_variant(self) -> StructMetadata:
        """Returns the flat struct of the table."""
        if self.flat_variant is None:
            raise ValueError(
                f"The flat struct has not been set for the table {self.name}."
            )
        return self.flat_variant

    def flat_variant_name(self) -> str:
        return self.flat_variant.name

    def new_flat_variant_name(self) -> str:
        return self.get_new_flat_variant().name

    def update_flat_variant_name(self) -> str:
        return self.get_update_flat_variant().name

    def richest_struct_name(self) -> str:
        return self.richest_struct.name

    def camel_cased(self) -> str:
        return "".join(word.capitalize() for word in self.name.split("_"))

    def has_updated_at_column(self) -> bool:
        return self.flat_variant.is_updatable()

    def get_primary_keys(self) -> List[AttributeMetadata]:
        return self.flat_variant.get_primary_keys()

    def is_junktion_table(self) -> bool:
        """Returns whether the table is a junktion table.
        
        Implementation details
        -----------------------
        A table is a junktion table if it has a primary key that is
        a combination of foreign keys.
        """
        return self.flat_variant.is_junktion_table()

    def has_associated_roles(self) -> bool:
        """Returns whether the table has an associated roles table.
        
        Implementation details
        -----------------------
        A table has an associated roles table if it has a column
        that is a foreign key to the roles table.
        """
        return self.flat_variant.has_associated_roles()

    def has_public_column(self) -> bool:
        """Returns whether the table has a public column.
        
        Implementation details
        -----------------------
        A table has a public column if it has a column that is
        a boolean and that is named "public".
        """
        return self.flat_variant.has_public_column()

    def get_public_column(self) -> str:
        """Returns the name of the public column."""
        return self.flat_variant.get_public_column()

    def may_be_hidden(self) -> bool:
        """Returns whether the table may be hidden.
        
        Implementation details
        -----------------------
        A table may be hidden if it has a column that is a boolean
        and that is named "hidden".
        """
        return self.flat_variant.may_be_hidden()

    def get_foreign_key_flat_variant(self, column_name: AttributeMetadata) -> StructMetadata:
        """Returns the flat variant of the foreign key."""
        return self.flat_variant.get_foreign_key_flat_variant(column_name)

    def has_column(self, column_name: str) -> bool:
        """Returns whether the table has a column with the given name."""
        return column_name in self.flat_variant.table_metadata.get_columns(self.name)

    def has_filter_variant(self) -> bool:
        """Returns whether the table has a filter variant."""
        return self.flat_variant.has_filter_variant()
        
    def get_filter_variant(self) -> StructMetadata:
        """Returns the filter variant of the table."""
        return self.flat_variant.get_filter_variant()

    def is_searchable(self) -> bool:
        """Returns whether the table is searchable."""
        return self.flat_variant.is_searchable()
    
    def editability_always_depend_on_parent_column(self) -> bool:
        """Returns whether the editability of the table depends on the parent column."""
        return self.flat_variant.editability_always_depend_on_parent_column()

    def editability_may_depend_on_parent_columns(self) -> bool:
        """Returns whether the editability of the table may depend on the parent column."""
        return self.flat_variant.editability_may_depend_on_parent_columns()

    def get_editability_determinant_columns(self) -> List[AttributeMetadata]:
        """Returns the columns that determine the editability of the table."""
        return self.flat_variant.get_editability_determinant_columns()

    def get_can_update_function_name(self) -> str:
        """Returns the name of the can_update function."""
        return self.flat_variant.get_can_update_function_name()

    def has_can_update_function(self) -> bool:
        """Returns whether the table has a can_update function."""
        return self.flat_variant.has_can_update_function()

    def get_can_view_function_name(self) -> str:
        """Returns the name of the can_view function."""
        return self.flat_variant.get_can_view_function_name()

    def get_can_admin_function_name(self) -> str:
        """Returns the name of the can_admin function."""
        return self.flat_variant.get_can_admin_function_name()

    def get_can_update_trigger_name(self) -> str:
        """Returns the name of the can_update trigger."""
        return self.flat_variant.get_can_update_trigger_name()

    def has_can_update_trigger(self) -> bool:
        """Returns whether the table has a can_update trigger."""
        return self.flat_variant.has_can_update_trigger()

    def has_can_view_function(self) -> bool:
        """Returns whether the table has a can_view function."""
        return self.flat_variant.has_can_view_function()

    def has_can_admin_function(self) -> bool:
        """Returns whether the table has a can_admin function."""
        return self.flat_variant.has_can_admin_function()

    def get_foreign_key_table_name(self, column_name: str) -> str:
        """Returns the table name of the foreign key."""
        return self.flat_variant.table_metadata.get_foreign_key_table_name(self.name, column_name)