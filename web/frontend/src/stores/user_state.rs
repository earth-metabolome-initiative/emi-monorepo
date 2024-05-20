use serde::{Deserialize, Serialize};
use web_common::database::User;
use yewdux::prelude::*;

use super::app_state::AppState;

#[derive(Default, PartialEq, Serialize, Deserialize, Store, Clone)]
/// The following macro will make sure that the store is saved across sessions.
#[store(storage = "session", storage_tab_sync)]
pub struct UserState {
    user: Option<User>,
}

impl UserState {
    pub fn has_user(&self) -> bool {
        self.user.is_some()
    }

    pub fn user(&self) -> Option<&User> {
        self.user.as_ref()
    }

    pub fn id(&self) -> Option<i32> {
        self.user.as_ref().map(|user| user.id)
    }

    /// Set the user to the provided value and returns whether any changes were made.
    pub fn set_user(&mut self, user: User) -> bool {
        if self.user.as_ref() != Some(&user) {
            self.user = Some(user);
            true
        } else {
            false
        }
    }

    pub fn has_incomplete_profile(&self) -> bool {
        self.user.as_ref().map_or(false, |user| false)
    }
}

pub fn logout(dispatch: Dispatch<UserState>, dispatch_app: Dispatch<AppState>) {
    wasm_bindgen_futures::spawn_local(async move {
        let _ = crate::api::oauth::logout::logout().await;
        dispatch.reduce_mut(move |store| {
            *store = UserState::default();
        });
        dispatch_app.reduce_mut(move |store| {
            *store = AppState::default();
        });
    });
}
