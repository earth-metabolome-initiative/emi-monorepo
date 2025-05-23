//! Logout button for the sidebar.
//!
//! The logout button is a simple button that sends a request to the backend to
//! remove the user's session by using the logout method from the APIs.
//! Furthermore, upon success, proceeds to delete the user session and redirect
//! the user to the login page.

use yew::prelude::*;

use crate::components::font_awesome_icon::SolidIcon;

#[function_component(Logout)]
/// Logout button for the sidebar.
pub fn logout() -> Html {
    html! {
        <button class="btn btn-primary logout">
            <SolidIcon icon="right-from-bracket"/>
            {'\u{00a0}'}
            {"Logout"}
        </button>
    }
}
