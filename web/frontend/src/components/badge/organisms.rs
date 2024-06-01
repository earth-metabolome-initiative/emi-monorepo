use super::RowToBadge;
use web_common::database::NestedOrganism;

impl RowToBadge for NestedOrganism {
    fn badge_title(&self) -> String {
        self.nameplate.badge_title()
    }

    fn path(&self) -> Option<crate::router::AppRoute> {
        Some(<Self as crate::router::Viewable>::view_route(self))
    }
}
