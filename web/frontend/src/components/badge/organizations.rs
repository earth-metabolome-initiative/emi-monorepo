use super::{Badge, BadgeSize, RowToBadge};
use web_common::{database::NestedOrganization, traits::CapitalizeString};

impl RowToBadge for NestedOrganization {
    fn badge_title(&self) -> String {
        self.inner.name.capitalize()
    }

    fn path(&self) -> Option<crate::router::AppRoute> {
        Some(<Self as crate::router::Viewable>::view_route(self))
    }

    fn font_awesome_icon(&self) -> Option<&str> {
        Some("building")
    }
}
