"""This module contains the function to derive the New{Model} structs."""
from typing import List
from tqdm.auto import tqdm
from constraint_checkers.struct_metadata import StructMetadata

def derive_webcommon_new_variants(
    struct_metadatas: List[StructMetadata],
) -> List[StructMetadata]:
    """Returns list of the New{Model} structs.

    Parameters
    ----------
    struct_metadatas : List[StructMetadata]
        The list of the StructMetadata objects.

    Implementation details
    ----------------------
    The New variants are used to either insert data in the database or,
    when used in the frontend, employed as base for the html Forms.
    The primary difference between the New variant and the original
    variant is that the New variant does not contain the primary key
    attribute and the automatically determined columns, such as the
    created_at and updated_at columns, unless the primary key is a UUID.
    """
    # Temporary list of tables for which to refrain from generating
    # the form component.

    new_structs = []

    for struct in tqdm(
        struct_metadatas,
        desc="Deriving new structs",
        unit="struct",
        leave=False,
    ):
        if not struct.is_insertable():
            continue

        assert not struct.is_nested()

        new_struct = StructMetadata(
            struct_name=f"New{struct.name}",
            table_name=struct.table_name,
        )

        new_struct.set_flat_variant(struct)

        primary_keys = struct.get_primary_keys()

        for derive in struct.derives():
            new_struct.add_derive(derive)

        for attribute in struct.attributes:
            if attribute.is_automatically_determined_column():
                continue

            if attribute in primary_keys:
                if attribute.is_uuid():
                    new_struct.add_attribute(attribute)
                    continue
                if len(primary_keys) == 1:
                    continue
                new_struct.add_attribute(attribute)
                continue
            new_struct.add_attribute(attribute)

        assert (
            len(new_struct.attributes) > 0
        ), f"New struct {new_struct.name} has no attributes."

        new_structs.append(new_struct)

    return new_structs