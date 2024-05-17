"""Submodule providing a class for struct metadata."""

from typing import List, Optional, Union, Tuple
import re
from functools import cache
from constraint_checkers.find_foreign_keys import TableMetadata


class AttributeMetadata:

    def __init__(
        self,
        original_name: str,
        name: str,
        data_type: Union[str, "StructMetadata"],
        optional: bool,
        reference: bool = False
    ):
        self.original_name = original_name
        self.name = name
        self._data_type = data_type
        self.optional = optional
        self.reference = reference

    def as_ref(self) -> "AttributeMetadata":
        """Returns the attribute as a reference."""
        if self._data_type == "String":
            return AttributeMetadata(
                self.original_name, self.name, "str", self.optional, reference=True
            )

        return AttributeMetadata(
            self.original_name, self.name, self._data_type, self.optional, reference=True
        )

    def is_undefined_nested_dependencies(self) -> bool:
        return not self.has_struct_data_type() and self.data_type().startswith("Nested")

    def has_struct_data_type(self) -> bool:
        return isinstance(self._data_type, StructMetadata)

    def format_data_type(self) -> str:
        data_type = self.data_type()

        if self.reference:
            data_type = f"&{data_type}"

        if self.optional:
            data_type = f"Option<{data_type}>"

        return data_type

    def raw_data_type(self) -> Union[str, "StructMetadata"]:
        return self._data_type

    def data_type(self) -> str:
        if isinstance(self._data_type, StructMetadata):
            return self._data_type.name
        elif isinstance(self._data_type, str):
            return self._data_type

        raise ValueError("The data type must be either a string or a StructMetadata.")

    def human_readable_name(self) -> str:
        return " ".join(self.name.split("_")).lower().capitalize()

    def capitalized_name(self) -> str:
        return "".join(word.capitalize() for word in self.name.split("_"))

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

    def implements_default(self) -> bool:
        return (
            not isinstance(self._data_type, StructMetadata)
            or self._data_type.can_implement_default()
        )

    def implements_eq(self) -> bool:
        return (
            isinstance(self._data_type, StructMetadata)
            and self._data_type.can_implement_eq()
            or isinstance(self._data_type, str)
            and self._data_type not in ["f32", "f64"]
        )

    def is_creator_user_id(self) -> bool:
        return self.name in ("inserted_by", "created_by")

    def is_creation_timestamp(self) -> bool:
        return self.name in ("inserted_at", "created_at")

    def is_updater_user_id(self) -> bool:
        return self.name in ("updated_by",)

    def is_update_timestamp(self) -> bool:
        return self.name in ("updated_at",)

    def is_automatically_determined_column(self) -> bool:
        return any(
            [
                self.is_creator_user_id(),
                self.is_creation_timestamp(),
                self.is_updater_user_id(),
                self.is_update_timestamp(),
            ]
        )

    def requires_authentication(self) -> bool:
        return (
            isinstance(self._data_type, StructMetadata)
            and self._data_type.requires_authentication()
            or self.is_creator_user_id()
            or self.is_updater_user_id()
        )

    def __eq__(self, other: "AttributeMetadata") -> bool:
        if not isinstance(other, AttributeMetadata):
            return False

        return self.name == other.name and self._data_type == other._data_type

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
                "Vec<ApiError>",
                "Vec<u8>",
                "ApiError",
                "NaiveDateTime",
            ]
            or isinstance(self._data_type, StructMetadata)
            and self._data_type.can_implement_clone()
        )

    def is_uuid(self) -> bool:
        """Returns whether the attribute is a UUID."""
        return self.data_type() == "Uuid"

    def __repr__(self) -> str:
        return f"AttributeMetadata({self.name}, {self.data_type()}, {self.optional})"


