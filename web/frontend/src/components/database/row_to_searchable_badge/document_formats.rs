use super::RowToSearchableBadge;
use crate::traits::format_match::FormatMatch;
use web_common::database::DocumentFormat;
use yew::prelude::*;

impl RowToSearchableBadge for DocumentFormat {
    fn to_datalist_badge(&self, query: &str) -> Html {
        html! {
            <div>
                <p>
                <i class="fas fa-question grey"></i>
                    <span>{self.extension.format_match(query)}</span>
                    <span>{self.mime_type.format_match(query)}</span>
                </p>
            </div>
        }
    }

    fn to_selected_datalist_badge(&self) -> Html {
        html! {
            <div>
                <p>
                <i class="fas fa-question grey"></i>
                    <span>{self.extension.clone()}</span>
                </p>
            </div>
        }
    }
    fn matches(&self, query: &str) -> bool {
        self.extension == query
    }
    fn similarity_score(&self, query: &str) -> isize {
        self.extension.similarity_score(query) + self.mime_type.similarity_score(query)
    }
    fn primary_color_class(&self) -> &str {
        "grey"
    }
    fn description(&self) -> &str {
        ""
    }
}
