use serde::{Deserialize, Serialize};
use web_common::user::User;
use yewdux::prelude::*;

#[derive(Default, PartialEq, Serialize, Deserialize, Store, Clone)]
#[store(storage = "local", storage_tab_sync)]
pub struct Store {
    sidebar_open: bool,
    is_offline: bool,
    user: Option<User>,
}

impl Store {
    pub fn toggle_sidebar(&mut self) {
        self.sidebar_open = !self.sidebar_open;
    }

    pub fn is_offline(&self) -> bool {
        self.is_offline
    }

    pub fn set_offline(&mut self, is_offline: bool) {
        self.is_offline = is_offline;
    }

    pub fn is_logged_in(&self) -> bool {
        self.user.is_some()
    }

    pub fn set_user(&mut self, user: Option<User>) {
        self.user = user;
    }

    pub fn user(&self) -> Option<User> {
        self.user.clone()
    }
}
