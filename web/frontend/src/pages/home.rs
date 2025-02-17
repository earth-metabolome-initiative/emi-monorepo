//! Home page of the application.

// use crate::router::AppRoute;
// use crate::stores::user_state::UserState;
use yew::prelude::*;
// use yew_router::prelude::*;
// use yewdux::prelude::*;

#[function_component(Home)]
pub fn home() -> Html {
    // let user = use_store_value::<UserState>();
    // let navigator = use_navigator().unwrap();

    // if user.has_incomplete_profile() {
    //     // If the user is logged in, but has yet to complete their profile, we
    // redirect them to the profile page.     navigator.push(&
    // AppRoute::Profile); }

    html! {
        <div class="fullscreen_center_app">
            <h1>{"Home"}</h1>
        </div>
    }
}
