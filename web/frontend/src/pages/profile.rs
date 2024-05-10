//! Page of the user profile, allowing the user to edit their profile.

use crate::{components::forms::automatic_forms::UpdateUserForm, stores::user_state::UserState};
use yewdux::prelude::*;
use yew::prelude::*;

#[function_component(Profile)]
pub fn profile() -> Html {
    // We proceed to render the profile page, which contains a form
    // to edit the user's name and surname.
    let (user_store, _) = use_store::<UserState>();

    html! {
        <div class="profile">
            <h2>{ "Profile" }</h2>
            <UpdateUserForm id={user_store.id().unwrap()} />
        </div>
    }
}
