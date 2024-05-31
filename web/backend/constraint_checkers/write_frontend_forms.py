"""This module contains the functions to write the frontend forms for the builder structs."""

import os
import re
from time import sleep
from typing import List

from constraint_checkers.indices import PGIndex
from constraint_checkers.struct_metadata import AttributeMetadata, StructMetadata
from insert_migration import insert_migration
from tqdm.auto import tqdm
from userinput import userinput

TEXTUAL_DATA_TYPES = ["String"]

INPUT_TYPE_MAP = {
    "String": "Text",
    "f32": "Numeric",
    "f64": "Numeric",
    "i8": "Numeric",
    "i16": "Numeric",
    "i32": "Numeric",
    "i64": "Numeric",
    "i128": "Numeric",
    "u8": "Numeric",
    "u16": "Numeric",
    "u32": "Numeric",
    "u64": "Numeric",
    "u128": "Numeric",
}


def is_error_vector(attribute: AttributeMetadata) -> bool:
    """Checks whether the attribute is an error vector.

    Parameters
    ----------
    attribute : AttributeMetadata
        The attribute to check.

    Returns
    -------
    bool
        True if the attribute is an error vector, False otherwise.
    """
    assert isinstance(attribute, AttributeMetadata)
    return (
        attribute.name.startswith("errors_")
        and attribute.data_type() == "Vec<ApiError>"
    )


def is_builder_reserved_attribute(attribute: AttributeMetadata) -> bool:
    """Checks whether the attribute is a reserved attribute of the builder struct.

    Parameters
    ----------
    attribute : AttributeMetadata
        The attribute to check.

    Returns
    -------
    bool
        True if the attribute is a reserved attribute of the builder struct, False otherwise.
    """
    assert isinstance(attribute, AttributeMetadata)
    return is_error_vector(attribute) or attribute.name in ("form_updated_at",)


def write_frontend_builder_default_implementation(
    builder: StructMetadata,
    document: "io.TextIO",
):
    """Writes the implementation of the Default trait for the builder struct.

    Parameters
    ----------
    builder : StructMetadata
        The builder struct for which to write the implementation.
    table_metadata : TableMetadata
        The metadata of the table associated with the builder struct.
    document : io.TextIO
        The document to write to.

    Implementation details
    ----------------------
    This method extracts from the database the default values set for the
    attributes of the builder struct and writes the implementation of the
    Default trait for the builder struct. If an attribute does not have a
    default value, the attribute is set to None.
    """

    flat_variant = builder.get_flat_variant()

    document.write(
        f"impl Default for {builder.name} {{\n"
        "    fn default() -> Self {\n"
        "        Self {\n"
    )

    primary_keys = flat_variant.get_primary_keys()

    for attribute in builder.attributes:
        # If this is an error vector, we set it to
        # an empty vector.

        # If this is the primary key, we set it to None.
        if attribute in primary_keys:
            document.write(f"            {attribute.name}: None,\n")
            continue

        # If the current attribute does not exist in the flat struct,
        # we set it to None.
        if flat_variant.get_attribute_by_name(attribute.name) is None:
            # Otherwise, we set it to the default value of the data type.
            document.write(f"            {attribute.name}: Default::default(),\n")
            continue

        default_value = builder.table_metadata.get_default_column_value(
            builder.table_name, attribute.name
        )

        if default_value is not None:
            default_value = default_value.replace("'", '"')

            if default_value.endswith("::character varying"):
                default_value = default_value.replace(
                    "::character varying", ".to_string()"
                )

            document.write(f"            {attribute.name}: Some({default_value}),\n")
            continue

        # If the current value is an option, we set it to None.
        if attribute.optional:
            document.write(f"            {attribute.name}: None,\n")
            continue

    document.write("        }\n    }\n}\n\n")


