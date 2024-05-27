"""Submodule writing frontend router page to the filesystem."""

from typing import List
from constraint_checkers.struct_metadata import StructMetadata, AttributeMetadata


def write_frontend_sidebar(builders: List[StructMetadata]):
    """Writes out the frontend sidebar.

    Parameters
    ----------
    builders : List[StructMetadata]
        The list of structs to write out.

    Implementative details
    ----------------------
    This function writes out the frontend sidebar to the filesystem.
    The sidebar contains:
    * The sidebar component, which is a list of links to the different pages.
    """

    document = open("../frontend/src/components/sidebar.rs", "w", encoding="utf-8")

    # Preliminarly, we write out a comment that warns the
    # reader that this page is auto-generated and as such
    # it would be a very bad idea to edit it manually.

    document.write(
        "//! This file is auto-generated. Do not edit it manually.\n"
        "//!\n"
        "//! This file contains the sidebar for the frontend.\n"
    )

    imports = [
        "use yew::prelude::*;",
        "use yew_router::prelude::*;",
        "use yewdux::use_store;",
        "use web_common::database::*;",
        "use crate::router::AppRoute;",
        "use super::logout::Logout;",
        "use crate::components::basic_page::PageLike;",
        "use crate::stores::user_state::UserState;",
    ]

    document.write("\n".join(imports) + "\n\n")

    document.write(
        "#[derive(Properties, Clone, PartialEq, Debug)]\n"
        "pub struct SidebarProps {\n"
        "    pub visible: bool,\n"
        "}\n\n"
    )

    document.write(
        "#[function_component(Sidebar)]\n"
        "pub fn sidebar(props: &SidebarProps) -> Html {\n"
        "    let (user, _) = use_store::<UserState>();\n"
        "    let route: AppRoute = use_route().unwrap_or_default();\n"
        "\n"
        "    let sidebar_class = if props.visible {\n"
        '        "sidebar"\n'
        "    } else {\n"
        '        "sidebar hidden"\n'
        "    };\n"
        "\n"
        "    html! {\n"
        "        <div class={sidebar_class}>\n"
        '            <div class="sidebar-content">\n'
        "                <ul>\n"
    )

    deny_list = [
        "roles",
        "invitations",
        "requests",
    ]

    for builder in builders:

        found_skip = False
        for deny in deny_list:
            if builder.table_name.endswith(deny):
                found_skip = True
                break
        
        if found_skip:
            continue

        if builder.is_junktion_table():
            continue

        rich_variant = builder.get_richest_variant()

        document.write(
            f'                    <li class={{if route == AppRoute::{builder.get_capitalized_table_name()} {{ "active" }} else {{ "" }}}}>\n'
            f"                        <Link<AppRoute> to={{AppRoute::{builder.get_capitalized_table_name()}}}>\n"
            f"                            <i class={{format!(\"fas fa-{{}}\", {rich_variant.name}::icon())}}></i>\n"
            "                             {'\\u{00a0}'}\n"
            f'                            <span>{{{rich_variant.name}::section()}}</span>\n'
            "                        </Link<AppRoute>>\n"
            "                    </li>\n"
        )

    document.write(
        "                    {if user.has_user() {\n"
        "                        html! {\n"
        "                            <li><Logout /></li>\n"
        "                        }\n"
        "                    } else {\n"
        "                        html! {\n"
        "                            <li>\n"
        "                                <Link<AppRoute> to={AppRoute::Login}>\n"
        '                                    <i class="fas fa-right-to-bracket"></i>\n'
        "                                     {'\\u{00a0}'}\n"
        '                                    <span>{"Login"}</span>\n'
        "                                </Link<AppRoute>>\n"
        "                            </li>\n"
        "                        }\n"
        "                    }}\n"
        "                </ul>\n"
        "            </div>\n"
        "        </div>\n"
        "    }\n"
        "}\n"
    )

    document.flush()
    document.close()


