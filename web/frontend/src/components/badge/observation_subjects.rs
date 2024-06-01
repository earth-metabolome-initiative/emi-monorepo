use super::{Badge, BadgeSize, RowToBadge};
use web_common::database::NestedObservationSubject;
use web_common::traits::CapitalizeString;

impl RowToBadge for NestedObservationSubject {
    fn badge_title(&self) -> String {
        self.inner.name.capitalize()
    }

    fn path(&self) -> Option<crate::router::AppRoute> {
        Some(<Self as crate::router::Viewable>::view_route(self))
    }

    fn font_awesome_icon(&self) -> Option<&str> {
        self.icon.font_awesome_icon()
    }
}
