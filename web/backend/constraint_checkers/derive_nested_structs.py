"""Derive the nested structs from the struct metadata."""

from typing import List
from tqdm import tqdm
from constraint_checkers.struct_metadata import StructMetadata, AttributeMetadata

def derive_nested_structs(
    struct_metadatas: List[StructMetadata],
) -> List[StructMetadata]:
    """Generate the nested structs.

    Implementative details
    -------------------------
    Normally, a table struct is generated from a row in the database. However, in some cases,
    a table row may contain a reference id to another table. In this case, we generate a nested
    struct for the referenced table. Depending on whether this referenced row contains also a
    reference to another table, we may generate the nested struct version of the referenced row
    or the flat version, i.e. the row itself.

    For each table, we query the postgres to get the foreign keys. We then generate the nested
    structs for the referenced tables. The nested structs are written to the file `src/models.rs`.
    """

    def get_struct_by_table_name(table_name: str) -> StructMetadata:
        for struct in struct_metadatas:
            if struct.table_name == table_name:
                return struct
        raise ValueError(f"Table name {table_name} not found in the struct metadata.")

    nested_structs = []

    # For each of the struct, we generated the Nested{struct_name} version
    # if the struct contains a reference to another struct.
    for struct in tqdm(
        struct_metadatas, desc="Generating nested structs", leave=False, unit="struct"
    ):
        foreign_keys = struct.get_foreign_keys()

        if len(foreign_keys) == 0:
            continue

        nested_struct = StructMetadata(f"Nested{struct.name}", struct.table_name)
        nested_struct.set_flat_variant(struct)

        if not struct.has_only_foreign_keys():
            nested_struct.add_attribute(
                AttributeMetadata(
                    original_name="inner",
                    name="inner",
                    data_type=struct,
                    optional=False,
                )
            )

        for attribute in foreign_keys:
            # If the current attribute is among the foreign keys, we replace it
            # with the foreign struct. This struct may be also nested if the foreign
            # table has foreign keys, which we check by using the `has_foreign_keys`
            # method of the `tables_metadata` object.
            foreign_key_table_name = struct.table_metadata.get_foreign_key_table_name(
                struct.table_name, attribute.name
            )

            normalized_attribute_name = attribute.normalized_name()
            foreign_struct = get_struct_by_table_name(foreign_key_table_name)

            if (
                struct.name == foreign_struct.name
                or not foreign_struct.has_foreign_keys()
            ):
                nested_struct.add_attribute(
                    AttributeMetadata(
                        original_name=attribute.name,
                        name=normalized_attribute_name,
                        data_type=foreign_struct,
                        optional=attribute.optional,
                    )
                )
            else:
                nested_struct.add_attribute(
                    AttributeMetadata(
                        original_name=attribute.name,
                        name=normalized_attribute_name,
                        data_type=f"Nested{foreign_struct.name}",
                        optional=attribute.optional,
                    )
                )

        foreign_keys = nested_struct.get_foreign_keys()
        for foreign_key in foreign_keys:
            assert any(
                attribute.original_name == foreign_key.name
                for attribute in nested_struct.attributes
            ), (
                f"Foreign key {foreign_key.name} not found in the nested struct {nested_struct.name}. "
                f"The struct has the following attributes: {nested_struct.attributes}."
            )

        nested_structs.append(nested_struct)

    # We replace until convergence the data type of the structs with the structs themselves.
    # This is necessary as the nested structs may contain references to other structs, which
    # in turn may contain references to other structs and so on.

    changed = True

    while changed:
        changed = False
        updated_struct_metadatas = []
        for nested_struct in nested_structs:
            if not nested_struct.has_undefined_nested_dependencies():
                updated_struct_metadatas.append(nested_struct)
                continue
            new_attributes = []
            converged = True
            for attribute in nested_struct.attributes:
                if attribute.is_undefined_nested_dependencies():
                    struct_identified = False
                    for struct in nested_structs + struct_metadatas:
                        if struct.name == attribute.data_type():
                            struct_identified = True
                            if struct.has_undefined_nested_dependencies():
                                converged = False
                                continue
                            new_attributes.append(
                                AttributeMetadata(
                                    original_name=attribute.original_name,
                                    name=attribute.name,
                                    data_type=struct,
                                    optional=attribute.optional,
                                )
                            )
                            changed = True
                            break
                    if not struct_identified:
                        raise RuntimeError(
                            f"Could not identify the struct with the name {attribute.data_type()}. "
                            f"Found while processing the nested struct {nested_struct.name}."
                        )
                else:
                    new_attributes.append(attribute)
            if converged:
                assert len(new_attributes) == len(nested_struct.attributes), (
                    f"Expected the length of the new attributes to be equal to the length of the old attributes. "
                    f"Expected {len(new_attributes)}, got {len(nested_struct.attributes)}."
                )
                nested_struct.attributes = []
                for new_attribute in new_attributes:
                    nested_struct.add_attribute(new_attribute)
            updated_struct_metadatas.append(nested_struct)
        nested_structs = updated_struct_metadatas

    # Consistency check:
    # We check that all the nested struct contain the foreign keys
    # present in the original struct.
    for nested_struct in nested_structs:
        assert nested_struct.is_nested()
        foreign_keys = nested_struct.get_foreign_keys()
        for foreign_key in foreign_keys:
            assert any(
                attribute.original_name == foreign_key.name
                for attribute in nested_struct.attributes
            ), (
                f"Foreign key {foreign_key.name} not found in the nested struct {nested_struct.name}. "
                f"The struct has the following attributes: {nested_struct.attributes}."
            )

    return nested_structs
