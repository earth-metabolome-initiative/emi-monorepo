"""This module contains the function to derive the Update{Model} structs."""
from typing import List
from tqdm.auto import tqdm
from constraint_checkers.struct_metadata import StructMetadata

def derive_webcommon_update_variants(
    struct_metadatas: List[StructMetadata],
) -> List[StructMetadata]:
    """Returns list of the Update{Model} structs.

    Parameters
    ----------
    struct_metadatas : List[StructMetadata]
        The list of the StructMetadata objects.

    Implementation details
    ----------------------
    The Update variants are used to update data in the database.
    The primary difference between the Update variant and the original
    variant is that the Update variant does not contain columns relative
    to the automatically determined columns, such as the created_at and
    updated_at columns. It does contain the primary key column, and as
    such in the case of a UUID primary key, the Update variant is for all
    intents and purposes identical to the New variant, and as such there
    is no need to derive an Update variant.
    """
    update_structs = []

    for struct in tqdm(
        struct_metadatas,
        desc="Deriving update structs",
        unit="struct",
        leave=False,
    ):
        if not struct.is_updatable():
            continue

        assert not struct.is_nested()

        # If the struct has a single attribute,
        # we do not derive an Update variant.
        if len(struct.attributes) == 1:
            continue

        primary_keys = struct.get_primary_keys()

        if len(primary_keys) == 1 and primary_keys[0].is_uuid():
            continue

        update_struct = StructMetadata(
            struct_name=f"Update{struct.name}",
            table_name=struct.table_name,
        )

        update_struct.set_flat_variant(struct)

        for derive in struct.derives():
            update_struct.add_derive(derive)

        for attribute in struct.attributes:
            if attribute.is_automatically_determined_column():
                continue
            update_struct.add_attribute(attribute.into_new_owner(update_struct))

        assert (
            len(update_struct.attributes) > 0
        ), f"Update struct {update_struct.name} has no attributes."

        update_structs.append(update_struct)

    return update_structs