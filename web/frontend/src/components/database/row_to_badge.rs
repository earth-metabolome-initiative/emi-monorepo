//! Submodule defining a trait to be implemented by types that can be converted to a badge.

pub mod bio_ott_ranks;
pub mod bio_ott_taxon_items;
pub mod colors;
pub mod document_formats;
pub mod font_awesome_icons;
pub mod observation_subjects;
pub mod observations;
pub mod organizations;
pub mod project_state;
pub mod projects;
pub mod roles;
pub mod sample_container;
pub mod sample_states;
pub mod samples;
pub mod team_states;
pub mod teams;
pub mod units;
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

    fn to_small_badge(&self) -> yew::Html {
        self.to_badge()
    }
}

impl RowToBadge for NestedOrganism {}
impl RowToBadge for NestedSpectraCollection {}
impl RowToBadge for NestedSpectra {}
impl RowToBadge for Country {}
impl RowToBadge for NestedSampleContainerCategory {}
impl RowToBadge for NestedNameplateCategory {}
impl RowToBadge for NestedNameplate {}
