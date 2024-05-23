"""This module contains the function to extract the structs from a Rust file."""
from typing import List
from constraint_checkers.struct_metadata import StructMetadata, AttributeMetadata

def extract_structs(path: str) -> List[StructMetadata]:
    """Extract the structs from the Rust file at the given path.

    Parameters
    ----------
    path : str
        The path to the Rust file.
    """
    # A dictionary to store the table names and their
    # respective structs.
    struct_metadatas: List[StructMetadata] = []
    derives = []
    last_table_name = None
    inside_struct = False

    with open(path, "r", encoding="utf8") as file:
        document = file.read()

    for line in document.split("\n"):
        # We skip all lines beginning with `//!` as they are comments
        if line.startswith("//!"):
            continue

        # We find the table name by searching lines like
        # #[diesel(table_name = item_continuous_quantities)]
        if "table_name =" in line:
            last_table_name = line.split("=")[1].strip(" )]").split(":")[-1]

        # If we are in a derive line, we extract the derives:
        if line.startswith("#[derive("):
            derives = line.split("(")[1].strip(")]").split(", ")

        # We determine whether a new struct has started
        # by checking if the `struct` keyword is present
        # in the line.
        if "struct" in line:
            struct_name = line.split(" ")[2]

            struct_metadata = StructMetadata(
                table_name=last_table_name,
                struct_name=struct_name,
            )

            for derive in derives:
                struct_metadata.add_derive(derive)

            inside_struct = True

        if inside_struct:
            # If the current line contains the id field,
            # we store the type of the id field.
            if "pub" in line and ":" in line:
                field_name = line.strip().split(" ")[1].strip(":")
                field_type = line.split(":")[1].strip(", ")
                option = False
                if field_type.startswith("Option<"):
                    option = True
                    field_type = field_type[7:-1]
                struct_metadata.add_attribute(
                    AttributeMetadata(
                        original_name=field_name,
                        name=field_name,
                        data_type=field_type,
                        optional=option,
                    )
                )

            # We determine whether the struct has ended
            # by checking if the `}` keyword is present
            # in the line.
            if "}" in line:
                inside_struct = False
                struct_metadata.table_metadata.register_flat_variant(
                    struct_metadata.table_name, struct_metadata
                )
                struct_metadatas.append(struct_metadata)

    return struct_metadatas