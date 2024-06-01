use super::{Badge, BadgeSize, RowToBadge};
use web_common::database::NestedObservation;

impl RowToBadge for NestedObservation {
    fn badge_title(&self) -> String {
        "Observation yet to be named".to_string()
    }

    fn path(&self) -> Option<crate::router::AppRoute> {
        Some(<Self as crate::router::Viewable>::view_route(self))
    }

    fn primary_image_url(&self) -> Option<String> {
        Some(self.inner.get_picture_as_url())
    }
}
