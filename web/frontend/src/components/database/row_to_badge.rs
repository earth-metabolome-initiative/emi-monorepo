//! Submodule defining a trait to be implemented by types that can be converted to a badge.

pub mod procedures;
pub mod projects;
pub mod samples;
pub mod sampling_procedure;
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

impl RowToBadge for NestedContainerVerticalRule {}
impl RowToBadge for NestedContainerHorizontalRule {}
impl RowToBadge for NestedItemCategory {}
impl RowToBadge for NestedProjectRequirement {}
impl RowToBadge for NestedSampledIndividual {}
