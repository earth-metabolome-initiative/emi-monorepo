use super::RowToBadge;
use web_common::database::NestedPermanenceCategory;

impl RowToBadge for NestedPermanenceCategory {
    fn badge_title(&self) -> String {
        self.inner.name.clone()
    }

    fn font_awesome_icon(&self) -> Option<&str> {
        self.icon.font_awesome_icon()
    }
}
