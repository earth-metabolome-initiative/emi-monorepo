//! Logout button for the sidebar.
//!
//! The logout button is a simple button that sends a request to the backend to
//! remove the user's session by using the logout method from the APIs.
//! Furthermore, upon success, proceeds to delete the user session and redirect
//! the user to the login page.

use yew::prelude::*;
use yewdux::prelude::*;

use crate::stores::app_state;

#[function_component(Logout)]
/// Logout button for the sidebar.
pub fn logout() -> Html {
    let (_, app_dispatch) = use_store::<app_state::AppState>();

    html! {
        <button class="btn btn-primary logout">
            <i class="fas fa-right-from-bracket"></i>
            {'\u{00a0}'}
            {"Logout"}
        </button>
    }
}
