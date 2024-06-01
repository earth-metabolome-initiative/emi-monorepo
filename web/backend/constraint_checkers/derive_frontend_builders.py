"""This module contains the implementation of the derive_frontend_builders function."""
from typing import List
from tqdm.auto import tqdm
from constraint_checkers.struct_metadata import StructMetadata, AttributeMetadata

def derive_frontend_builders(
    new_or_update_struct_metadatas: List[StructMetadata],
) -> List[StructMetadata]:
    """Returns list of the {struct_name}Builder structs.

    Parameters
    ----------
    new_or_update_struct_metadatas : List[StructMetadata]
        The list of the StructMetadata objects.
        This list includes both the New and Update variants.

    Implementation details
    ----------------------
    The {struct_name}Builder structs are used to build either the New{struct_name} or Update{stuct_name} structs.
    Since they are builders, they contain option variants of the attributes
    of the {struct_name} structs. When the original attribute is already optional (Option<T>)
    the {struct_name}Builder attribute is not doubly optional (Option<Option<T>>), but simply optional
    as the original attribute (Option<T>).
    """
    assert isinstance(new_or_update_struct_metadatas, list)
    assert len(new_or_update_struct_metadatas) > 0
    assert all(
        isinstance(struct, StructMetadata) for struct in new_or_update_struct_metadatas
    )
    assert all(
        struct.is_new_variant() or struct.is_update_variant()
        for struct in new_or_update_struct_metadatas
    )
    assert all(
        struct.has_flat_variant()
        for struct in new_or_update_struct_metadatas
    )

    builders = []

    deny_list_tables = [
        "user_emails",
    ]

    for struct in tqdm(
        new_or_update_struct_metadatas,
        desc="Deriving builders",
        unit="struct",
        leave=False,
    ):
        assert not struct.is_nested()
        assert struct.is_new_variant() or struct.is_update_variant()

        if struct.table_name in deny_list_tables:
            continue

        if struct.is_update_variant() and not struct.is_new_variant():
            found = False
            for builder in builders:
                if builder.table_name == struct.table_name:
                    found = True
                    builder.set_update_variant(struct)
                    break

            if found:
                continue

        flat_variant = struct.get_flat_variant()
        richest_variant = struct.get_richest_variant()

        builder = StructMetadata(
            struct_name=f"{flat_variant.name}Builder", table_name=struct.table_name
        )

        builder.set_flat_variant(flat_variant)
        if richest_variant.is_nested():
            builder.set_richest_variant(richest_variant)

        if struct.is_new_variant():
            builder.set_new_variant(struct)

        if struct.is_update_variant():
            builder.set_update_variant(struct)

        builder.add_derive("Store")
        for derive in richest_variant.derives():
            builder.add_derive(derive)

        builder.add_decorator('store(storage = "local", storage_tab_sync)')

        foreign_keys = flat_variant.get_foreign_keys()
        primary_keys = flat_variant.get_primary_keys()

        for primary_key in primary_keys:
            if primary_key not in foreign_keys:
                builder.add_attribute(
                    AttributeMetadata(
                        original_name=primary_key.original_name,
                        name=primary_key.name,
                        data_type=primary_key.raw_data_type(),
                        optional=True,
                    )
                )

        for attribute in flat_variant.attributes:
            if attribute in foreign_keys or attribute in primary_keys:
                continue

            if attribute.is_automatically_determined_column():
                continue

            if not attribute.implements_copy():
                attribute = attribute.as_rc()

            builder.add_attribute(attribute.as_option())

        if len(foreign_keys) > 0:
            assert richest_variant.is_nested()

        if richest_variant.is_nested():
            for attribute in richest_variant.attributes:

                if attribute.name == "inner":
                    continue

                if attribute.is_automatically_determined_column():
                    continue

                if attribute.data_type() == flat_variant.name:
                    builder.add_attribute(
                        AttributeMetadata(
                            original_name=attribute.original_name,
                            name=attribute.name,
                            data_type=richest_variant,
                            optional=True,
                            rc=True
                        )
                    )
                    continue

                builder.add_attribute(attribute.as_option())

        # For each attribute, for add new parameters that are vectors
        # for ApiError and collect the errors associated to the specific
        # attribute. This is useful for the frontend, where we can display
        # the errors associated to the specific attribute. In order to
        # easily distinguish these fields from the other fields and avoid
        # name clashes, we prefix the attribute name with `errors_`.
        new_attributes = []
        for attribute in builder.attributes:
            if attribute in primary_keys:
                continue

            new_attributes.append(
                AttributeMetadata(
                    original_name=attribute.original_name,
                    name=f"errors_{attribute.name}",
                    data_type="Vec<ApiError>",
                    optional=False,
                )
            )

        for attribute in new_attributes:
            builder.add_attribute(attribute)

        # Finally, we add an attribute to the builder to be used primarily
        # by yew to detect when the object has been updated. This is needed
        # for cases when the attempted update is invalid, and the data inserted
        # in the form needs to be resetted to the original values. In this attribute
        # we store the datetime of the last update of the object.
        builder.add_attribute(
            AttributeMetadata(
                original_name="form_updated_at",
                name="form_updated_at",
                data_type="NaiveDateTime",
                optional=False,
            )
        )

        # Self-consistency check - all of the foreign keys must appear in the builder,
        # eventually in the normalized version.

        for foreign_key in foreign_keys:

            if foreign_key.is_automatically_determined_column():
                continue

            found = False
            for attribute in builder.attributes:
                if attribute.original_name == foreign_key.name:
                    found = True
                    break

            if not found:
                raise RuntimeError(
                    f"Could not find the foreign key {foreign_key.name} in the builder {builder.name}. "
                    f"The attributes of the flat variant are {flat_variant.attributes}. "
                    f"The attributes of the richest variant are {richest_variant.attributes}."
                )

        builders.append(builder)

    # We run a simple self-consistency check to ensure that the
    # builders have been correctly derived.
    for struct in new_or_update_struct_metadatas:

        if struct.table_name in deny_list_tables:
            continue

        # We identify the curresponding builder by the matching table name.
        found = False
        for builder in builders:
            if builder.table_name == struct.table_name:
                found = True
                break

        if not found:
            raise RuntimeError(
                f"Could not find the builder for the struct {struct.name}."
            )

        if struct.is_new_variant():
            assert builder.get_new_variant() == struct

        if struct.is_update_variant():
            assert builder.get_update_variant() == struct

    return builders