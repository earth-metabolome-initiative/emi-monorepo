use super::RowToBadge;
use crate::components::database::row_to_searchable_badge::RowToSearchableBadge;
use web_common::database::NestedBioOttRank;

impl RowToBadge for NestedBioOttRank {
    fn to_badge(&self) -> yew::Html {
        self.to_searchable_badge(None)
    }

    fn to_small_badge(&self) -> yew::Html {
        self.to_searchable_small_badge(None)
    }
}
