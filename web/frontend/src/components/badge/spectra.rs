use super::{Badge, BadgeSize, RowToBadge};
use web_common::database::NestedSpectra;

impl RowToBadge for NestedSpectra {
    fn badge_title(&self) -> String {
        "Spectra".to_string()
    }

    fn path(&self) -> Option<crate::router::AppRoute> {
        Some(<Self as crate::router::Viewable>::view_route(self))
    }

    fn font_awesome_icon(&self) -> Option<&str> {
        self.spectra_collection.font_awesome_icon()
    }
}
