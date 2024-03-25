use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
pub struct PublicUser {
    id: Uuid,
    first_name: String,
    middle_name: Option<String>,
    last_name: String,
    profile_picture_url: String,
    thumbnail_profile_picture_url: String,
}

impl PublicUser {
    pub fn new(
        id: Uuid,
        first_name: String,
        middle_name: Option<String>,
        last_name: String,
        profile_picture_url: String,
        thumbnail_profile_picture_url: String,
    ) -> Self {
        PublicUser {
            id,
            first_name,
            middle_name,
            last_name,
            profile_picture_url,
            thumbnail_profile_picture_url,
        }
    }

    pub fn id(&self) -> Uuid {
        self.id
    }

    pub fn profile_picture_url(&self) -> &str {
        &self.profile_picture_url
    }

    pub fn thumbnail_profile_picture_url(&self) -> &str {
        &self.thumbnail_profile_picture_url
    }

    pub fn full_name(&self) -> String {
        match &self.middle_name {
            Some(middle_name) => format!("{} {} {}", self.first_name, middle_name, self.last_name),
            None => format!("{} {}", self.first_name, self.last_name),
        }
    }
}
