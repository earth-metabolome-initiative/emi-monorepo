use super::RowToBadge;
use web_common::{database::Country, traits::CapitalizeString};

impl RowToBadge for Country {
    fn badge_title(&self) -> String {
        format!("{} {}", self.emoji, self.name.capitalize())
    }

    fn path(&self) -> Option<crate::router::AppRoute> {
        Some(<Self as crate::router::Viewable>::view_route(self))
    }
}
