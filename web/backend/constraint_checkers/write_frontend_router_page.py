"""Submodule writing frontend router page to the filesystem."""

from typing import List
from constraint_checkers.struct_metadata import StructMetadata
from constraint_checkers.find_foreign_keys import find_foreign_keys

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
        "use crate::router::AppRoute;",
        "use super::logout::Logout;",
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
        "        \"sidebar\"\n"
        "    } else {\n"
        "        \"sidebar hidden\"\n"
        "    };\n"
        "\n"
        "    html! {\n"
        "        <div class={sidebar_class}>\n"
        "            <div class=\"sidebar-content\">\n"
        "                <ul>\n"
    )

    for builder in builders:

        document.write(
            f"                    <li class={{if route == AppRoute::{builder.get_capitalized_table_name()} {{ \"active\" }} else {{ \"\" }}}}>\n"
            f"                        <Link<AppRoute> to={{AppRoute::{builder.get_capitalized_table_name()}}}>\n"
            f"                            {{\"{builder.capitalized_human_readable_table_name()}\"}}\n"
            "                        </Link<AppRoute>>\n"
        )
        if builder.get_flat_variant().is_insertable():
            document.write(
                f"                        {{if route == AppRoute::{builder.get_capitalized_table_name()} && user.has_user() {{\n"
                "                            html! {\n"
                "                                <ul>\n"
                "                                    <li>\n"
                f"                                        <Link<AppRoute> to={{AppRoute::{builder.get_capitalized_table_name()}New}}>\n"
                f"                                            {{\"New {builder.human_readable_name()}\"}}\n"
                "                                        </Link<AppRoute>>\n"
                "                                    </li>\n"
                "                                </ul>\n"
                "                            }\n"
                "                        } else {\n"
                "                            html! {<></>}\n"
                "                        }}\n"
            )
            
        document.write(
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
        "                                    {\"Login\"}\n"
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

    tables = find_foreign_keys()
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
        "use crate::components::BasicPages;",
        "use web_common::database::*;",
        "use crate::components::forms::automatic_forms::*;"
    ]

    document.write("\n".join(imports) + "\n\n")

    document.write(
        "#[derive(Debug, Clone, Copy, PartialEq, Routable)]\n"
        "pub enum AppRoute {\n"
    )

    for builder in builders:
        flat_variant = builder.get_flat_variant()

        primary_key = flat_variant.get_primary_key()

        document.write(
            f"    #[at(\"/{builder.table_name}\")]\n"
            f"    {builder.get_capitalized_table_name()},\n"
            f"    #[at(\"/{builder.table_name}/:{primary_key.name}\")]\n"
            f"    {builder.get_capitalized_table_name()}View{{{primary_key.name}: {primary_key.format_data_type()}}},\n"
        )

        if flat_variant.is_insertable():
            # We also add the /new sub-route
            document.write(
                f"    #[at(\"/{builder.table_name}/new\")]\n"
                f"    {builder.get_capitalized_table_name()}New,\n"
            )
        
        if flat_variant.is_updatable():
            # We also add the /update sub-route
            document.write(
                f"    #[at(\"/{builder.table_name}/:{primary_key.name}/update\")]\n"
                f"    {builder.get_capitalized_table_name()}Update{{{primary_key.name}: {primary_key.format_data_type()}}},\n"
            )

    # Last, we insert the additional pages.
    document.write(
        "    #[at(\"/\")]\n"
        "    Home,\n"
        "    #[at(\"/login\")]\n"
        "    Login,\n"
        "    #[not_found]\n"
        "    #[at(\"/404\")]\n"
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

    for builder in builders:
        flat_variant = builder.get_flat_variant()
        richest_variant = builder.get_richest_variant()

        primary_key = flat_variant.get_primary_key()

        document.write(
            f"        AppRoute::{builder.get_capitalized_table_name()} => {{\n"
            f"            html! {{ <BasicPages<{richest_variant.name}> /> }}\n"
            f"        }}\n"
            f"        AppRoute::{builder.get_capitalized_table_name()}View{{{primary_key.name}}} => {{\n"
            f"             html! {{ <span>{{\"Specific {builder.human_readable_name()} page\"}}</span> }}\n"
            #f"            html! {{ <{builder.get_capitalized_table_name()}View {primary_key.name}={primary_key.name} /> }}\n"
            f"        }}\n"
        )

        if flat_variant.is_insertable():
            document.write(
                f"        AppRoute::{builder.get_capitalized_table_name()}New => {{\n"
                f"            html! {{ <Create{flat_variant.name}Form /> }}\n"
                f"        }}\n"
            )

        if flat_variant.is_updatable():
            document.write(
                f"        AppRoute::{builder.get_capitalized_table_name()}Update{{{primary_key.name}}} => {{\n"
                f"            html! {{ <Update{flat_variant.name}Form {primary_key.name}={{{primary_key.name}}} /> }}\n"
                f"        }}\n"
            )

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