class StructMetadata:
    """Class representing the metadata of a struct."""

    def __init__(
        self, struct_name: str, table_name: str, table_metadata: TableMetadata
    ):
        self.name = struct_name
        self.table_name = table_name
        self.table_metadata = table_metadata
        self.attributes: List[AttributeMetadata] = []
        self._derives: List[str] = []
        self._decorators: List[str] = []
        self._flat_variant: Optional[StructMetadata] = None
        self._richest_variant: Optional[StructMetadata] = None
        self._new_variant: Optional[StructMetadata] = None
        self._update_variant: Optional[StructMetadata] = None

    def human_readable_name(self) -> str:
        """Returns the human readable name of the struct.

        Implementation details
        -----------------------
        The structs are camel cased, and this method returns the name
        splitting on the capital letters and joining the words with spaces.
        """
        if self._flat_variant is not None:
            return self._flat_variant.human_readable_name()
        return " ".join(re.findall(r"[A-Z][a-z]*", self.name)).lower().capitalize()

    def capitalized_human_readable_table_name(self) -> str:
        return self.table_name.replace("_", " ").capitalize()

    def requires_authentication(self) -> bool:
        """Returns whether the struct requires authentication.

        Implementation details
        -----------------------
        A struct requires authentication to operate on it (either
        insert or update) if it contains a field that is either the
        creator user ID or the updater user ID.
        """
        if self._flat_variant is not None:
            return self._flat_variant.requires_authentication()
        return (
            any(attribute.requires_authentication() for attribute in self.attributes)
            or self.table_name == "users"
        )

    def is_updatable(self) -> bool:
        """Returns whether the struct is updatable.

        Implementation details
        -----------------------
        A struct is updatable if it contains at least a field containing
        the updater user ID.
        """
        if self._flat_variant is not None:
            return self._flat_variant.is_updatable()
        return any(attribute.is_updater_user_id() for attribute in self.attributes) or (
            self.table_name == "users"
        )

    def is_insertable(self) -> bool:
        """Returns whether the struct is insertable.

        Implementation details
        -----------------------
        A struct is insertable if it contains at least a field containing
        the creator user ID.
        """
        if self._flat_variant is not None:
            return self._flat_variant.is_insertable()
        return (
            any(attribute.is_creator_user_id() for attribute in self.attributes)
            or self.table_name == "users"
        )

    def write_to(
        self,
        file: "File",
        diesel: Optional[str] = None,
        derives_deny_list: Optional[List[str]] = None,
    ):
        """Writes the struct to the file."""
        if diesel is not None:
            if diesel not in ["tables", "views"]:
                raise ValueError("The table type must be either 'tables' or 'views'.")

        if derives_deny_list is None:
            derives_deny_list = []

        joined_derives = ", ".join(
            [
                derive
                for derive in self.derives(diesel=diesel)
                if derive not in derives_deny_list
            ]
        )
        file.write(f"#[derive({joined_derives})]\n")
        if diesel is not None:
            file.write(f"#[diesel(table_name = {self.table_name})]\n")

        if diesel is not None:
            file.write(
                f"#[diesel(primary_key({self.get_formatted_primary_keys(include_prefix=False, include_parenthesis=False)}))]\n"
            )

        for decorator in self._decorators:
            file.write(f"#[{decorator}]\n")
        file.write(f"pub struct {self.name} {{\n")
        for attribute in self.attributes:
            file.write(f"    pub {attribute.name}: {attribute.format_data_type()},\n")
        file.write("}\n\n")

    def is_new_variant(self) -> bool:
        return (
            self.is_insertable()
            and self._flat_variant is not None
            and self.name.startswith("New")
        )

    def is_update_variant(self) -> bool:
        return self.is_updatable() and (
            (not self.has_uuid_primary_key())
            and self._flat_variant is not None
            and self.name.startswith("Update")
            or (self.has_uuid_primary_key() and self.is_new_variant())
        )

    def set_flat_variant(self, struct: "StructMetadata"):
        assert struct.table_name == self.table_name
        assert not struct.is_nested()
        self._flat_variant = struct

    def get_flat_variant(self) -> "StructMetadata":
        if self._flat_variant is None:
            raise ValueError("The flat variant has not been set.")

        return self._flat_variant

    def set_richest_variant(self, struct: "StructMetadata"):
        assert struct.table_name == self.table_name
        self._richest_variant = struct

    def get_richest_variant(self) -> "StructMetadata":
        if self._richest_variant is None:
            raise ValueError("The richest variant has not been set.")

        return self._richest_variant

    def set_new_variant(self, struct: "StructMetadata"):
        assert struct.table_name == self.table_name
        assert struct.is_new_variant()
        self._new_variant = struct

    def get_new_variant(self) -> "StructMetadata":
        if self._new_variant is None:
            raise ValueError(
                "The new variant has not been set for the struct "
                f"{self.name} associated with the table {self.table_name}."
            )

        if not self.is_insertable():
            raise ValueError(
                f"The struct {self.name} does not contain an updator user ID attribute "
                f"in the table {self.table_name}."
            )

        return self._new_variant

    def set_update_variant(self, struct: "StructMetadata"):
        assert struct.table_name == self.table_name
        assert struct.is_update_variant()
        self._update_variant = struct

    def get_update_variant(self) -> "StructMetadata":
        if self._new_variant is not None and self.get_new_variant().is_update_variant():
            return self.get_new_variant()

        if not self.is_updatable():
            raise ValueError(
                f"The struct {self.name} does not contain an updator "
                f"user ID attribute in the table {self.table_name}."
            )

        if self._update_variant is None:
            raise ValueError(
                "The update variant has not been set for the struct "
                f"{self.name} associated with the table {self.table_name}."
            )

        return self._update_variant

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
        assert isinstance(attribute_name, str), (
            "The attribute name must be a string. "
            f"The provided attribute name is a {type(attribute_name)}."
        )
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
        """Adds an attribute to the struct.
        
        Parameters
        ----------
        attribute_metadata : AttributeMetadata
            The attribute metadata to add to the struct.

        Raises
        ------
        ValueError
            If the attribute is already in the struct.
        """
        # We check if the attribute is already in the struct.
        # If it is, we raise an error.
        if self.has_attribute(attribute_metadata):
            raise ValueError(
                f"The attribute {attribute_metadata.name} is already in the struct {self.name}."
            )

        self.attributes.append(attribute_metadata)

    def add_derive(self, derive: str):
        self._derives.append(derive)

    def add_decorator(self, decorator: str):
        self._decorators.append(decorator)

    def contains_optional_fields(self) -> bool:
        return any(attribute.optional for attribute in self.attributes)

    def contains_only_optional_fields(self) -> bool:
        return all(attribute.optional for attribute in self.attributes)

    @cache
    def only_primary_key(self) -> bool:
        """Returns whether the struct contains only the primary key."""
        primary_keys = self.get_primary_keys()

        return len(self.attributes) == len(primary_keys)

    def get_creator_user_id_attribute(self) -> AttributeMetadata:
        if self.is_new_variant():
            return self.get_flat_variant().get_creator_user_id_attribute()
        for attribute in self.attributes:
            if attribute.is_creator_user_id():
                return attribute
        raise ValueError("The struct does not contain a creator user ID attribute.")

    def get_updator_user_id_attribute(self) -> AttributeMetadata:
        if self.is_update_variant() or self.is_new_variant():
            return self.get_flat_variant().get_updator_user_id_attribute()
        for attribute in self.attributes:
            if attribute.is_updater_user_id():
                return attribute
        raise ValueError(
            f"The struct {self.name} does not contain an updator user ID attribute. "
            f"The table name is {self.table_name}."
        )

    @cache
    def get_foreign_keys(self) -> List[AttributeMetadata]:
        """Returns the foreign keys of the struct.

        Implementation details
        -----------------------
        This method returns the foreign keys of the struct.
        """
        if self._flat_variant is not None:
            return self._flat_variant.get_foreign_keys()

        foreign_key_names = self.table_metadata.get_foreign_keys(self.table_name)

        foreign_keys = []

        for foreign_key_name in foreign_key_names:
            attribute = self.get_attribute_by_name(foreign_key_name)
            if attribute is None:
                raise ValueError(
                    f"The attribute {foreign_key_name} is not in the struct {self.name} associated "
                    f"with the table {self.table_name}."
                )
            foreign_keys.append(attribute)

        return foreign_keys

    def has_foreign_keys(self) -> bool:
        """Returns whether the struct has foreign keys."""
        return len(self.get_foreign_keys()) > 0

    @cache
    def get_primary_keys(self) -> List[AttributeMetadata]:
        """Returns the primary key of the struct.

        Implementation details
        -----------------------
        This method returns the primary key of the struct.
        """
        if self._flat_variant is not None:
            return self._flat_variant.get_primary_keys()

        primary_keys = self.table_metadata.get_primary_key_names_and_types(
            self.table_name
        )

        if len(primary_keys) == 0:
            raise ValueError("The table does not have a primary key.")

        primary_key_attributes = []

        for primary_key in primary_keys:
            attribute = self.get_attribute_by_name(primary_key[0])
            if attribute is None:
                raise ValueError(
                    f"The attribute {primary_key[0]} is not in the struct {self.name} associated "
                    f"with the table {self.table_name}."
                )
            primary_key_attributes.append(attribute)

        return primary_key_attributes

    @cache
    def has_uuid_primary_key(self) -> bool:
        """Returns whether the struct has a UUID primary key.

        Implementation details
        -----------------------
        This method returns whether the primary key of the struct is
        a UUID.
        """
        primary_keys = self.get_primary_keys()
        return all(attribute.data_type() == "Uuid" for attribute in primary_keys)

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
        if self.can_implement_default() and "Default" not in self._derives:
            derives.append("Default")

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
        return all(attribute.implements_serialize() for attribute in self.attributes)

    def can_implement_deserialize(self) -> bool:
        return all(attribute.implements_deserialize() for attribute in self.attributes)

    def can_implement_default(self) -> bool:
        return all(attribute.implements_default() for attribute in self.attributes)

    def has_attribute(self, attribute: AttributeMetadata) -> bool:
        """Returns whether the struct has the attribute."""
        return any(
            attribute == existing_attribute for existing_attribute in self.attributes
        )

    def get_capitalized_table_name(self) -> str:
        return "".join(word.capitalize() for word in self.table_name.split("_"))

    def is_junktion_table(self) -> bool:
        """Returns whether the table is a junktion table.

        Implementation details
        -----------------------
        A table is a junktion table if it has a primary key that is
        a combination of foreign keys.
        """
        return len(self.get_primary_keys()) > 1

    def get_formatted_primary_keys(
        self,
        include_prefix: bool,
        prefix: str = "self",
        include_parenthesis: bool = True,
    ) -> str:
        """Returns the formatted primary keys.

        Parameters
        ----------
        include_prefix : bool
            Whether to include the prefix keyword in the formatted primary keys.
        prefix : str
            The prefix to use for the formatted primary keys.
            By default, it is set to "self".
        include_parenthesis : bool
            Whether to include the parenthesis in the formatted primary keys.
            By default, it is set to True and applied only when there are
            more than one primary keys.
        """
        keys = self.get_primary_keys()

        formatted_keys = ", ".join(
            f"{prefix}.{attribute.name}" if include_prefix else attribute.name
            for attribute in keys
        )

        if len(keys) > 1 and include_parenthesis:
            return f"( {formatted_keys} )"
        return formatted_keys

    def get_formatted_primary_key_data_types(self) -> str:
        """Returns the formatted primary key data types."""
        keys = self.get_primary_keys()

        formatted_keys = ", ".join(attribute.format_data_type() for attribute in keys)

        if len(keys) > 1:
            return f"( {formatted_keys} )"
        return formatted_keys

    def has_only_foreign_keys(self) -> bool:
        """Returns whether the struct has only foreign keys."""
        foreign_keys = self.get_foreign_keys()

        return len(self.attributes) == len(foreign_keys)

    def has_associated_roles(self) -> bool:
        """Returns whether there is a roles table associated with the struct."""
        return self.table_metadata.has_associated_roles(self.table_name)

    def get_unique_constraints(self) -> List[List[AttributeMetadata]]:
        """Returns the unique constraints of the struct.

        Implementation details
        -----------------------
        This method returns the unique constraints of the struct.
        """
        if self._flat_variant is not None:
            return self._flat_variant.get_unique_constraints()

        unique_constraints: List[Tuple[str]] = self.table_metadata.get_unique_constraint_columns(self.table_name)

        unique_constraints_attributes: List[List[AttributeMetadata]] = []

        for unique_constraint in unique_constraints:
            unique_constraint_attributes: List[AttributeMetadata] = []

            for attribute_name in unique_constraint:
                attribute = self.get_attribute_by_name(attribute_name)
                if attribute is None:
                    raise ValueError(
                        f"The attribute {attribute_name} is not in the struct {self.name} associated "
                        f"with the table {self.table_name}."
                    )
                unique_constraint_attributes.append(attribute)

            unique_constraints_attributes.append(unique_constraint_attributes)

        return unique_constraints_attributes