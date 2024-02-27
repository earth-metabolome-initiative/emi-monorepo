//! Page of the user profile, allowing the user to edit their profile.

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

    html! {
        <div class="fullscreen_center_app">
            {"Profile page."}
        </div>
    }
}
