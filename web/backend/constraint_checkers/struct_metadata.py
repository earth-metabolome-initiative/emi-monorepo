"""Submodule providing a class for struct metadata."""

from typing import List, Optional, Union, Tuple, Dict
import re
from constraint_checkers.find_foreign_keys import (
    TableMetadata,
    find_foreign_keys,
    is_role_invitation_table,
    is_role_request_table,
)
from constraint_checkers.indices import (
    PGIndices,
    find_primary_search_indices,
    PGIndex,
    DerivedPGIndex,
)
from constraint_checkers.utils import infer_route_from_document
from constraint_checkers.gluesql_types_mapping import (
    GLUESQL_TYPES_MAPPING,
)

TYPES_FROM_CORE_AND_STANDARD_LIBRARIES = [
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
    "Vec<u8>",
    "String",
    "str",
]


def rust_type_to_diesel_type(rust_type: str) -> str:
    """Converts a Rust type to a diesel type, including full crate path.

    Parameters
    ----------
    rust_type : str
        The Rust type to convert.

    Returns
    -------
    str
        The converted diesel type.

    Examples
    --------
    >>> rust_type_to_diesel_type("i32")
    "diesel::sql_types::Integer"
    >>> rust_type_to_diesel_type("Uuid")
    "diesel::sql_types::Uuid"
    """
    if rust_type == "i32":
        return "diesel::sql_types::Integer"
    if rust_type == "Uuid":
        return "diesel::sql_types::Uuid"
    if rust_type == "String":
        return "diesel::sql_types::Text"
    if rust_type == "bool":
        return "diesel::sql_types::Bool"
    if rust_type == "f32":
        return "diesel::sql_types::Float4"
    if rust_type == "f64":
        return "diesel::sql_types::Float8"
    if rust_type == "NaiveDateTime":
        return "diesel::sql_types::Timestamp"
    if rust_type == "Vec<u8>":
        return "diesel::sql_types::Binary"

    raise ValueError(f"Unsupported Rust type: {rust_type}")


def data_type_to_absolute_import_path(
    data_type: Union[str, "StructMetadata"], route: str
) -> str:
    """Converts a data type to an absolute import path.

    Parameters
    ----------
    data_type : str
        The data type to convert.
    route : str
        The route of the file.

    Returns
    -------
    str
        The converted absolute import path.
    """
    assert isinstance(route, str), (
        "The route must be a string. " f"The provided route is a {type(route)}."
    )
    assert route in ("backend", "web_common", "frontend"), (
        "The route must be either 'backend', 'web_common', or 'frontend'. "
        f"The provided route is {route}."
    )

    if data_type in TYPES_FROM_CORE_AND_STANDARD_LIBRARIES:
        return data_type

    if isinstance(data_type, StructMetadata):
        return data_type.full_path(route)

    if "ApiError" in data_type:
        return data_type

    if data_type in ("Uuid", "uuid::Uuid"):
        return "uuid::Uuid"

    if "uuid::Uuid" in data_type:
        return data_type

    if data_type == "PrimaryKey":
        if route in ("backend", "frontend"):
            return "web_common::database::PrimaryKey"
        if route == "web_common":
            return "crate::database::PrimaryKey"

    if data_type in ["( i32, i32 )"]:
        return data_type

    if data_type == "NaiveDateTime" or data_type == "chrono::NaiveDateTime":
        return "chrono::NaiveDateTime"

    if data_type in [
        "PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>",
        "diesel::r2d2::PooledConnection<diesel::r2d2::ConnectionManager<diesel::PgConnection>>",
    ]:
        return "diesel::r2d2::PooledConnection<diesel::r2d2::ConnectionManager<diesel::PgConnection>>"

    if data_type in [
        "Result<usize, gluesql::prelude::Error>",
    ]:
        return "Result<usize, gluesql::prelude::Error>"

    if data_type in ["gluesql::prelude::Glue<C>"]:
        return "gluesql::prelude::Glue<C>"

    if data_type == "JPEG":
        if route == "backend":
            return "crate::database::diesel_types::JPEG"
        if route == "web_common":
            return "crate::types::JPEG"
        if route == "frontend":
            return "web_common::types::JPEG"

    if data_type == "Point":
        if route == "backend":
            return "postgis_diesel::types::Point"
        if route == "web_common":
            return "crate::types::Point"
        if route == "frontend":
            return "web_common::types::Point"

    raise NotImplementedError(f"Unsupported data type: {data_type}")


