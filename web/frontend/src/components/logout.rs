//! Logout button for the sidebar.
//!
//! The logout button is a simple button that sends a request to the backend to
//! remove the user's session by using the logout method from the APIs. Furthermore,
//! upon success, proceeds to delete the user session and redirect the user to the
//! login page.

use yew::prelude::*;
use yewdux::prelude::*;

use crate::stores::app_state;
use crate::stores::user_state;

#[function_component(Logout)]
/// Logout button for the sidebar.
pub fn logout() -> Html {
    let (_, dispatch) = use_store::<user_state::UserState>();
    let (_, app_dispatch) = use_store::<app_state::AppState>();

    let on_logout = {
        Callback::from(move |_: MouseEvent| {
            user_state::logout(dispatch.clone(), app_dispatch.clone());
        })
    };

    html! {
        <button onclick={on_logout} class="btn btn-primary">
            {"Logout"}
        </button>
    }
}
