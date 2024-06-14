"""Writes the new variants of the database models to the web_common crate."""

from typing import List
from tqdm.auto import tqdm
from constraint_checkers.struct_metadata import StructMetadata, AttributeMetadata
from constraint_checkers.gluesql_types_mapping import GLUESQL_TYPES_MAPPING
from constraint_checkers.write_update_method_for_gluesql import (
    write_update_method_for_gluesql,
)
from constraint_checkers.is_file_changed import is_file_changed
from constraint_checkers.migrations_changed import are_migrations_changed


def write_web_common_new_variants(
    new_struct_metadatas: List[StructMetadata],
):
    """Writes the new structs to the web_common crate."""

    if not (are_migrations_changed() or is_file_changed(__file__)):
        print("No change in migrations or file. Skipping writing frontend new variants.")
        return

    # For the time being, we simply write out the structs.
    # In the near future, we will also implement several
    # traits for these structs.

    path = "../web_common/src/database/new_variants.rs"

    document = open(path, "w", encoding="utf8")

    # Preliminarly, we write a docstring at the very head
    # of this submodule to explain what it does and warn the
    # reader not to write anything in this file as it is
    # automatically generated.

    document.write(
        "//! This module contains the new variants of the database models.\n"
        "//!\n"
        "//! This module is automatically generated. Do not write anything here.\n\n"
    )

    imports = [
        "use super::*;",
    ]

    document.write("\n".join(imports) + "\n\n")

    for struct in tqdm(
        new_struct_metadatas,
        desc="Writing new structs",
        unit="struct",
        leave=False,
    ):
        assert struct.is_new_variant()

        struct.write_to(document)

        # We implement the Tabular trait for the new struct. This trait
        # is used to display the new struct in a table format in the frontend.

        document.write(
            f"impl Tabular for {struct.name} {{\n"
            f"    const TABLE: Table = Table::{struct.capitalized_table_name()};\n"
            "}\n"
        )

        if not struct.has_uuid_primary_key():
            continue

        # When the frontend flag is enables, we implement the insert method for the new flat struct.
        # This method receives the user id of the user inserting the row and the connection to the database.
        # The method returns a Result, where the Ok variant is the flat variant of the struct associated with
        # the newly inserted row, while the Err variant contains an ApiError.

        # First thing, we determine the name of the attribute that contains the user id. This is not directly
        # an attribute of the new variant, as it is not set by the user, but it is available as part of the
        # associated flat variant.
        creator_user_id_attribute: AttributeMetadata = (
            struct.get_creator_user_id_attribute()
        )

        attributes = [
            creator_user_id_attribute,
        ] + struct.attributes

        if struct.is_updatable():
            updator_user_id_attribute: AttributeMetadata = (
                struct.get_updator_user_id_attribute()
            )
            attributes.append(updator_user_id_attribute)
        else:
            updator_user_id_attribute = None

        columns = []

        document.write(f'#[cfg(feature = "frontend")]\n' f"impl {struct.name} {{\n")

        document.write(
            f"    pub fn into_row(self, {creator_user_id_attribute.name}: {creator_user_id_attribute.format_data_type(route='web_common')}) -> Vec<gluesql::core::ast_builder::ExprNode<'static>> {{\n"
        )

        document.write("        vec![\n")
        for attribute in attributes:
            columns.append(attribute.name)

            if attribute in (
                creator_user_id_attribute,
                updator_user_id_attribute,
            ):
                self_attribute_name = creator_user_id_attribute.name
            else:
                self_attribute_name = f"self.{attribute.name}"

            if attribute.optional:
                if attribute.raw_data_type() in GLUESQL_TYPES_MAPPING:
                    document.write(
                        f"            match {self_attribute_name} {{\n"
                        f"                Some({attribute.name}) => {GLUESQL_TYPES_MAPPING[attribute.raw_data_type()].format(value=attribute.name)},\n"
                        "                None => gluesql::core::ast_builder::null(),\n"
                        "            },\n"
                    )
                else:
                    raise NotImplementedError(
                        f"The type {attribute.raw_data_type()} is not supported. "
                        f"The struct {struct.name} contains an {attribute.raw_data_type()}. "
                    )
            elif attribute.raw_data_type() in GLUESQL_TYPES_MAPPING:
                document.write(
                    f"            {GLUESQL_TYPES_MAPPING[attribute.raw_data_type()].format(value=self_attribute_name)},\n"
                )
            else:
                raise NotImplementedError(
                    f"The type {attribute.raw_data_type()} is not supported."
                )

        document.write("        ]\n    }\n\n")

        # We implement the `insert` method for the struct. This method
        # receives a connection to the GlueSQL database and inserts the
        # struct into the database.
        document.write(
            f"    /// Insert the {struct.name} into the database.\n"
            "    ///\n"
            "    /// # Arguments\n"
            f"    /// * `{creator_user_id_attribute.name}` - The id of the user inserting the row.\n"
            "    /// * `connection` - The connection to the database.\n"
            "    ///\n"
            "    /// # Returns\n"
            f"    /// The number of rows inserted in table {struct.name}\n"
            "    pub async fn insert<C>(\n"
            "        self,\n"
            f"        {creator_user_id_attribute.name}: {creator_user_id_attribute.format_data_type(route='web_common')},\n"
            "        connection: &mut gluesql::prelude::Glue<C>,\n"
            f"    ) -> Result<super::{struct.get_flat_variant().name}, gluesql::prelude::Error> where\n"
            "        C: gluesql::core::store::GStore + gluesql::core::store::GStoreMut,\n"
            "    {\n"
            "        use gluesql::core::ast_builder::*;\n"
            f"        let {struct.get_formatted_primary_keys(include_prefix=False)} = {struct.get_formatted_primary_keys(include_prefix=True)};\n"
            f'        table("{struct.table_name}")\n'
            "            .insert()\n"
            f'            .columns("{",".join(columns)}")\n'
            f"            .values(vec![self.into_row({creator_user_id_attribute.name})])\n"
            "            .execute(connection)\n"
            "            .await\n"
            "             .map(|payload| match payload {\n"
            "                 gluesql::prelude::Payload::Insert ( number_of_inserted_rows ) => number_of_inserted_rows,\n"
            '                 _ => unreachable!("Payload must be an Insert"),\n'
            "             })?;\n"
            f"        super::{struct.get_flat_variant().name}::get({struct.get_formatted_primary_keys(include_prefix=False)}, connection).await.map(|maybe_row| maybe_row.unwrap())\n"
            "    }\n\n"
        )

        if struct.is_updatable():
            write_update_method_for_gluesql(struct, document)

        document.write("}\n")

    document.flush()
    document.close()
