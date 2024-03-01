//! Home page of the application.

use yew::prelude::*;
use yewdux::prelude::*;
use yew_router::prelude::*;
use crate::router::AppRoute;
use crate::stores::user_state::UserState;

#[function_component(Home)]
pub fn home() -> Html {
    let (user, _dispatch) = use_store::<UserState>();
    let navigator = use_navigator().unwrap();

    if user.is_logged_in() && !user.has_complete_profile(){
        // If the user is logged in, but has yet to complete their profile, we redirect them to the profile page.
        navigator.push(&AppRoute::Profile);
    }

    html! {
        <div class="fullscreen_center_app">
            <h1>{"Home"}</h1>
        </div>
    }
}
