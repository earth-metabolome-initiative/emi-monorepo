"""This module contains functions for finding derived indices."""
from typing import List, Optional
from tqdm.auto import tqdm
from constraint_checkers.struct_metadata import StructMetadata
from constraint_checkers.indices import PGIndex, DerivedPGIndex, NON_SEARCHABLE_THIRD_PARTY_TABLES


def register_derived_search_indices(
    flat_variants: List[StructMetadata],
):
    """Determines the list of existing derived indices.
    
    Parameters
    ----------
    flat_variants : List[StructMetadata]
        The list of flat variants.
    pg_indices : PGIndices
        The list of existing indices.
    """

    number_of_first_order_derived_indices = 0

    number_of_primary_indices = 0

    for flat_variant in tqdm(
        flat_variants,
        desc="Finding derived search indices",
        unit="flat variant",
        leave=False,
    ):
        # We search and register search indices directly related to the flat variant.
        search_index: Optional[PGIndex] = StructMetadata.pg_indices.get_table_index(flat_variant.table_name)
        if search_index is not None:
            number_of_primary_indices += 1
            flat_variant.set_primary_search_index(search_index)
            continue

        # We iter the foreign keys associated to the flat variant.
        for foreign_key in flat_variant.get_foreign_keys():
            # We get the struct associated to the foreign key.
            foreign_key_struct = flat_variant.get_foreign_key_flat_variant(
                foreign_key
            )

            if foreign_key_struct == flat_variant:
                continue

            if foreign_key_struct.table_name in NON_SEARCHABLE_THIRD_PARTY_TABLES:
                continue

            # If there is an index associated to the foreign key,
            # we add it to the dictionary.
            foreign_key_index = StructMetadata.pg_indices.get_table_index(
                foreign_key_struct.table_name
            )
            if foreign_key_index is not None:
                number_of_first_order_derived_indices += 1
                flat_variant.register_first_order_derived_search_index(
                    DerivedPGIndex(
                        table_name=flat_variant.table_name,
                        foreign_key_id=foreign_key.name,
                        index=foreign_key_index,
                        optional=foreign_key.optional,
                        unique=foreign_key.unique
                    )
                )

    number_of_second_order_derived_indices = 0

    for flat_variant in tqdm(
        flat_variants,
        desc="Finding derived search indices",
        unit="flat variant",
        leave=False,
    ):
        for foreign_key in flat_variant.get_foreign_keys():
            # We get the struct associated to the foreign key.
            foreign_key_struct = flat_variant.get_foreign_key_flat_variant(
                foreign_key
            )

            if foreign_key_struct == flat_variant:
                continue

            if flat_variant.has_first_order_derived_search_index_for_foreign_key(foreign_key):
                continue

            # We get the first order derived search index associated to the foreign key.
            for first_order_derived_search_index in foreign_key_struct.get_first_order_derived_search_indices():
                if first_order_derived_search_index.table_name in NON_SEARCHABLE_THIRD_PARTY_TABLES:
                    continue

                number_of_second_order_derived_indices += 1
                flat_variant.register_second_order_derived_search_index(
                    DerivedPGIndex(
                        table_name=flat_variant.table_name,
                        foreign_key_id=foreign_key.name,
                        index=first_order_derived_search_index,
                        optional=foreign_key.optional,
                        unique=foreign_key.unique
                    )
                )
    
    print(f"Found {number_of_primary_indices} primary search indices.")
    print(f"Derived {number_of_first_order_derived_indices} first order derived search indices.")
    print(f"Derived {number_of_second_order_derived_indices} second order derived search indices.")