def write_frontend_builder_action_enumeration(
    builder: StructMetadata,
    document: "io.TextIO",
):
    """Writes the enumeration of the builder actions for the builder struct.

    Parameters
    ----------
    builder : StructMetadata
        The builder struct for which to write the enumeration.
    table_metadata : TableMetadata
        The metadata of the table associated with the builder struct.
    document : io.TextIO
        The document to write to.

    Implementation details
    ----------------------
    This method writes the enumeration of the builder actions for the builder struct.
    """

    flat_variant = builder.get_flat_variant()
    richest_variant = builder.get_richest_variant()
    primary_keys = flat_variant.get_primary_keys()

    action_enum_name = f"{flat_variant.name}Actions"

    derives = ", ".join(
        [derive for derive in builder.derives() if derive not in ("Store", "Default")]
    )
    document.write(f"#[derive({derives})]\npub(super) enum {action_enum_name} {{\n")

    for attribute in builder.attributes:
        if attribute in primary_keys:
            continue

        # We do not want to include the errors attribute in the builder actions.
        if is_builder_reserved_attribute(attribute):
            continue

        if (
            attribute.data_type() in INPUT_TYPE_MAP
            or attribute.data_type() == "NaiveDateTime"
        ):
            document.write(
                f"    Set{attribute.capitalized_normalized_name()}(Option<String>),\n"
            )
        else:
            document.write(
                f"    Set{attribute.capitalized_normalized_name()}({attribute.format_data_type()}),\n"
            )

    document.write("}\n\n")

    # We implement the FromOperation trait for the action enum. This trait
    # is used to convert a named get operation into an action to apply to
    # the current builder object. The trait only implements a single method,
    # namely from_operation, which receives a generic S that implements AsRef<str>
    # and a vector of bytes which contains the struct corresponding the action
    # that needs to be built. The method panics if none of the expected operation
    # names are supported. The operation name are equal to the attribute names
    # of types that are equal to the richest variant associated to the builder,
    # i.e. the variants for which we must run additional requests to the backend.
    # If the builder in question does not contain any attributes of the richest
    # variant, the method solely contains an unreachable!() macro.

    document.write(f"impl FromOperation for {action_enum_name} {{\n")

    if not flat_variant.has_manually_determined_foreign_keys():
        document.write(
            "    fn from_operation<S: AsRef<str>>(_operation: S, _row: Vec<u8>) -> Self {\n"
            f'        unreachable!("No operations are expected to be needed for the builder {builder.name}.")\n'
        )
    else:
        document.write(
            "    fn from_operation<S: AsRef<str>>(operation: S, row: Vec<u8>) -> Self {\n"
            "        match operation.as_ref() {\n"
        )

        for foreign_key in flat_variant.get_manually_determined_foreign_keys():
            document.write(
                f'            "{foreign_key.normalized_name()}" => {action_enum_name}::Set{foreign_key.capitalized_normalized_name()}(Some(bincode::deserialize(&row).unwrap())),\n'
            )

        document.write(
            "            operation_name => unreachable!(\"The operation name '{}' is not supported.\", operation_name),\n"
            "        }\n"
        )

    document.write("    }\n}\n\n")

    document.write(
        f"impl Reducer<{builder.name}> for {action_enum_name} {{\n"
        f"    fn apply(self, mut state: std::rc::Rc<{builder.name}>) -> std::rc::Rc<{builder.name}> {{\n"
        "        let state_mut = Rc::make_mut(&mut state);\n"
        "        match self {\n"
    )

    largest_type_variants = {
        "i8": "i128",
        "i16": "i128",
        "i32": "i128",
        "i64": "i128",
        "i128": "i128",
        "u8": "u128",
        "u16": "u128",
        "u32": "u128",
        "u64": "u128",
        "u128": "u128",
        "f32": "f64",
        "f64": "f64",
    }

    for attribute in builder.attributes:
        if attribute in primary_keys:
            continue

        # We do not want to include the errors attribute in the builder actions.
        if is_builder_reserved_attribute(attribute):
            continue

        struct_attribute = richest_variant.get_attribute_by_name(attribute.name)

        if struct_attribute is None:
            struct_attribute = flat_variant.get_attribute_by_name(attribute.name)

        assert (
            struct_attribute is not None
        ), f"Attribute {attribute.name} not found in the struct {flat_variant.name}."

        document.write(
            f"            {action_enum_name}::Set{attribute.capitalized_normalized_name()}({attribute.name}) => '{attribute.name}: {{\n"
        )

        # First we clear out the existing errors associated with the attribute.
        document.write(f"                state_mut.errors_{attribute.name}.clear();\n")

        # If the attribute is solely optional in the builder, we need to check
        # whether it is currently populated. If it is not, we return an error.
        if not struct_attribute.optional and attribute.optional:
            document.write(
                f"        if {attribute.name}.is_none() {{\n"
                f"            state_mut.errors_{attribute.name}.push(ApiError::BadRequest(vec![\n"
                f'                "The {attribute.human_readable_name()} field is required.".to_string()\n'
                "             ]));\n"
                f"            state_mut.{attribute.name} = None;\n"
                f"             break '{attribute.name};\n"
                f"        }}\n"
            )

        # If the provided value is a StructMetadata, and it is of the same type as the richest variant,
        # we check that all of the primary keys are distinct from the primary keys of the richest variant.
        # We do not want to find an entry equal to the one we are trying to insert.
        if attribute.data_type() == richest_variant.name:
            document.write(
                f"                match {attribute.name}.as_ref() {{\n"
                f"                    Some({attribute.name}) => {{\n"
            )
            is_first = True
            for primary_key in primary_keys:
                prefix = "if" if is_first else "&&"
                is_first = False
                document.write(
                    f"                            {prefix} state_mut.{primary_key.name}.map_or(false, |{primary_key.name}| {primary_key.name} == {attribute.get_attribute_path(primary_key)})\n"
                )
            document.write(
                "                        {\n"
                f"                            state_mut.errors_{attribute.name}.push(ApiError::BadRequest(vec![\n"
                f'                                "The {attribute.human_readable_name()} field must be distinct from the current value.".to_string()\n'
                "                             ]));\n"
                f"                            break '{attribute.name};\n"
                "                        }\n"
                "                    }\n"
                "                    None => (),\n"
                "                }\n"
            )

        # If the provided value is a String, we need to check whether it is empty.
        # If it is, we add an error to the errors vector.
        if attribute.data_type() == "String":
            document.write(
                f"                if let Some(value) = {attribute.name}.as_ref() {{\n"
                "                    if value.is_empty() {\n"
                f"                        state_mut.errors_{attribute.name}.push(ApiError::BadRequest(vec![\n"
                f'                            "The {attribute.human_readable_name()} field cannot be left empty.".to_string()\n'
                "                        ]));\n"
                f"                         state_mut.{attribute.name} = None;\n"
                f"                          break '{attribute.name};\n"
                "                    }\n"
                "                }\n"
            )

        if attribute.data_type() == "NaiveDateTime":
            # We convert the dates provided from the date picker to the NaiveDateTime format.
            # The dates from a datetime-local input are in the format "YYYY-MM-DDTHH:MM".
            document.write(
                f"                match {attribute.name} {{\n"
                f'                    Some(value) => match NaiveDateTime::parse_from_str(&value, "%Y-%m-%dT%H:%M") {{\n'
                f"                        Ok({attribute.name}) => state_mut.{attribute.name} = Some({attribute.name}),\n"
                f"                        Err(_) => state_mut.errors_{attribute.name}.push(ApiError::BadRequest(vec![\n"
                f'                            "The {attribute.name} field must be a valid date and time.".to_string()\n'
                "                        ])),\n"
                "                    },\n"
                f"                    None => state_mut.{attribute.name} = None,\n"
                "                }\n"
            )
        elif (
            attribute.data_type() in INPUT_TYPE_MAP
            and attribute.data_type() != "String"
        ):
            # We try to convert the values to the largest possible type.
            # If the conversion fails, we add an error to the errors vector. Subsequently, we
            # verify whether the value is within the expected range. If it is not, we add an error to the
            # errors vector. Finally, we set the attribute to the provided value.
            largest_type_variant = largest_type_variants[attribute.data_type()]

            document.write(
                f"                state_mut.form_updated_at = chrono::Utc::now().naive_utc();\n"
                f"                match {attribute.name} {{\n"
                f"                    Some(value) => match value.parse::<{largest_type_variant}>() {{\n"
                "                        Ok(value) => {\n"
            )

            # In the case of floats, we also check for NaN and Infinity.
            if attribute.data_type() in ("f32", "f64"):
                document.write(
                    f"                            if value.is_nan() || value.is_infinite() {{\n"
                    f"                                state_mut.errors_{attribute.name}.push(ApiError::BadRequest(vec![\n"
                    f'                                    "The {attribute.name} field must be a valid {attribute.data_type()}.".to_string()\n'
                    "                                ]));\n"
                    "                            } else "
                )

            document.write(
                f"                            if value < {attribute.data_type()}::MIN as {largest_type_variant} || value > {attribute.data_type()}::MAX as {largest_type_variant} {{\n"
                f"                                state_mut.errors_{attribute.name}.push(ApiError::BadRequest(vec![\n"
                f"                                    format!("
                f'                                            "The {attribute.name} field must be between {{}} and {{}}.",\n'
                f"                                            {attribute.data_type()}::MIN,\n"
                f"                                            {attribute.data_type()}::MAX\n"
                "                                    )\n"
                "                                ]));\n"
                "                            } else {\n"
                f"                                state_mut.{attribute.name} = Some(value as {attribute.data_type()});\n"
                "                            }\n"
                "                        }\n"
                "                        Err(_) => {\n"
                f"                            state_mut.errors_{attribute.name}.push(ApiError::BadRequest(vec![\n"
                f'                                "The {attribute.name} field must be a valid {attribute.data_type()}.".to_string()\n'
                "                            ]));\n"
                "                        }\n"
                "                    },\n"
                f"                    None => state_mut.{attribute.name} = None,\n"
                "                }\n"
            )
        else:
            document.write(
                f"                state_mut.{attribute.name} = {attribute.name};\n"
            )

        document.write(
            f"                // To avoid having a codesmell relative to the cases where we are not\n"
            f"                // yet handling more corner cases, we always use the break here.\n"
            f"                break '{attribute.name};\n"
            "            }\n"
        )

    document.write("        }\n        state\n    }\n}\n")