def write_frontend_router_page(builders: List[StructMetadata]):
    """Writes out the frontend router page.

    Parameters
    ----------
    builders : List[StructMetadata]
        The list of structs to write out.

    Implementative details
    ----------------------
    This function writes out the frontend router page to the filesystem.
    The router page contains:
    * The yew-router AppRoute enumeration, with the visitable structs and its flat new and update variants (when they exist).
    * The AppRoute also contains some of the additional pages, such as Login, Home and NotFound.
    * The page contains the switch to map each instance of the AppRoute to the corresponding page.
    * The component providing the left-side sidebar content.
    """

    document = open("../frontend/src/router.rs", "w", encoding="utf-8")

    # Preliminarly, we write out a comment that warns the
    # reader that this page is auto-generated and as such
    # it would be a very bad idea to edit it manually.
    document.write(
        "//! This file is auto-generated. Do not edit it manually.\n"
        "//!\n"
        "//! This file contains the router for the frontend.\n"
    )

    imports = [
        "use yew::prelude::*;",
        "use yew_router::prelude::*;",
        "use crate::pages::*;",
        "use uuid::Uuid;",
        "use crate::components::BasicList;",
        "use web_common::database::*;",
        "use crate::components::forms::automatic_forms::*;",
    ]

    document.write("\n".join(imports) + "\n\n")

    document.write(
        "#[derive(Debug, Clone, Copy, PartialEq, Routable)]\npub enum AppRoute {\n"
    )

    enum_variants = []

    deny_list = [
        "roles",
        "invitations",
        "requests",
    ]

    for builder in builders:
        flat_variant = builder.get_flat_variant()
        richest_variant = builder.get_richest_variant()
        primary_keys = flat_variant.get_primary_keys()

        found_skip = False
        for deny in deny_list:
            if builder.table_name.endswith(deny):
                found_skip = True
                break
        
        if found_skip:
            continue

        if builder.is_junktion_table():
            continue

        ids_url = "".join([f"/:{primary_key.name}" for primary_key in primary_keys])
        ids_struct = ", ".join(
            [
                f"{primary_key.name}: {primary_key.format_data_type()}"
                for primary_key in primary_keys
            ]
        )

        document.write(
            f'    #[at("/{builder.table_name}")]\n'
            f"    {builder.get_capitalized_table_name()},\n"
            f'    #[at("/{builder.table_name}{ids_url}")]\n'
            f"    {builder.get_capitalized_table_name()}View{{{ids_struct}}},\n"
        )

        enum_variants.extend([
            builder.get_capitalized_table_name(),
            f"{builder.get_capitalized_table_name()}View",
        ])

        if flat_variant.is_insertable():
            # We also add the /new sub-route
            document.write(
                f'    #[at("/{builder.table_name}/new")]\n'
                f"    {builder.get_capitalized_table_name()}New,\n"
            )

            enum_variants.append(f"{builder.get_capitalized_table_name()}New")

            for foreign_key in flat_variant.get_foreign_keys():
                # We retrieve from the rich variant the struct associated with the foreign key.
                normalized_foreign_key_name = foreign_key.normalized_name()
                foreign_key_attribute: AttributeMetadata = richest_variant.get_attribute_by_name(normalized_foreign_key_name)

                assert foreign_key_attribute.has_struct_data_type()

                if foreign_key.is_automatically_determined_column():
                    continue

                foreign_key_struct: StructMetadata = foreign_key_attribute.raw_data_type()

                if not foreign_key_struct.is_updatable():
                    continue

                capitalized_normalized_foreign_key_name = "".join(
                    word.capitalize() for word in normalized_foreign_key_name.split("_")
                )

                enum_variant_name = f"{builder.get_capitalized_table_name()}NewWith{capitalized_normalized_foreign_key_name}"

                document.write(
                    f'    #[at("/{builder.table_name}/new/{normalized_foreign_key_name}/:{foreign_key.name}")]\n'
                    f"    {enum_variant_name}{{{foreign_key.name}: {foreign_key.data_type()}}},\n"
                )

                enum_variants.append(enum_variant_name)

        if flat_variant.is_updatable():
            # We also add the /update sub-route
            document.write(
                f'    #[at("/{builder.table_name}{ids_url}/update")]\n'
                f"    {builder.get_capitalized_table_name()}Update{{{ids_struct}}},\n"
            )

            enum_variants.append(f"{builder.get_capitalized_table_name()}Update")

    # Last, we insert the additional pages.
    document.write(
        '    #[at("/")]\n'
        "    Home,\n"
        '    #[at("/login")]\n'
        "    Login,\n"
        "    #[not_found]\n"
        '    #[at("/404")]\n'
        "    NotFound,\n"
        "}\n\n"
    )

    # Next, we write the switch to map each instance of the AppRoute to the corresponding page.
    document.write(
        "/// The switch to map each instance of the AppRoute to the corresponding page.\n"
        "///\n"
        "/// # Arguments\n"
        "/// * `route` - The route to map.\n"
        "///\n"
        "pub fn switch(route: AppRoute) -> Html {\n"
        "    match route {\n"
    )

    covered_variants = []

    for builder in builders:
        flat_variant = builder.get_flat_variant()
        richest_variant = builder.get_richest_variant()
        primary_keys = flat_variant.get_primary_keys()

        found_skip = False
        for deny in deny_list:
            if builder.table_name.endswith(deny):
                found_skip = True
                break
        
        if found_skip:
            continue

        if builder.is_junktion_table():
            continue

        properties = []

        for primary_key in primary_keys:
            properties.append(f"{primary_key.name} = {{{primary_key.name}}}")

        document.write(
            f"        AppRoute::{builder.get_capitalized_table_name()} => {{\n"
            f"            html! {{ <BasicList<{richest_variant.name}> /> }}\n"
            f"        }}\n"
            f"        AppRoute::{builder.get_capitalized_table_name()}View{{{flat_variant.get_formatted_primary_keys(include_prefix=False, include_parenthesis=False)}}} => {{\n"
            f"            html! {{ <{flat_variant.name}Page {' '.join(properties)} /> }}\n"
            f"        }}\n"
        )

        covered_variants.extend([
            builder.get_capitalized_table_name(),
            f"{builder.get_capitalized_table_name()}View",
        ])

        if flat_variant.is_insertable():
            if flat_variant.name == "SpectraCollection":
                document.write(
                    f"        AppRoute::{builder.get_capitalized_table_name()}New => {{\n"
                    f"            html! {{ \"<Create{flat_variant.name}Form />\" }}\n"
                    f"        }}\n"
                )
            else:
                document.write(
                    f"        AppRoute::{builder.get_capitalized_table_name()}New => {{\n"
                    f"            html! {{ <Create{flat_variant.name}Form /> }}\n"
                    f"        }}\n"
                )

            covered_variants.append(f"{builder.get_capitalized_table_name()}New")

            for foreign_key in flat_variant.get_foreign_keys():
                # We retrieve from the rich variant the struct associated with the foreign key.
                normalized_foreign_key_name = foreign_key.normalized_name()
                foreign_key_attribute: AttributeMetadata = richest_variant.get_attribute_by_name(normalized_foreign_key_name)

                assert foreign_key_attribute.has_struct_data_type()

                if foreign_key.is_automatically_determined_column():
                    continue

                foreign_key_struct: StructMetadata = foreign_key_attribute.raw_data_type()

                if not foreign_key_struct.is_updatable():
                    continue

                capitalized_normalized_foreign_key_name = "".join(
                    word.capitalize() for word in normalized_foreign_key_name.split("_")
                )

                enum_variant_name = f"{builder.get_capitalized_table_name()}NewWith{capitalized_normalized_foreign_key_name}"

                document.write(
                f"        AppRoute::{enum_variant_name}{{{foreign_key.name}}} => {{\n"
                f"            html! {{ <Create{flat_variant.name}Form {foreign_key.name}={{{foreign_key.name}}} /> }}\n"
                f"        }}\n"
                )

                covered_variants.append(enum_variant_name)

        if flat_variant.is_updatable():
            form_primary_key_properties = " ".join(
                f"{primary_key.name}={{{primary_key.name}}}"
                for primary_key in primary_keys
            )
            document.write(
                f"        AppRoute::{builder.get_capitalized_table_name()}Update{{{flat_variant.get_formatted_primary_keys(include_prefix=False, include_parenthesis=False)}}} => {{\n"
                f"            html! {{ <Update{flat_variant.name}Form {form_primary_key_properties} /> }}\n"
                f"        }}\n"
            )

            covered_variants.append(f"{builder.get_capitalized_table_name()}Update")

    
    for expected_variant in enum_variants:
        assert expected_variant in covered_variants, f"Expected variant {expected_variant} not covered."

    for expected_variant in covered_variants:
        assert expected_variant in enum_variants, f"Variant {expected_variant} not expected."

    document.write(
        "        AppRoute::Home => {\n"
        "            html! { <Home /> }\n"
        "        }\n"
        "        AppRoute::Login => {\n"
        "            html! { <Login /> }\n"
        "        }\n"
        "        AppRoute::NotFound => {\n"
        "            html! { <NotFound /> }\n"
        "        }\n"
        "    }\n"
        "}\n"
    )

    document.flush()
    document.close()

    # Last, we write the component providing the left-side dashboard content.
    write_frontend_sidebar(builders)
