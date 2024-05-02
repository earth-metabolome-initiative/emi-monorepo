//! Page of the user profile, allowing the user to edit their profile.

use crate::components::forms::automatic_forms::NewTeamForm;
use yew::prelude::*;

#[function_component(NewTeamPage)]
pub fn new_team_page() -> Html {
    // We proceed to render the team page, which contains a form
    // to edit the user's name and surname.
    html! {
        // <div class="fullscreen_center_app">
            <div class="new_team">
                <NewTeamForm />
            </div>
        // </div>
    }
}
