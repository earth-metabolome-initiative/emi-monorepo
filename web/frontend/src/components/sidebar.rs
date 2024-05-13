//! A sidebar Yew component is responsible for rendering the sidebar on the left side of the page.

use super::logout::Logout;
use crate::router::AppRoute;
use crate::stores::user_state::UserState;
use yew::prelude::*;
use yew_router::prelude::*;
use crate::components::forms::automatic_forms::NewFormsRouter;
use yewdux::use_store;

#[derive(Properties, Clone, PartialEq, Debug)]
pub struct SidebarProps {
    pub visible: bool,
}

#[function_component(Sidebar)]
pub fn sidebar(props: &SidebarProps) -> Html {
    let (user, _) = use_store::<UserState>();

    let sidebar_class = if props.visible {
        "sidebar"
    } else {
        "sidebar hidden"
    };

    html! {
        <div class={sidebar_class}>
            <div class="sidebar-content">
                <ul>
                    <li>
                        <Link<AppRoute> to={AppRoute::Home}>{"Home"}</Link<AppRoute>>
                    </li>
                    if user.has_user() {
                        <li>
                            <Link<NewFormsRouter> to={NewFormsRouter::Projects}>{"New Project"}</Link<NewFormsRouter>>
                        </li>
                        <li>
                            <Link<NewFormsRouter> to={NewFormsRouter::Samples}>{"New Sample"}</Link<NewFormsRouter>>
                        </li>
                        <li>
                            <Link<NewFormsRouter> to={NewFormsRouter::Teams}>{"New Team"}</Link<NewFormsRouter>>
                        </li>
                        <li><Logout /></li>
                    }
                </ul>
            </div>
        </div>
    }
}
