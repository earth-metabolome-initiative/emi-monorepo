use uuid::Uuid;

use crate::custom_validators::ImageSize;

use super::PublicUser;

impl PublicUser {
    pub fn full_name(&self) -> String {
        match (
            self.first_name.as_ref(),
            self.middle_name.as_ref(),
            self.last_name.as_ref(),
        ) {
            (Some(first_name), Some(middle_name), Some(last_name)) => {
                format!("{} {} {}", first_name, middle_name, last_name)
            }
            (Some(first_name), Some(last_name), _) => {
                format!("{} {}", first_name, last_name)
            }
            (Some(first_name), _, _) => first_name.clone(),
            _ => "Anonymous".to_string(),
        }
    }

    pub fn profile_picture_path(id: Uuid, image_size: &ImageSize) -> String {
        format!(
            "{}/{}/{}.png",
            crate::api::documents::profile::picture::FULL_ENDPOINT,
            id.to_string().to_lowercase(),
            image_size.to_string()
        )
    }

    pub fn thumbnail_path(&self) -> String {
        Self::profile_picture_path(self.id, &ImageSize::Thumbnail)
    }

    pub fn standard_profile_picture_path(&self) -> String {
        Self::profile_picture_path(self.id, &ImageSize::Standard)
    }
}
