//! Page of the user profile, allowing the user to edit their profile.

use crate::components::forms::profile::Name;
use crate::components::forms::BasicForm;
use crate::router::AppRoute;
use crate::stores::user_state::UserState;
use yew::prelude::*;
use yew_router::prelude::*;
use yewdux::prelude::*;

#[function_component(Profile)]
pub fn profile() -> Html {
    let navigator = use_navigator().unwrap();
    let (user, _) = use_store::<UserState>();

    if !user.is_logged_in() {
        navigator.push(&AppRoute::Home);
    }

    if let Some(user) = user.user() {
        html! {
            <div class="fullscreen_center_app">
                <BasicForm<Name> first_name={user.first_name().unwrap_or_else(|| "".to_string())} last_name={user.last_name().unwrap_or_else(|| "".to_string())} />
            </div>
        }
    } else {
        unreachable!("User is not logged in.")
    }
}
