use super::RowToBadge;
use crate::traits::format_match::FormatMatch;
use web_common::database::NestedSampleState;
use yew::prelude::*;

impl RowToBadge for NestedSampleState {
    fn to_datalist_badge(&self, query: &str) -> Html {
        html! {
            <div>
                <p>
                <i class={format!("{} {}", self.font_awesome_icon.name, self.color.name)}></i>
                    <span>{self.inner.name.format_match(query)}</span>
                    <span>{self.inner.description.format_match(query)}</span>
                </p>
            </div>
        }
    }

    fn to_selected_datalist_badge(&self) -> Html {
        html! {
            <div>
                <p>
                <i class={format!("{} {}", self.font_awesome_icon.name, self.color.name)}></i>
