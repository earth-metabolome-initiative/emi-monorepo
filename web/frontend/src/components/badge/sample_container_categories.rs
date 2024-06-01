use super::{Badge, BadgeSize, RowToBadge};
use web_common::database::NestedSampleContainerCategory;

impl RowToBadge for NestedSampleContainerCategory {
    fn badge_title(&self) -> String {
        self.inner.name.clone()
    }

    fn font_awesome_icon(&self) -> Option<&str> {
        self.icon.font_awesome_icon()
    }
}