def write_frontend_form_builder_implementation(
    builder: StructMetadata,
    document: "io.TextIO",
):
    """Writes the implementation of the FormBuilder trait for the builder struct.

    Parameters
    ----------
    builder : StructMetadata
        The builder struct for which to write the implementation.
    table_metadata : TableMetadata
        The metadata of the table associated with the builder struct.
    document : io.TextIO
        The document to write to.

    Implementation details
    ----------------------
    This method implements the FormBuilder trait for the provided builder struct.
    """

    flat_variant = builder.get_flat_variant()
    richest_variant = builder.get_richest_variant()

    variants = []

    flat_variant = builder.get_flat_variant()

    if flat_variant.is_insertable() and flat_variant.table_name != "users":
        variants.append(builder.get_new_variant())

    # If the new variant is not also used as an update
    # variant, we add it to the list of variants.
    if flat_variant.is_updatable():
        update_variant = builder.get_update_variant()
        if not update_variant.is_new_variant():
            variants.append(update_variant)

    assert len(variants) > 0

    primary_keys = flat_variant.get_primary_keys()

    document.write(
        f"impl FormBuilder for {builder.name} {{\n"
        f"    type Actions = {flat_variant.name}Actions;\n\n"
        f"    type RichVariant = {richest_variant.name};\n\n"
        "    fn has_errors(&self) -> bool {\n"
    )

    error_vectors = [
        attribute for attribute in builder.attributes if is_error_vector(attribute)
    ]

    assert (
        len(error_vectors) > 0
    ), f"The builder {builder.name} does not contain any error vectors. "

    document.write(
        " || ".join(
            [f"!self.{error_vector.name}.is_empty()" for error_vector in error_vectors]
        )
        + "\n"
    )

    document.write("    }\n\n")

    # We implement the update method, which operated on a mutable reference of the builder
    # and receives a Rich Variant type, the type associated with the FormBuilder trait,
    # which is the richest variant of the flat struct. The method updates the builder with the
    # values of the rich variant, which do not require validation as they are already validated,
    # being the result of a successful query to the database. This also means that the error
    # vectors are cleared when the method is called.
    #
    # Some of the structs composing the builder may appear Nested in the builder while flat
    # in the rich variant that has been provided when they have the same type of the rich variant.
    # For instance, a NestedProject containts a Project, which is flat in the rich variant so as
    # to avoid an infinitely-sized struct. In this case, we need to run some additional requests
    # to the backend to obtain the nested versions of the object that are not present in the rich
    # variant. These additional request are named get requests.
    document.write(
        "    fn update(dispatcher: &Dispatch<Self>, richest_variant: Self::RichVariant) -> Vec<ComponentMessage> {\n"
    )

    named_requests: List[str] = []
    named_option_requests: List[str] = []

    for attribute in builder.attributes:
        if is_builder_reserved_attribute(attribute):
            continue

        if attribute in primary_keys:
            if attribute in builder.get_foreign_keys():
                continue
            if richest_variant.is_nested():
                document.write(
                    f"        dispatcher.reduce_mut(|state| {{state.{attribute.name} = Some(richest_variant.inner.{attribute.name});}});\n"
                )
            else:
                document.write(
                    f"        dispatcher.reduce_mut(|state| {{state.{attribute.name} = Some(richest_variant.{attribute.name});}});\n"
                )
            continue

        # We need to check whether is will be necessary to make a request to the backend
        # to obtain the nested version of the attribute. The request name is always equal
        # to the name of the attribute.
        if attribute.data_type() == richest_variant.name:
            assert richest_variant.is_nested()

            flat_struct = attribute.raw_data_type().get_flat_variant()
            flat_attribute = flat_variant.get_attribute_by_name(attribute.name)

            if flat_attribute is None:
                flat_attribute = flat_variant.get_attribute_by_name(
                    f"{attribute.name}_id"
                )

            if flat_attribute is None:
                raise RuntimeError(
                    f"Attribute {attribute.name} not found in the flat struct {flat_variant.name}."
                )

            if flat_attribute.optional:
                named_option_requests.append(
                    (
                        f"if let Some({flat_attribute.name}) = richest_variant.inner.{flat_attribute.name} {{\n"
                        f'    named_requests.push(ComponentMessage::get_named::<&str, {flat_struct.name}>("{attribute.name}", {flat_attribute.name}.into()));\n'
                        " } else {\n"
                        f"    dispatcher.apply({flat_variant.name}Actions::Set{attribute.capitalized_normalized_name()}(None));\n"
                        " }\n"
                    )
                )
            else:
                named_requests.append(
                    f'ComponentMessage::get_named::<&str, {flat_struct.name}>("{attribute.name}", richest_variant.inner.{flat_attribute.name}.into())'
                )
            continue

        struct_attribute = richest_variant.get_attribute_by_name(attribute.name)

        if struct_attribute is not None:
            if struct_attribute.optional:
                document.write(
                    f"        dispatcher.apply({flat_variant.name}Actions::Set{attribute.capitalized_normalized_name()}(richest_variant.{attribute.name}));\n"
                )
            else:
                document.write(
                    f"        dispatcher.apply({flat_variant.name}Actions::Set{attribute.capitalized_normalized_name()}(Some(richest_variant.{attribute.name})));\n"
                )
        else:
            struct_attribute = flat_variant.get_attribute_by_name(attribute.name)

            if struct_attribute is not None:
                if (
                    attribute.data_type() in INPUT_TYPE_MAP
                    or attribute.data_type() == "NaiveDateTime"
                ):
                    if struct_attribute.optional:
                        document.write(
                            f"    dispatcher.apply({flat_variant.name}Actions::Set{attribute.capitalized_normalized_name()}(richest_variant.inner.{attribute.name}.map(|{attribute.name}| {attribute.name}.to_string())));\n"
                        )
                    else:
                        document.write(
                            f"    dispatcher.apply({flat_variant.name}Actions::Set{attribute.capitalized_normalized_name()}(Some(richest_variant.inner.{attribute.name}.to_string())));\n"
                        )
                elif struct_attribute.optional:
                    document.write(
                        f"        dispatcher.apply({flat_variant.name}Actions::Set{attribute.capitalized_normalized_name()}(richest_variant.inner.{attribute.name}));\n"
                    )
                else:
                    document.write(
                        f"        dispatcher.apply({flat_variant.name}Actions::Set{attribute.capitalized_normalized_name()}(Some(richest_variant.inner.{attribute.name})));"
                    )
            else:
                raise RuntimeError(
                    f"Attribute {attribute.name} present in builder struct {builder.name} "
                    f"not found in neither the rich variant {richest_variant.name} nor the flat variant {flat_variant.name}."
                )

    if len(named_requests) == 0 and len(named_option_requests) == 0:
        document.write("        vec![]\n")
    elif len(named_requests) > 0 and len(named_option_requests) == 0:
        document.write("        vec![\n")

        for named_request in named_requests:
            document.write(f"            {named_request},\n")

        document.write("        ]\n")
    else:
        document.write("        let mut named_requests = Vec::new();\n")

        for named_request in named_requests:
            document.write(f"        named_requests.push({named_request});\n")

        for named_option_request in named_option_requests:
            document.write(f"        {named_option_request}")

        document.write("        named_requests\n")
    document.write("    }\n\n")

    # We implement the can submit method, which checks whether the form
    # contains errors as specified by the has_errors method, plus checks
    # that all non-optional fields are populated.
    document.write("    fn can_submit(&self) -> bool {\n        !self.has_errors()\n")

    for attribute in builder.attributes:
        if is_builder_reserved_attribute(attribute):
            continue

        if attribute in primary_keys:
            continue

        struct_attribute = flat_variant.get_attribute_by_name(attribute.name)

        if struct_attribute is None:
            # We check whether the _id variant of the attribute is present.
            struct_attribute = flat_variant.get_attribute_by_name(
                f"{attribute.name}_id"
            )

        if struct_attribute is None:
            raise RuntimeError(
                f"Attribute {attribute.name} not found in the build target struct {flat_variant.name}."
            )

        if not struct_attribute.optional and attribute.optional:
            document.write(f"        && self.{attribute.name}.is_some()\n")

    document.write("    }\n\n")

    document.write("}\n\n")

    for variant in variants:

        # We implement the From method to convert the builder to the target struct.
        document.write(
            f"impl From<{builder.name}> for {variant.name} {{\n"
            f"    fn from(builder: {builder.name}) -> Self {{\n"
            "        Self {\n"
        )

        if variant.is_new_variant() and variant.has_uuid_primary_key():
            primary_key = primary_keys[0]
            document.write(
                f"            {primary_key.name}: builder.{primary_key.name}.unwrap_or_else(Uuid::new_v4),\n"
            )
        elif variant.is_new_variant() and len(primary_keys) > 1:
            for primary_key in primary_keys:
                richest_variant_attribute = richest_variant.get_attribute_by_name(
                    primary_key.normalized_name()
                )
                assert (
                    richest_variant_attribute is not None
                ), f"Attribute {primary_key.normalized_name()} not found in the variant {richest_variant.name}."
                richest_variant_attribute_struct = (
                    richest_variant_attribute.raw_data_type()
                )
                inner_primary_keys = richest_variant_attribute_struct.get_primary_keys()
                assert len(inner_primary_keys) == 1
                inner_primary_key = inner_primary_keys[0]
                if richest_variant_attribute_struct.is_nested():
                    document.write(
                        f"            {primary_key.name}: builder.{richest_variant_attribute.name}.as_ref().map(|{richest_variant_attribute.name}| {richest_variant_attribute.name}.inner.{inner_primary_key.name}).unwrap(),\n"
                    )
                else:
                    document.write(
                        f"            {primary_key.name}: builder.{richest_variant_attribute.name}.as_ref().map(|{richest_variant_attribute.name}| {richest_variant_attribute.name}.{inner_primary_key.name}).unwrap(),\n"
                    )
        elif variant.is_update_variant():
            for primary_key in primary_keys:
                if builder.has_attribute(primary_key) and not builder.get_attribute_by_name(primary_key.name).has_struct_data_type():
                    document.write(
                        f"            {primary_key.name}: builder.{primary_key.name}.unwrap(),\n"
                    )
                else:
                    richest_variant_attribute = richest_variant.get_attribute_by_name(
                        primary_key.normalized_name()
                    )
                    richest_variant_attribute_struct = richest_variant_attribute.raw_data_type()

                    inner_primary_keys = richest_variant_attribute_struct.get_primary_keys()
                    assert len(inner_primary_keys) == 1, (
                        f"Expected a single primary key in the struct {richest_variant_attribute_struct.name}, "
                        f"but found {len(inner_primary_keys)}."
                    )

                    inner_primary_key = inner_primary_keys[0]

                    if richest_variant_attribute_struct.is_nested():
                        document.write(
                            f"            {primary_key.name}: builder.{richest_variant_attribute.name}.as_ref().map(|{richest_variant_attribute.name}| {richest_variant_attribute.name}.inner.{inner_primary_key.name}).unwrap(),\n"
                        )
                    else:
                        document.write(
                            f"            {primary_key.name}: builder.{richest_variant_attribute.name}.as_ref().map(|{richest_variant_attribute.name}| {richest_variant_attribute.name}.{inner_primary_key.name}).unwrap(),\n"
                        )

        for attribute in flat_variant.attributes:

            if attribute.is_automatically_determined_column():
                continue

            if attribute in primary_keys:
                continue

            # There are 3 cases to consider:
            # 1. The attribute is present in the builder, and is not a nested attribute.
            # 2. The attribute is present in the builder, and is a nested attribute, so we need to recover the inner attribute.
            # 3. The attribute is not present in the builder, so we need to check whether the normalized version, with the _id suffix, is present.

            builder_attribute = builder.get_attribute_by_name(
                attribute.normalized_name()
            )

            if builder_attribute is None:

                raise RuntimeError(
                    f"It was impossible to find the attribute names '{attribute.name}' in "
                    f"the builder struct {builder.name}. The attributes present in the struct "
                    f"are {[attribute.name for attribute in builder.attributes]}."
                )

            # At this point, we need to handle the case where the attribute is expected to be
            # an option by the build target, and therefore we do not unwrap it, or alternatively
            # the attribute is not expected to be an option by the build target, and therefore we
            # unwrap it.
            if attribute.optional:
                if builder_attribute.has_struct_data_type():
                    inner_primary_keys = (
                        builder_attribute.raw_data_type().get_primary_keys()
                    )
                    assert len(inner_primary_keys) == 1
                    inner_primary_key = inner_primary_keys[0]
                    if builder_attribute.raw_data_type().is_nested():
                        document.write(
                            f"            {attribute.name}: builder.{builder_attribute.name}.map(|{builder_attribute.name}| {builder_attribute.name}.inner.{inner_primary_key.name}),\n"
                        )
                    else:
                        document.write(
                            f"            {attribute.name}: builder.{builder_attribute.name}.map(|{builder_attribute.name}| {builder_attribute.name}.{inner_primary_key.name}),\n"
                        )
                else:
                    document.write(
                        f"            {attribute.name}: builder.{attribute.name},\n"
                    )
            else:
                if builder_attribute.has_struct_data_type():
                    inner_primary_keys = (
                        builder_attribute.raw_data_type().get_primary_keys()
                    )

                    assert len(inner_primary_keys) == 1
                    inner_primary_key = inner_primary_keys[0]

                    if builder_attribute.raw_data_type().is_nested():
                        document.write(
                            f"            {attribute.name}: builder.{builder_attribute.name}.unwrap().inner.{inner_primary_key.name},\n"
                        )
                    else:
                        document.write(
                            f"            {attribute.name}: builder.{builder_attribute.name}.unwrap().{inner_primary_key.name},\n"
                        )
                else:
                    document.write(
                        f"            {attribute.name}: builder.{attribute.name}.unwrap(),\n"
                    )

        document.write("        }\n    }\n}\n")


