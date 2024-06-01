use super::{Badge, BadgeSize, RowToBadge};
use web_common::database::User;

impl RowToBadge for User {
    fn badge_title(&self) -> String {
        self.full_name()
    }

    fn path(&self) -> Option<crate::router::AppRoute> {
        Some(<Self as crate::router::Viewable>::view_route(self))
    }

    fn primary_image_url(&self) -> Option<String> {
        Some(self.get_profile_picture_as_url())
    }
}
