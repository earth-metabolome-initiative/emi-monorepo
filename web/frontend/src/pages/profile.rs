//! Page of the user profile, allowing the user to edit their profile.

use crate::components::forms::profile::NameForm;
use crate::components::forms::BasicForm;
use crate::router::AppRoute;
use crate::stores::user_state::UserState;
use wasm_bindgen::UnwrapThrowExt;
use yew::prelude::*;
use yew_router::prelude::*;
use yewdux::prelude::*;

#[function_component(Profile)]
pub fn profile() -> Html {
    // We use the state of the navigator from yew-router
    // If it happens that it doesn't exist, we throw an error
    // that can appear on the console.
    let navigator = use_navigator().unwrap_throw();
    // Similarly, we retrieve the stored user using yewdux
    // which is stored across the app, including the different
    // open tabs. This user is the one stored within the current
    // session in the Session Local Storage.
    let (user_state, _) = use_store::<UserState>();

    // If the user happens to not be logged in, we redirect
    // them to the home page.
    if !user_state.is_logged_in() {
        // This is done by pushing the route defined in the AppRoute
        // enum to the navigator.
        navigator.push(&AppRoute::Home);
    }

    // Now we know that the user is necessarily logged in, and as such,
    // we can safely unwrap the user from the user state.

    let user = user_state.user().unwrap_throw();

    // We proceed to render the profile page, which contains a form
    // to edit the user's name and surname.
    html! {
        <div class="fullscreen_center_app">
            <div class="profile">
                <h2>{ "Profile" }</h2>
                <NameForm />
            </div>
        </div>
    }
}
