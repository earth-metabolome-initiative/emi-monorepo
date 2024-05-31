use super::RowToBadge;
use crate::components::database::row_to_searchable_badge::RowToSearchableBadge;
use web_common::database::Unit;

impl RowToBadge for Unit {
    fn to_badge(&self) -> yew::Html {
        self.to_searchable_badge(None)
    }

    fn to_small_badge(&self) -> yew::Html {
        self.to_searchable_small_badge(None)
    }
}