class AttributeMetadata:
    """Class representing the metadata of an attribute."""

    def __init__(
        self,
        original_name: str,
        name: str,
        data_type: Union[str, "StructMetadata"],
        optional: bool = False,
        unique: bool = False,
        reference: bool = False,
        mutable: bool = False,
        rc: bool = False,
        lifetime: Optional[str] = None,
    ):
        assert isinstance(original_name, str), (
            "The original name must be a string. "
            f"The provided original name is a {type(original_name)}."
        )
        assert isinstance(name, str), (
            "The name must be a string. " f"The provided name is a {type(name)}."
        )
        assert len(name) > 0, "The name must not be empty."
        assert isinstance(data_type, (str, StructMetadata)), (
            "The data type must be a string or a StructMetadata. "
            f"The provided data type is a {type(data_type)}."
        )
        assert isinstance(optional, bool), (
            "The optional flag must be a boolean. "
            f"The provided optional flag is a {type(optional)}."
        )
        assert isinstance(unique, bool), (
            "The unique flag must be a boolean. "
            f"The provided unique flag is a {type(unique)}."
        )
        assert isinstance(reference, bool), (
            "The reference flag must be a boolean. "
            f"The provided reference flag is a {type(reference)}."
        )
        assert isinstance(mutable, bool), (
            "The mutable flag must be a boolean. "
            f"The provided mutable flag is a {type(mutable)}."
        )
        assert isinstance(rc, bool), (
            "The Rc flag must be a boolean. " f"The provided Rc flag is a {type(rc)}."
        )
        assert lifetime is None or isinstance(lifetime, str), (
            "The lifetime must be a string or None. "
            f"The provided lifetime is a {type(lifetime)}."
        )
        self.original_name = original_name
        self.name = name
        self._data_type = data_type
        self.optional = optional
        self.unique = unique
        self.reference = reference
        self.mutable = mutable
        self.rc = rc
        self.lifetime = lifetime

        if self.rc is not None:
            assert lifetime is None, (
                "The lifetime must be None if the RC is not None. "
                f"The provided lifetime is {lifetime}."
            )

        if self.lifetime is not None and not self.reference:
            raise ValueError("The lifetime must be set only for references.")

        if self.lifetime is not None:
            assert "'" not in self.lifetime, (
                "The lifetime must not contain the ' character. "
                f"The provided lifetime is {self.lifetime}."
            )
            assert " " not in self.lifetime, (
                "The lifetime must not contain the space character. "
                f"The provided lifetime is '{self.lifetime}'."
            )

        self.owner: "StructMetadata" = None

    def as_ref(self) -> "AttributeMetadata":
        """Returns the attribute as a reference."""
        if self._data_type == "String":
            return AttributeMetadata(
                self.original_name, self.name, "str", self.optional, reference=True
            )

        return AttributeMetadata(
            self.original_name,
            self.name,
            self._data_type,
            self.optional,
            unique=self.unique,
            rc=self.rc,
            reference=True,
        )

    def as_option(self) -> "AttributeMetadata":
        """Returns the attribute as an option."""
        return AttributeMetadata(
            self.original_name,
            self.name,
            self._data_type,
            True,
            reference=self.reference,
            unique=self.unique,
            rc=self.rc,
            mutable=self.mutable,
        )

    def as_rc(self) -> "AttributeMetadata":
        """Returns the attribute as an Rc."""
        return AttributeMetadata(
            self.original_name,
            self.name,
            self._data_type,
            self.optional,
            reference=self.reference,
            unique=self.unique,
            mutable=self.mutable,
            rc=True,
            lifetime=self.lifetime,
        )

    def has_backend_type(self) -> bool:
        """Returns whether the attribute has a backend data type."""
        return data_type_to_absolute_import_path(
            self._data_type, route="backend"
        ) != data_type_to_absolute_import_path(self._data_type, route="frontend")

    def is_file(self) -> bool:
        """Returns whether the attribute is an image blob."""
        return (
            not self.has_struct_data_type()
            and GLUESQL_TYPES_MAPPING[self.raw_data_type()]
            == "gluesql::core::ast_builder::bytea({value})"
        )

    def is_jpeg(self) -> bool:
        """Returns whether the attribute is an image blob."""
        return self.data_type(route="frontend") == "web_common::types::JPEG"

    def is_point(self) -> bool:
        """Returns whether the attribute is a point."""
        return self.data_type(route="frontend") == "web_common::types::Point"

    def is_boolean(self) -> bool:
        """Returns whether the attribute is a boolean."""
        return self.data_type(route="frontend") == "bool"

    def is_date_type(self) -> bool:
        """Returns whether the attribute is a date type."""
        return self.data_type(route="frontend") == "chrono::NaiveDateTime"

    def is_undefined_nested_dependencies(self) -> bool:
        """Returns whether the attribute is an undefined nested dependencies."""
        return not self.has_struct_data_type() and self._data_type.startswith("Nested")

    def has_struct_data_type(self) -> bool:
        """Returns whether the attribute has a struct data type."""
        return isinstance(self._data_type, StructMetadata)

    def format_data_type(self, route: str, diesel: bool = False, skip_remap: bool = False) -> str:
        """Returns the formatted data type of the attribute.

        Parameters
        ----------
        route : str
            The route of the file.
            Can either be 'backend', 'web_common', or 'frontend'.
        diesel : bool
            Whether to format the data type for the diesel crate.
        """
        if diesel:
            assert not self.has_struct_data_type()

        if skip_remap:
            data_type = self.raw_data_type()
        else:
            try:
                data_type = self.data_type(route=route)
            except NotImplementedError as e:
                raise NotImplementedError(
                    f"Could not format the data type of the attribute {self.name}."
                ) from e

        if diesel:
            data_type = rust_type_to_diesel_type(data_type)

        if self.mutable:
            data_type = f"mut {data_type}"

        if self.lifetime is not None:
            data_type = f"'{self.lifetime} {data_type}"

        if self.reference:
            data_type = f"&{data_type}"

        if self.rc:
            data_type = f"Rc<{data_type}>"

        if self.optional:
            if diesel:
                data_type = f"diesel::sql_types::Nullable<{data_type}>"
            else:
                data_type = f"Option<{data_type}>"

        return data_type

    def raw_data_type(self) -> Union[str, "StructMetadata"]:
        """Returns the raw data type of the attribute."""
        return self._data_type

    def data_type(self, route: str) -> str:
        """Returns the data type of the attribute.

        Parameters
        ----------
        route : str
            The route of the file.
            Can either be 'backend', 'web_common', or 'frontend'.
        """
        if isinstance(self._data_type, StructMetadata) or isinstance(self._data_type, str):
            return data_type_to_absolute_import_path(self._data_type, route)

        raise ValueError("The data type must be either a string or a StructMetadata.")

    def sql_data_type(self) -> str:
        """Returns the SQL data type of the attribute."""
        if self.has_struct_data_type():
            raise ValueError(
                "The attribute does not have a SQL data type as it is a struct."
            )
        assert isinstance(self._data_type, str)
        if self._data_type == "i32":
            return "INTEGER"
        if self._data_type == "uuid::Uuid":
            return "UUID"

        raise ValueError(
            f"The data type {self._data_type} is not supported for SQL data types."
        )

    def human_readable_name(self) -> str:
        """Returns the human-readable name of the attribute."""
        return " ".join(self.name.split("_")).lower().capitalize()

    def implements_serialize(self) -> bool:
        """Returns whether the attribute implements the Serialize trait."""
        return (
            not isinstance(self._data_type, StructMetadata)
            or self._data_type.can_implement_serialize()
        )

    def implements_deserialize(self) -> bool:
        """Returns whether the attribute implements the Deserialize trait."""
        return (
            not isinstance(self._data_type, StructMetadata)
            or self._data_type.can_implement_deserialize()
        )

    def implements_default(self) -> bool:
        """Returns whether the attribute implements the Default trait."""
        return (
            isinstance(self._data_type, str)
            and self._data_type
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
                "String",
                "Vec<u8>",
                "uuid::Uuid",
                "chrono::NaiveDateTime",
                "JPEG",
            ]
            or isinstance(self._data_type, StructMetadata)
            and self._data_type.can_implement_default()
        )

    def implements_eq(self) -> bool:
        """Returns whether the attribute implements the Eq trait."""
        return (
            isinstance(self._data_type, StructMetadata)
            and self._data_type.can_implement_eq()
            or isinstance(self._data_type, str)
            and self._data_type
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
                "uuid::Uuid",
                "chrono::NaiveDateTime",
            ]
        )

    def implements_as_ref_str(self) -> bool:
        """Returns whether the attribute implements AsRef<str>"""
        return self.data_type(route="frontend") in ("str", "String")

    def is_creator_user_id(self) -> bool:
        """Returns whether the attribute is the creator user ID."""
        return self.name in ("created_by")

    def is_creation_timestamp(self) -> bool:
        """Returns whether the attribute is the creation timestamp."""
        return self.name in ("created_at",)

    def is_updater_user_id(self) -> bool:
        """Returns whether the attribute is the updater user ID."""
        return self.name in ("updated_by",)

    def is_update_timestamp(self) -> bool:
        """Returns whether the attribute is the update timestamp."""
        return self.name in ("updated_at",)

    def is_sampled_by(self) -> bool:
        """Returns whether the attribute is the sampled by."""
        return self.name in ("sampled_by",) and (
            (self.has_struct_data_type() and self.raw_data_type().table_name == "users")
            or self.data_type(route="frontend") == "i32"
        )

    def is_inner(self) -> bool:
        """Returns whether the attribute is the inner attribute."""
        return (
            self.name == "inner"
            and self.has_struct_data_type()
            and self.owner.is_nested()
            and self.raw_data_type() == self.owner.get_flat_variant()
        )

    def into_new_owner(self, owner: "StructMetadata") -> "AttributeMetadata":
        """Creates a new attribute with a new owner.

        Parameters
        ----------
        owner : StructMetadata
            The new owner of the attribute.
        """
        new_attribute = AttributeMetadata(
            original_name=self.original_name,
            name=self.name,
            data_type=self._data_type,
            optional=self.optional,
            unique=self.unique,
            reference=self.reference,
            mutable=self.mutable,
            rc=self.rc,
            lifetime=self.lifetime,
        )
        new_attribute.set_owner(owner)
        return new_attribute

    def set_owner(self, owner: "StructMetadata"):
        """Sets the owner of the attribute."""
        assert isinstance(owner, StructMetadata), (
            "The owner must be a StructMetadata. "
            f"The provided owner is a {type(owner)}."
        )
        assert self.owner is None or self.owner == owner, (
            f"The attribute {self.name} already has an owner. "
            f"The owner is {self.owner.name}, and the new owner would be {owner.name}."
        )
        self.owner = owner

    def is_automatically_determined_column(self) -> bool:
        """Returns whether the attribute is an automatically determined column."""
        return any(
            [
                self.is_creator_user_id(),
                self.is_creation_timestamp(),
                self.is_updater_user_id(),
                self.is_update_timestamp(),
            ]
        )

    def requires_authentication(self) -> bool:
        """Returns whether the attribute requires authentication."""
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

    def implements_copy(self) -> bool:
        """Returns whether the attribute implements the Copy trait."""
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
                "Point",
                "uuid::Uuid",
                "chrono::NaiveDateTime",
            ]
            or isinstance(self._data_type, StructMetadata)
            and self._data_type.can_implement_copy()
        )

    def implements_clone(self) -> bool:
        """Returns whether the attribute implements the Clone trait."""
        return (
            self.implements_copy()
            or self._data_type
            in [
                "String",
                "Vec<ApiError>",
                "Vec<u8>",
                "JPEG",
                "ApiError",
            ]
            or isinstance(self._data_type, StructMetadata)
            and self._data_type.can_implement_clone()
        )

    def implements_ord(self) -> bool:
        """Returns whether the attribute implements the Ord trait."""
        return (
            self._data_type
            in [
                "u8",
                "u16",
                "u32",
                "u64",
                "u128",
                "i8",
                "i16",
                "i32",
                "i64",
                "i128",
                "chrono::NaiveDateTime",
            ]
            or isinstance(self._data_type, StructMetadata)
            and self._data_type.can_implement_ord()
        )

    def implements_partial_ord(self) -> bool:
        """Returns whether the attribute implements the PartialOrd trait."""
        return (
            self.implements_ord()
            or self._data_type in ["f32", "f64"]
            or isinstance(self._data_type, StructMetadata)
            and self._data_type.can_implement_partial_ord()
        )

    def is_uuid(self) -> bool:
        """Returns whether the attribute is a UUID."""
        return self.data_type(route="frontend") == "uuid::Uuid"

    def is_description(self) -> bool:
        """Returns whether the attribute is a description."""
        # An attribute may be considered a description when its
        # name is either 'notes' or 'description' and has a string
        # data type.
        return (
            self.name in ("notes", "description")
            and self.data_type(route="frontend") == "String"
        )

    def is_color(self) -> bool:
        """Returns whether the attribute is a color."""
        return (
            self.data_type(route="frontend")
            == "web_common::database::flat_variants::Color"
            and self.has_struct_data_type()
        )

    def normalized_name(self) -> str:
        """Returns the name of the attribute eventually without the _id suffix."""
        if self.name.endswith("_id"):
            return self.name[:-3]
        return self.name

    def capitalized_normalized_name(self) -> str:
        """Returns the name of the attribute eventually without the _id suffix."""
        return "".join(word.capitalize() for word in self.normalized_name().split("_"))

    def __repr__(self) -> str:
        return f"AttributeMetadata({self.name}, {self.data_type(route='frontend')}, optional={self.optional})"

    def __hash__(self) -> int:
        return hash((self.name, self.data_type(route="frontend")))

    def get_attribute_path(self, attribute: "AttributeMetadata") -> str:
        """Returns the path to the attribute.

        Parameters
        ----------
        attribute : AttributeMetadata
            The attribute to get the path to.

        Raises
        ------
        ValueError
            If the attribute is not in the struct.
        """
        isinstance(attribute, AttributeMetadata)

        if attribute == self:
            return self.name

        if not self.has_struct_data_type():
            raise ValueError(
                f"The attribute {self.name} does not have a struct data type. "
                f"As such, it cannot contain the attribute {attribute.name}."
            )

        for inner_attribute in self._data_type.attributes:
            try:
                path = inner_attribute.get_attribute_path(attribute)
                return f"{self.name}.{path}"
            except ValueError:
                continue

        raise ValueError(
            f"The attribute {attribute.name} is not in the struct {self.name}."
        )


