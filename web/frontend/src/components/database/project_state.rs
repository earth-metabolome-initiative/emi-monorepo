/// impl row to badge for project state
/// add function components
use crate::components::database::row_to_badge::RowToBadge;
use sublime_fuzzy::{best_match, format_simple};
use web_common::database::ProjectState;
use yew::prelude::*;

impl RowToBadge for ProjectState {
    fn to_badge(&self, query: Option<&str>) -> Html {
        let formatted_name = query.map_or_else(
            || self.name.clone(),
            |query| {
                best_match(&query, &self.name).map_or_else(
                    || self.name.clone(),
                    |match_value| format_simple(&match_value, &self.name, "<strong>", "</strong>"),
                )
            },
        );
        let formatted_name = Html::from_html_unchecked(AttrValue::from(formatted_name));

        let formatted_description = query.map_or_else(
            || self.description.clone(),
            |query| {
                best_match(&query, &self.description).map_or_else(
                    || self.description.clone(),
                    |match_value| {
                        format_simple(&match_value, &self.description, "<strong>", "</strong>")
                    },
                )
            },
        );
        let formatted_description =
            Html::from_html_unchecked(AttrValue::from(formatted_description));
        html! {
            <div class="datalist-badge">
                <div>
                    <i class={format!("{} {}", self.font_awesome_icon, self.icon_color)}></i>
                    <span>{formatted_name}</span>
                </div>
                <p>{formatted_description}</p>
            </div>
        }
    }
}
