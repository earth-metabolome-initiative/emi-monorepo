use serde::{Deserialize, Serialize};

#[derive(PartialEq, Debug, Serialize, Deserialize, Clone)]
pub struct User {
    name: Option<String>,
    middle_name: Option<String>,
    last_name: Option<String>,
    id: i32,
}

impl User {
    pub fn new(name: Option<String>, middle_name: Option<String>, last_name: Option<String>, id: i32) -> User {
        User {
            name,
            middle_name,
            last_name,
            id,
        }
    }

    pub fn full_name(&self) -> String {
        let mut full_name = String::new();
        if let Some(name) = &self.name {
            full_name.push_str(name);
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

    pub fn id(&self) -> i32 {
        self.id
    }

    pub fn name(&self) -> String {
        self.name.clone().unwrap_or("Anonymous".to_string())
    }

    pub fn middle_name(&self) -> Option<String> {
        self.middle_name.clone()
    }

    pub fn last_name(&self) -> Option<String> {
        self.last_name.clone()
    }

    pub fn set_name(&mut self, name: Option<String>) {
        self.name = name;
    }

    pub fn set_middle_name(&mut self, middle_name: Option<String>) {
        self.middle_name = middle_name;
    }

    pub fn set_last_name(&mut self, last_name: Option<String>) {
        self.last_name = last_name;
    }

    pub fn set_id(&mut self, id: i32) {
        self.id = id;
    }

    pub fn is_named(&self) -> bool {
        self.name.is_some() && self.last_name.is_some()
    }
}