use super::RowToBadge;
use web_common::database::Color;
use web_common::traits::CapitalizeString;

impl RowToBadge for Color {
    fn badge_title(&self) -> String {
        self.name.capitalize()
    }

    fn font_awesome_icon(&self) -> Option<&str> {
        Some("palette")
    }
}
