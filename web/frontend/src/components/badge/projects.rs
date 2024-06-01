use super::{Badge, BadgeSize, RowToBadge};
use web_common::database::NestedProject;

impl RowToBadge for NestedProject {
    fn badge_title(&self) -> String {
        self.inner.name.clone()
    }

    fn path(&self) -> Option<crate::router::AppRoute> {
        Some(<Self as crate::router::Viewable>::view_route(self))
    }

    fn font_awesome_icon(&self) -> Option<&str> {
        self.icon.font_awesome_icon()
    }
}
