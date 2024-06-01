use super::{Badge, BadgeSize, RowToBadge};
use web_common::database::NestedSampleContainer;

impl RowToBadge for NestedSampleContainer {
    fn badge_title(&self) -> String {
        format!("#{}", self.inner.barcode)
    }

    fn path(&self) -> Option<crate::router::AppRoute> {
        Some(<Self as crate::router::Viewable>::view_route(self))
    }

    fn font_awesome_icon(&self) -> Option<&str> {
        self.category.font_awesome_icon()
    }
}
