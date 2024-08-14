use serde::{Deserialize, Serialize};
use std::rc::Rc;
use web_common::database::{NestedProject, NestedUser};
use yewdux::prelude::*;

use super::app_state::AppState;

#[derive(Default, PartialEq, Serialize, Deserialize, Store, Clone)]
/// The following macro will make sure that the store is saved across sessions.
#[store(storage = "session", storage_tab_sync)]
pub struct UserState {
    user: Option<Rc<NestedUser>>,
    // The primary project to be currently considered by the user.
    project: Option<Rc<NestedProject>>,
}

impl UserState {
    pub fn has_user(&self) -> bool {
        self.user.is_some()
    }

    pub fn has_project(&self) -> bool {
        self.project.is_some()
    }

    pub fn has_complete_profile(&self) -> bool {
        self.user
            .as_ref()
            .map(|user| user.inner.has_complete_profile())
            .unwrap_or(false)
    }

    pub fn user(&self) -> Option<Rc<NestedUser>> {
        self.user.clone()
    }

    pub fn project(&self) -> Option<Rc<NestedProject>> {
        self.project.clone()
    }

    pub fn id(&self) -> Option<i32> {
        self.user.as_ref().map(|user| user.inner.id)
    }

    /// Set the user to the provided value and returns whether any changes were made.
    pub fn set_user(&mut self, user: Rc<NestedUser>) -> bool {
        let maybe_user = Some(user);
        if self.user != maybe_user {
            self.user = maybe_user;
            true
        } else {
            false
        }
    }

    /// Set the project to the provided value and returns whether any changes were made.
    pub fn set_project(&mut self, project: Rc<NestedProject>) -> bool {
        let maybe_project = Some(project);
        if self.project != maybe_project {
            self.project = maybe_project;
            true
        } else {
            false
        }
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