def handle_missing_gin_index(
    attribute: AttributeMetadata,
    builder: StructMetadata,
):
    """Handles the case where a GIN trigram index is missing.

    Parameters
    ----------
    attribute : AttributeMetadata
        The attribute for which the GIN trigram index is missing.
    builder : StructMetadata
        The builder struct associated with the attribute.
    """
    # We prepare a message for the user to ask them whether we should generate the index
    # automatically for them. The exception will be raised nevertheless, as creating an
    # index in-medias-res will change many of the other metadata collected earlier on,
    # and the pipeline has to be re-run from the beginning.

    # First, we identify the migration the migration after which the index should be created.
    # The index has to be created AFTER either the creation of the table or the population of the
    # table with data, as the index will be created on the populated table. These names are the
    # suffixes of the migrations, as the prefix is the number of the migration.
    target_path_names = [
        f"populate_{attribute.raw_data_type().table_name}_table",
        f"create_{attribute.raw_data_type().table_name}_table",
    ]

    print(f"I need for the {attribute.raw_data_type().table_name} table to be searchable to generate the form for the builder {builder.name}.")

    # We find the migration after which the index should be created.
    target_migration = None
    for target_path_name in target_path_names:
        for migration in os.listdir("../backend/migrations"):
            if migration.endswith(target_path_name):
                target_migration = migration
                break
        if target_migration is not None:
            break

    migration_number = int(target_migration.split("_")[0])

    index_migration_name = f"create_{attribute.raw_data_type().table_name}_gin_index"

    full_migration_name = (
        f"{(str(migration_number + 1)).zfill(14)}_{index_migration_name}"
    )

    textual_columns = []

    flat_variant = None

    assert not attribute.raw_data_type().has_only_foreign_keys()

    if attribute.raw_data_type().is_nested():
        inner_attribute = attribute.raw_data_type().get_attribute_by_name("inner")

        if inner_attribute is None:
            raise RuntimeError(
                f"The attribute {attribute.name} is nested, but we could not find the inner attribute. "
                f"The builder struct is {builder.name}. "
                f"It is of type {attribute.raw_data_type().name}. "
                f"The other attributes in the struct are {', '.join(inner_attribute.name for inner_attribute in attribute.raw_data_type().attributes)}."
            )

        flat_variant = inner_attribute.raw_data_type()
    else:
        flat_variant = attribute.raw_data_type()

    for inner_attribute in flat_variant.attributes:
        if inner_attribute.data_type() in TEXTUAL_DATA_TYPES:
            textual_columns.append(inner_attribute)

    if len(textual_columns) == 0:
        raise RuntimeError(
            f"The table {attribute.raw_data_type().table_name} is not searchable as "
            "we did not find a GIN trigram index for it. We cannot generate a datalist "
            "for it - please create a GIN trigram index for it and try again. "
            "If you have just created the index, recall that you may still need to run "
            "that particular migration. Furthermore, we have not even found any textual "
            "columns in the table, so we cannot help you creating the index. "
            f"The builder struct is {builder.name}. "
            f"The attribute name is {attribute.name} and is of type {attribute.data_type()}."
        )

    if len(textual_columns) > 0:
        print(
            "The table is not searchable as we did not find a GIN trigram index for it.\n"
            f"The following columns are searchable: {', '.join(textual_column.name for textual_column in textual_columns)}\n"
            f"We can generate a GIN trigram index for the table {attribute.raw_data_type().table_name}.\n"
            f"We will generate part of the index in the migration {full_migration_name}.\n"
            "You will still need to refine the index afterwards to your liking."
        )

        user_answer = userinput(
            name="Create GIN index?",
            default=False,
            validator="human_bool",
            sanitizer="human_bool",
            cache=False,
        )

        assert isinstance(user_answer, bool)

        if user_answer:
            print("We will generate the index in the migration.")
            print("Please re-run pipeline once the index has been created.")
            sleep(2)
            insert_migration(
                counter=migration_number + 1,
                name=index_migration_name,
            )

            concatenate_columns = "_".join(
                textual_column.name for textual_column in textual_columns
            )

            index_name = (
                f"{attribute.raw_data_type().table_name}_{concatenate_columns}_trgm_idx"
            )
            function_name = None

            with open(
                f"./migrations/{full_migration_name}/up.sql", "w", encoding="utf8"
            ) as up_index_migration:

                up_index_migration.write(
                    f"-- Create index to run approximate search queries on the {attribute.raw_data_type().table_name} table.\n"
                    f"-- The search will be case insensitive and will use the trigram index.\n\n"
                )

                if len(textual_columns) > 1:
                    function_name = f"concat_{attribute.raw_data_type().table_name}_{concatenate_columns}"

                    up_index_migration.write(f"CREATE FUNCTION {function_name}(\n")
                    for inner_attribute in textual_columns:
                        up_index_migration.write(f"{inner_attribute.name} text,\n")
                    up_index_migration.write(
                        ") RETURNS text AS $$\n"
                        "BEGIN\n"
                        "-- TODO! Add the concatenation logic here!\n"
                        "END;\n"
                        "$$ LANGUAGE plpgsql IMMUTABLE STRICT PARALLEL SAFE;\n\n"
                    )

                    up_index_migration.write(
                        f"CREATE INDEX {index_name} ON {attribute.raw_data_type().table_name} USING gin (\n"
                        f"{function_name}(\n"
                    )
                    for inner_attribute in textual_columns:
                        up_index_migration.write(f"{inner_attribute.name},\n")

                    up_index_migration.write(") gin_trgm_ops\n);\n")
                else:
                    up_index_migration.write(
                        f"CREATE INDEX {index_name} ON {attribute.raw_data_type().table_name} USING gin (\n"
                        f"    {textual_columns[0].name} gin_trgm_ops\n"
                        ");\n"
                    )

            with open(
                f"./migrations/{full_migration_name}/down.sql", "w", encoding="utf8"
            ) as down_index_migration:
                down_index_migration.write(
                    f"-- Drop index on the {attribute.raw_data_type().table_name} table.\n"
                    f"-- The index was used to run approximate search queries on the table.\n\n"
                    f"DROP INDEX {index_name};\n"
                )

                if function_name is not None:
                    down_index_migration.write(f"DROP FUNCTION {function_name}(")
                    for inner_attribute in textual_columns:
                        down_index_migration.write(f"{inner_attribute.name} text,\n")
                    down_index_migration.write(");\n")
        else:
            print("Please create the index manually and re-run the pipeline.")
            sleep(2)

    raise RuntimeError(
        f"The table {attribute.raw_data_type().table_name} is not searchable as "
        "we did not find a GIN trigram index for it. We cannot generate a datalist "
        "for it - please create a GIN trigram index for it and try again. "
        "If you have just created the index, recall that you may still need to run "
        "that particular migration. This error was encountered while trying to generate "
        f"the form for the {builder.name} builder."
    )