class MethodDefinition:
    """Class representing the definition of a method."""

    def __init__(
        self, name: str, summary: str, visibility: str = "pub", is_async: bool = False
    ):
        self.name = name
        self.summary = summary
        self.visibility = visibility
        self.arguments: List[AttributeMetadata] = []
        self.argument_descriptions: Dict[str, str] = {}
        self.return_type: Optional[AttributeMetadata] = None
        self.owner: Optional["StructMetadata"] = None
        self.is_async = is_async
        self.generics: List[str] = []
        self.where_clauses: List[str] = []
        assert isinstance(summary, str), (
            "The summary must be a string. "
            f"The provided summary is a {type(summary)}."
        )
        assert len(summary) > 0, "The summary must not be empty."
        assert " " not in self.name, (
            "The method name must not contain the space character. "
            f"The provided method name is {self.name}."
        )

    def is_private(self) -> bool:
        """Returns whether the method is private."""
        return self.visibility == ""

    def has_primary_key_as_argument(self) -> bool:
        """Returns whether the method has the primary key as an argument."""
        return self.get_primary_key_argument() is not None

    def is_primary_key_argument(self, argument: AttributeMetadata) -> bool:
        """Returns whether the argument is the primary key argument.

        Parameters
        ----------
        argument : AttributeMetadata
            The argument to check.
        """
        return self.get_primary_key_argument() == argument

    def get_primary_key_argument(self) -> Optional[AttributeMetadata]:
        """Returns the primary key argument."""
        for argument in self.arguments:
            if argument.name == self.owner.get_formatted_primary_keys(
                include_prefix=False
            ):
                return argument
        return None

    def into_new_owner(self, owner: "StructMetadata") -> "MethodDefinition":
        """Creates a new method definition with a new owner.

        Parameters
        ----------
        owner : StructMetadata
            The new owner of the method.
        """
        new_method = MethodDefinition(self.name, self.summary)
        new_method.set_owner(owner)
        if self.has_self_reference():
            new_method.include_self_ref()
        if self.has_self_owned():
            new_method.include_self()
        for argument in self.arguments:
            if argument.name == "self":
                continue
            new_method.add_argument(argument, self.argument_descriptions[argument.name])
        new_method.return_type = self.return_type
        new_method.is_async = self.is_async
        new_method.generics = self.generics
        new_method.where_clauses = self.where_clauses
        new_method.visibility = self.visibility
        return new_method

    def add_generic(self, generic: str):
        """Adds a generic to the method.

        Parameters
        ----------
        generic : str
            The generic to add to the method.
        """
        assert len(generic) > 0, "The generic must not be empty."
        self.generics.append(generic)

    def add_where_clause(self, where_clause: str):
        """Adds a where clause to the method.

        Parameters
        ----------
        where_clause : str
            The where clause to add to the method.
        """
        assert len(where_clause) > 0, "The where clause must not be empty."
        self.where_clauses.append(where_clause)

    def set_owner(self, owner: "StructMetadata"):
        """Sets the owner of the method.

        Parameters
        ----------
        owner : StructMetadata
            The owner of the method.
        """
        assert isinstance(owner, StructMetadata), (
            "The owner must be a StructMetadata. "
            f"The provided owner is a {type(owner)}."
        )
        assert self.owner is None or self.owner == owner, (
            f"The method {self.name} already has an owner. "
            f"The owner is {self.owner.name}."
        )
        self.owner = owner

    def include_self_ref(self):
        """Whether to include the self argument in the method."""
        assert len(self.arguments) == 0
        assert self.owner is not None, (
            "The owner of the method must be set before including the self argument. "
            f"The method is {self.name}."
        )
        self.arguments.append(
            AttributeMetadata(
                original_name="self",
                name="self",
                data_type=self.owner,
                optional=False,
                reference=True,
            )
        )

    def include_self(self):
        """Whether to include the self argument in the method."""
        assert len(self.arguments) == 0
        self.arguments.append(
            AttributeMetadata("self", "self", self.owner, False, reference=False)
        )

    def get_argument_description(self, argument: AttributeMetadata) -> str:
        """Returns the description of the argument.

        Parameters
        ----------
        argument : AttributeMetadata
            The argument to get the description of.
        """
        assert argument in self.arguments, (
            "The argument must be in the method. "
            f"The provided argument is {argument}."
        )
        if argument.name not in self.argument_descriptions:
            raise ValueError(
                f"The argument {argument.name} does not have a description in the method {self.name}."
            )
        return self.argument_descriptions[argument.name]

    def add_argument(self, argument: AttributeMetadata, description: str):
        """Adds an argument to the method.

        Parameters
        ----------
        argument : AttributeMetadata
            The argument to add to the method.
        """
        if argument in self.arguments:
            raise ValueError(
                f"The argument {argument.name} is already in the method {self.name}."
            )

        assert isinstance(description, str), (
            "The description must be a string. "
            f"The provided description is a {type(description)}."
        )
        assert len(description) > 0, "The description must not be empty."

        assert argument.name != "self", "The argument name must not be 'self'. "

        self.arguments.append(argument)
        self.argument_descriptions[argument.name] = description

    def set_return_type(self, return_type: AttributeMetadata):
        """Sets the return type of the method.

        Parameters
        ----------
        return_type : AttributeMetadata
            The return type of the method.
        """
        if self.return_type is not None:
            raise ValueError(
                f"The return type of the method {self.name} has already been set."
            )
        self.return_type = return_type

    def get_return_type(self) -> AttributeMetadata:
        """Returns the return type of the method."""
        if self.return_type is None:
            raise ValueError(
                f"The return type of the method '{self.name}' has not been set. "
                f"The owner of the method is {self.owner.name}."
            )
        return self.return_type

    def get_argument_by_name(self, argument_name: str) -> Optional[AttributeMetadata]:
        """Returns the argument by name.

        Parameters
        ----------
        argument_name : str
            The name of the argument to retrieve.
        """
        assert isinstance(argument_name, str), (
            "The argument name must be a string. "
            f"The provided argument name is a {type(argument_name)}."
        )
        for argument in self.arguments:
            if argument.name == argument_name:
                return argument
        return None

    def has_self_reference(self) -> bool:
        """Returns whether the method has a reference to self."""
        result = any(
            argument.name == "self"
            and argument.reference
            and argument.has_struct_data_type()
            and argument.raw_data_type().name == self.owner.name
            for argument in self.arguments
        )
        if not result:
            for argument in self.arguments:
                if argument.name == "self" and argument.reference:
                    raise RuntimeError(
                        f"The method has a reference to self, but the owner is {self.owner.name} "
                        f"while the argument data type is {argument.data_type(route='frontend')}. "
                        f"The method is {self.name}."
                    )

        return result

    def has_self_owned(self) -> bool:
        """Returns whether the method has a reference to self."""
        return any(
            argument.name == "self"
            and not argument.reference
            and argument.has_struct_data_type()
            and argument.raw_data_type().name == self.owner.name
            for argument in self.arguments
        )

    def __eq__(self, other: "MethodDefinition") -> bool:
        if not isinstance(other, MethodDefinition):
            return False

        return self.name == other.name

    def __hash__(self) -> int:
        return hash(self.name)

    def __repr__(self) -> str:
        return f"Method({self.name})"

    def write_header_to(self, document: "File"):
        """Writes the method header to the document.

        Parameters
        ----------
        document : File
            The document to write the method header to.
        """
        route = infer_route_from_document(document)

        document.write(f"    /// {self.summary}\n")

        if self.argument_descriptions:
            document.write("    ///\n")
            for argument in self.arguments:
                if argument.name in self.argument_descriptions:
                    document.write(
                        f"    /// * `{argument.name}` - {self.argument_descriptions[argument.name]}\n"
                    )

        async_name = ""
        if self.is_async:
            async_name = "async "

        generics = ""
        if len(self.generics) > 0:
            generics = "<" + ", ".join(self.generics) + "> "

        document.write(f"    {self.visibility} {async_name}fn {self.name}{generics}(\n")
        for argument in self.arguments:
            if argument.name == "self":
                if argument.reference:
                    document.write("        &self,\n")
                    continue
                else:
                    document.write("        self,\n")
                    continue
            try:
                document.write(
                    f"{argument.name}: {argument.format_data_type(route=route)},\n"
                )
            except NotImplementedError as e:
                raise NotImplementedError(
                    f"Could not write the header of the method {self.name}."
                ) from e
        document.write(")")
        if self.return_type is not None:
            if not self.return_type.has_struct_data_type() and any(
                self.return_type.raw_data_type() == generic or generic.startswith(f"{self.return_type.raw_data_type()}:")
                for generic in self.generics
            ):
                return_type = self.return_type.format_data_type(route=route, skip_remap=True)
            else:
                return_type = self.return_type.format_data_type(route=route)
            document.write(f" -> {return_type}")
        
        if len(self.where_clauses) > 0:
            document.write(" where\n")
            document.write(",\n".join(self.where_clauses))


