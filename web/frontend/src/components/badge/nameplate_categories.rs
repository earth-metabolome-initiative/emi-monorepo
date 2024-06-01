use super::{Badge, BadgeSize, RowToBadge};
use web_common::database::NestedNameplateCategory;
use web_common::traits::CapitalizeString;

impl RowToBadge for NestedNameplateCategory {
    fn badge_title(&self) -> String {
        self.inner.name.capitalize()
    }

    fn font_awesome_icon(&self) -> Option<&str> {
        self.icon.font_awesome_icon()
    }
}