def implements_row_to_searchable_badge(
    struct: StructMetadata,
):
    """Returns whether the struct implements the RowToSearchableBadge trait.

    Parameters
    ----------
    struct : StructMetadata
        The struct for which to check the implementation.

    Returns
    -------
    bool
        Whether the struct implements the RowToSearchableBadge trait.

    Implementation details
    ----------------------
    This method checks whether the provided struct implements the RowToSearchableBadge trait.
    """
    # The standard position of the RowToSearchableBadge trait implementation is in the
    # frontend crate, in the /src/components/row_to_searchable_badge/{table_name}.rs file.
    # We check whether the file exists, and if it does, we check whether the
    # struct implements the trait, meaning whether there appears therein the
    # implementation of the RowToSearchableBadge trait for the struct:
    #
    # impl RowToSearchableBadge for {struct_name} {

    # We check that this method is called at the correct position, in the
    # backend crate.

    assert os.getcwd().endswith("/backend")

    path = f"../frontend/src/components/database/row_to_searchable_badge/{struct.table_name}.rs"

    if not os.path.exists(path):
        return False

    module_path = "../frontend/src/components/database/row_to_searchable_badge.rs"

    # We check that the module is imported in the row_to_searchable_badge.rs file.
    with open(module_path, "r", encoding="utf8") as module:
        for line in module:
            if line.startswith(f"pub mod {struct.table_name};"):
                break
        else:
            return False

    with open(path, "r", encoding="utf8") as document:
        for line in document:
            if line.startswith(f"impl RowToSearchableBadge for {struct.name} {{"):
                return True

    return False

