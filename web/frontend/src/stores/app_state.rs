use serde::{Deserialize, Serialize};
use yewdux::prelude::*;

#[derive(Default, PartialEq, Serialize, Deserialize, Store, Clone)]
/// The following macro will make sure that the store is saved across sessions.
#[store(storage = "local", storage_tab_sync)]
pub struct AppState {
    sidebar_open: bool,
}

impl AppState {
    pub fn sidebar_open(&self) -> bool {
        self.sidebar_open
    }

    pub fn toggle_sidebar(&mut self) {
        self.sidebar_open = !self.sidebar_open;
    }
}