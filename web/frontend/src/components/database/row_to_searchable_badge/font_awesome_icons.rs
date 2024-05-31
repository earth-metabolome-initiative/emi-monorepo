use super::RowToSearchableBadge;
use crate::traits::format_match::FormatMatch;
use web_common::database::FontAwesomeIcon;
use yew::prelude::*;

impl RowToSearchableBadge for FontAwesomeIcon {
    fn to_searchable_badge(&self, query: Option<&str>) -> Html {
        html! {
            <div>
                <p>
                    <i class={format!("fas fa-{} grey", self.name)}></i>
                    <span>{self.name.maybe_format_match(query)}</span>
                </p>
                <p>{self.description.maybe_format_match(query)}</p>
            </div>
        }
    }
    fn similarity_score(&self, query: &str) -> isize {
        self.name.similarity_score(query)
    }
    fn primary_color_class(&self) -> &str {
        "grey"
    }
}
