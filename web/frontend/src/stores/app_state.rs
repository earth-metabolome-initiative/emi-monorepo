use serde::{Deserialize, Serialize};
use web_common::database::Task;
use yewdux::prelude::*;

#[derive(Default, PartialEq, Serialize, Deserialize, Store, Clone)]
/// The following macro will make sure that the store is saved across sessions.
#[store(storage = "local", storage_tab_sync)]
pub struct AppState {
    sidebar_open: bool,
    connect_to_internet: bool,
    tasks: Vec<Task>,
}

impl AppState {
    pub fn sidebar_open(&self) -> bool {
        self.sidebar_open
    }

    pub fn toggle_sidebar(&mut self) {
        self.sidebar_open = !self.sidebar_open;
    }

    pub fn add_task(&mut self, task: Task) {
        self.tasks.push(task);
    }

    pub fn remove_task(&mut self, task_id: uuid::Uuid) {
        self.tasks.retain(|task| task.id() != task_id);
    }

    pub fn tasks(&self) -> &[Task] {
        &self.tasks
    }

    pub fn tasks_mut(&mut self) -> &mut [Task] {
        &mut self.tasks
    }

    pub fn connect_to_internet(&self) -> bool {
        self.connect_to_internet
    }

    pub fn set_connect_to_internet(&mut self, connect_to_internet: bool) {
        self.connect_to_internet = connect_to_internet;
    }
}
