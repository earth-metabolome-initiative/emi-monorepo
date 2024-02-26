use crate::api::auth::users;
use crate::api::oauth::jwt_cookies::refresh_jwt_cookie;
use crate::cookies::is_logged_in;
use serde::{Deserialize, Serialize};
use web_common::{api::oauth::jwt_cookies::AccessToken, user::User};
use yewdux::prelude::*;

#[derive(Default, PartialEq, Serialize, Deserialize, Store, Clone)]
/// The following macro will make sure that the store is saved across sessions.
#[store(storage = "session", storage_tab_sync)]
pub struct UserState {
    user: Option<User>,
    access_token: Option<AccessToken>,
}

impl UserState {
    pub fn is_logged_in(&self) -> bool {
        self.user.is_some()
    }

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

    pub fn set_user(&mut self, user: User) {
        self.user = Some(user);
    }

    pub fn access_token(&self) -> Option<&AccessToken> {
        self.access_token.as_ref()
    }

    pub fn user(&self) -> Option<&User> {
        self.user.as_ref()
    }
}

pub fn logout(dispatch: Dispatch<UserState>) {
    dispatch.reduce_mut(move |store| {
        store.user = None;
        store.access_token = None;
    })
}

pub fn refresh_access_token(dispatch: Dispatch<UserState>) {
    wasm_bindgen_futures::spawn_local(async move {
        if is_logged_in() {
            match refresh_jwt_cookie().await {
                Ok(access_token) => {
                    dispatch.reduce_mut(move |store| {
                        store.set_access_token(access_token);
                    });
                }
                Err(_) => {
                    logout(dispatch);
                }
            }
        }
    });
}

pub fn update_user_informations(dispatch: Dispatch<UserState>, access_token: AccessToken) {
    wasm_bindgen_futures::spawn_local(async move {
        match users::me::me(&access_token).await {
            Ok(user) => {
                dispatch.reduce_mut(move |store| {
                    store.set_user(user);
                });
            }
            Err(_) => {
                logout(dispatch);
            }
        }
    })
}
