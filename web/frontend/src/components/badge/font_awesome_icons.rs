use super::{Badge, BadgeSize, RowToBadge};
use web_common::database::FontAwesomeIcon;
use web_common::traits::CapitalizeString;

impl RowToBadge for FontAwesomeIcon {
    fn badge_title(&self) -> String {
        self.name.capitalize()
    }

    fn font_awesome_icon(&self) -> Option<&str> {
        Some(&self.name)
    }
}
