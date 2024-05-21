"""Write the structs in the `web_common` crate."""

from typing import List
from tqdm import tqdm
from constraint_checkers.struct_metadata import StructMetadata
from constraint_checkers.gluesql_types_mapping import GLUESQL_TYPES_MAPPING
from constraint_checkers.write_update_method_for_gluesql import write_update_method_for_gluesql


def write_web_common_flat_variants(
    structs: List[StructMetadata],
    target: str,
):
    """Write the structs in the target file in the `web_common` crate.

    Parameters
    ----------
    structs : List[StructMetadata]
        The list of structs to write in the target file.
    target : str
        The path where to write the structs in the `web_common` crate.
    table_metadata : TableMetadata
        The metadata of the tables.
    """
    # The derive statements to include in the `src/database/tables.rs` document
    imports = [
        "use serde::Deserialize;",
        "use serde::Serialize;",
        "use uuid::Uuid;",
        "use chrono::NaiveDateTime;",
        "use crate::database::*;",
    ]

    document = open(f"../web_common/src/database/{target}.rs", "w", encoding="utf8")

    for import_statament in imports:
        document.write(f"{import_statament}\n")

    # First, we define the Tabular & Filtrable traits which we will implement for all of the
    # structs.
    if target == "tables":
        document.write(
            "\npub trait Tabular {\n"
            "    const TABLE: Table;\n"
            "}\n\n"
            "pub trait Filtrable: PartialEq {\n"
            "    type Filter: Serialize + PartialEq + Clone;\n"
            "}\n\n"
        )

    for struct in tqdm(
        structs,
        desc="Writing frontend structs",
        unit="struct",
        leave=False,
    ):
        # We write the struct definition.
        struct.write_to(document)

        # We implement the Tabular trait for the struct. This trait
        # is used to convert the struct into a Table variant.

        document.write(f"impl Tabular for {struct.name} {{\n")
        document.write(
            f"    const TABLE: Table = Table::{struct.capitalized_table_name()};\n"
        )
        document.write("}\n")

        # We implement the Filtrable trait for the struct. This trait
        # is used to provide the informations on the filter struct that
        # can be used to filter the struct in the database.
        if struct.has_filter_variant():
            filter_struct = struct.get_filter_variant()
            document.write(f"\nimpl Filtrable for {struct.name} {{\n")
            document.write(f"    type Filter = {filter_struct.name};\n")
            document.write("}\n")
        elif struct.table_name == "users":
            document.write(f"\nimpl Filtrable for {struct.name} {{\n")
            document.write("    type Filter = EmptyFilter;\n")
            document.write("}\n")

        # This variant of the struct implementation is only
        # available when in the web_common is enabled the frontend
        # feature. It provides several methods including the use
        # of GlueSQL. Fortunately, it does not force us like Diesel
        # to create yet again another duplicate of the struct.
        document.write('#[cfg(feature = "frontend")]\n')
        document.write(f"impl {struct.name} {{\n")
        columns = ", ".join([attribute.name for attribute in struct.attributes])

        # As first thing, we implement the `into_row` method for the struct. This method
        # converts the struct into a vector of `gluesql::core::ast_builder::ExprList`
        # variants, which are used to insert the struct into the GlueSQL database.

        document.write(
            "    pub fn into_row(self) -> Vec<gluesql::core::ast_builder::ExprNode<'static>> {\n"
        )

        document.write("        vec![\n")
        for attribute in struct.attributes:

            if attribute.optional:
                if attribute.data_type() in GLUESQL_TYPES_MAPPING:
                    document.write(f"            match self.{attribute.name} {{\n")
                    document.write(
                        f"                Some({attribute.name}) => {GLUESQL_TYPES_MAPPING[attribute.data_type()].format(attribute.name)},\n"
                    )
                    document.write(
                        "                None => gluesql::core::ast_builder::null(),\n"
                    )
                    document.write("            },\n")
                else:
                    raise NotImplementedError(
                        f"The type {attribute.data_type()} is not supported. "
                        f"The struct {struct.name} contains an {attribute.data_type()}. "
                    )
            elif attribute.data_type() in GLUESQL_TYPES_MAPPING:
                document.write(
                    f"            {GLUESQL_TYPES_MAPPING[attribute.data_type()].format(f'self.{attribute.name}')},\n"
                )
            else:
                raise NotImplementedError(
                    f"The type {attribute.data_type()} is not supported."
                )

        document.write("        ]\n")

        document.write("    }\n\n")

        # We implement the `insert` method for the struct. This method
        # receives a connection to the GlueSQL database and inserts the
        # struct into the database.
        document.write(f"    /// Insert the {struct.name} into the database.\n")
        document.write("    ///\n")
        document.write("    /// # Arguments\n")
        document.write("    /// * `connection` - The connection to the database.\n")
        document.write("    ///\n")
        document.write("    /// # Returns\n")
        document.write(f"    /// The number of rows inserted in table {struct.name}\n")
        document.write("    pub async fn insert<C>(\n")
        document.write("        self,\n")
        document.write("        connection: &mut gluesql::prelude::Glue<C>,\n")
        document.write("    ) -> Result<usize, gluesql::prelude::Error> where\n")
        document.write(
            "        C: gluesql::core::store::GStore + gluesql::core::store::GStoreMut,\n"
        )
        document.write("    {\n")
        document.write("        use gluesql::core::ast_builder::*;\n")
        # We use the AST builder as much as possible so to avoid SQL injection attacks.
        document.write(f'        table("{struct.table_name}")\n')
        document.write("            .insert()\n")
        document.write(f'            .columns("{columns}")\n')
        document.write("            .values(vec![self.into_row()])\n")
        document.write("            .execute(connection)\n")
        document.write("            .await\n")
        document.write("             .map(|payload| match payload {\n")
        document.write(
            "                 gluesql::prelude::Payload::Insert ( number_of_inserted_rows ) => number_of_inserted_rows,\n"
        )
        document.write(
            '                 _ => unreachable!("Payload must be an Insert"),\n'
        )
        document.write("             })\n")
        document.write("    }\n\n")

        # We implement the `get` method for the struct. This method
        # receives the ID of the struct and a connection to the GlueSQL
        # database. The method returns the struct from the database.
        document.write(f"    /// Get {struct.name} from the database by its ID.\n")
        document.write("    ///\n")
        document.write("    /// # Arguments\n")
        primary_keys = struct.get_primary_keys()

        document.write(
            f"    /// * `{struct.get_formatted_primary_keys(include_prefix=False)}` - The primary key(s) of the struct to get.\n"
        )
        document.write(
            "    /// * `connection` - The connection to the database.\n"
            "    ///\n"
            "    pub async fn get<C>(\n"
            f"        {struct.get_formatted_primary_keys(include_prefix=False)}: {struct.get_formatted_primary_key_data_types()},\n"
            "        connection: &mut gluesql::prelude::Glue<C>,\n"
            "    ) -> Result<Option<Self>, gluesql::prelude::Error> where\n"
            "        C: gluesql::core::store::GStore + gluesql::core::store::GStoreMut,\n"
            "    {\n"
            "        use gluesql::core::ast_builder::*;\n"
            f'        let select_row = table("{struct.table_name}")\n'
            "            .select()\n"
        )
        for primary_key in primary_keys:
            document.write(
                f'            .filter(col("{primary_key.name}").eq({primary_key.name}.to_string()))\n'
            )
        document.write(
            f'            .project("{columns}")\n'
            "            .limit(1)\n"
            "            .execute(connection)\n"
            "            .await?;\n"
            "         Ok(select_row.select()\n"
            "            .unwrap()\n"
            "            .map(Self::from_row)\n"
            "            .collect::<Vec<_>>()\n"
            "            .pop())\n"
            "    }\n\n"
        )

        # We implement the `delete` method for the struct. This method deletes
        # the struct from the GlueSQL database.
        document.write(
            f"    /// Delete {struct.name} from the database.\n"
            "    ///\n"
            "    /// # Arguments\n"
            f"    /// * `{struct.get_formatted_primary_keys(include_prefix=False)}` - The primary key(s) of the struct to delete.\n"
            "    /// * `connection` - The connection to the database.\n"
            "    ///\n"
            "    /// # Returns\n"
            "    /// The number of rows deleted.\n"
            "    pub async fn delete_from_id<C>(\n"
            f"        {struct.get_formatted_primary_keys(include_prefix=False)}: {struct.get_formatted_primary_key_data_types()},\n"
            "        connection: &mut gluesql::prelude::Glue<C>,\n"
            "    ) -> Result<usize, gluesql::prelude::Error> where\n"
            "        C: gluesql::core::store::GStore + gluesql::core::store::GStoreMut,\n"
            "    {\n"
            "        use gluesql::core::ast_builder::*;\n"
            f'        table("{struct.table_name}")\n'
            "            .delete()\n"
        )
        for primary_key in primary_keys:
            document.write(
                f'            .filter(col("{primary_key.name}").eq({primary_key.name}.to_string()))\n'
            )
        document.write(
            "            .execute(connection)\n"
            "            .await\n"
            "             .map(|payload| match payload {\n"
            "                 gluesql::prelude::Payload::Delete(number_of_deleted_rows) => number_of_deleted_rows,\n"
            '                 _ => unreachable!("Payload must be a Delete"),\n'
            "             })\n"
            "    }\n\n"
        )

        # We implement the `delete` method for the struct. This method deletes
        # the current instance of the struct from the GlueSQL database.
        document.write(
            f"    /// Delete the current instance of {struct.name} from the database.\n"
            "    ///\n"
            "    /// # Arguments\n"
            "    /// * `connection` - The connection to the database.\n"
            "    ///\n"
            "    /// # Returns\n"
            "    /// The number of rows deleted.\n"
            "    pub async fn delete<C>(\n"
            "        self,\n"
            "        connection: &mut gluesql::prelude::Glue<C>,\n"
            "    ) -> Result<usize, gluesql::prelude::Error> where\n"
            "        C: gluesql::core::store::GStore + gluesql::core::store::GStoreMut,\n"
            "    {\n"
            f"        Self::delete_from_id({struct.get_formatted_primary_keys(include_prefix=True)}, connection).await\n"
            "    }\n"
        )

        # We implement the `update` method for the struct. This method updates
        # the struct in the GlueSQL database.
        write_update_method_for_gluesql(struct, document)

        # Next, we implement the `update_or_insert` method for the struct. This method
        # inserts the struct into the GlueSQL database if it does not exist, otherwise
        # it updates the struct in the database.
        document.write(
            "    /// Update the struct in the database if it exists, otherwise insert it.\n"
            "    ///\n"
            "    /// # Arguments\n"
            "    /// * `connection` - The connection to the database.\n"
            "    ///\n"
            "    /// # Returns\n"
            "    /// The number of rows updated or inserted.\n"
            "    pub async fn update_or_insert<C>(\n"
            "        self,\n"
            "        connection: &mut gluesql::prelude::Glue<C>,\n"
            "    ) -> Result<usize, gluesql::prelude::Error> where\n"
            "        C: gluesql::core::store::GStore + gluesql::core::store::GStoreMut,\n"
            "    {\n"
            "        let number_of_rows = self.clone().update(connection).await?;\n"
            "        if number_of_rows == 0 {\n"
            "            self.insert(connection).await\n"
            "        } else {\n"
            "            Ok(number_of_rows)\n"
            "        }\n"
            "    }\n"
        )

        # We implement the `all` method for the struct. This method returns all of the
        # structs in the GlueSQL database.
        document.write(
            f"    /// Get all {struct.name} from the database.\n"
            "    ///\n"
            "    /// # Arguments\n"
        )
        if struct.has_filter_variant():
            document.write("    /// * `filter` - The filter to apply to the results.\n")
        document.write(
            "    /// * `limit` - The maximum number of results, by default `10`.\n"
            "    /// * `offset` - The offset of the results, by default `0`.\n"
            "    /// * `connection` - The connection to the database.\n"
            "    ///\n"
            "    pub async fn all<C>(\n"
        )
        if struct.has_filter_variant():
            filter_variant = struct.get_filter_variant()
            document.write(f"        filter: Option<&{filter_variant.name}>,\n")
        document.write(
            "        limit: Option<i64>,\n"
            "        offset: Option<i64>,\n"
            "        connection: &mut gluesql::prelude::Glue<C>,\n"
            "    ) -> Result<Vec<Self>, gluesql::prelude::Error> where\n"
            "        C: gluesql::core::store::GStore + gluesql::core::store::GStoreMut,\n"
            "    {\n"
            "        use gluesql::core::ast_builder::*;\n"
            f'        let select_row = table("{struct.table_name}")\n'
            "            .select()\n"
        )
        if struct.has_filter_variant():
            document.write(
                "            .filter(filter.map_or_else(|| gluesql::core::ast::Expr::Literal(gluesql::core::ast::AstLiteral::Boolean(true)).into(), |filter| filter.as_filter_expression()))\n"
            )
        document.write(
            f'           .project("{columns}")\n'
            "            .offset(offset.unwrap_or(0))\n"
            "            .limit(limit.unwrap_or(10))\n"
            "            .execute(connection)\n"
            "            .await?;\n"
            "        Ok(select_row.select()\n"
            "            .unwrap()\n"
            "            .map(Self::from_row)\n"
            "            .collect::<Vec<_>>())\n"
            "    }\n"
        )

        # We implement for all tables that implement the `updated_at` column
        # the `all_by_updated_at` method. This method returns all of the structs
        # in the GlueSQL database ordered by the `updated_at` column.
        if struct.table_metadata.has_updated_at_column(struct.table_name):
            document.write(
                f"    /// Get all {struct.name} from the database ordered by the `updated_at` column.\n"
                "    ///\n"
                "    /// # Arguments\n"
            )
            if struct.has_filter_variant():
                document.write(
                    "    /// * `filter` - The filter to apply to the results.\n"
                )
            document.write(
                "    /// * `limit` - The maximum number of results, by default `10`.\n"
                "    /// * `offset` - The offset of the results, by default `0`.\n"
                "    /// * `connection` - The connection to the database.\n"
                "    ///\n"
                "    pub async fn all_by_updated_at<C>(\n"
            )
            if struct.has_filter_variant():
                filter_variant = struct.get_filter_variant()
                document.write(f"        filter: Option<&{filter_variant.name}>,\n")
            document.write(
                "        limit: Option<i64>,\n"
                "        offset: Option<i64>,\n"
                "        connection: &mut gluesql::prelude::Glue<C>,\n"
                "    ) -> Result<Vec<Self>, gluesql::prelude::Error> where\n"
                "        C: gluesql::core::store::GStore + gluesql::core::store::GStoreMut,\n"
                "    {\n"
                "        use gluesql::core::ast_builder::*;\n"
                f'        let select_row = table("{struct.table_name}")\n'
                "            .select()\n"
            )
            if struct.has_filter_variant():
                document.write(
                    "            .filter(filter.map_or_else(|| gluesql::core::ast::Expr::Literal(gluesql::core::ast::AstLiteral::Boolean(true)).into(), |filter| filter.as_filter_expression()))\n"
                )
            document.write(
                f'           .project("{columns}")\n'
                '            .order_by("updated_at desc")\n'
                "            .offset(offset.unwrap_or(0))\n"
                "            .limit(limit.unwrap_or(10))\n"
                "            .execute(connection)\n"
                "            .await?;\n"
                "        Ok(select_row.select()\n"
                "            .unwrap()\n"
                "            .map(Self::from_row)\n"
                "            .collect::<Vec<_>>())\n"
                "    }\n"
            )

        # We implement the `from_row` method for the struct. This method
        # receives a row from the GlueSQL database, which is a `HashMap<&str, &&Value>`.
        # The method returns the struct from the row.
        document.write(
            "    pub fn from_row(row: std::collections::HashMap<&str, &gluesql::prelude::Value>) -> Self {\n"
        )
        document.write("        Self {\n")

        clonables = {
            "bool": "Bool",
            "i8": "I8",
            "i16": "I16",
            "i32": "I32",
            "i64": "I64",
            "i128": "I128",
            "u8": "U8",
            "u16": "U16",
            "u32": "U32",
            "u64": "U64",
            "u128": "U128",
            "f32": "F32",
            "f64": "F64",
            "String": "Str",
            "NaiveDateTime": "Timestamp",
            "Vec<u8>": "Bytea",
        }

        for attribute in struct.attributes:
            if attribute.format_data_type() == "Uuid":
                document.write(
                    f'            {attribute.name}: match row.get("{attribute.name}").unwrap() {{\n'
                    f"                gluesql::prelude::Value::Uuid({attribute.name}) => Uuid::from_u128(*{attribute.name}),\n"
                    '                _ => unreachable!("Expected Uuid"),\n'
                    "            },\n"
                )
            elif attribute.format_data_type() == "Option<Uuid>":
                document.write(
                    f'            {attribute.name}: match row.get("{attribute.name}").unwrap() {{\n'
                )
                document.write(
                    "                gluesql::prelude::Value::Null => None,\n"
                )
                document.write(
                    f"                gluesql::prelude::Value::Uuid({attribute.name}) => Some(Uuid::from_u128(*{attribute.name})),\n"
                )
                document.write('                _ => unreachable!("Expected Uuid"),\n')
                document.write("            },\n")
            elif attribute.implements_clone():
                if attribute.optional:
                    document.write(
                        f'            {attribute.name}: match row.get("{attribute.name}").unwrap() {{\n'
                    )
                    document.write(
                        "                gluesql::prelude::Value::Null => None,\n"
                    )
                    document.write(
                        f"                gluesql::prelude::Value::{clonables[attribute.data_type()]}({attribute.name}) => Some({attribute.name}.clone()),\n"
                    )
                    document.write(
                        f'                _ => unreachable!("Expected {clonables[attribute.data_type()]}")\n'
                    )
                    document.write("            },\n")
                else:
                    document.write(
                        f'            {attribute.name}: match row.get("{attribute.name}").unwrap() {{\n'
                    )
                    document.write(
                        f"                gluesql::prelude::Value::{clonables[attribute.data_type()]}({attribute.name}) => {attribute.name}.clone(),\n"
                    )
                    document.write(
                        f'                _ => unreachable!("Expected {clonables[attribute.data_type()]}")\n'
                    )
                    document.write("            },\n")
            else:
                raise NotImplementedError(
                    f"Found an unsupported attribute type for the struct {struct.name}: {attribute.data_type()} "
                    f"for the attribute {attribute.name}."
                )
        document.write("        }\n    }\n}\n")

    document.flush()
    document.close()
