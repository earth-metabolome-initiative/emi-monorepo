use crate::custom_validators::ImageSize;

use super::PublicUser;

impl PublicUser {
    pub fn full_name(&self) -> String {
        format!("{} {}", self.first_name, self.last_name)
    }

    pub fn profile_picture_path<I>(id: I, image_size: &ImageSize) -> String
    where
        I: ToString,
    {
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