def write_frontend_yew_form(
    builder: StructMetadata,
    document: "io.TextIO",
):
    """Writes the Yew form for the builder struct.

    Parameters
    ----------
    builder : StructMetadata
        The builder struct for which to write the form.
    document : io.TextIO
        The document to write to.

    Implementation details
    ----------------------
    This method writes the Yew form for the provided builder struct.
    """

    flat_variant = builder.get_flat_variant()
    richest_variant = builder.get_richest_variant()
    primary_keys = flat_variant.get_primary_keys()

    variants = []

    if flat_variant.is_insertable() and flat_variant.table_name != "users":
        variants.append((builder.get_new_variant(), "POST"))

    if flat_variant.is_updatable():
        variants.append((builder.get_update_variant(), "PUT"))

    for variant, method in variants:

        action_name = "Create" if method == "POST" else "Update"

        form_component_name = f"{action_name}{flat_variant.name}Form"

        # We generate the lowercased name of the form component by splitting
        # on the uppercased letters and joining the resulting list with an
        # underscore.
        form_method_name = "_".join(
            re.findall("[A-Z][^A-Z]*", form_component_name)
        ).lower()

        # When we are creating an update variant, the form needs to receive the ID associated
        # to the variant, so that the frontend can request the correct row from the backend,
        # and upon submission, the frontend can send the correct ID to the backend. For this
        # reason, we need to create a struct that derives the Properties trait, which will
        # contain the ID of the row.

        properties_attributes: List[AttributeMetadata] = []

        if method == "PUT":
            document.write(
                f"#[derive(Clone, PartialEq, Properties)]\n"
                f"pub struct {form_component_name}Prop {{\n"
            )
            for primary_key in primary_keys:
                document.write(
                    f"    pub {primary_key.name}: {primary_key.format_data_type()},\n"
                )
                properties_attributes.append(primary_key)
            document.write("}\n\n")
        elif method == "POST" and flat_variant.has_manually_determined_foreign_keys():
            document.write(
                f"#[derive(Clone, PartialEq, Properties)]\n"
                f"pub struct {form_component_name}Prop {{\n"
            )
            for foreign_key in flat_variant.get_manually_determined_foreign_keys():
                # If the provided foreign key has an associated default value, we use it.
                default_value = flat_variant.table_metadata.get_default_column_value(
                    builder.table_name, foreign_key.name
                )
                foreign_key_struct = richest_variant.get_attribute_by_name(
                    foreign_key.normalized_name()
                ).raw_data_type()

                if foreign_key_struct.is_nested():
                    foreign_key_struct = foreign_key_struct.get_flat_variant()

                if default_value is not None:
                    document.write(
                        f"     #[prop_or({default_value})]\n"
                        f"     pub {foreign_key.name}: {foreign_key.data_type()},\n"
                    )
                    properties_attributes.append(foreign_key)
                else:
                    # If the foreign key struct does not have an associated trigram index,
                    # we require that the attribute is provided by the user and cannot be
                    # optional.
                    if foreign_key_struct.is_searchable():
                        document.write(
                            "     #[prop_or_default]\n"
                            f"    pub {foreign_key.name}: Option<{foreign_key.data_type()}>,\n"
                        )
                        properties_attributes.append(foreign_key.as_option())
                    else:
                        if foreign_key.optional:
                            handle_missing_gin_index(
                                builder.get_attribute_by_name(
                                    foreign_key.normalized_name()
                                ),
                                builder,
                            )

                        document.write(
                            f"    pub {foreign_key.name}: {foreign_key.data_type()},\n"
                        )
                        properties_attributes.append(foreign_key)
            document.write("}\n\n")

        document.write(f"#[function_component({form_component_name})]\n")
        if method == "PUT" or flat_variant.has_manually_determined_foreign_keys():
            document.write(
                f"pub fn {form_method_name}(props: &{form_component_name}Prop) -> Html {{\n"
                "     let mut named_requests: Vec<ComponentMessage> = Vec::new();\n"
            )
        else:
            document.write(f"pub fn {form_method_name}() -> Html {{\n")
        document.write(
            f"    let (builder_store, builder_dispatch) = use_store::<{builder.name}>();\n"
        )

        if method == "PUT":
            document.write(
                "    // We push the ID of the row to the named requests.\n"
                "    let props = props.clone();\n"
                f"   named_requests.push(ComponentMessage::get::<{variant.name}>({flat_variant.get_formatted_primary_keys(include_prefix=True, prefix='props')}.into()));\n"
            )
        elif method == "POST":
            if builder.has_sampled_by():
                document.write(
                    "    let user_state = use_store_value::<UserState>();\n"
                )
                
            for property_attribute in properties_attributes:
                property_struct = richest_variant.get_attribute_by_name(
                    property_attribute.normalized_name()
                ).raw_data_type()

                if property_struct.is_nested():
                    property_struct = property_struct.get_flat_variant()

                if property_attribute.optional:
                    document.write(
                        f"   if let Some({property_attribute.name}) = props.{property_attribute.name} {{\n"
                        f'         named_requests.push(ComponentMessage::get_named::<&str, {property_struct.name}>("{property_attribute.normalized_name()}", {property_attribute.name}.into()));\n'
                        "    }\n"
                    )
                    if property_attribute.is_sampled_by():
                        document.write(
                            "    else if let Some(user) = user_state.as_ref().user() {\n"
                            f"        builder_dispatch.apply(SampleActions::SetSampledBy(Some(user.clone())));\n"
                            "    }\n"
                        )
                else:
                    document.write(
                        f'    named_requests.push(ComponentMessage::get_named::<&str, {property_struct.name}>("{property_attribute.normalized_name()}", props.{property_attribute.name}.into()));\n'
                    )

        for attribute in builder.attributes:
            # We do not want to include the errors attribute in the builder actions.
            if is_builder_reserved_attribute(attribute):
                continue

            if attribute in primary_keys:
                continue

            if attribute.data_type() == "bool":
                document.write(
                    f"    let set_{attribute.name} = builder_dispatch.apply_callback(|{attribute.name}: bool| {flat_variant.name}Actions::Set{attribute.capitalized_normalized_name()}(Some({attribute.name})));\n"
                )
            elif (
                attribute.data_type() in INPUT_TYPE_MAP
                or attribute.data_type() == "NaiveDateTime"
            ):
                document.write(
                    f"    let set_{attribute.name} = builder_dispatch.apply_callback(|{attribute.name}: Option<String>| {flat_variant.name}Actions::Set{attribute.capitalized_normalized_name()}({attribute.name}));\n"
                )
            elif attribute.data_type() == "Vec<u8>":
                if "picture" in attribute.name:
                    document.write(
                        f"    let set_{attribute.name} = builder_dispatch.apply_callback(|{attribute.name}: Option<Image>| {flat_variant.name}Actions::Set{attribute.capitalized_normalized_name()}({attribute.name}.map(|{attribute.name}| {attribute.name}.into())));\n"
                    )
                else:
                    raise RuntimeError(
                        f"Attribute {attribute.name} of type {attribute.data_type()} not supported in the frontend form generation."
                    )
            else:
                document.write(
                    f"    let set_{attribute.name} = builder_dispatch.apply_callback(|{attribute.name}: {attribute.format_data_type()}| {flat_variant.name}Actions::Set{attribute.capitalized_normalized_name()}({attribute.name}));\n"
                )

        document.write(
            "    html! {\n"
            f"        <BasicForm<{variant.name}>\n"
            f"            method={{FormMethod::{method}}}\n"
        )
        if method == "PUT" or flat_variant.has_manually_determined_foreign_keys():
            document.write("            named_requests={named_requests}\n")
        document.write(
            "            builder={builder_store.deref().clone()} builder_dispatch={builder_dispatch}>\n"
        )

        for attribute in builder.attributes:

            if is_builder_reserved_attribute(attribute):
                continue

            if attribute in primary_keys:
                continue

            error_attribute = builder.get_attribute_by_name(f"errors_{attribute.name}")

            assert (
                error_attribute is not None
            ), f"Error attribute not found for {attribute.name} in {builder.name}."

            struct_attribute = richest_variant.get_attribute_by_name(attribute.name)

            if struct_attribute is None:
                struct_attribute = flat_variant.get_attribute_by_name(attribute.name)

            assert (
                struct_attribute is not None
            ), f"Attribute {attribute.name} not found in the struct {flat_variant.name}."

            optional = "true" if struct_attribute.optional else "false"

            if (
                attribute.data_type() in INPUT_TYPE_MAP
                or attribute.data_type() == "NaiveDateTime"
            ):
                attribute_data_type = attribute.data_type()
                if "barcode" in attribute.name:
                    attribute_data_type = "BarCode"

                attribute_conversion = f"builder_store.{attribute.name}.clone()"

                if "barcode" in attribute.name:
                    attribute_conversion = f"builder_store.{attribute.name}.clone().map(BarCode::from)"

                document.write(
                    f'            <BasicInput<{attribute_data_type}> label="{attribute.human_readable_name()}" optional={{{optional}}} errors={{builder_store.{error_attribute.name}.clone()}} builder={{set_{attribute.name}}} value={{{attribute_conversion}}} />\n'
                )
                continue

            if attribute.data_type() == "bool":
                document.write(
                    f'            <Checkbox label="{attribute.human_readable_name()}" errors={{builder_store.{error_attribute.name}.clone()}} builder={{set_{attribute.name}}} value={{builder_store.{attribute.name}.unwrap_or(false)}} />\n'
                )
                continue

            if attribute.data_type() == "Vec<u8>":
                if "picture" in attribute.name:
                    allowed_formats = ["GenericFileFormat::Image"]

                    document.write(
                        f'            <FileInput<Image> label="{attribute.human_readable_name()}" optional={{{optional}}} errors={{builder_store.{error_attribute.name}.clone()}} builder={{set_{attribute.name}}} allowed_formats={{vec![{", ".join(allowed_formats)}]}} value={{builder_store.{attribute.name}.clone().map(|{attribute.name}| {attribute.name}.into())}} />\n'
                    )
                else:
                    raise RuntimeError(
                        f"Attribute {attribute.name} of type {attribute.data_type()} not supported in the frontend form generation."
                    )

                continue

            # If the attribute is a nested struct, we need to generate a Datalist
            # that will allow the user to select the nested struct.
            if attribute.has_struct_data_type():
                struct: StructMetadata = attribute.raw_data_type()

                flat_attribute = flat_variant.get_attribute_by_name(attribute.name)
                if flat_attribute is None:
                    flat_attribute = flat_variant.get_attribute_by_name(
                        f"{attribute.name}_id"
                    )

                assert (
                    flat_attribute is not None
                ), f"Attribute {attribute.name} not found in the struct {flat_variant.name}."

                # We check that the table associated to the nested struct is searchable, otherwise
                # we cannot generate the datalist for it and we need to raise an exception.
                if not struct.is_searchable():
                    if flat_attribute not in properties_attributes:
                        if struct.table_name == "spectra_collections":
                            document.write(
                                f'<p>{{"{flat_attribute.human_readable_name()} has to be selected with a ScannerInput, which is not yet available."}}</p>\n'
                            )
                            continue
                        else:
                            handle_missing_gin_index(attribute, builder)
                    else:
                        # If the attribute does not have an index but appears in the mandatory properties,
                        # it will be loaded shortly from the backend and fed into the current builder
                        # via the actions callbacks in the Reducer. As such, we can display here the selected
                        # badge for the attribute once it has been loaded into the builder.

                        document.write(
                            f"if let Some({attribute.name}) = builder_store.{attribute.name}.as_ref() {{\n"
                            f'    <span>{{"TODO Selected {attribute.name}"}}</span>\n'
                            # f"    {{{attribute.name}.to_selected_datalist_badge()}}\n"
                            "}\n"
                        )
                        continue

                # We check that the nested struct implements the RowToSearchableBadge trait, as we need to
                # be able to convert the nested struct to a badge within the Datalist.
                if not implements_row_to_searchable_badge(attribute.raw_data_type()):
                    print(f"The form for the {builder.name} requires the {struct.name} struct to implement the RowToSearchableBadge trait.")
                    raise RuntimeError(
                        f"The struct {attribute.raw_data_type().name} does not implement the RowToSearchableBadge trait."
                    )

                updatables = (
                    "true"
                    if struct.has_associated_roles() and struct.table_name != "users"
                    else "false"
                )
                scannable = "true" if "barcode" in attribute.name else "false"

                attribute_data_type = attribute.data_type()

                document.write(
                    f'            <Datalist<{attribute.data_type()}, {updatables}> builder={{set_{attribute.name}}} optional={{{optional}}} errors={{builder_store.{error_attribute.name}.clone()}} value={{builder_store.{attribute.name}.clone()}} label="{attribute.human_readable_name()}" scanner={{{scannable}}} />\n'
                )
                continue

            # TODO! ADD MORE INPUT TYPES HERE!

            # raise Exception(
            #     f"Attribute {attribute.name} of type {attribute.data_type()} not supported in the frontend form generation."
            # )

        document.write(f"        </BasicForm<{variant.name}>>\n" f"    }}\n" f"}}\n")


