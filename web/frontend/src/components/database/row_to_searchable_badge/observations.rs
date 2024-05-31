/// impl row to badge for project state
/// add function components
use crate::{
    components::database::row_to_searchable_badge::RowToSearchableBadge, traits::FormatMatch,
};
use web_common::database::NestedObservation;
use yew::prelude::*;

impl RowToSearchableBadge for NestedObservation {
    fn to_searchable_badge(&self, query: Option<&str>) -> Html {
        html! {
            <div>
                <p>
                    {"OBSERVATION"}
                </p>
            </div>
        }
    }

    fn similarity_score(&self, query: &str) -> isize {
        0
    }

    fn primary_color_class(&self) -> &str {
        "gray"
    }
}
