//! Submodule defining a trait to be implemented by types that can be converted to a badge.

pub mod projects;
pub mod samples;
pub mod teams;
pub mod users;
use web_common::database::*;

/// Trait for types that can be converted to a badge.
pub trait RowToBadge {
    /// Convert the implementing type to a badge.

    fn to_badge(&self) -> yew::Html {
        yew::html! {
            <div class="badge">{ "badge" }</div>
        }
    }
}

impl RowToBadge for NestedSampledIndividual {}
impl RowToBadge for NestedSpectraCollection {}
impl RowToBadge for NestedSpectra {}
