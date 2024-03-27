use crate::api::oauth::jwt_cookies::refresh_jwt_cookie;
use crate::cookies::is_logged_in;
use crate::router::AppRoute;
use log::info;
use serde::{Deserialize, Serialize};
use web_common::{api::oauth::jwt_cookies::AccessToken, database::PublicUser};
use yew_router::prelude::*;
use yewdux::prelude::*;

#[derive(Default, PartialEq, Serialize, Deserialize, Store, Clone)]
/// The following macro will make sure that the store is saved across sessions.
#[store(storage = "session", storage_tab_sync)]
pub struct UserState {
    user: Option<PublicUser>,
    access_token: Option<AccessToken>,
}

impl UserState {
    pub fn has_no_access_token(&self) -> bool {
        self.access_token.is_none()
    }

    pub fn has_access_token(&self) -> bool {
        self.access_token.is_some()
    }

    pub fn has_no_user(&self) -> bool {
        self.user.is_none()
    }

    pub fn has_user(&self) -> bool {
        self.user.is_some()
    }

    pub fn set_access_token(&mut self, access_token: AccessToken) {
        self.access_token = Some(access_token);
    }

    /// Set the user to the provided value and returns whether any changes were made.
    pub fn set_user(&mut self, user: PublicUser) -> bool {
        if self.user.as_ref() != Some(&user) {
            self.user = Some(user);
            true
        } else {
            false
        }
    }

    pub fn access_token(&self) -> Option<&AccessToken> {
        self.access_token.as_ref()
    }

    pub fn user(&self) -> Option<&PublicUser> {
        self.user.as_ref()
    }
}

pub fn logout(
    dispatch: Dispatch<UserState>,
    navigator: Navigator,
    access_token: Option<AccessToken>,
) {
    wasm_bindgen_futures::spawn_local(async move {
        let _ = crate::api::oauth::logout::logout(access_token.as_ref()).await;
        dispatch.reduce_mut(move |store| {
            store.user = None;
            store.access_token = None;
        });
        navigator.push(&AppRoute::Login);
    });
}

pub fn refresh_access_token(dispatch: Dispatch<UserState>, navigator: Navigator) {
    wasm_bindgen_futures::spawn_local(async move {
        if is_logged_in() {
            info!("No access token found, attempting to refresh it.");
            match refresh_jwt_cookie().await {
                Ok(access_token) => {
                    dispatch.reduce_mut(move |store| {
                        store.set_access_token(access_token);
                    });
                }
                Err(_) => {
                    logout(dispatch, navigator, None);
                }
            }
        }
    });
}
