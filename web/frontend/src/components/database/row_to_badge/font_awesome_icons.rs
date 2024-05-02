use super::RowToBadge;
use crate::traits::format_match::FormatMatch;
use web_common::database::FontAwesomeIcon;
use yew::prelude::*;

impl RowToBadge for FontAwesomeIcon {
    fn to_datalist_badge(&self, query: &str) -> Html {
        html! {
            <div>
                <p>
                <i class="fas fa-question grey"></i>
                    <span>{self.name.format_match(query)}</span>
                </p>
            </div>
        }
    }

    fn to_selected_datalist_badge(&self) -> Html {
        html! {
            <div>
                <p>
                <i class="fas fa-question grey"></i>
                    <span>{self.name.clone()}</span>
                </p>
            </div>
        }
    }
    fn matches(&self, query: &str) -> bool {
        self.name == query
    }
    fn similarity_score(&self, query: &str) -> isize {
        self.name.similarity_score(query)
    }
    fn primary_color_class(&self) -> &str {
        "grey"
    }
    fn description(&self) -> &str {
        ""
    }
}
