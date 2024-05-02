use super::RowToBadge;
use crate::traits::format_match::FormatMatch;
use web_common::database::NestedBioOttRank;
use yew::prelude::*;

impl RowToBadge for NestedBioOttRank {
    fn to_datalist_badge(&self, query: &str) -> Html {
        html! {
            <div>
                <p>
                <i class={format!("{} grey", self.font_awesome_icon.name)}></i>
                    <span>{self.inner.name.format_match(query)}</span>
                </p>
            </div>
        }
    }

    fn to_selected_datalist_badge(&self) -> Html {
        html! {
            <div>
                <p>
                <i class={format!("{} grey", self.font_awesome_icon.name)}></i>
                    <span>{self.inner.name.clone()}</span>
                </p>
            </div>
        }
    }
    fn matches(&self, query: &str) -> bool {
        self.inner.name == query
    }
    fn similarity_score(&self, query: &str) -> isize {
        self.inner.name.similarity_score(query)
    }
fn primary_color_class(&self) -> &str {
        "grey"
    }
fn description(&self) -> &str {
        ""
    }
}
