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
use yew::prelude::*;
use yew_router::prelude::*;
use yewdux::prelude::*;

// We retrieve Navigator in order to check whether we are online.
use wasm_bindgen::JsCast;
use wasm_bindgen::closure::Closure;

use crate::components::hamburger::Hamburger;
use crate::components::search_bar::SearchBar;
use crate::components::sidebar::Sidebar;
use crate::store::Store;
use crate::api::retrieve_logged_user_info;

#[function_component(Navigator)]
pub fn navigator() -> Html {
    let show_side_bar = use_state(|| false);
    let route = use_route::<AppRoute>();
    let navigator = use_navigator().unwrap();
    let (store, dispatch) = use_store::<Store>();

    let user = store.user();

    {
        // TODO: run the following check only when token cookie is available
        // https://docs.rs/wasm-cookies/latest/wasm_cookies/cookies/index.html
        let dispatch = dispatch.clone();
        use_effect_with((), move |_| {
            wasm_bindgen_futures::spawn_local(async move {
                let user = retrieve_logged_user_info().await.ok();
                dispatch.reduce_mut(move |store| {
                    store.set_user(user);
                    if store.is_logged_in() {
                        // If the current page is the login page, we redirect to the home page.
                        if route.map_or(false, |r| r == AppRoute::Login) {
                            // We trigger a redirect to the home page.
                            navigator.push(&AppRoute::Home)
                        }
                    }
                });
            });
        });
    }

    // In order to continuously check whether we are online, we need to create
    // a timed callback that is called multiple times every few seconds, say 5.
    {
        let dispatch = dispatch.clone();
        use_effect_with((), move |_| {
            let callback = Closure::wrap(Box::new(move || {
                let is_online = web_sys::window().map(|w| w.navigator().on_line()).unwrap_or(false);
                dispatch.reduce_mut(move |store| {
                    store.set_offline(!is_online);
                });
            }) as Box<dyn FnMut()>);

            let window = web_sys::window().unwrap();
            let _ = window.set_interval_with_callback_and_timeout_and_arguments_0(
                callback.as_ref().unchecked_ref(),
                5000,
            );

            // We need to keep the callback alive, otherwise it will be deallocated.
            callback.forget();
        });
    }

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
                if let Some(user) = user {
                    <div class="user">
                        <img src={format!("/api/user/{}/avatar", user.id())} alt={format!("{}'s avatar", user.name())} />
                        <span>{user.full_name()}</span>
                        {if store.is_offline() {
                            html! {
                                <span class="badge offline">{"Offline"}</span>
                            }
                        } else {
                            html! {}
                        }}
                    </div>
                } else {
                    <Link<AppRoute> classes="login" to={AppRoute::Login}>{"Login"}</Link<AppRoute>>
                }
            </nav>
            <Sidebar visible={*show_side_bar} />
        </>
    }
}
