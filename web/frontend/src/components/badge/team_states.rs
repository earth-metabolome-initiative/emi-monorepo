use super::RowToBadge;
use web_common::database::NestedTeamState;

impl RowToBadge for NestedTeamState {
    fn badge_title(&self) -> String {
        self.inner.name.clone()
    }

    fn font_awesome_icon(&self) -> Option<&str> {
        self.icon.font_awesome_icon()
    }
}
