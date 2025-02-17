//! This file is auto-generated. Do not edit it manually.
//!
//! This file contains the router for the frontend.
use yew::prelude::*;
use yew_router::prelude::*;

use crate::{components::BasicList, pages::*};

#[derive(Debug, Clone, PartialEq, Routable, Eq)]
pub enum AppRoute {
    #[at("/login")]
    Login,
    #[not_found]
    #[at("/404")]
    NotFound,
}

/// The switch to map each instance of the AppRoute to the corresponding page.
///
/// # Arguments
/// * `route` - The route to map.
pub fn switch(route: AppRoute) -> Html {
    match route {
        AppRoute::Login => {
            html! { <Login /> }
        }
        AppRoute::NotFound => {
            html! { <NotFound /> }
        }
    }
}
