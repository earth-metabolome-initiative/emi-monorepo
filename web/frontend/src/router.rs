//! This file is auto-generated. Do not edit it manually.
//!
//! This file contains the router for the frontend.
use yew::prelude::*;
use yew_router::prelude::*;

use crate::pages::{home::Home, login::Login, not_found::NotFound};

#[derive(Debug, Clone, PartialEq, Routable, Eq)]
pub enum AppRoute {
    #[at("/")]
    Home,
    #[at("/login")]
    Login,
    #[not_found]
    #[at("/404")]
    NotFound,
}

#[allow(clippy::needless_pass_by_value)]
/// The switch to map each instance of the `AppRoute` to the corresponding page.
///
/// # Arguments
/// * `route` - The route to map.
pub fn switch(route: AppRoute) -> Html {
    match route {
        AppRoute::Home => {
            html! { <Home /> }
        }
        AppRoute::Login => {
            html! { <Login /> }
        }
        AppRoute::NotFound => {
            html! { <NotFound /> }
        }
    }
}
