use super::RowToBadge;
use web_common::database::User;

impl RowToBadge for User {
    fn badge_title(&self) -> String {
        self.full_name()
    }

    fn path(&self) -> Option<crate::router::AppRoute> {
        Some(<Self as crate::router::Viewable>::view_route(self))
    }
}
