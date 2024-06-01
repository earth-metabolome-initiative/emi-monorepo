use super::{Badge, BadgeSize, RowToBadge};
use web_common::database::NestedOrganism;

impl RowToBadge for NestedOrganism {
    fn badge_title(&self) -> String {
        self.nameplate.badge_title()
    }

    fn path(&self) -> Option<crate::router::AppRoute> {
        Some(<Self as crate::router::Viewable>::view_route(self))
    }

    fn primary_image_url(&self) -> Option<String> {
        Some(self.inner.get_picture_as_url())
    }
}
