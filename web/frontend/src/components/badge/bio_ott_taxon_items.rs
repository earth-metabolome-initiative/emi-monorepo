use super::RowToBadge;
use web_common::{database::NestedBioOttTaxonItem, traits::CapitalizeString};

impl RowToBadge for NestedBioOttTaxonItem {
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
