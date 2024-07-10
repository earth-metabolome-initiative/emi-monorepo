"""This module contains the function to extract the structs from a Rust file."""
from typing import List
from tqdm.auto import tqdm
from constraint_checkers.struct_metadata import StructMetadata, AttributeMetadata
from constraint_checkers.find_foreign_keys import postgres_type_to_rust_type
import inflect

inflect_engine = inflect.engine()

IGNORED_TABLES = [
    "spatial_ref_sys",
    "__diesel_schema_migrations"
]

def struct_name_from_table_name(table_name: str) -> str:
    """Convert a table name to a struct name."""
    last_term = table_name.split("_")[-1]
    singularized_last_term = inflect_engine.singular_noun(last_term)

    if last_term == "taxa":
        singularized_last_term = "taxon"
    if last_term == "spectra":
        singularized_last_term = "spectrum"

    if not singularized_last_term:
        raise ValueError(f"Could not singularize the last term of the table name: {table_name}")
    
    return "".join(
        [
            term.capitalize()
            for term in table_name.split("_")[:-1]
        ]
    ) + singularized_last_term.capitalize()

def extract_structs() -> List[StructMetadata]:
    """Extract the structs from the Rust file at the given path.

    Parameters
    ----------
    path : str
        The path to the Rust file.
    """
    
    struct_metadatas: List[StructMetadata] = []

    for table_name in tqdm(
        StructMetadata.table_metadata.all_tables(),
        leave=False,
        desc="Extracting structs",
        unit="table",
    ):
        if table_name in IGNORED_TABLES:
            continue
        struct_name = struct_name_from_table_name(table_name)
        struct_metadata = StructMetadata(
            table_name=table_name,
            struct_name=struct_name,
        )

        unique_constraints: List[List[str]] = StructMetadata.table_metadata.get_unique_constraint_columns(
            struct_metadata.table_name,
        )

        for column in StructMetadata.table_metadata.extract_table_columns(struct_metadata.table_name):
            struct_metadata.add_attribute(
                AttributeMetadata(
                    original_name=column.column_name,
                    name=column.column_name,
                    data_type=postgres_type_to_rust_type(column.data_type),
                    optional=column.nullable,
                    unique=any([
                        unique_constraint[0] == column.column_name
                        for unique_constraint in unique_constraints
                        if len(unique_constraints) == 1
                    ])
                )
            )
        
        struct_metadata.table_metadata.register_flat_variant(
            struct_metadata.table_name, struct_metadata
        )
        struct_metadatas.append(struct_metadata)

    return struct_metadatas