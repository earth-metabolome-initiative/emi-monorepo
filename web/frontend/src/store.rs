use serde::{Deserialize, Serialize};
use yewdux::prelude::*;


#[derive(Default, PartialEq, Serialize, Deserialize, Store, Clone)]
#[store(storage = "local", storage_tab_sync)]
pub struct Store {
    pub sidebar_open: bool,
    pub is_offline: bool,
}


impl Store {
    pub fn toggle_sidebar(&mut self) {
        self.sidebar_open = !self.sidebar_open;
    }

    pub fn set_offline(&mut self, is_offline: bool) {
        self.is_offline = is_offline;
    }
}