use serde::{Deserialize, Serialize};
use yewdux::prelude::*;

#[derive(Default, PartialEq, Serialize, Deserialize, Store, Clone)]
/// The following macro will make sure that the store is saved across sessions.
#[store(storage = "local")]
pub struct AppState {
    sidebar_open: bool,
}

impl AppState {
    pub fn sidebar_open(&self) -> bool {
        self.sidebar_open
    }

    /// Set the sidebar visibility.
    pub fn set_sidebar_visibility(&mut self, visibility: bool) {
        self.sidebar_open = visibility;
    }
}
