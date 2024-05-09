//! Page of the user profile, allowing the user to edit their profile.

use crate::components::forms::automatic_forms::UpdateUserForm;
use yew::prelude::*;

#[function_component(Profile)]
pub fn profile() -> Html {
    // We proceed to render the profile page, which contains a form
    // to edit the user's name and surname.
    html! {
        <div class="profile">
            <h2>{ "Profile" }</h2>
            <UpdateUserForm id=0 />
        </div>
    }
}
