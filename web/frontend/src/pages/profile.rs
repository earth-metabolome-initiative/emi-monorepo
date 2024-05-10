//! Page of the user profile, allowing the user to edit their profile.

use crate::{
    components::forms::automatic_forms::UpdateUserForm, router::AppRoute,
    stores::user_state::UserState, utils::is_online,
};
use yew::prelude::*;
use yew::suspense::{Suspension, SuspensionResult};
use yew_router::hooks::use_navigator;
use yewdux::prelude::*;

#[hook]
pub fn use_user_id() -> SuspensionResult<i32> {
    // We proceed to render the profile page, which contains a form
    // to edit the user's name and surname.
    let (user_store, _) = use_store::<UserState>();

    match user_store.id() {
        Some(id) => Ok(id),
        None => {
            let (s, _handle) = Suspension::new();
            Err(s)
        }
    }
}

#[function_component(LoadedProfile)]
pub fn loaded_profile() -> HtmlResult {
    let id = use_user_id()?;
    Ok(html! {
        <div class="profile">
            <h2>{ "Profile" }</h2>
            <UpdateUserForm id={id} />
        </div>
    })
}

#[function_component(Profile)]
pub fn profile() -> Html {
    let navigator = use_navigator().unwrap();

    if !is_online() {
        navigator.push(&AppRoute::Login);
    }

    let fallback = html! {<div>{"Loading..."}</div>};

    html! {
        <Suspense fallback={fallback}>
            <LoadedProfile />
        </Suspense>
    }
}
