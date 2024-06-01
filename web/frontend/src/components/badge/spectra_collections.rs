use super::{Badge, BadgeSize, RowToBadge};
use web_common::database::NestedSpectraCollection;

impl RowToBadge for NestedSpectraCollection {
    fn badge_title(&self) -> String {
        "Spectra Collection".to_string()
    }

    fn path(&self) -> Option<crate::router::AppRoute> {
        Some(<Self as crate::router::Viewable>::view_route(self))
    }

    fn font_awesome_icon(&self) -> Option<&str> {
        self.sample.font_awesome_icon()
    }
}
