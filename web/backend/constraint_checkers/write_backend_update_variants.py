"""This module contains the function that writes the update variants to the backend."""

from typing import List
from tqdm.auto import tqdm
from constraint_checkers.struct_metadata import StructMetadata, AttributeMetadata


def write_backend_update_variants(
    update_struct_metadatas: List[StructMetadata],
):
    """Writes to the backend the diesel methods for the update structs.

    Parameters
    ----------
    update_struct_metadatas : List[StructMetadata]
        The list of the StructMetadata objects.
    """

    path = "./src/update_variants.rs"

    document = open(path, "w", encoding="utf8")

    # First of all, we write a docstring that warns the reader
    # not to write anything in this file as it is automatically
    # generated.

    document.write(
        "//! This module contains the update variants of the database models.\n"
        "//!\n"
        "//! This module is automatically generated. Do not write anything here.\n\n"
    )

    imports = [
        "use diesel::prelude::*;",
        "use crate::models::*;",
        "use crate::schema::*;",
        "use diesel::r2d2::PooledConnection;",
        "use diesel::r2d2::ConnectionManager;",
        "use uuid::Uuid;",
        "use chrono::NaiveDateTime;",
    ]

    document.write("\n".join(imports) + "\n")

    # Since the update variants are defined in the web_common crate, in order to
    # implement methods for the backend we need to import the update variants from
    # the web_common crate and define new traits for the update variants. We also
    # need to implement these traits for the update variants, of course.

    # Because of how Diesel works, we need to define new structs that are an
    # intermediate representation of the row. These structs have all of the
    # attributes of the update variant, plus the attribute associated with the
    # updator user id. They derive the AsChangeset trait from
    # Diesel, which is used to insert the row in the database.

    # We start by defining the trait UpdateRow, which is implemented by the update
    # variants and provides the update method for the update variants. The update
    # method receives the user id of the user updating the row and the connection
    # to the database. The same trait also has an associated type, which is the
    # intermediate variant that is used to update the row in the database, and a
    # method that receives the self and user id and returns the intermediate variant.
    # The update method returns the newly updated row, which is the flat variant
    # of the update flat variant.

    document.write(
        "/// Trait providing the update method for the update variants.\n"
        "pub(super) trait UpdateRow {\n"
        "    /// The intermediate representation of the row.\n"
        "    type Intermediate;\n\n"
        "    /// The flat variant of the update variant.\n"
        "    type Flat;\n\n"
        "    /// Convert the update variant into the intermediate representation.\n"
        "    fn to_intermediate(self, user_id: i32) -> Self::Intermediate;\n\n"
        "    /// Update the row in the database.\n"
        "    fn update(\n"
        "        self,\n"
        "        user_id: i32,\n"
        "        connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>\n"
        "    ) -> Result<Self::Flat, diesel::result::Error>;\n"
        "}\n\n"
    )

    for struct in tqdm(
        update_struct_metadatas,
        desc="Writing update structs",
        unit="struct",
        leave=False,
    ):
        assert struct.is_update_variant(), (
            f"The struct {struct.name} is not an update variant, but it should be."
            f"It is associated to the table {struct.table_name}."
        )

        if struct.table_name == "users":
            updator_user_id_attribute = None
        else:
            updator_user_id_attribute: AttributeMetadata = (
                struct.get_updator_user_id_attribute()
            )

            assert not updator_user_id_attribute.optional, (
                f"The attribute {updator_user_id_attribute.name} of the struct {struct.name} "
                "is optional, but it should not be. Most likely, you forgot to add NOT NULL "
                f"to the attribute in the database im the table {struct.table_name}."
            )

            assert not isinstance(updator_user_id_attribute, StructMetadata)

        intermediate_struct_name = f"Intermediate{struct.name}"

        # First, we write the intermediate struct that is used to update the row in the database.
        document.write(
            f"/// Intermediate representation of the update variant {struct.name}.\n"
            "#[derive(Identifiable, AsChangeset)]\n"
            f"#[diesel(table_name = {struct.table_name})]\n"
            "#[diesel(treat_none_as_null = true)]\n"
            f"#[diesel(primary_key({struct.get_formatted_primary_keys(include_prefix=False, include_parenthesis=False)}))]\n"
            f"pub(super) struct {intermediate_struct_name} {{\n"
        )

        all_attributes: List[AttributeMetadata] = struct.attributes

        if struct.table_name != "users":
            all_attributes = [updator_user_id_attribute] + all_attributes

        for attribute in all_attributes:
            document.write(f"    {attribute.name}: {attribute.format_data_type()},\n")

        document.write("}\n\n")

        # Next, we implement the UpdateRow trait for the update variant.
        document.write(
            f"impl UpdateRow for web_common::database::{struct.name} {{\n"
            f"    type Intermediate = {intermediate_struct_name};\n"
            f"    type Flat = {struct.get_flat_variant().name};\n\n"
        )
        if struct.table_name != "users":
            document.write(
                "    fn to_intermediate(self, user_id: i32) -> Self::Intermediate {\n"
            )
        else:
            document.write(
                "    fn to_intermediate(self, _user_id: i32) -> Self::Intermediate {\n"
            )
        document.write(f"        {intermediate_struct_name} {{\n")

        for attribute in all_attributes:
            if struct.get_attribute_by_name(attribute.name) is not None:
                document.write(
                    f"            {attribute.name}: self.{attribute.name},\n"
                )
            else:
                document.write(f"            {attribute.name}: user_id,\n")

        document.write("        }\n    }\n\n")

        document.write(
            "    fn update(\n"
            "        self,\n"
            "        user_id: i32,\n"
            "        connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>\n"
            "    ) -> Result<Self::Flat, diesel::result::Error> {\n"
            "        self.to_intermediate(user_id)\n"
            "            .save_changes(connection)\n"
            "    }\n"
            "}\n\n"
        )

    document.flush()
    document.close()
