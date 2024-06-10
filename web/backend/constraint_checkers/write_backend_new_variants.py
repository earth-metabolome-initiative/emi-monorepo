"""This module contains the function that writes the new variants of the database models to the backend."""

from typing import List
from tqdm.auto import tqdm
from constraint_checkers.struct_metadata import StructMetadata, AttributeMetadata
from constraint_checkers.is_file_changed import is_file_changed
from constraint_checkers.migrations_changed import are_migrations_changed


def write_backend_new_variants(
    new_struct_metadatas: List[StructMetadata],
):
    """Writes to the backend the diesel methods for the new structs.

    Parameters
    ----------
    new_struct_metadatas : List[StructMetadata]
        The list of the StructMetadata objects.
    """
    if not (are_migrations_changed() or is_file_changed(__file__)):
        print("No change in migrations or file. Skipping writing backend new variants.")
        return

    path = "./src/database/new_variants.rs"
    document = open(path, "w", encoding="utf8")

    # First of all, we write a docstring that warns the reader
    # not to write anything in this file as it is automatically
    # generated.

    document.write(
        "//! This module contains the new variants of the database models.\n"
        "//!\n"
        "//! This module is automatically generated. Do not write anything here.\n\n"
    )

    imports = [
        "use diesel::prelude::*;",
        "use crate::database::*;",
    ]

    document.write("\n".join(imports) + "\n")

    # Since the new variants are defined in the web_common crate, in order to
    # implement methods for the backend we need to import the new variants from
    # the web_common crate and define new traits for the new variants. We also
    # need to implement these traits for the new variants, of course.

    # Because of how Diesel works, we need to define new structs that are an
    # intermediate representation of the row. These structs have all of the
    # attributes of the new variant, plus the attribute associated with the
    # creator user id. These intermediate variants are private to this document
    # as they are not used outside of it. They derive the Insertable trait from
    # Diesel, which is used to insert the row in the database.

    # We start by defining the trait InsertRow, which is implemented by the new
    # variants and provides the insert method for the new variants. The insert
    # method receives the user id of the user inserting the row and the connection
    # to the database. The same trait also has an associated type, which is the
    # intermediate variant that is used to insert the row in the database, and a
    # method that receives the self and user id and returns the intermediate variant.
    # The insert method returns the newly inserted row, which is the flat variant
    # of the new flat variant.

    document.write(
        "/// Trait providing the insert method for the new variants.\n"
        "pub(crate) trait InsertRow {\n"
        "    /// The intermediate representation of the row.\n"
        "    type Intermediate;\n\n"
        "    /// The flat variant of the new variant.\n"
        "    type Flat;\n\n"
        "    /// Convert the new variant into the intermediate representation.\n"
        "    fn to_intermediate(self, user_id: i32) -> Self::Intermediate;\n\n"
        "    /// Insert the row into the database.\n"
        "    fn insert(\n"
        "        self,\n"
        "        user_id: i32,\n"
        "        connection: &mut diesel::r2d2::PooledConnection<diesel::r2d2::ConnectionManager<diesel::PgConnection>>\n"
        "    ) -> Result<Self::Flat, diesel::result::Error>;\n"
        "}\n\n"
    )

    for struct in tqdm(
        new_struct_metadatas,
        desc="Writing new structs",
        unit="struct",
        leave=False,
    ):
        assert struct.is_new_variant()

        if struct.table_name == "users":
            creator_user_id_attribute = None
        else:
            creator_user_id_attribute: AttributeMetadata = (
                struct.get_creator_user_id_attribute()
            )

            assert not creator_user_id_attribute.optional, (
                f"The attribute {creator_user_id_attribute.name} of the struct {struct.name} "
                "is optional, but it should not be. Most likely, you forgot to add NOT NULL "
                f"to the attribute in the database im the table {struct.table_name}."
            )

            assert not creator_user_id_attribute.has_struct_data_type()

        intermediate_struct_name = f"Intermediate{struct.name}"

        # First, we write the intermediate struct that is used to insert the row in the database.
        document.write(
            f"/// Intermediate representation of the new variant {struct.name}.\n"
            "#[derive(Insertable)]\n"
            f"#[diesel(table_name = crate::database::schema::{struct.table_name})]\n"
            f"pub(crate) struct {intermediate_struct_name} {{\n"
        )

        all_attributes: List[AttributeMetadata] = struct.attributes

        if creator_user_id_attribute is not None:
            all_attributes = [creator_user_id_attribute] + all_attributes

        # If this struct also has an updator attribute, we add it to the list of attributes.
        # This value initially will be set to the same value as the creator user id, but it
        # will be updated when the row is updated.

        if struct.is_updatable() and struct.table_name != "users":
            updator_user_id_attribute: AttributeMetadata = (
                struct.get_updator_user_id_attribute()
            )
            all_attributes.append(updator_user_id_attribute)
        else:
            updator_user_id_attribute = creator_user_id_attribute

        for attribute in all_attributes:
            document.write(f"    {attribute.name}: {attribute.format_data_type()},\n")

        document.write("}\n\n")

        underscored_user_id = ""
        if struct.table_name == "users":
            underscored_user_id = "_"

        # Next, we implement the InsertRow trait for the new variant.
        document.write(
            f"impl InsertRow for web_common::database::{struct.name} {{\n"
            f"    type Intermediate = {intermediate_struct_name};\n"
            f"    type Flat = crate::database::flat_variants::{struct.get_flat_variant().name};\n\n"
            f"    fn to_intermediate(self, {underscored_user_id}user_id: i32) -> Self::Intermediate {{\n"
            f"        {intermediate_struct_name} {{\n"
        )

        for attribute in all_attributes:
            if attribute in (
                creator_user_id_attribute,
                updator_user_id_attribute,
            ):
                document.write(f"            {attribute.name}: user_id,\n")
                continue
            if attribute.is_jpeg():
                document.write(
                    f"            {attribute.name}: self.{attribute.name}.into(),\n"
                )
                continue
            document.write(f"            {attribute.name}: self.{attribute.name},\n")

        document.write("        }\n    }\n\n")

        defaulted_user_id = "user_id"
        if struct.table_name == "users":
            defaulted_user_id = "0"

        assert_user_id_check = ""
        if struct.table_name == "users":
            assert_user_id_check = "        assert_eq!(user_id, 0);\n"

        document.write(
            "    fn insert(\n"
            "        self,\n"
            f"       user_id: i32,\n"
            "        connection: &mut diesel::r2d2::PooledConnection<diesel::r2d2::ConnectionManager<diesel::PgConnection>>\n"
            "    ) -> Result<Self::Flat, diesel::result::Error> {\n"
            f"        use crate::database::schema::{struct.table_name};\n{assert_user_id_check}"
            f"        diesel::insert_into({struct.table_name}::dsl::{struct.table_name})\n"
            f"            .values(InsertRow::to_intermediate(self, {defaulted_user_id}))\n"
            "            .get_result(connection)\n"
            "    }\n"
            "}\n\n"
        )

    document.flush()
    document.close()
