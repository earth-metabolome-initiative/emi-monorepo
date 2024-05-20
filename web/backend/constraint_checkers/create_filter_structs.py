"""Submodule for creating structs using to define filter queries, both for Diesel and GlueSQL."""

from typing import List
import copy
from constraint_checkers.struct_metadata import StructMetadata, AttributeMetadata


def create_filter_structs(flat_variants: List[StructMetadata]) -> List[StructMetadata]:
    """Create structs for filter queries.
    
    Parameters
    ----------
    flat_variants : List[StructMetadata]
        List of flat variants to create filter structs for.
    """
    assert len(flat_variants) > 0
    filter_structs = []
    for flat_variant in flat_variants:
        assert not flat_variant.is_nested()
        if not flat_variant.has_foreign_keys():
            continue

        # At this time we only support filtering for foreign
        # keys so when the provided flat variant does not have
        # any foreign keys, we skip it as we cannot create a
        # filter struct for it.

        filter_struct = StructMetadata(
            flat_variant.name + "Filter",
            table_name=flat_variant.table_name,
            table_metadata=flat_variant.table_metadata,
        )

        for foreign_key in flat_variant.get_foreign_keys():
            assert isinstance(foreign_key, AttributeMetadata)
            foreign_key = copy.deepcopy(foreign_key)
            foreign_key.optional = True
            filter_struct.add_attribute(foreign_key)

        for derive in flat_variant.derives():
            filter_struct.add_derive(derive)

        filter_struct.set_flat_variant(flat_variant)
        flat_variant.set_filter_variant(filter_struct)

        filter_structs.append(filter_struct)

    # We write out the filter structs into the webcommon crate
    # in the src/filter_structs.rs file.

    document = open("../web_common/src/database/filter_structs.rs", "w", encoding="utf-8")

    imports = [
        "use serde::{Deserialize, Serialize};",
        "use uuid::Uuid;",
    ]

    document.write("\n".join(imports) + "\n\n")

    # We also define an EmptyFilter struct which is used for structs
    # that cannot be filtered, but for semantics it is easier to have
    # a filter struct for them.

    document.write(
        "#[derive(Debug, Clone, Serialize, Deserialize, Eq, PartialEq)]\n"
        "pub struct EmptyFilter;\n\n"
    )

    for filter_struct in filter_structs:
        filter_struct.write_to(document)

        # For when the frontend feature is enabled, we implement the
        # as_filter_expression method which takes an filter struct 
        # reference and returns a GlueSQL filter expression.

        document.write(
            "\n#[cfg(feature = \"frontend\")]\n"
            f"impl {filter_struct.name} {{\n\n"
            "    pub fn as_filter_expression(&self) -> gluesql::core::ast_builder::ExprNode<'_> {\n"
            "        let mut filter: gluesql::core::ast_builder::ExprNode<'_> = gluesql::core::ast::Expr::Literal(gluesql::core::ast::AstLiteral::Boolean(true)).into();\n"
        )

        for attr in filter_struct.attributes:
            document.write(
                f"        if let Some({attr.name}) = &self.{attr.name} {{\n"
                f"            filter = filter.and(gluesql::core::ast_builder::col(\"{filter_struct.table_name}.{attr.name}\").eq({attr.name}.to_string()));\n"
                "        }\n\n"
            )

        document.write(
            "        filter\n"
            "    }\n"
            "}\n\n"
        )

    return filter_structs