class StructMetadata:
    """Class representing the metadata of a struct."""

    table_metadata: TableMetadata = None
    pg_indices: PGIndices = None

    @staticmethod
    def init_indices():
        """Initializes the search indices."""
        assert StructMetadata.table_metadata is not None
        assert StructMetadata.pg_indices is None
        StructMetadata.pg_indices = find_primary_search_indices(
            table_metadata=StructMetadata.table_metadata
        )

    @staticmethod
    def init_table_metadata():
        """Initializes the table metadata."""
        assert StructMetadata.table_metadata is None
        StructMetadata.table_metadata = find_foreign_keys()

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
        self._filter_variant: Optional[StructMetadata] = None
        self._backend_methods: List[MethodDefinition] = []
        self._webcommon_methods: List[MethodDefinition] = []
        self._child_variants: Dict[(str, AttributeMetadata), StructMetadata] = {}
        self._primary_search_index: Optional[PGIndex] = None
        self._first_order_derived_search_indices: List[DerivedPGIndex] = []
        self._second_order_derived_search_indices: List[DerivedPGIndex] = []
        self._foreign_keys: List[AttributeMetadata] = None
        self._primary_keys: List[AttributeMetadata] = None

    def set_primary_search_index(self, index: PGIndex):
        """Sets the primary search index of the struct."""
        assert isinstance(index, PGIndex)
        if self._primary_search_index is not None:
            raise ValueError(
                "The primary search index has already been set for the struct."
            )
        assert index.table_name == self.table_name, (
            "The table name of the index must be the same as the table name of the struct. "
            f"The provided table name is {index.table_name}, and the table name of the struct is {self.table_name}."
        )
        self._primary_search_index = index

    def into_rc(self) -> "StructMetadata":
        """Returns the struct as an Rc."""
        for attribute in self.attributes:
            if not attribute.implements_copy():
                attribute.rc = True

        return self

    def has_primary_search_index(self) -> bool:
        """Returns whether the struct has a primary search index."""
        return self._primary_search_index is not None

    def full_path(self, route: str) -> str:
        """Returns the full path of the struct.

        Parameters
        ----------
        route : str
            The route of the file.
            Can either be 'backend', 'web_common', or 'frontend'.
        """
        if (
            self.is_filter_variant()
            or self.is_new_variant()
            or self.is_update_variant()
        ):
            crate = "crate" if route == "web_common" else "web_common"
            if self.is_filter_variant():
                submodule = "filter_variants"
            elif self.is_new_variant():
                submodule = "new_variants"
            elif self.is_update_variant():
                submodule = "update_variants"
            return f"{crate}::database::{submodule}::{self.name}"

        crate = "crate" if route in ("backend", "web_common") else "web_common"
        if self.is_nested():
            return f"{crate}::database::nested_variants::{self.name}"
        return f"{crate}::database::flat_variants::{self.name}"

    def _retro_fix_zero_alias_number(self, table_name: str, alias_number: int):
        assert isinstance(alias_number, int)
        assert isinstance(table_name, str)
        # Only in the case where the alias number is 1 we need to go back
        # and fix the case where it needs to be set to zero.
        if alias_number != 1:
            return
        matches_found = 0
        for other_index in self.get_all_search_indices():
            if other_index.index_table_name() == table_name:
                assert not other_index.has_alias_number()
                other_index.set_alias_number(0)
                matches_found += 1
            if (
                isinstance(other_index, DerivedPGIndex)
                and other_index.is_second_order()
            ):
                if other_index.index.index_table_name() == table_name:
                    assert not other_index.has_inner_alias_number()
                    other_index.set_inner_alias_number(0)
                    matches_found += 1
        assert matches_found == 1, (
            "The alias number must be set to zero for exactly one index. "
            f"The provided table name is {table_name}."
        )

    def register_first_order_derived_search_index(self, index: DerivedPGIndex):
        """Registers a first-order derived search index."""
        assert isinstance(index, DerivedPGIndex)
        assert index.table_name == self.table_name, (
            "The table name of the index must be the same as the table name of the struct. "
            f"The provided table name is {index.table_name}, and the table name of the struct is {self.table_name}."
        )
        assert self.has_attribute_by_name(index.foreign_key_id), (
            "The foreign key ID of the index must be an attribute of the struct. "
            f"The provided foreign key ID is {index.foreign_key_id}."
        )
        assert index.is_first_order(), (
            "The index must be a first-order derived search index. "
            f"The provided index is {index}."
        )

        for other_index in self._first_order_derived_search_indices:
            assert other_index.foreign_key_id != index.foreign_key_id, (
                "The index searches over a table that is already covered by another "
                "first-order derived search index. The current struct is "
                f"{self.name}, and the provided index searches over the table {index.table_name}'s "
                f"foreign key {index.foreign_key_id}, which searches over the table {index.index.table_name}. "
                f"The other first-order derived search index searches over the table {other_index.index.table_name} "
                "using the SAME foreign key."
            )

        # We get the number of existing indices that search over the same table,
        # and if the value is greater than 0, we set the index the alias counter
        # of the index.
        alias_number = sum(
            other_index.index_table_name() == index.index_table_name()
            for other_index in self.get_all_search_indices()
        )

        if alias_number > 0:
            index.set_alias_number(alias_number)
        self._retro_fix_zero_alias_number(index.index_table_name(), alias_number)

        self._first_order_derived_search_indices.append(index)

    def get_first_order_derived_search_indices(self) -> List[DerivedPGIndex]:
        """Returns the first-order derived search indices."""
        return self._first_order_derived_search_indices

    def has_first_order_derived_search_indices(self) -> bool:
        """Returns whether the struct has first-order derived search indices."""
        return len(self._first_order_derived_search_indices) > 0

    def has_first_order_derived_search_index_for_foreign_key(
        self, foreign_key_id: AttributeMetadata
    ) -> bool:
        """Returns whether the struct has a first-order derived search index for the foreign key."""
        return any(
            index.foreign_key_id == foreign_key_id.name
            for index in self._first_order_derived_search_indices
        )

    def register_second_order_derived_search_index(self, index: DerivedPGIndex):
        """Registers a second-order derived search index."""
        assert isinstance(index, DerivedPGIndex)
        assert index.table_name == self.table_name, (
            "The table name of the index must be the same as the table name of the struct. "
            f"The provided table name is {index.table_name}, and the table name of the struct is {self.table_name}."
        )
        assert self.has_attribute_by_name(index.foreign_key_id), (
            "The foreign key ID of the index must be an attribute of the struct. "
            f"The provided foreign key ID is {index.foreign_key_id}."
        )
        assert index.is_second_order(), (
            "The index must be a second-order derived search index. "
            f"The provided index is {index}."
        )
        # We check that this index does not search over a table that can be
        # covered by a more efficient first-order derived search index.
        for first_order_index in self._first_order_derived_search_indices:
            assert first_order_index.foreign_key_id != index.foreign_key_id, (
                "The index searches over a table that can be covered by a more efficient "
                "first-order derived search index. The current struct is "
                f"{self.name}, and the provided index searches over the table {index.table_name}'s "
                f"foreign key {index.foreign_key_id}, which searches over the table {index.index.table_name}. "
                f"The first-order derived search index searches over the table {first_order_index.index.table_name} "
                "using the SAME foreign key."
            )

        # We check that no other second-order derived search index searches over the same foreign key.
        for other_index in self._second_order_derived_search_indices:
            assert (
                other_index.index != index.index
                or other_index.foreign_key_id != index.foreign_key_id
            ), (
                "The index searches over a table that is already covered by another "
                "second-order derived search index. The current struct is "
                f"{self.name}, and the provided index searches over the table {index.table_name}'s "
                f"foreign key {index.foreign_key_id}, which searches over the table {index.index.table_name}. "
                f"The other second-order derived search index searches over the table {other_index.index.table_name} "
                "using the SAME foreign key."
            )

        # We get the number of existing indices that search over the same table,
        # and if the value is greater than 0, we set the index the alias counter
        # of the index.
        alias_number = sum(
            other_index.index_table_name() == index.index_table_name()
            for other_index in self.get_all_search_indices()
        ) + sum(
            second_other_index.index.index_table_name() == index.index_table_name()
            for second_other_index in self._second_order_derived_search_indices
        )

        inner_alias_number = sum(
            other_index.index_table_name() == index.index.index_table_name()
            for other_index in self.get_all_search_indices()
        ) + sum(
            second_other_index.index.index_table_name()
            == index.index.index_table_name()
            for second_other_index in self._second_order_derived_search_indices
        )

        if alias_number > 0:
            index.set_alias_number(alias_number)
        self._retro_fix_zero_alias_number(index.index_table_name(), alias_number)
        if inner_alias_number > 0:
            index.set_inner_alias_number(inner_alias_number)
        self._retro_fix_zero_alias_number(
            index.index.index_table_name(), inner_alias_number
        )

        self._second_order_derived_search_indices.append(index)

    def get_second_order_derived_search_indices(self) -> List[DerivedPGIndex]:
        """Returns the second-order derived search indices."""
        return self._second_order_derived_search_indices

    def has_second_order_derived_search_indices(self) -> bool:
        """Returns whether the struct has second-order derived search indices."""
        return len(self._second_order_derived_search_indices) > 0

    def has_derived_search_indices(self) -> bool:
        """Returns whether the struct has derived search indices."""
        return (
            self.has_first_order_derived_search_indices()
            or self.has_second_order_derived_search_indices()
        )

    def format_diesel_search_join(self) -> str:
        """Returns the diesel search join for the struct."""
        assert (
            self.has_derived_search_indices()
        ), "The struct must have derived search indices to format the diesel search join."
        return "\n".join(
            index.format_diesel_search_join()
            for index in self.get_first_order_derived_search_indices()
            + self.get_second_order_derived_search_indices()
        )

    def get_all_search_indices(self) -> List[Union[PGIndex, DerivedPGIndex]]:
        """Returns all search indices of the struct."""
        return (
            ([self._primary_search_index] if self.has_primary_search_index() else [])
            + self.get_first_order_derived_search_indices()
            + self.get_second_order_derived_search_indices()
        )

    def number_of_search_indices(self) -> int:
        """Returns the number of search indices of the struct."""
        return len(self.get_all_search_indices())

    def format_diesel_search_aliases(self) -> str:
        """Returns the diesel search aliases for the struct.

        Implementative details
        ----------------------
        In settings such as the multi-index search queries, we execute several
        join operations. In some tables, there exist several foreign keys to the
        same table. In such cases, we need to alias the tables to avoid conflicts.
        """
        # First, we identify the foreign keys that are used in the search indices
        # that refer to the same table.
        table_alias_numbers: Dict[str, List[int]] = {}
        for index in self.get_all_search_indices():
            if isinstance(index, DerivedPGIndex):
                if index.has_alias_number():
                    table_name = index.index_table_name()
                    if table_name not in table_alias_numbers:
                        table_alias_numbers[table_name] = []
                    table_alias_numbers[table_name].append(index.get_alias_number())
                if index.has_inner_alias_number():
                    table_name = index.index.index_table_name()
                    if table_name not in table_alias_numbers:
                        table_alias_numbers[table_name] = []
                    table_alias_numbers[table_name].append(
                        index.get_inner_alias_number()
                    )

        if len(table_alias_numbers) == 0:
            return ""

        assert all(
            len(alias_numbers) > 1 for alias_numbers in table_alias_numbers.values()
        )

        # We then create the aliases for the tables associated to the foreign keys
        # that are used in the search indices that refer to the same table.
        aliases = ""
        for table_name, alias_numbers in table_alias_numbers.items():
            if len(alias_numbers) > 1:
                assert len(alias_numbers) == len(set(alias_numbers)), (
                    "The foreign keys must be unique. "
                    f"The provided foreign keys are {alias_numbers}."
                )

                # let (users1, users2) = diesel::alias!(schema::users as user1, schema::users as user2);
                new_table_names = [
                    f"{table_name}{alias_number}" for alias_number in alias_numbers
                ]
                schema_aliases = ", ".join(
                    [
                        f"crate::database::schema::{table_name} as {new_column_name}"
                        for new_column_name in new_table_names
                    ]
                )
                aliases += f"let ({', '.join(new_table_names)}) = diesel::alias!({schema_aliases});\n"

        return aliases

    def format_diesel_search_filter(self, query: str, similarity_method: str) -> str:
        """Returns the index in Diesel format.

        Parameters
        ----------
        query : str
            The query to search for.
        similarity_method : str
            The similarity method to use.
        """
        assert (
            self.has_derived_search_indices() or self.has_primary_search_index()
        ), "The struct must have derived search indices or a primary search index to format the diesel search filter."

        search_filter = ".filter(\n"
        for i, index in enumerate(self.get_all_search_indices()):
            if i > 0:
                search_filter += "    .or(\n"
            search_filter += index.format_diesel_search_filter(query, similarity_method)
            if i > 0:
                search_filter += "    )\n"
        search_filter += ")"

        return search_filter

    def format_diesel_search_score(self, query: str, similarity_method: str) -> str:
        """Returns the index in Diesel format.

        Parameters
        ----------
        query : str
            The query to search for.
        similarity_method : str
            The similarity method to use.
        """
        assert (
            self.has_derived_search_indices() or self.has_primary_search_index()
        ), "The struct must have derived search indices or a primary search index to format the diesel search order."

        search_order = ""
        for i, index in enumerate(self.get_all_search_indices()):
            if i > 0:
                search_order += "    +\n"
            search_order += index.format_distance_operator_diesel(
                query, similarity_method
            )

        return search_order

    def format_diesel_search_order(self, query: str, similarity_method: str) -> str:
        """Returns the index in Diesel format.

        Parameters
        ----------
        query : str
            The query to search for.
        similarity_method : str
            The similarity method to use.
        """
        assert (
            self.has_derived_search_indices() or self.has_primary_search_index()
        ), "The struct must have derived search indices or a primary search index to format the diesel search order."

        return f".order({self.format_diesel_search_score(query, similarity_method)})"

    def backend_methods(self) -> List[MethodDefinition]:
        """Returns the methods of the struct."""
        return self._backend_methods

    def webcommon_methods(self) -> List[MethodDefinition]:
        """Returns the methods of the struct."""
        return self._webcommon_methods

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

    def has_sampled_by(self) -> bool:
        """Returns whether the struct has a sampled by field."""
        return any(attribute.is_sampled_by() for attribute in self.attributes)

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

    def is_immutable(self) -> bool:
        """Returns whether the struct is immutable.

        Implementation details
        -----------------------
        A struct is immutable if it does not contain any field that is
        either the creator user ID or the updater user ID.
        """
        return not self.is_insertable() and not self.is_updatable()

    def add_decorator(self, decorator: str):
        """Adds a decorator to the struct.

        Parameters
        ----------
        decorator : str
            The decorator to add to the struct.
        """
        assert isinstance(decorator, str), (
            "The decorator must be a string. "
            f"The provided decorator is a {type(decorator)}."
        )
        assert len(decorator) > 0, "The decorator must not be empty."
        self._decorators.append(decorator)

    # def get_belonging_structs(self) -> List[Tuple["StructMetadata", AttributeMetadata]]:
    #     """Returns the structs that the struct belongs to."""

    #     # Diesel for the time being only supports one single foreign key
    #     # per table in the belongs_to attribute. For this reason, we skip
    #     # the keys associated to duplicated foreign keys.

    #     encountered_tables = set()
    #     belonging_structs = []

    #     for attribute in self.get_foreign_keys():
    #         foreign_key_table = (
    #             StructMetadata.table_metadata.get_foreign_key_table_name(
    #                 self.table_name, attribute.name
    #             )
    #         )
    #         if foreign_key_table in encountered_tables:
    #             continue

    #         encountered_tables.add(foreign_key_table)
    #         foreign_key_flat_variant = (
    #             StructMetadata.table_metadata.get_flat_variant(foreign_key_table)
    #         )

    #         assert foreign_key_flat_variant is not None, (
    #             f"The foreign key flat variant for the table {foreign_key_table} "
    #             "is not defined."
    #         )

    #         belonging_structs.append((foreign_key_flat_variant, attribute))

    #     return belonging_structs

    def write_to(
        self,
        file: "File",
        diesel: Optional[str] = None,
        derives_deny_list: Optional[List[str]] = None,
    ):
        """Writes the struct to the file."""
        route = infer_route_from_document(file)

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
            file.write(
                f"#[diesel(table_name = crate::database::schema::{self.table_name})]\n"
            )

        # For each of the attribute that is a foreign key, we add the
        # #[diesel(belongs_to({foreign struct name}, foreign_key = {attribute name}))]
        # decorator.
        # if diesel is not None:
        # Diesel for the time being only supports one single foreign key
        # per table in the belongs_to attribute. For this reason, we skip
        # the keys associated to duplicated foreign keys.

        # for belonging_struct, attribute in self.get_belonging_structs():
        #     file.write(
        #         f"#[diesel(belongs_to(crate::database::flat_variants::{belonging_struct.table_name}::{belonging_struct.name}, foreign_key = {attribute.name}))]\n"
        #     )

        if diesel is not None:
            file.write(
                f"#[diesel(primary_key({self.get_formatted_primary_keys(include_prefix=False, include_parenthesis=False)}))]\n"
            )

        for decorator in self._decorators:
            file.write(f"#[{decorator}]\n")
        file.write(f"pub struct {self.name} {{\n")
        for attribute in self.attributes:
            file.write(
                f"    pub {attribute.name}: {attribute.format_data_type(route=route)},\n"
            )
        file.write("}\n\n")

        # Next, we always implement the Send and Sync traits for the struct.
        file.write(f"unsafe impl Send for {self.name} {{}}\n")
        file.write(f"unsafe impl Sync for {self.name} {{}}\n")

    def is_filter_variant(self) -> bool:
        """Returns whether the struct is a filter variant."""
        return self.name.endswith("Filter") and all(
            attribute.optional for attribute in self.attributes
        )

    def is_new_variant(self) -> bool:
        """Returns whether the struct is a new variant."""
        return (
            self.is_insertable()
            and self._flat_variant is not None
            and self.name.startswith("New")
        )

    def is_update_variant(self) -> bool:
        """Returns whether the struct is an update variant."""
        return self.is_updatable() and (
            (not self.has_uuid_primary_key())
            and self._flat_variant is not None
            and self.name.startswith("Update")
            or (self.has_uuid_primary_key() and self.is_new_variant())
        )

    def set_flat_variant(self, struct: "StructMetadata"):
        """Sets the flat variant of the struct."""
        assert struct.table_name == self.table_name
        assert not struct.is_nested()
        self._flat_variant = struct

    def has_flat_variant(self) -> bool:
        """Returns whether the struct has a flat variant."""
        return self._flat_variant is not None

    def get_flat_variant(self) -> "StructMetadata":
        """Returns the flat variant of the struct."""
        if self._flat_variant is None:
            if self.is_nested():
                raise ValueError(
                    f"The struct {self.name} is nested but somehow the flat variant has not been set. "
                    f"The table associated with the struct is {self.table_name}."
                )

            raise ValueError(
                f"The flat variant of the struct {self.name} has not been set."
            )

        return self._flat_variant

    def set_richest_variant(self, struct: "StructMetadata"):
        """Sets the richest variant of the struct."""
        assert isinstance(struct, StructMetadata), (
            "The struct must be a StructMetadata. "
            f"The provided struct is a {type(struct)}."
        )
        assert struct.table_name == self.table_name
        assert self.has_foreign_keys(), (
            "The struct must have foreign keys to set the richest variant. "
            f"The provided struct is {struct.name}, and "
            f"the table associated with the struct is {struct.table_name}."
        )
        assert struct.is_nested(), (
            "The richest variant must be a nested struct. "
            f"The provided struct is {struct.name}, and "
            f"the table associated with the struct is {struct.table_name}. "
            f"You are currently trying to set the richest variant of the struct {self.name}."
        )
        if self._richest_variant is not None:
            assert self._richest_variant == struct, (
                f"The richest variant of the struct {self.name} has already been set. "
                f"The current richest variant is {self._richest_variant.name}, "
                f"and the new richest variant would be {struct.name}."
            )

        if self._flat_variant is not None:
            self._flat_variant.set_richest_variant(struct)
        else:
            self._richest_variant = struct

    def get_richest_variant(self) -> "StructMetadata":
        """Returns the richest variant of the struct."""
        if self._richest_variant is None:
            if self._flat_variant is not None:
                return self._flat_variant.get_richest_variant()

            if self.has_foreign_keys():
                raise ValueError(
                    f"The richest variant of the struct {self.name} has not been set. "
                    f"The table associated with the struct is {self.table_name}. "
                    f"This struct is associated to a table containing foreign keys, "
                    "and therefore, it must have a richest variant. The foreign keys are "
                    f"{self.get_foreign_keys()}."
                )

            assert not self.is_update_variant()
            assert not self.is_new_variant()

            return self

        return self._richest_variant

    def set_new_variant(self, struct: "StructMetadata"):
        """Sets the new variant of the struct."""
        assert struct.table_name == self.table_name
        assert struct.is_new_variant()
        self._new_variant = struct

    def get_new_variant(self) -> "StructMetadata":
        """Returns the new variant of the struct."""
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
        """Returns the attribute by name.

        Parameters
        ----------
        attribute_name : str
            The name of the attribute to retrieve.

        """
        assert isinstance(attribute_name, str), (
            "The attribute name must be a string. "
            f"The provided attribute name is a {type(attribute_name)}."
        )
        for attribute in self.attributes:
            if attribute.name == attribute_name:
                return attribute
        return None

    def capitalized_table_name(self) -> str:
        """Returns the capitalized table name."""
        return "".join(word.capitalize() for word in self.table_name.split("_"))

    def get_description_attribute(self) -> Optional[AttributeMetadata]:
        """Returns the description attribute if it exists."""
        if self._flat_variant is not None:
            return self._flat_variant.get_description_attribute()

        for attribute in self.attributes:
            if attribute.is_description():
                return attribute
        return None

    def get_color_attribute(self) -> Optional[AttributeMetadata]:
        """Returns the color attribute if it exists."""
        if self.name == "Color" and self.table_name == "colors":
            return self.get_attribute_by_name("name")

        for attribute in self.attributes:
            if attribute.is_color():
                return attribute
        return None

    def is_nested(self) -> bool:
        """Returns whether the struct is nested."""
        return any(attribute.has_struct_data_type() for attribute in self.attributes)

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

        if attribute_metadata.has_struct_data_type():
            if not (
                attribute_metadata.name == "inner"
                and attribute_metadata.raw_data_type() == self._flat_variant
            ):
                inner_attribute = self._flat_variant.get_attribute_by_name(
                    attribute_metadata.name
                )
                if inner_attribute is None:
                    inner_attribute = self._flat_variant.get_attribute_by_name(
                        f"{attribute_metadata.name}_id"
                    )
                if inner_attribute is None:
                    raise ValueError(
                        f"The attribute {attribute_metadata.name} is not in the struct {self.name}."
                    )
                foreign_struct: StructMetadata = attribute_metadata.raw_data_type()
                foreign_struct.register_child_variant(inner_attribute, self)

        self.attributes.append(attribute_metadata)
        attribute_metadata.set_owner(self)

    def add_backend_method(self, method: MethodDefinition) -> MethodDefinition:
        """Adds a method to the struct.

        Parameters
        ----------
        method : MethodDefinition
            The method to add to the struct.
        """
        if method in self._backend_methods:
            raise ValueError(
                f"The method {method.name} is already in the struct {self.name}."
            )

        self._backend_methods.append(method)
        method.set_owner(self)
        return method

    def add_webcommon_method(self, method: MethodDefinition) -> MethodDefinition:
        """Adds a method to the struct.

        Parameters
        ----------
        method : MethodDefinition
            The method to add to the struct.
        """
        if method in self._webcommon_methods:
            raise ValueError(
                f"The method {method.name} is already in the struct {self.name}."
            )

        self._webcommon_methods.append(method)
        method.set_owner(self)
        return method

    def get_backend_method_by_name(self, method_name: str) -> Optional[MethodDefinition]:
        """Returns the method by name.

        Parameters
        ----------
        method_name : str
            The name of the method to retrieve.
        """
        assert isinstance(method_name, str), (
            "The method name must be a string. "
            f"The provided method name is a {type(method_name)}."
        )
        for method in self._backend_methods:
            if method.name == method_name:
                return method
        return None

    def get_webcommon_method_by_name(
        self, method_name: str
    ) -> Optional[MethodDefinition]:
        """Returns the method by name.

        Parameters
        ----------
        method_name : str
            The name of the method to retrieve.
        """
        assert isinstance(method_name, str), (
            "The method name must be a string. "
            f"The provided method name is a {type(method_name)}."
        )
        for method in self._webcommon_methods:
            if method.name == method_name:
                return method
        return None

    def add_derive(self, derive: str):
        """Adds a derive to the struct.

        Parameters
        ----------
        derive : str
            The derive to add to the struct.
        """
        self._derives.append(derive)

    def contains_optional_fields(self) -> bool:
        """Returns whether the struct contains optional fields."""
        return any(attribute.optional for attribute in self.attributes)

    def contains_only_optional_fields(self) -> bool:
        """Returns whether the struct contains only optional fields."""
        return all(attribute.optional for attribute in self.attributes)

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

    def get_foreign_keys(self) -> List[AttributeMetadata]:
        """Returns the foreign keys of the struct.

        Implementation details
        -----------------------
        This method returns the foreign keys of the struct.
        """
        if self._flat_variant is not None:
            return self._flat_variant.get_foreign_keys()

        if self._foreign_keys is not None:
            return self._foreign_keys

        foreign_key_names = StructMetadata.table_metadata.get_foreign_keys(
            self.table_name
        )

        foreign_keys = []

        for foreign_key_name in foreign_key_names:
            attribute = self.get_attribute_by_name(foreign_key_name)
            if attribute is None:
                raise ValueError(
                    f"The attribute {foreign_key_name} is not in the struct {self.name} associated "
                    f"with the table {self.table_name}."
                )
            foreign_keys.append(attribute)

        self._foreign_keys = foreign_keys

        return foreign_keys

    def register_child_variant(
        self, attribute: AttributeMetadata, child_variant: "StructMetadata"
    ):
        """Registers a child variant for the struct.

        Parameters
        ----------
        attribute : AttributeMetadata
            The attribute that is a foreign key.
        child_variant : StructMetadata
            The child variant to register.
        """
        assert not attribute.has_struct_data_type()
        assert attribute in child_variant.get_foreign_keys()
        if self._flat_variant is not None:
            self._flat_variant.register_child_variant(attribute, child_variant)
            return

        key = (child_variant.table_name, attribute)

        if key in self._child_variants and self._child_variants[key] != child_variant:
            if child_variant.name.endswith("Builder"):
                return
            raise ValueError(
                f"The attribute {attribute.name} is already associated with a child variant in the struct {self.name}. "
                f"The child variant is {self._child_variants[key].name}. "
                f"The new child variant is {child_variant.name}."
            )

        self._child_variants[key] = child_variant

    def get_child_structs(self) -> Dict[AttributeMetadata, "StructMetadata"]:
        """Returns the child foreign keys of the struct.

        Implementation details
        -----------------------
        This method returns the child foreign keys of the struct.
        """
        if self._flat_variant is not None:
            return self._flat_variant.get_child_structs()

        return self._child_variants

    def get_manually_determined_foreign_keys(self) -> List[AttributeMetadata]:
        """Returns the manually determined foreign keys of the struct.

        Implementation details
        -----------------------
        This method returns the foreign keys of the struct that have been
        manually determined.
        """
        return [
            attribute
            for attribute in self.get_foreign_keys()
            if not attribute.is_automatically_determined_column()
        ]

    def has_foreign_keys(self) -> bool:
        """Returns whether the struct has foreign keys."""
        return len(self.get_foreign_keys()) > 0

    def get_primary_keys(self) -> List[AttributeMetadata]:
        """Returns the primary key of the struct.

        Implementation details
        -----------------------
        This method returns the primary key of the struct.
        """
        if self._flat_variant is not None:
            return self._flat_variant.get_primary_keys()

        if self._primary_keys is not None:
            return self._primary_keys

        primary_keys = StructMetadata.table_metadata.get_primary_key_names_and_types(
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

        self._primary_keys = primary_key_attributes

        return primary_key_attributes

    def has_uuid_primary_key(self) -> bool:
        """Returns whether the struct has a UUID primary key.

        Implementation details
        -----------------------
        This method returns whether the primary key of the struct is
        a UUID.
        """
        primary_keys = self.get_primary_keys()
        return (
            all(attribute.is_uuid() for attribute in primary_keys)
            and len(primary_keys) == 1
        )

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

        if self.can_implement_partial_ord() and "PartialOrd" not in self._derives:
            derives.append("PartialOrd")
        if "Debug" not in self._derives:
            derives.append("Debug")
        if self.can_implement_clone() and "Clone" not in self._derives:
            derives.append("Clone")
        if self.can_implement_copy() and "Copy" not in self._derives:
            derives.append("Copy")
        if self.can_implement_ord() and "Ord" not in self._derives:
            derives.append("Ord")
        if (
            self.can_implement_serialize()
            and "Serialize" not in self._derives
            and "serde::Serialize" not in self._derives
        ):
            derives.append("serde::Serialize")
        if (
            self.can_implement_deserialize()
            and "Deserialize" not in self._derives
            and "serde::Deserialize" not in self._derives
        ):
            derives.append("serde::Deserialize")
        if self.can_implement_default() and "Default" not in self._derives:
            derives.append("Default")

        diesel_derives = [
            "Identifiable",
            "QueryableByName",
            "Queryable",
        ]

        # if diesel is not None and self.has_foreign_keys():
        #     diesel_derives.append("Associations")

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
        """Returns whether the struct can implement the Clone trait."""
        return all(attribute.implements_clone() for attribute in self.attributes)

    def can_implement_copy(self) -> bool:
        """Returns whether the struct can implement the Copy trait."""
        return all(attribute.implements_copy() for attribute in self.attributes)

    def can_implement_ord(self) -> bool:
        """Returns whether the struct can implement the Ord trait."""
        return all(attribute.implements_ord() for attribute in self.attributes)

    def can_implement_partial_ord(self) -> bool:
        """Returns whether the struct can implement the PartialOrd trait."""
        return all(attribute.implements_partial_ord() for attribute in self.attributes)

    def can_implement_eq(self) -> bool:
        """Returns whether the struct can implement the Eq trait."""
        return all(attribute.implements_eq() for attribute in self.attributes)

    def can_implement_serialize(self) -> bool:
        """Returns whether the struct can implement the Serialize trait."""
        return all(attribute.implements_serialize() for attribute in self.attributes)

    def can_implement_deserialize(self) -> bool:
        """Returns whether the struct can implement the Deserialize trait."""
        return all(attribute.implements_deserialize() for attribute in self.attributes)

    def can_implement_default(self) -> bool:
        """Returns whether the struct can implement the Default trait."""
        return all(attribute.implements_default() for attribute in self.attributes)

    def has_attribute(self, attribute: AttributeMetadata) -> bool:
        """Returns whether the struct has the attribute."""
        return any(
            attribute == existing_attribute for existing_attribute in self.attributes
        )

    def has_attribute_by_name(self, attribute_name: str) -> bool:
        """Returns whether the struct has the attribute by name.

        Parameters
        ----------
        attribute_name : str
            The name of the attribute to check.
        """
        return any(attribute.name == attribute_name for attribute in self.attributes)

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

    def get_editability_determinant_columns(self) -> Optional[List[AttributeMetadata]]:
        """Returns the column that determines the editability of the struct.

        Implementation details
        -----------------------
        The column that determines the editability of the struct is the
        column that is a foreign key to the users table.
        """
        if self._flat_variant is not None:
            return self._flat_variant.get_editability_determinant_columns()

        if not self.has_foreign_keys():
            return None

        columns = []

        for attribute in self.get_foreign_keys():
            if attribute.is_automatically_determined_column():
                continue
            # We retrieve the table name associated to the foreign key.
            foreign_key_table = (
                StructMetadata.table_metadata.get_foreign_key_table_name(
                    self.table_name, attribute.name
                )
            )
            foreign_key_table_struct = StructMetadata.table_metadata.get_flat_variant(
                foreign_key_table
            )

            if foreign_key_table_struct.has_associated_roles():
                columns.append(attribute)
                continue

            if self.table_name == foreign_key_table:
                continue

            if foreign_key_table_struct.editability_always_depend_on_parent_column():
                columns.append(attribute)

        if len(columns) == 0:
            return None
        return columns

    def editability_may_depend_on_parent_columns(self) -> bool:
        """Returns whether the editability of the struct may depend on the parent column.

        Implementation details
        -----------------------
        The editability of the struct may depend on the parent column if the
        struct is a child variant and the parent variant is a new variant.
        """
        if self._flat_variant is not None:
            return self._flat_variant.editability_may_depend_on_parent_columns()

        columns = self.get_editability_determinant_columns()

        return columns is not None and all(column.optional for column in columns)

    def editability_always_depend_on_parent_column(self) -> bool:
        """Returns whether the editability of the struct depends on the parent column.

        Implementation details
        -----------------------
        The editability of the struct depends on the parent column if the
        struct is a child variant and the parent variant is not a new variant.
        """
        if self._flat_variant is not None:
            return self._flat_variant.editability_always_depend_on_parent_column()

        determinant_columns = self.get_editability_determinant_columns()

        if determinant_columns is None:
            return False

        return any(not attribute.optional for attribute in determinant_columns)

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

        formatted_keys = ", ".join(
            attribute.format_data_type(route="frontend") for attribute in keys
        )

        if len(keys) > 1:
            return f"( {formatted_keys} )"
        return formatted_keys

    def has_only_foreign_keys(self) -> bool:
        """Returns whether the struct has only foreign keys."""
        foreign_keys = self.get_foreign_keys()

        return len(self.attributes) == len(foreign_keys)

    def has_manually_determined_foreign_keys(self) -> bool:
        """Returns whether the struct has manually determined foreign keys."""
        return len(self.get_manually_determined_foreign_keys()) > 0

    def has_associated_roles(self) -> bool:
        """Returns whether there is a roles table associated with the struct."""
        return StructMetadata.table_metadata.has_associated_roles(self.table_name)

    def has_public_column(self) -> bool:
        """Returns whether the struct has a public column."""
        return self.get_public_column() is not None

    def has_images(self) -> bool:
        """Returns whether the struct has JPEGs."""
        return any(attribute.is_jpeg() for attribute in self.attributes)

    def get_public_column(self) -> Optional[AttributeMetadata]:
        """Returns the public column of the struct."""
        if self._flat_variant is not None:
            return self._flat_variant.get_public_column()

        for attribute in self.attributes:
            if (
                attribute.data_type(route="frontend") == "bool"
                and attribute.name == "public"
            ):
                return attribute

        return None

    def may_be_hidden(self) -> bool:
        """Returns whether the struct may be hidden.

        Implementation details
        -----------------------
        A struct may be hidden if it has a public column or if it has
        its editability fully defined by the parent column, and the parent
        table associated with the struct may be hidden.
        """
        if self.table_name in ("user_emails",):
            return True
        if is_role_invitation_table(self.table_name) or is_role_request_table(
            self.table_name
        ):
            return True
        if self.has_public_column():
            return True
        if self.editability_always_depend_on_parent_column():
            editability_parent_columns = self.get_editability_determinant_columns()
            assert editability_parent_columns is not None
            return any(
                self.get_foreign_key_flat_variant(
                    editability_parent_column
                ).may_be_hidden()
                for editability_parent_column in editability_parent_columns
            )

        return False

    def has_attribute_that_may_be_hidden(self) -> bool:
        """Returns whether the struct has an attribute that may be hidden.

        Implementation details
        -----------------------
        A struct has an attribute that may be hidden if the attribute
        is a foreign key and the associated table may be hidden.
        """
        return any(
            attribute.has_struct_data_type()
            and attribute.raw_data_type().may_be_hidden()
            for attribute in self.attributes
            if not attribute.is_inner()
        )

    def get_inner_attribute(self) -> Optional[AttributeMetadata]:
        """Returns the inner attribute of the struct."""
        assert self.is_nested(), (
            "Non-nested structs do not have inner attributes. "
            f"The struct {self.name} associated with the table {self.table_name} is not nested."
        )

        for attribute in self.attributes:
            if attribute.is_inner():
                return attribute

        return None

    def get_attribute_path(self, needle: AttributeMetadata) -> str:
        """Returns the attribute path.

        Parameters
        ----------
        needle : AttributeMetadata
            The attribute to get the path from.
        """
        for attribute in self.attributes:
            try:
                return attribute.get_attribute_path(needle)
            except ValueError:
                continue

        raise ValueError(
            f"The attribute {needle.name} is not in the struct {self.name}."
        )

    def get_foreign_key_flat_variant(
        self, foreign_key: AttributeMetadata
    ) -> "StructMetadata":
        """Returns the flat variant associated with the foreign key.

        Parameters
        ----------
        foreign_key : AttributeMetadata
            The foreign key to get the flat variant from.
        """
        foreign_key_table = StructMetadata.table_metadata.get_foreign_key_table_name(
            self.table_name, foreign_key.name
        )
        return StructMetadata.table_metadata.get_flat_variant(foreign_key_table)

    def set_filter_variant(self, struct: "StructMetadata"):
        """Sets the filter variant of the struct.

        Parameters
        ----------
        struct : StructMetadata
            The filter variant to set.

        """
        assert struct.table_name == self.table_name
        assert not struct.is_nested()
        assert not self.is_nested()
        assert not self.has_filter_variant()
        assert self.has_foreign_keys()
        assert all(atttribute.optional for atttribute in struct.attributes)
        assert all(
            self.has_attribute(filter_attribute)
            for filter_attribute in struct.attributes
        )

        self._filter_variant = struct

    def has_filter_variant(self) -> bool:
        """Returns whether the struct has a filter variant."""
        if self._flat_variant is not None:
            return self._flat_variant.has_filter_variant()

        if self.is_filter_variant():
            assert self._filter_variant is None

        return self._filter_variant is not None

    def get_filter_variant(self) -> "StructMetadata":
        """Returns the filter variant of the struct."""
        if self._flat_variant is not None:
            return self._flat_variant.get_filter_variant()

        if not self.has_foreign_keys():
            raise ValueError(
                f"The struct {self.name} associated with the table {self.table_name} "
                "does not have any foreign keys, and as such, it cannot have a filter variant."
            )

        if self._filter_variant is None:
            raise ValueError(
                "The filter variant has not been set for the struct "
                f"{self.name} associated with the table {self.table_name}."
            )

        return self._filter_variant

    def is_searchable(self) -> bool:
        """Returns whether the struct is searchable.

        Implementation details
        -----------------------
        A struct is searchable if it has a trigram index on the table.
        """
        if self._flat_variant is not None:
            return self._flat_variant.is_searchable()
        return self.has_primary_search_index() or self.has_derived_search_indices()

    def has_searchable_attributes(self) -> bool:
        """Returns whether the struct has searchable attributes.

        Implementation details
        -----------------------
        A struct has searchable attributes if it has a trigram index on the table.
        """
        return any(
            attribute.implements_as_ref_str() or attribute.has_struct_data_type() and attribute.raw_data_type().is_searchable()
            for attribute in self.attributes
        )

    def get_can_update_function_name(self) -> str:
        """Returns the name of the can_update function."""
        return f"can_update_{self.table_name}"

    def has_can_update_function(self) -> bool:
        """Returns whether the table has a can_update function."""
        return self.table_metadata.has_postgres_function(
            self.get_can_update_function_name()
        )

    def get_can_view_function_name(self) -> str:
        """Returns the name of the can_view function."""
        return f"can_view_{self.table_name}"

    def get_can_admin_function_name(self) -> str:
        """Returns the name of the can_admin function."""
        return f"can_admin_{self.table_name}"

    def get_can_update_trigger_name(self) -> str:
        """Returns the name of the can_update trigger."""
        return f"can_update_{self.table_name}"

    def has_can_update_trigger(self) -> bool:
        """Returns whether the table has a can_update trigger."""
        return self.table_metadata.has_trigger_by_name(
            self.table_name, self.get_can_update_trigger_name()
        )

    def has_can_view_function(self) -> bool:
        """Returns whether the table has a can_view function."""
        return self.table_metadata.has_postgres_function(
            self.get_can_view_function_name()
        )

    def has_can_admin_function(self) -> bool:
        """Returns whether the table has a can_admin function."""
        return self.table_metadata.has_postgres_function(
            self.get_can_admin_function_name()
        )

    def has_created_at(self) -> bool:
        """Returns whether the struct has a created at attribute."""
        if self._flat_variant is not None:
            return self._flat_variant.has_created_at()
        return any(attribute.is_creation_timestamp() for attribute in self.attributes)

    def has_updated_at(self) -> bool:
        """Returns whether the struct has an updated at attribute."""
        if self._flat_variant is not None:
            return self._flat_variant.has_updated_at()
        return any(attribute.is_update_timestamp() for attribute in self.attributes)

    def get_unique_constraints(self) -> List[List[AttributeMetadata]]:
        """Returns the unique constraints of the struct.

        Implementation details
        -----------------------
        This method returns the unique constraints of the struct.
        """
        if self._flat_variant is not None:
            return self._flat_variant.get_unique_constraints()

        unique_constraints: List[Tuple[str]] = (
            StructMetadata.table_metadata.get_unique_constraint_columns(self.table_name)
        )

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
