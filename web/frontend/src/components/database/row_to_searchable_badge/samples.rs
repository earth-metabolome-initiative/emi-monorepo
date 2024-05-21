use super::RowToSearchableBadge;
use crate::traits::format_match::FormatMatch;
use web_common::database::NestedSample;
use yew::prelude::*;

impl RowToSearchableBadge for NestedSample {
    fn to_datalist_badge(&self, query: &str) -> Html {
        html! {
            <div>
                <p>
                <i class="fas fa-question grey"></i>
                    <span>{self.inner.barcode_id.format_match(query)}</span>
                </p>
            </div>
        }
    }

    fn to_selected_datalist_badge(&self) -> Html {
        html! {
            <div>
                <p>
                <i class="fas fa-question grey"></i>
                <span>{self.inner.barcode_id.to_string()}</span>
                </p>
            </div>
        }
    }
    fn similarity_score(&self, query: &str) -> isize {
        self.inner.barcode_id.similarity_score(query)
    }
fn primary_color_class(&self) -> &str {
        "grey"
    }
fn description(&self) -> &str {
        ""
    }
}
