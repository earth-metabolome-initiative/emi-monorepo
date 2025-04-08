mod api;
pub mod components;
mod cookies;
mod pages;
mod router;
use router::AppRoute;
mod stores;
pub mod traits;
mod utils;
pub mod workers;

use yew_router::Routable;

impl core::str::FromStr for AppRoute {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s == AppRoute::Collect.to_path() {
            return Ok(AppRoute::Collect);
        }
        if s == AppRoute::Observe.to_path() {
            return Ok(AppRoute::Observe);
        }
        if s == AppRoute::Home.to_path() {
            return Ok(AppRoute::Home);
        }
        Err(format!("Could not parse route: {}", s))
    }
}

impl TryFrom<String> for AppRoute {
    type Error = String;

    fn try_from(s: String) -> Result<Self, Self::Error> {
        let route = s.parse::<AppRoute>();
        match route {
            Ok(route) => Ok(route),
            Err(_) => Err(format!("Could not parse route: {}", s)),
        }
    }
}
