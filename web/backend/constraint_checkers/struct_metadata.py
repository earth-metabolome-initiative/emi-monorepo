"""Submodule providing a class for struct metadata."""

from typing import List, Optional, Union
import re
from functools import cache
from constraint_checkers.find_foreign_keys import find_foreign_keys


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
                "Vec<ApiError>",
                "Vec<u8>",
                "ApiError",
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
        return any(attribute.is_creator_user_id() for attribute in self.attributes)

    def write_to(self, file: "File", diesel: Optional[str] = None):
        if diesel is not None:
            if diesel not in ["tables", "views"]:
                raise ValueError("The table type must be either 'tables' or 'views'.")

        file.write(f"#[derive({', '.join(self.derives(diesel=diesel))})]\n")
        if diesel is not None:
            file.write(f"#[diesel(table_name = {self.table_name})]\n")
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
        self.attributes.append(attribute_metadata)

    def add_derive(self, derive: str):
        self._derives.append(derive)

    def add_decorator(self, decorator: str):
        self._decorators.append(decorator)

    def contains_optional_fields(self) -> bool:
        return any(attribute.optional for attribute in self.attributes)

    def contains_only_optional_fields(self) -> bool:
        return all(attribute.optional for attribute in self.attributes)

    def only_primary_key(self) -> bool:
        """Returns whether the struct contains only the primary key."""
        if len(self.attributes) != 1:
            return False

        table_metadata = find_foreign_keys()
        return (
            self.attributes[0].name
            == table_metadata.get_primary_key_name_and_type(self.table_name)[0]
        )

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
    def get_primary_key(self) -> AttributeMetadata:
        """Returns the primary key of the struct.

        Implementation details
        -----------------------
        This method returns the primary key of the struct.
        """
        table_metadata = find_foreign_keys()
        primary_key_name, _primary_key_type = (
            table_metadata.get_primary_key_name_and_type(self.table_name)
        )
        return self.get_attribute_by_name(primary_key_name)

    @cache
    def has_uuid_primary_key(self) -> bool:
        """Returns whether the struct has a UUID primary key.

        Implementation details
        -----------------------
        This method returns whether the primary key of the struct is
        a UUID.
        """
        table_metadata = find_foreign_keys()
        primary_key_name, primary_key_type = (
            table_metadata.get_primary_key_name_and_type(self.table_name)
        )
        return primary_key_type == "uuid"

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

    def is_nested(self) -> bool:
        return any(
            isinstance(attribute._data_type, StructMetadata)
            for attribute in self.attributes
        )

    def get_capitalized_table_name(self) -> str:
        return "".join(word.capitalize() for word in self.table_name.split("_"))
