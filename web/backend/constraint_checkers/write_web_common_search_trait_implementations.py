"""This module contains the function that writes the Searchable trait implementations for the structs."""
import os
from typing import List
from constraint_checkers.struct_metadata import StructMetadata

def write_web_common_search_trait_implementations(
    struct_metadatas: List[StructMetadata],
):
    """Writes the Searchable trait implementations for the structs.

    Parameters
    ----------
    struct_metadatas : List[StructMetadata]
        The list of the StructMetadata objects.

    """
    # We check that we are currently executing in the `backend` crate
    # so to make sure that the relative path to the `web_common` crate
    # is correct.
    if not os.getcwd().endswith("backend"):
        raise RuntimeError("This script must be executed in the `backend` crate.")

    document = open("../web_common/src/database/search_tables.rs", "w", encoding="utf8")

    imports = [
        "use crate::database::*;",
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

    document.flush()
    document.close()