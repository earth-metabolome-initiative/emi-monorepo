//! Page of the user profile, allowing the user to edit their profile.

use crate::components::forms::automatic_forms::CreateSampleForm;
use yew::prelude::*;

#[function_component(NewSamplePage)]
pub fn new_sample_page() -> Html {
    // We proceed to render the profile page, which contains a form
    // to edit the user's name and surname.
    html! {
        // <div class="fullscreen_center_app">
            <div class="new_sample">
                <CreateSampleForm />
            </div>
        // </div>
    }
}
