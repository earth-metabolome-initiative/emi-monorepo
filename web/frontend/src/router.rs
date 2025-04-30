//! This file is auto-generated. Do not edit it manually.
//!
//! This file contains the router for the frontend.
use yew_router::prelude::*;

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
