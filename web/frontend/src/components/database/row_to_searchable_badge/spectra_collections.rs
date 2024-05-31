/// impl row to badge for project state
/// add function components
use crate::{
    components::database::row_to_searchable_badge::RowToSearchableBadge, traits::FormatMatch,
};
use web_common::database::NestedSpectraCollection;
use yew::prelude::*;

impl RowToSearchableBadge for NestedSpectraCollection {
    fn to_datalist_badge(&self, query: &str) -> Html {
        html! {
            <div>
                <p>
                    {self.sample.container.inner.barcode.format_match(query)}
                </p>
            </div>
        }
    }

    fn to_selected_datalist_badge(&self) -> Html {
        self.sample.to_selected_datalist_badge()
    }

    fn similarity_score(&self, query: &str) -> isize {
        // self.inner.name.similarity_score(query) + self.inner.description.similarity_score(query)
        0
    }

    fn primary_color_class(&self) -> &str {
        "grey"
    }

    fn description(&self) -> &str {
        ""
    }
}