def write_frontend_form_buildable_implementation(
    builder: StructMetadata,
    document: "io.TextIO",
):
    """Writes the implementation of the Buildable trait for the target struct.

    Parameters
    ----------
    builder : StructMetadata
        The builder struct for which to write the implementation.
    document : io.TextIO
        The document to write to.

    Implementation details
    ----------------------
    This method implements the Buildable trait for the provided struct.
    """

    flat_variant = builder.get_flat_variant()

    variants: List[StructMetadata] = []

    if flat_variant.is_insertable() and flat_variant.table_name != "users":
        variants.append(builder.get_new_variant())

    if flat_variant.is_updatable():
        update_variant = builder.get_update_variant()
        if not update_variant.is_new_variant():
            variants.append(update_variant)

    for variant in variants:
        # We implement the FormBuildable trait for the struct.
        document.write(
            f"impl FormBuildable for {variant.name} {{\n"
            f"    type Builder = {builder.name};\n"
            "    fn title() -> &'static str {\n"
            f'        "{variant.human_readable_name()}"\n'  # TODO! Add the title
            "    }\n"
            "    fn task_target() -> &'static str {\n"
            f'        "{variant.human_readable_name()}"\n'  # TODO! Add the task target name
            "    }\n"
            "    fn requires_authentication() -> bool {\n"
            f"        {'true' if variant.requires_authentication() else 'false'}\n"
            "    }\n"
            "    fn can_operate_offline() -> bool {\n"
            f"        {'true' if variant.has_uuid_primary_key() or variant.is_update_variant() else 'false'}\n"
            "    }\n"
            "}\n\n"
        )


