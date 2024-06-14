"""This module writes the `update` method for the struct in the GlueSQL database."""

from constraint_checkers.struct_metadata import StructMetadata, AttributeMetadata
from constraint_checkers.gluesql_types_mapping import GLUESQL_TYPES_MAPPING


def write_update_method_for_gluesql(
    struct: StructMetadata,
    writer: "io.TextIOWrapper",
):
    """Write the `update` method for the struct in the GlueSQL database."""
    if struct.is_update_variant() and struct.table_name != "users":
        updator_user_id_attribute: AttributeMetadata = (
            struct.get_updator_user_id_attribute()
        )
    else:
        updator_user_id_attribute = None

    update_types_and_methods = GLUESQL_TYPES_MAPPING.copy()
    update_types_and_methods["bool"] = "{value}"

    # We implement the `update` method for the struct. This method updates
    # the struct in the GlueSQL database.
    writer.write(
        "    /// Update the struct in the database.\n"
        "    ///\n"
        "    /// # Arguments\n"
    )

    if updator_user_id_attribute is not None:
        writer.write(
            "    /// * `user_id` - The ID of the user who is updating the struct.\n"
        )

    writer.write(
        "    /// * `connection` - The connection to the database.\n"
        "    ///\n"
        "    /// # Returns\n"
        "    /// The number of rows updated.\n"
        "    pub async fn update<C>(\n"
        "        self,\n"
    )

    if updator_user_id_attribute is not None:
        writer.write("        user_id: i32,\n")

    writer.write(
        "        connection: &mut gluesql::prelude::Glue<C>,\n"
        "    ) -> Result<usize, gluesql::prelude::Error> where\n"
        "        C: gluesql::core::store::GStore + gluesql::core::store::GStoreMut,\n"
        "    {\n"
        "        use gluesql::core::ast_builder::*;\n"
    )
    # We use the AST builder as much as possible so to avoid SQL injection attacks.

    # First, we determine whether the current struct has at least an optional field.

    if struct.contains_optional_fields():
        writer.write(f'        let mut update_row = table("{struct.table_name}")\n')
    else:
        writer.write(f'        table("{struct.table_name}")\n')
    writer.write("            .update()")

    if struct.contains_only_optional_fields():
        raise NotImplementedError(
            f"The struct {struct.name} does not contain any non-optional attributes. "
            "It is not well defined how to update a struct without any attributes."
        )

    for attribute in struct.attributes:
        if attribute.optional:
            # We handle this in the next loop
            continue
        if attribute.raw_data_type() in update_types_and_methods:
            conversion = update_types_and_methods[attribute.raw_data_type()].format(
                value=f"self.{attribute.name}"
            )
            writer.write(f'        \n.set("{attribute.name}", {conversion})')
        else:
            raise NotImplementedError(
                f"The type {attribute.raw_data_type()} is not supported."
                f"The struct {struct.name} contains an {attribute.raw_data_type()}."
            )

    if updator_user_id_attribute is not None:
        conversion = update_types_and_methods["i32"].format(value="user_id")
        writer.write(
            f'        \n.set("{updator_user_id_attribute.name}", {conversion})'
        )

    if struct.contains_optional_fields():
        writer.write(";\n")

    # After all of the non-optional fields, we handle the optional fields.
    for attribute in struct.attributes:
        if not attribute.optional:
            continue
        conversion = update_types_and_methods[attribute.raw_data_type()].format(
            value=f"self.{attribute.name}"
        )
        if attribute.raw_data_type() in update_types_and_methods:
            writer.write(
                f"        if let Some({attribute.name}) = self.{attribute.name} {{\n"
                f'            update_row = update_row.set("{attribute.name}", {update_types_and_methods[attribute.raw_data_type()].format(value=attribute.name)});\n'
                "        }\n"
            )
        else:
            raise NotImplementedError(
                f"The type {attribute.raw_data_type()} is not supported. "
                f"The struct {attribute.name} contains an {attribute.raw_data_type()}. "
            )

    if struct.contains_optional_fields():
        writer.write("            update_row.execute(connection)\n")
    else:
        writer.write("            .execute(connection)\n")
    writer.write(
        "            .await\n"
        "             .map(|payload| match payload {\n"
        "                 gluesql::prelude::Payload::Update(number_of_updated_rows) => number_of_updated_rows,\n"
        '                 _ => unreachable!("Expected Payload::Update")\n'
        "})\n"
        "    }\n\n"
    )
