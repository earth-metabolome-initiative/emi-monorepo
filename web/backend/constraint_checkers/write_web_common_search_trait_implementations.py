"""This module contains the function that writes the Searchable trait implementations for the structs."""
import os
from typing import List
from constraint_checkers.struct_metadata import StructMetadata
from constraint_checkers.is_file_changed import is_file_changed
from constraint_checkers.migrations_changed import are_migrations_changed


def write_web_common_search_trait_implementations(
    struct_metadatas: List[StructMetadata],
):
    """Writes the Searchable trait implementations for the structs.

    Parameters
    ----------
    struct_metadatas : List[StructMetadata]
        The list of the StructMetadata objects.

    """
    if not (are_migrations_changed() or is_file_changed(__file__)):
        print("No change in migrations or file. Skipping writing frontend search trait implementations.")
        return
    
    # We check that we are currently executing in the `backend` crate
    # so to make sure that the relative path to the `web_common` crate
    # is correct.
    if not os.getcwd().endswith("backend"):
        raise RuntimeError("This script must be executed in the `backend` crate.")

    document = open("../web_common/src/database/search_tables.rs", "w", encoding="utf8")

    imports = [
        "use crate::database::*;",
        "use std::rc::Rc;",
        "use serde::{Serialize, Deserialize};",
    ]

    # Preliminarly, we write a docstring at the very head
    # of this submodule to explain what it does and warn the
    # reader not to write anything in this file as it is
    # automatically generated.

    document.write(
        "//! This module contains the table names enumeration.\n"
        "//!\n"
        "//! This module is automatically generated. Do not write anything here.\n\n"
    )

    document.write("\n".join(imports) + "\n\n")

    # First, we create the Searchable trait that will be implemented by all the structs
    # that are searchable.

    document.write(
        "pub trait Searchable<const EDIT: bool>: Filtrable {\n"
        "    fn search_task(filter: Option<&Self::Filter>, query: String, limit: i64, offset: i64) -> super::Select;\n"
        "}\n"
    )

    for struct in struct_metadatas:
        if struct.is_searchable():
            document.write(
                f"impl Searchable<false> for {struct.name} {{\n"
                "    fn search_task(filter: Option<&Self::Filter>, query: String, limit: i64, offset: i64) -> super::Select {\n"
                "        super::Select::search(\n"
                f"             Table::{struct.capitalized_table_name()},\n"
                "              filter,\n"
                "              query,\n"
                "              limit,\n"
                "              offset,\n"
                "        )\n"
                "    }\n"
                "}\n"
            )

            if struct.is_updatable():
                document.write(
                    f"impl Searchable<true> for {struct.name} {{\n"
                    "    fn search_task(filter: Option<&Self::Filter>, query: String, limit: i64, offset: i64) -> super::Select {\n"
                    "        super::Select::search_updatables(\n"
                    f"             Table::{struct.capitalized_table_name()},\n"
                    "              filter,\n"
                    "              query,\n"
                    "              limit,\n"
                    "              offset,\n"
                    "        )\n"
                    "    }\n"
                    "}\n"
                )

    # Next, we implement the SearchableStruct enumeration, which contains all the structs
    # that are searchable, and we implement the Searchable trait for this enumeration.
    # This boxed variant of the Searchable trait is used to call the search_task method
    # on all structs at once.
    document.write(
        "#[derive(Debug, Clone, PartialEq, PartialOrd, Serialize, Deserialize)]\n"
        "pub enum SearchableStruct {\n"
    )

    enum_variants = []

    for struct in struct_metadatas:
        if struct.is_searchable():
            if struct.is_nested():
                richest_variant = struct
            else:
                richest_variant = struct.get_richest_variant()
                if richest_variant.is_nested():
                    continue
            enum_variants.append((richest_variant.name, struct.name))
            document.write(
                f"    {struct.name}(Rc<{richest_variant.name}>),\n"
            )

    document.write(
        "}\n\n"
    )

    # Implements the Filtrable trait for the SearchableStruct enumeration.
    # In this case, this function is functionally non-filtrable, so it
    # uses the EmptyFilter struct.

    document.write(
        "impl Filtrable for SearchableStruct {\n"
        "    type Filter = EmptyFilter;\n"
        "}\n"
    )

    document.write(
        "impl Searchable<false> for SearchableStruct {\n"
        "    fn search_task(_filter: Option<&Self::Filter>, query: String, limit: i64, _offset: i64) -> super::Select {\n"
        "        super::Select::search_all(\n"
        "            query,\n"
        "            limit,\n"
        "        )\n"
        "    }\n"
        "}\n"
    )

    # We implement the From trait for the SearchableStruct enumeration
    # for each of the structs that are searchable.
    for richest_variant_name, struct_name in enum_variants:
        document.write(
            f"impl From<{richest_variant_name}> for SearchableStruct {{\n"
            f"    fn from(value: {richest_variant_name}) -> Self {{\n"
            f"        SearchableStruct::{struct_name}(Rc::from(value))\n"
            "    }\n"
            "}\n"
        )

    # We implement web_common::database::Describable

    document.write(
        "impl Describable for SearchableStruct {\n"
        "    fn description(&self) -> Option<&str> {\n"
        "        match self {\n"
    )

    for _, struct_name in enum_variants:
        document.write(
            f"            SearchableStruct::{struct_name}(value) => value.description(),\n"
        )

    document.write(
        "        }\n"
        "    }\n"
        "}\n"
    )

    # We implement web_common::database::Colorable

    document.write(
        "impl Colorable for SearchableStruct {\n"
        "    fn color(&self) -> Option<&str> {\n"
        "        match self {\n"
    )

    for _, struct_name in enum_variants:
        document.write(
            f"            SearchableStruct::{struct_name}(value) => value.color(),\n"
        )

    document.write(
        "        }\n"
        "    }\n"
        "}\n"
    )

    document.flush()
    document.close()

    document = open("../frontend/src/components/badge/searchable_struct.rs", "w", encoding="utf8")

    imports = [
        "use super::{BadgeProps, RowToBadge};",
        "use web_common::database::*;",
        "use crate::router::AppRoute;",
        "use yew::prelude::*;"
    ]

    document.write(
        "//! This module contains the RowToBadge implementation for the SearchableStruct enumeration.\n"
        "//!\n"
        "//! This module is automatically generated. Do not write anything here.\n\n"
    )

    document.write("\n".join(imports) + "\n\n")

    document.write(
        "impl RowToBadge for SearchableStruct {\n"
        "    fn similarity_score<S: AsRef<str>>(&self, query: S) -> isize {\n"
        "        match self {\n"
    )

    for _, struct_name in enum_variants:
        document.write(
            f"            SearchableStruct::{struct_name}(value) => value.similarity_score(query),\n"
        )

    document.write(
        "        }\n"
        "    }\n\n"
    )

    document.write(
        "    fn badge_title(&self) -> String {\n"
        "        match self {\n"
    )

    for _, struct_name in enum_variants:
        document.write(
            f"            SearchableStruct::{struct_name}(value) => value.badge_title(),\n"
        )

    document.write(
        "        }\n"
        "    }\n\n"
    )

    document.write(
        "    fn path(&self) -> Option<AppRoute> {\n"
        "        match self {\n"
    )

    for _, struct_name in enum_variants:
        document.write(
            f"            SearchableStruct::{struct_name}(value) => value.path(),\n"
        )

    document.write(
        "        }\n"
        "    }\n\n"
    )

    document.write(
        "    fn primary_image_url(&self) -> Option<String> {\n"
        "        match self {\n"
    )

    for _, struct_name in enum_variants:
        document.write(
            f"            SearchableStruct::{struct_name}(value) => value.primary_image_url(),\n"
        )

    document.write(
        "        }\n"
        "    }\n\n"
    )

    document.write(
        "    fn font_awesome_icon(&self) -> Option<&str> {\n"
        "        match self {\n"
    )

    for _, struct_name in enum_variants:
        document.write(
            f"            SearchableStruct::{struct_name}(value) => value.font_awesome_icon(),\n"
        )

    document.write(
        "        }\n"
        "    }\n\n"
    )

    document.write(
        "    fn children(&self, props: &BadgeProps<Self>) -> Option<Html> {\n"
        "        match self {\n"
    )

    for richest_variant_name, struct_name in enum_variants:
        document.write(
            f"            SearchableStruct::{struct_name}(value) => value.children(&props.to_child_props(value.clone())),\n"
        )

    document.write(
        "        }\n"
        "    }\n"
        "}\n\n"
    )

    document.flush()
    document.close()