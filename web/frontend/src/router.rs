//! Router of the single-page application.


use yew::prelude::*;
use yew_router::prelude::*;
use crate::pages::*;

#[derive(Debug, Clone, Copy, PartialEq, Routable)]
pub enum AppRoute {
    #[at("/login")]
    Login,
    #[at("/")]
    Home,
    #[not_found]
    #[at("/404")]
    NotFound,
    #[at("/500")]
    ServerError,
}

pub fn switch(switch: AppRoute) -> Html {
    match switch {
        AppRoute::Home => html!{<Home />},
        AppRoute::Login => html!{<Login />},
        AppRoute::NotFound => html!{<NotFound />},
        AppRoute::ServerError => html!{<ServerErrorPage />},
    }
}
