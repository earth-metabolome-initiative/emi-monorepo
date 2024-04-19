/// impl row to badge for project state
/// add function components
use crate::{components::database::row_to_badge::RowToBadge, traits::FormatMatch};
use web_common::database::ProjectState;
use yew::prelude::*;

impl RowToBadge for ProjectState {
    fn to_badge(&self, query: Option<&str>) -> Html {
        html! {
            <div class="datalist-badge">
                <div>
                    <i class={format!("{} {}", self.font_awesome_icon, self.icon_color)}></i>
                    <span>{self.name.format_match(query)}</span>
                </div>
                <p>{self.description.format_match(query)}</p>
            </div>
        }
    }
}
