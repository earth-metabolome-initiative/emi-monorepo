use super::RowToBadge;
use web_common::{database::NestedRank, traits::CapitalizeString};

impl RowToBadge for NestedRank {
    fn badge_title(&self) -> String {
        self.inner.name.capitalize()
    }

    fn font_awesome_icon(&self) -> Option<&str> {
        self.icon.font_awesome_icon()
    }

    fn path(&self) -> Option<crate::router::AppRoute> {
        Some(<Self as crate::router::Viewable>::view_route(self))
    }
}
