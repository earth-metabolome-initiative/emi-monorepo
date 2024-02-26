//! Logout button for the sidebar.
//!
//! The logout button is a simple button that sends a request to the backend to
//! remove the user's session by using the logout method from the APIs. Furthermore,
//! upon success, proceeds to delete the user session and redirect the user to the
//! login page.

use crate::api::oauth::logout;
use yew::prelude::*;
use yew_router::prelude::*;
use yewdux::prelude::*;

use crate::router::AppRoute;
use crate::stores::user_state;
use crate::stores::UserState;

#[function_component(Logout)]
/// Logout button for the sidebar.
pub fn logout() -> Html {
    let (_user, dispatch) = use_store::<UserState>();
    let navigator = use_navigator().unwrap();

    let on_logout = Callback::from(move |_| {
        user_state::logout(dispatch.clone(), navigator.clone());
    });

    html! {
        <button onclick={on_logout} class="btn btn-primary">
            {"Logout"}
        </button>
    }
}
