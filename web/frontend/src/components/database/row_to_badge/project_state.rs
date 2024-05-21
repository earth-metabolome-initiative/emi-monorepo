//! Submodule implementing the RowToBadge for the NestedProjectState struct.

use super::RowToBadge;
use web_common::database::NestedProjectState;

impl RowToBadge for NestedProjectState {
    fn to_badge(&self) -> yew::Html {
        yew::html! {
            <div class={format!("badge {}", self.color.name)} title={self.inner.description.clone()}>
                <p>
                    <i class={format!("fas fa-{}", self.icon.name)}></i>
                    {'\u{00a0}'}
                    <span>{self.inner.name.clone()}</span>
                </p>
            </div>
        }
    }
}
