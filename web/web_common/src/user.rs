use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(PartialEq, Debug, Serialize, Deserialize, Clone)]
pub struct User {
    first_name: Option<String>,
    middle_name: Option<String>,
    last_name: Option<String>,
    id: Uuid,
}

impl User {
    pub fn new(first_name: Option<String>, middle_name: Option<String>, last_name: Option<String>, id: Uuid) -> User {
        User {
            first_name,
            middle_name,
            last_name,
            id,
        }
    }

    pub fn full_name(&self) -> String {
        let mut full_name = String::new();
        if let Some(first_name) = &self.first_name {
            full_name.push_str(first_name);
        }
        if let Some(middle_name) = &self.middle_name {
            full_name.push_str(" ");
            full_name.push_str(middle_name);
        }
        if let Some(last_name) = &self.last_name {
            full_name.push_str(" ");
            full_name.push_str(last_name);
        }

        if full_name.is_empty() {
            "Anonymous".to_string()
        } else {
            full_name
        }
    }

    pub fn id(&self) -> Uuid {
        self.id
    }

    pub fn first_name(&self) -> Option<String> {
        self.first_name.clone()
    }

    pub fn middle_name(&self) -> Option<String> {
        self.middle_name.clone()
    }

    pub fn last_name(&self) -> Option<String> {
        self.last_name.clone()
    }

    pub fn set_first_name(&mut self, first_name: Option<String>) {
        self.first_name = first_name;
    }

    pub fn set_middle_name(&mut self, middle_name: Option<String>) {
        self.middle_name = middle_name;
    }

    pub fn set_last_name(&mut self, last_name: Option<String>) {
        self.last_name = last_name;
    }

    pub fn set_id(&mut self, id: Uuid) {
        self.id = id;
    }

    pub fn has_complete_profile(&self) -> bool {
        self.first_name.is_some() && self.last_name.is_some()
    }
}