def write_frontend_forms(
    builder_structs: List[StructMetadata],
):
    """Writes the frontend forms to the web_common crate."""

    # For the time being, we simply write out the structs.
    # In the near future, we will also implement several
    # traits for these structs.

    path = "../frontend/src/components/forms/automatic_forms.rs"

    document = open(path, "w", encoding="utf8")

    # Preliminarly, we write a docstring at the very head
    # of this submodule to explain what it does and warn the
    # reader not to write anything in this file as it is
    # automatically generated.

    document.write(
        "//! This module contains the forms for the frontend.\n"
        "//!\n"
        "//! This module is automatically generated. Do not write anything here.\n\n"
    )

    imports = [
        "use serde::{Deserialize, Serialize};",
        "use web_common::database::*;",
        "use yew::prelude::*;",
        "use yewdux::{use_store, use_store_value, Reducer, Store};",
        "use crate::components::forms::*;",
        "use web_common::api::form_traits::FormMethod;",
        "use std::rc::Rc;",
        "use uuid::Uuid;",
        "use std::ops::Deref;",
        "use yewdux::Dispatch;",
        "use chrono::NaiveDateTime;",
        "use web_common::api::ApiError;",
        "use crate::stores::user_state::UserState;",
        "use crate::workers::ws_worker::ComponentMessage;",
        "use web_common::custom_validators::Image;",
        "use web_common::file_formats::GenericFileFormat;",
    ]

    for import_statement in imports:
        document.write(f"{import_statement}\n")

    document.write("\n")

    for builder in tqdm(
        builder_structs,
        desc="Writing new buiders",
        unit="struct",
        leave=False,
    ):
        builder.write_to(document, derives_deny_list=["Default"])
        write_frontend_builder_default_implementation(builder, document)
        write_frontend_builder_action_enumeration(builder, document)
        write_frontend_form_builder_implementation(builder, document)
        write_frontend_form_buildable_implementation(builder, document)
        write_frontend_yew_form(builder, document=document)

    document.flush()
    document.close()

    # We verify that the forms generation has been successful by
    # checking that all of the builder names and their relative new
    # variants and update variants are present in the generated file.

    with open(path, "r", encoding="utf8") as document:
        content = document.read()

    for builder in builder_structs:
        assert (
            builder.name in content
        ), f"Builder {builder.name} not found in the generated file."

        flat_variant = builder.get_flat_variant()

        if flat_variant.is_insertable() and flat_variant.table_name != "users":
            assert (
                builder.get_new_variant().name in content
            ), f"New variant {builder.get_new_variant().name} not found in the generated file."

        if flat_variant.is_updatable():
            assert (
                builder.get_update_variant().name in content
            ), f"Update variant {builder.get_update_variant().name} not found in the generated file."
