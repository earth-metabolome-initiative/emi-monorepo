//! Router of the single-page application.

use crate::pages::*;
use yew::prelude::*;
use yew_router::prelude::*;

#[derive(Debug, Clone, Copy, PartialEq, Routable)]
pub enum AppRoute {
    #[at("/login")]
    Login,
    #[at("/profile")]
    Profile,
    #[at("/project/new")]
    NewProject,
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
        AppRoute::Login => html! {<Login />},
        AppRoute::Profile => html! {<Profile />},
        AppRoute::NewProject => html! {<NewProjectPage />},
        AppRoute::NotFound => html! {<NotFound />},
        AppRoute::ServerError => html! {<ServerErrorPage />},
        AppRoute::Home => html! {<Home />},
    }
}
