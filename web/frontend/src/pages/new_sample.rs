//! Page of the user profile, allowing the user to edit their profile.

use crate::components::forms::project::NewSampleForm;
use crate::components::forms::BasicForm;
use crate::router::AppRoute;
use crate::stores::user_state::UserState;
use wasm_bindgen::UnwrapThrowExt;
use yew::prelude::*;
use yew_router::prelude::*;
use yewdux::prelude::*;

#[function_component(NewSamplePage)]
pub fn new_sample_page() -> Html {
    // We proceed to render the profile page, which contains a form
    // to edit the user's name and surname.
    html! {
        // <div class="fullscreen_center_app">
            <div class="new_sample">
                <NewSampleForm />
            </div>
        // </div>
    }
}
