"""Submodule writing frontend router page to the filesystem."""

from typing import List
from tqdm.auto import tqdm
from constraint_checkers.struct_metadata import StructMetadata, AttributeMetadata
from constraint_checkers.regroup_tables import SUPPORT_TABLE_NAMES

def write_frontend_sidebar(flat_variants: List[StructMetadata]):
    """Writes out the frontend sidebar.

    Parameters
    ----------
    flat_variants : List[StructMetadata]
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
        "use yew_hooks::use_click_away;",
        "use crate::components::basic_page::PageLike;",
        "use crate::stores::user_state::UserState;",
    ]

    document.write("\n".join(imports) + "\n\n")

    document.write(
        "#[derive(Properties, Clone, PartialEq, Debug)]\n"
        "pub struct SidebarProps {\n"
        "    pub visible: bool,\n"
        "    pub onclose: Callback<bool>,"
        "}\n\n"
    )

    document.write(
        "#[function_component(Sidebar)]\n"
        "pub fn sidebar(props: &SidebarProps) -> Html {\n"
        "    let (user, _) = use_store::<UserState>();\n"
        "    let route: AppRoute = use_route().unwrap_or_default();\n"
        "    let node = use_node_ref();\n"
        "    let onclose = props.onclose.clone();\n"
        "    let visible = props.visible;\n"
        "    use_click_away(node.clone(), move |_: Event| {\n"
        "        if visible {\n"
        "            onclose.emit(!visible);\n"
        "        }\n"
        "    });\n"
        "\n"
        "    let sidebar_class = if props.visible {\n"
        '        "sidebar"\n'
        "    } else {\n"
        '        "sidebar hidden"\n'
        "    };\n"
        "    let on_click_close = {\n"
        "        let onclose = props.onclose.clone();\n"
        "        Callback::from(move |_| {\n"
        "            onclose.emit(false);\n"
        "        })\n"
        "    };\n"
        "\n"
        "    html! {\n"
        "        <div ref={node} class={sidebar_class}>\n"
        '            <div class="sidebar-content">\n'
        "                <ul>\n"
        '                    <li class={if route == AppRoute::Home {{ "active" }} else {{ "" }}} onclick={&on_click_close}>\n'
        "                        <Link<AppRoute> to={AppRoute::Home}>\n"
        '                            <i class="fas fa-home"></i>\n'
        "                             {'\\u{00a0}'}\n"
        '                            <span>{"Home"}</span>\n'
        "                        </Link<AppRoute>>\n"
        "                    </li>\n"
        "                    if user.has_user() {\n"
        '                    <li class={if route == AppRoute::Collect {{ "active" }} else {{ "" }}} onclick={&on_click_close}>\n'
        "                        <Link<AppRoute> to={AppRoute::Collect}>\n"
        '                            <i class="fas fa-boxes-packing"></i>\n'
        "                             {'\\u{00a0}'}\n"
        '                            <span>{"Collect"}</span>\n'
        "                        </Link<AppRoute>>\n"
        "                    </li>\n"
        '                    <li class={if route.is_project_selection() {{ "active" }} else {{ "" }}} onclick={&on_click_close}>\n'
        "                        <Link<AppRoute> to={AppRoute::ProjectSelection{source_page: route.to_path()}}>\n"
        '                            <i class="fas fa-project-diagram"></i>\n'
        "                             {'\\u{00a0}'}\n"
        '                            <span>{"Project Selection"}</span>\n'
        "                        </Link<AppRoute>>\n"
        "                    </li>\n"
        "                    }\n"
    )

    for flat_variant in tqdm(
        flat_variants, desc="Writing frontend sidebar", unit="page", leave=False
    ):

        if flat_variant.table_name in SUPPORT_TABLE_NAMES:
            continue
    
        if flat_variant.is_junktion_table():
            continue

        if not flat_variant.is_searchable():
            continue

        rich_variant = flat_variant.get_richest_variant()

        document.write(
            f'                    <li class={{if route == AppRoute::{flat_variant.get_capitalized_table_name()} {{ "active" }} else {{ "" }}}} onclick={{&on_click_close}}>\n'
            f"                        <Link<AppRoute> to={{AppRoute::{flat_variant.get_capitalized_table_name()}}}>\n"
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
        "                            <li onclick={&on_click_close}>\n"
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


def write_frontend_router_page(
    flat_variants: List[StructMetadata]
):
    """Writes out the frontend router page.

    Parameters
    ----------
    flat_variants : List[StructMetadata]
        The list of flat variants to build the frontend pages from.

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
        "use crate::components::BasicList;",
        "use web_common::database::*;",
        "use crate::components::forms::automatic_forms::*;",
    ]

    document.write("\n".join(imports) + "\n\n")

    # We define the traits to tightly integrate the structs with the router and avoid
    # having to manually define the URLs for each struct.

    document.write(
        "/// The trait defining whether a struct has a role request struct associated with it.\n"
        "pub(crate) trait RoleRequestable {\n"
        "    /// The role request struct associated with the struct.\n"
        "    type RoleRequest: serde::Serialize + Tabular;\n"
        "    /// Returns the role request struct associated with the struct.\n"
        "    fn role_request(&self, user_id: i32, role: i32) -> Self::RoleRequest;\n"
        "    /// Returns the role request operation associated with the struct.\n"
        "    fn role_request_operation(&self, user_id: i32, role: i32) -> Operation {\n"
        "        Operation::Insert(Self::RoleRequest::TABLE.to_string(), bincode::serialize(&self.role_request(user_id, role)).unwrap())\n"
        "    }\n"
        "    /// Returns the route associated with the struct role request page.\n"
        "    fn edit_role_request(&self, user_id: i32) -> Operation{\n"
        "        self.role_request_operation(user_id, 2)\n"
        "    }\n"
        "    /// Returns the route associated with the struct role request page.\n"
        "    fn admin_role_request(&self, user_id: i32) -> Operation{\n"
        "        self.role_request_operation(user_id, 1)\n"
        "    }\n"
        "}\n\n"
        "/// Trait defining a struct whose page is be visitable by the router.\n"
        "pub(crate) trait Viewable {\n"
        "    /// Returns the route associated to the page with the overall struct list.\n"
        "    fn list_route() -> AppRoute;\n"
        "    /// Returns the route associated with the struct.\n"
        "    fn view_route(&self) -> AppRoute;\n"
        "}\n\n"
        "/// Trait defining a struct that can be created, and therefore has a new page.\n"
        "pub(crate) trait Insertable: Filtrable {\n"
        "    /// Returns the route associated with the struct new page.\n"
        "    ///\n"
        "    /// # Arguments\n"
        "    /// * `filter` - The optional filter to use to populate part of the form.\n"
        "    fn new_route(filter: Option<&Self::Filter>) -> AppRoute;\n"
        "}\n\n"
        "/// Trait defining a struct that can be updated, and therefore has an update page.\n"
        "pub(crate) trait Updatable {\n"
        "    /// Returns the route associated with the struct update page.\n"
        "    fn update_route(&self) -> AppRoute;\n"
        "}\n\n"
    )

    for flat_variant in tqdm(
        flat_variants,
        desc="Writing Viewable traits for frontend structs",
        unit="page",
        leave=False,
    ):
        if flat_variant.table_name in SUPPORT_TABLE_NAMES:
            continue

        if not flat_variant.is_searchable():
            continue

        richest_variant = flat_variant.get_richest_variant()
        primary_keys = flat_variant.get_primary_keys()

        if flat_variant.table_name == "projects":
            document.write(
                f"impl RoleRequestable for {flat_variant.name} {{\n"
                "    type RoleRequest = NewProjectsUsersRoleRequest;\n"
                "    fn role_request(&self, user_id: i32, role: i32) -> Self::RoleRequest {\n"
                f"        NewProjectsUsersRoleRequest {{ table_id: self.id, user_id, role_id: role }}\n"
                "    }\n"
                "}\n\n"
                f"impl RoleRequestable for {richest_variant.name} {{\n"
                f"    type RoleRequest = <{flat_variant.name} as RoleRequestable>::RoleRequest;\n"
                "    fn role_request(&self, user_id: i32, role: i32) -> Self::RoleRequest {\n"
                f"        <{flat_variant.name} as RoleRequestable>::role_request(self.inner.as_ref(), user_id, role)\n"
                "    }\n"
                "}\n\n"
            )
        elif richest_variant.has_project_attribute() and not richest_variant.is_junktion_table():
            project_attributes: List[AttributeMetadata] = richest_variant.get_project_attribute()
            attributes_path = ".".join(attribute.name for attribute in project_attributes)
            last_struct = project_attributes[-1].raw_data_type()
            document.write(
                f"impl RoleRequestable for {richest_variant.name} {{\n"
                f"    type RoleRequest = <{last_struct.name} as RoleRequestable>::RoleRequest;\n"
                "    fn role_request(&self, user_id: i32, role: i32) -> Self::RoleRequest {\n"
                f"        <{last_struct.name} as RoleRequestable>::role_request(self.{attributes_path}.as_ref(), user_id, role)\n"
                "    }\n"
                "}\n\n"
            )

        viewable_variant_names = [flat_variant]

        if richest_variant.is_nested():
            viewable_variant_names.append(richest_variant)

        for variant in viewable_variant_names:
            document.write(
                f"impl Viewable for {variant.name} {{\n"
                "    fn list_route() -> AppRoute {\n"
                f"        AppRoute::{flat_variant.get_capitalized_table_name()}{{}}\n"
                "    }\n"
                "    fn view_route(&self) -> AppRoute {\n"
                f"        AppRoute::{flat_variant.get_capitalized_table_name()}View{{"
            )

            for primary_key in primary_keys:
                document.write(f"{primary_key.name}: self.{variant.get_attribute_path(primary_key)}, ")

            document.write("}\n    }\n}\n\n")

            if flat_variant.is_insertable() and flat_variant.table_name != "users" and flat_variant.has_frontend_form():
                document.write(
                    f"impl Insertable for {variant.name} {{\n"
                    "    fn new_route(filter: Option<&Self::Filter>) -> AppRoute {\n"
                    "        if let Some(filter) = filter {\n"
                )
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

                    enum_variant_name = f"{flat_variant.get_capitalized_table_name()}NewWith{capitalized_normalized_foreign_key_name}"

                    document.write(
                        f"            if let Some({foreign_key.name}) = filter.{foreign_key.name} {{\n"
                        f"                return AppRoute::{enum_variant_name}{{{foreign_key.name}}};\n"
                        "            }\n"
                    )
                
                document.write(
                    "        }\n"
                    f"        AppRoute::{flat_variant.get_capitalized_table_name()}New\n"
                    "    }\n"
                    "}\n\n"
                )

            if flat_variant.is_updatable() and flat_variant.has_frontend_form():
                document.write(
                    f"impl Updatable for {variant.name} {{\n"
                    "    fn update_route(&self) -> AppRoute {\n"
                    f"        AppRoute::{flat_variant.get_capitalized_table_name()}Update{{"
                )

                for primary_key in primary_keys:
                    document.write(f"{primary_key.name}: self.{variant.get_attribute_path(primary_key)}, ")

                document.write("}\n    }\n}\n\n")


    document.write(
        "#[derive(Debug, Clone, Copy, PartialEq, Routable, Eq)]\npub enum AppRoute {\n"
    )

    enum_variants = []

    for flat_variant in tqdm(
        flat_variants,
        desc="Writing frontend router page",
        unit="page",
        leave=False,
    ):
        richest_variant = flat_variant.get_richest_variant()
        primary_keys = flat_variant.get_primary_keys()

        if flat_variant.table_name in SUPPORT_TABLE_NAMES:
            continue

        if not flat_variant.is_searchable():
            continue

        ids_url = "".join([f"/:{primary_key.name}" for primary_key in primary_keys])
        ids_struct = ", ".join(
            [
                f"{primary_key.name}: {primary_key.format_data_type(route='frontend')}"
                for primary_key in primary_keys
            ]
        )

        document.write(
            f'    #[at("/{flat_variant.table_name}")]\n'
            f"    {flat_variant.get_capitalized_table_name()},\n"
            f'    #[at("/{flat_variant.table_name}{ids_url}")]\n'
            f"    {flat_variant.get_capitalized_table_name()}View{{{ids_struct}}},\n"
        )

        enum_variants.extend([
            flat_variant.get_capitalized_table_name(),
            f"{flat_variant.get_capitalized_table_name()}View",
        ])

        if flat_variant.is_insertable()  and flat_variant.table_name != "users" and flat_variant.has_frontend_form():
            # We also add the /new sub-route
            document.write(
                f'    #[at("/{flat_variant.table_name}/new")]\n'
                f"    {flat_variant.get_capitalized_table_name()}New,\n"
            )

            enum_variants.append(f"{flat_variant.get_capitalized_table_name()}New")

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

                enum_variant_name = f"{flat_variant.get_capitalized_table_name()}NewWith{capitalized_normalized_foreign_key_name}"

                document.write(
                    f'    #[at("/{flat_variant.table_name}/new/{normalized_foreign_key_name}/:{foreign_key.name}")]\n'
                    f"    {enum_variant_name}{{{foreign_key.name}: {foreign_key.data_type(route='frontend')}}},\n"
                )

                enum_variants.append(enum_variant_name)

        if flat_variant.is_updatable() and flat_variant.has_frontend_form():
            # We also add the /update sub-route
            document.write(
                f'    #[at("/{flat_variant.table_name}{ids_url}/update")]\n'
                f"    {flat_variant.get_capitalized_table_name()}Update{{{ids_struct}}},\n"
            )

            enum_variants.append(f"{flat_variant.get_capitalized_table_name()}Update")

    # Last, we insert the additional pages.
    document.write(
        '    #[at("/")]\n'
        "    Home,\n"
        '    #[at("/collect")]\n'
        "    Collect,\n"
        '    #[at("/project-selection/:source_page")]\n'
        "    ProjectSelection{source_page: Option<String>},\n"
        '    #[at("/login")]\n'
        "    Login,\n"
        "    #[not_found]\n"
        '    #[at("/404")]\n'
        "    NotFound,\n"
        "}\n\n"
        "impl AppRoute {\n"
        "    pub fn is_project_selection(&self) -> bool {\n"
        "        matches!(self, AppRoute::ProjectSelection {..})\n"
        "    }\n"
        "}\n\n"
    )

    # Next, we write the switch to map each instance of the AppRoute to the corresponding page.
    document.write(
        "/// The switch to map each instance of the AppRoute to the corresponding page.\n"
        "///\n"
        "/// # Arguments\n"
        "/// * `route` - The route to map.\n"
        "pub fn switch(route: AppRoute) -> Html {\n"
        "    match route {\n"
    )

    covered_variants = []

    for flat_variant in flat_variants:
        richest_variant = flat_variant.get_richest_variant()
        primary_keys = flat_variant.get_primary_keys()

        if flat_variant.table_name in SUPPORT_TABLE_NAMES:
            continue

        if not flat_variant.is_searchable():
            continue

        properties = []

        for primary_key in primary_keys:
            properties.append(f"{primary_key.name} = {{{primary_key.name}}}")

        document.write(
            f"        AppRoute::{flat_variant.get_capitalized_table_name()} => {{\n"
            f"            html! {{ <BasicList<{richest_variant.name}> /> }}\n"
            f"        }}\n"
            f"        AppRoute::{flat_variant.get_capitalized_table_name()}View{{{flat_variant.get_formatted_primary_keys(include_prefix=False, include_parenthesis=False)}}} => {{\n"
            f"            html! {{ <{flat_variant.name}Page {' '.join(properties)} /> }}\n"
            f"        }}\n"
        )

        covered_variants.extend([
            flat_variant.get_capitalized_table_name(),
            f"{flat_variant.get_capitalized_table_name()}View",
        ])

        if flat_variant.is_insertable() and flat_variant.table_name != "users" and flat_variant.has_frontend_form():
            document.write(
                f"        AppRoute::{flat_variant.get_capitalized_table_name()}New => {{\n"
                f"            html! {{ <Create{flat_variant.name}Form /> }}\n"
                f"        }}\n"
            )

            covered_variants.append(f"{flat_variant.get_capitalized_table_name()}New")

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

                enum_variant_name = f"{flat_variant.get_capitalized_table_name()}NewWith{capitalized_normalized_foreign_key_name}"

                document.write(
                f"        AppRoute::{enum_variant_name}{{{foreign_key.name}}} => {{\n"
                f"            html! {{ <Create{flat_variant.name}Form {foreign_key.name}={{{foreign_key.name}}} /> }}\n"
                f"        }}\n"
                )

                covered_variants.append(enum_variant_name)

        if flat_variant.is_updatable() and flat_variant.has_frontend_form():
            form_primary_key_properties = " ".join(
                f"{primary_key.name}={{{primary_key.name}}}"
                for primary_key in primary_keys
            )
            document.write(
                f"        AppRoute::{flat_variant.get_capitalized_table_name()}Update{{{flat_variant.get_formatted_primary_keys(include_prefix=False, include_parenthesis=False)}}} => {{\n"
                f"            html! {{ <Update{flat_variant.name}Form {form_primary_key_properties} /> }}\n"
                f"        }}\n"
            )

            covered_variants.append(f"{flat_variant.get_capitalized_table_name()}Update")

    
    for expected_variant in enum_variants:
        assert expected_variant in covered_variants, f"Expected variant {expected_variant} not covered."

    for expected_variant in covered_variants:
        assert expected_variant in enum_variants, f"Variant {expected_variant} not expected."

    document.write(
        "        AppRoute::Home => {\n"
        "            html! { <Home /> }\n"
        "        }\n"
        "        AppRoute::Collect => {\n"
        "            html! { <Collect /> }\n"
        "        }\n"
        "        AppRoute::ProjectSelection{source_page} => {\n"
        "            html! { <ProjectSelection source_page={source_page} /> }\n"
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
    write_frontend_sidebar(flat_variants)
