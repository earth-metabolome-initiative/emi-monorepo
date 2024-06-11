"""Submodule writing frontend pages to the filesystem."""

from typing import List
from tqdm.auto import tqdm
from constraint_checkers.struct_metadata import StructMetadata
from constraint_checkers.regroup_tables import SUPPORT_TABLE_NAMES
from constraint_checkers.is_file_changed import is_file_changed
from constraint_checkers.migrations_changed import are_migrations_changed


def is_deny_listed(struct: StructMetadata) -> bool:
    """Check whether we skip the current section."""
    return struct.table_name in SUPPORT_TABLE_NAMES


def write_frontend_pages(flat_variants: List[StructMetadata]):
    """Write frontend pages to the filesystem.

    Parameters
    ----------
    flat_variants: List[StructMetadata]
        The list of flat variants to build the frontend pages from.
    """
    assert isinstance(
        flat_variants, list
    ), "The flat_variants parameter must be a list."
    assert all(
        isinstance(flat_variant, StructMetadata) for flat_variant in flat_variants
    ), "All elements in the flat_variants list must be of type StructMetadata."

    if not (are_migrations_changed() or is_file_changed(__file__)):
        print("No change in migrations or file. Skipping writing frontend pages.")
        return

    document = open("../frontend/src/pages/automatic_pages.rs", "w", encoding="utf-8")

    imports = [
        "use yew::prelude::*;",
        "use web_common::database::*;",
        "use crate::components::*;",
    ]

    document.write("\n".join(imports) + "\n\n")

    # An automatic page is BasicPage Component with zero or more children
    # which are represented by BasicList Components receiving as filter the
    # parent struct primary key associated to the child struct foreign key.

    number_of_built_pages = 0

    for flat_variant in tqdm(
        flat_variants, desc="Writing frontend pages", unit="page", leave=False
    ):

        if is_deny_listed(flat_variant):
            continue

        if not flat_variant.is_searchable():
            continue

        has_content = False

        richest_variant = flat_variant.get_richest_variant()
        primary_keys = flat_variant.get_primary_keys()

        component_name = f"{flat_variant.name}Page"
        function_component_name = (
            flat_variant.human_readable_name().replace(" ", "_").lower() + "_page"
        )

        document.write(
            "#[derive(Clone, PartialEq, Properties)]\n"
            f"pub struct {component_name}Prop {{\n"
        )
        for primary_key in flat_variant.get_primary_keys():
            document.write(f"    pub {primary_key.name}: {primary_key.data_type(route='frontend')},\n")

        document.write("}\n\n")

        # We implement the From<&{component_name}Prop> for PrimaryKey.

        document.write(
            f"impl From<&{component_name}Prop> for PrimaryKey {{\n"
            f"    fn from(prop: &{component_name}Prop) -> Self {{\n"
            f"        {flat_variant.get_formatted_primary_keys(include_prefix=True, prefix='prop')}.into()\n"
            "    }\n"
            "}\n\n"
        )

        if len(primary_keys) == 1:
            primary_key = primary_keys[0]
            document.write(f"impl {component_name}Prop {{\n")

            for (
                _,
                foreign_key,
            ), child_struct in flat_variant.get_child_structs().items():
                # For each of the child struct, we need to implement the From trait
                # to convert the {component_name}Prop struct into their respective
                # filter struct.
                assert (
                    child_struct.has_filter_variant()
                ), f"Child struct {child_struct.name} does not have a filter variant."
                filter_variant = child_struct.get_filter_variant()

                if is_deny_listed(child_struct):
                    continue

                document.write(
                    f"    fn filter_{child_struct.table_name}_by_{foreign_key.name}(&self) -> {filter_variant.name} {{\n"
                    f"        let mut filter = {filter_variant.name}::default();\n"
                    f"        filter.{foreign_key.name} = Some(self.{primary_key.name});\n"
                    "        filter\n"
                    "    }\n"
                )

            document.write("}\n\n")

        document.write(
            f"#[function_component({component_name})]\n"
            f"pub fn {function_component_name}(props: &{component_name}Prop) -> Html {{\n"
            "    html! {\n"
            f"        <BasicPage<{richest_variant.name}> id={{PrimaryKey::from(props)}}>\n"
        )

        if len(primary_keys) == 1:
            for (
                _,
                foreign_key,
            ), child_struct in flat_variant.get_child_structs().items():
                assert (
                    child_struct.has_filter_variant()
                ), f"Child struct {child_struct.name} does not have a filter variant."

                if is_deny_listed(child_struct):
                    continue

                if not child_struct.is_searchable():
                    continue

                has_content = True

                document.write(
                    f"            // Linked with foreign key {child_struct.table_name}.{foreign_key.name}\n"
                    f"            <BasicList<{child_struct.name}> column_name={{\"{foreign_key.name}\"}} filters={{props.filter_{child_struct.table_name}_by_{foreign_key.name}()}}/>\n"
                )

        if not has_content:
            document.write('            <span>{"No content available yet."}</span>\n')

        document.write(
            f"        </BasicPage<{richest_variant.name}>>\n" "    }\n" "}\n\n"
        )

        number_of_built_pages += 1

    document.close()

    print(f"Built {number_of_built_pages} frontend pages.")
