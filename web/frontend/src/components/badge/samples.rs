use super::{Badge, BadgeSize, RowToBadge};
use web_common::database::NestedSample;

impl RowToBadge for NestedSample {
    fn badge_title(&self) -> String {
        self.container.badge_title()
    }

    fn path(&self) -> Option<crate::router::AppRoute> {
        Some(<Self as crate::router::Viewable>::view_route(self))
    }

    fn font_awesome_icon(&self) -> Option<&str> {
        self.container.font_awesome_icon()
    }
}
