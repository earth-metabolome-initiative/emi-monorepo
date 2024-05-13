//! Router of the single-page application.

use crate::pages::*;
use yew::prelude::*;
use yew_router::prelude::*;
use crate::components::forms::automatic_forms::{NewFormsRouter, UpdateFormsRouter, new_forms_router_switch, update_forms_router_switch};


#[derive(Debug, Clone, Copy, PartialEq, Routable)]
pub enum AppRoute {
    #[at("/login")]
    Login,
    #[at("/profile")]
    Profile,
    #[at("/new/*")]
    NewFormsRoute,
    #[at("/update/*")]
    UpdateFormsRoute,
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
        AppRoute::NewFormsRoute => html! { <Switch<NewFormsRouter> render={new_forms_router_switch} /> },
        AppRoute::UpdateFormsRoute => html! { <Switch<UpdateFormsRouter> render={update_forms_router_switch} /> },
        AppRoute::NotFound => html! {<NotFound />},
        AppRoute::ServerError => html! {<ServerErrorPage />},
        AppRoute::Home => html! {<Home />},
    }
}
