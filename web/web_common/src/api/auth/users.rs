pub mod me;
pub mod name;

use crate::combine_path;

pub const ENDPOINT: &str = "/users";
pub const FULL_ENDPOINT: &str = combine_path!(super::FULL_ENDPOINT, ENDPOINT);


use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(PartialEq, Debug, Serialize, Deserialize, Clone)]
pub struct User {
    name: name::Name,
    id: Uuid,
}

impl User {
    pub fn new(name: name::Name, id: Uuid) -> User {
        User { name, id }
    }

    pub fn full_name(&self) -> Result<String, String> {
        self.name.full_name()
    }

    pub fn id(&self) -> Uuid {
        self.id
    }

    pub fn has_complete_profile(&self) -> bool {
        self.name.is_complete()
    }

    pub fn name(&self) -> name::Name {
        self.name.clone()
    }

    pub fn first_name(&self) -> String {
        self.name.first_name()
    }
    
    pub fn last_name(&self) -> String {
        self.name.last_name()
    }

    pub fn middle_name(&self) -> Option<String> {
        self.name.middle_name()
    }
}