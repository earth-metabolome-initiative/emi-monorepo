//! The navigator Yew 0.21 component is responsible for rendering the navigation bar at the top of the page.
//!
//! Depending on whether we are on a large page, a middle page (such as table)
//! or a small page (such as a mobile device), the navigation bar will render differently.
//!
//! In the case of a large page, a side bar component is always shown on the left side of the page.
//! In the case of a middle page, the side bar is hidden by default and can be toggled by clicking the hamburger icon,
//! which is located on the left side of the navigation bar.
//! In the case of a small page, the side bar is hidden by default and can be toggled by clicking the hamburger icon,
//! which is located on the left side of the navigation bar.
//!
//! The navigator component, besides the occasional hamburger icon, also contains the logo of the application,
//! and in both the large and medium page cases, in the center shows a search bar. On the right side of the navigation bar,
//! it is shown in the large and medium page cases, the user name and the user avatar, and in the small page case, solely the user avatar.
//! On the right of the user name, in the large and medium page cases.
//!
//! The overall web application needs to function also offline, as the user may want to use it while
//! they do not have an internet connection. Therefore, the navigator will also display a message to the
//! user if they are offline by putting a badge with the text "Offline" on their avatar. Upon returning online,
//! the badge will disappear.
//!

use crate::router::AppRoute;
use crate::stores::UserState;
use yew::prelude::*;
use yew_router::prelude::*;
use yewdux::prelude::*;

use crate::components::hamburger::Hamburger;
use crate::components::search_bar::SearchBar;
use crate::components::sidebar::Sidebar;

#[function_component(Navigator)]
pub fn navigator() -> Html {
    let show_side_bar = use_state(|| false);
    let route = use_route::<AppRoute>();
    let navigator = use_navigator().unwrap();
    let (user, _) = use_store::<UserState>();

    // On click, we send a message to the store to toggle the sidebar.
    let onclick = {
        let show_side_bar = show_side_bar.clone();
        Callback::from(move |_| {
            show_side_bar.set(!*show_side_bar);
        })
    };

    html! {
        <>
            <nav>
                <Hamburger is_active = {*show_side_bar} onclick = {onclick}/>
                <h1>
                    <Link<AppRoute> classes="logo" to={AppRoute::Home}>
                        {"EMI"}
                    </Link<AppRoute>>
                </h1>
                <SearchBar />
                if let Some(user) = user.user() {
                    <div class="user">
                        <img src={format!("/api/user/{}/avatar", user.id())} alt={format!("{}'s avatar", user.name())} />
                        <span>{user.full_name()}</span>
                        // {if store.is_offline() {
                        //     html! {
                        //         <span class="badge offline">{"Offline"}</span>
                        //     }
                        // } else {
                        //     html! {}
                        // }}
                    </div>
                } else {
                    <Link<AppRoute> classes="login" to={AppRoute::Login}>{"Login"}</Link<AppRoute>>
                }
            </nav>
            <Sidebar visible={*show_side_bar} />
        </>
    }
}
