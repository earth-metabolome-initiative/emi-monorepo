"""Writes the update variants of the database models to the web_common crate."""

from typing import List
from tqdm.auto import tqdm
from constraint_checkers.gluesql_types_mapping import GLUESQL_TYPES_MAPPING
from constraint_checkers.struct_metadata import StructMetadata, AttributeMetadata
from constraint_checkers.write_update_method_for_gluesql import write_update_method_for_gluesql


def write_web_common_update_variants(
    update_struct_metadatas: List[StructMetadata],
):
    """Writes the update structs to the web_common crate."""

    # For the time being, we simply write out the structs.
    # In the near future, we will also implement several
    # traits for these structs.

    path = "../web_common/src/database/update_variants.rs"

    document = open(path, "w", encoding="utf8")

    # Preliminarly, we write a docstring at the very head
    # of this submodule to explain what it does and warn the
    # reader not to write anything in this file as it is
    # automatically generated.

    document.write(
        "//! This module contains the update variants of the database models.\n"
        "//!\n"
        "//! Some of the update variants would be identical to the new variants, "
        "//! and as such we do not generate them. You will find here the update variants "
        "//! only for the tables that have a primary key that is not a UUID.\n"
        "//! This module is automatically generated. Do not write anything here.\n\n"
    )

    imports = [
        "use serde::{Deserialize, Serialize};",
        "use chrono::NaiveDateTime;",
        "use uuid::Uuid;",
        "use super::*;",
    ]

    document.write("\n".join(imports) + "\n\n")

    for struct in tqdm(
        update_struct_metadatas,
        desc="Writing update structs",
        unit="struct",
        leave=False,
    ):
        assert struct.is_update_variant()

        struct.write_to(document)

        # We implement the Tabular trait for the new struct. This trait
        # is used to display the new struct in a table format in the frontend.

        document.write(
            f"impl Tabular for {struct.name} {{\n"
            f"    const TABLE: Table = Table::{struct.capitalized_table_name()};\n"
            "}\n"
        )

        # When the frontend flag is enables, we implement the insert method for the new flat struct.
        # This method receives the user id of the user inserting the row and the connection to the database.
        # The method returns a Result, where the Ok variant is the flat variant of the struct associated with
        # the newly inserted row, while the Err variant contains an ApiError.

        # First thing, we determine the name of the attribute that contains the user id. This is not directly
        # an attribute of the new variant, as it is not set by the user, but it is available as part of the
        # associated flat variant.

        attributes = struct.attributes

        if struct.table_name != "users":
            updator_user_id_attribute: AttributeMetadata = (
                struct.get_updator_user_id_attribute()
            )
            attributes = [updator_user_id_attribute] + attributes
        else:
            updator_user_id_attribute = None

        document.write(f'#[cfg(feature = "frontend")]\n' f"impl {struct.name} {{\n")

        if struct.table_name != "users":
            document.write(
                f"    pub fn into_row(self, {updator_user_id_attribute.name}: {updator_user_id_attribute.format_data_type()}) -> Vec<gluesql::core::ast_builder::ExprNode<'static>> {{\n"
            )
        else:
            document.write(
                "    pub fn into_row(self) -> Vec<gluesql::core::ast_builder::ExprNode<'static>> {\n"
            )

        document.write("        vec![\n")

        for attribute in attributes:

            if struct.get_attribute_by_name(attribute.name) is not None:
                self_attribute_name = f"self.{attribute.name}"
            else:
                self_attribute_name = attribute.name

            if attribute.optional:
                if attribute.data_type() in GLUESQL_TYPES_MAPPING:
                    document.write(
                        f"            match {self_attribute_name} {{\n"
                        f"                Some({attribute.name}) => {GLUESQL_TYPES_MAPPING[attribute.data_type()].format(attribute.name)},\n"
                        "                None => gluesql::core::ast_builder::null(),\n"
                        "            },\n"
                    )
                else:
                    raise NotImplementedError(
                        f"The type {attribute.data_type()} is not supported. "
                        f"The struct {struct.name} contains an {attribute.data_type()}. "
                    )
            elif attribute.data_type() in GLUESQL_TYPES_MAPPING:
                document.write(
                    f"            {GLUESQL_TYPES_MAPPING[attribute.data_type()].format(self_attribute_name)},\n"
                )
            else:
                raise NotImplementedError(
                    f"The type {attribute.data_type()} is not supported."
                )

        document.write("        ]\n    }\n\n")

        # We implement the `update` method for the struct. This method
        # receives a connection to the GlueSQL database and updates the
        # struct into the database.
        write_update_method_for_gluesql(struct, document)

        document.write("}\n")

    document.flush()
    document.close()
