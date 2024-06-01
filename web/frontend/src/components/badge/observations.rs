use super::RowToBadge;
use web_common::database::NestedObservation;

impl RowToBadge for NestedObservation {
    fn badge_title(&self) -> String {
        "Observation yet to be named".to_string()
    }

    fn path(&self) -> Option<crate::router::AppRoute> {
        Some(<Self as crate::router::Viewable>::view_route(self))
    }

    fn font_awesome_icon(&self) -> Option<&str> {
        self.subject.font_awesome_icon()
    }
}
