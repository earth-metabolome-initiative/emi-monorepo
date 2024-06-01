//! Defines a badge component that can be used to display a badge.

use crate::router::AppRoute;
use crate::traits::format_match::FormatMatch;
use core::fmt::Debug;
use web_common::database::{Colorable, Describable};
use yew::prelude::*;
use yew_router::prelude::*;

pub mod bio_ott_ranks;
pub mod bio_ott_taxon_items;
pub mod colors;
pub mod countries;
pub mod document_formats;
pub mod font_awesome_icons;
pub mod nameplate_categories;
pub mod nameplates;
pub mod observation_subjects;
pub mod observations;
pub mod organisms;
pub mod organizations;
pub mod project_states;
pub mod projects;
pub mod roles;
pub mod sample_container_categories;
pub mod sample_containers;
pub mod sample_states;
pub mod samples;
pub mod spectra;
pub mod spectra_collections;
pub mod team_states;
pub mod teams;
pub mod units;
pub mod users;

/// Trait for types that can be converted to a badge.
pub trait RowToBadge: Colorable + Describable + Sized + Clone + PartialEq + Debug {
    /// Returns the similarity score of the implementing type with respect to the query.
    ///
    /// # Arguments
    /// * `query` - The query to compare the implementing type with.
    fn similarity_score(&self, query: &str) -> isize {
        self.badge_title().similarity_score(query)
            + self.description().map_or(0, |d| d.similarity_score(query))
    }

    /// Returns the title for the badge.
    fn badge_title(&self) -> String;

    /// Returns the optional path where the badge should link to.
    fn path(&self) -> Option<AppRoute> {
        None
    }

    /// Returns the optional primary image URL of the implementing type.
    fn primary_image_url(&self) -> Option<String> {
        None
    }

    /// Returns the optional font-awesome icon of the implementing type.
    fn font_awesome_icon(&self) -> Option<&str> {
        None
    }
}

/// Enumeration of the badge sizes.
#[derive(Clone, Debug, PartialEq)]
pub enum BadgeSize {
    Small,
    Large,
}

impl BadgeSize {
    /// Returns whether the badge size is small.
    pub fn is_small(&self) -> bool {
        matches!(self, BadgeSize::Small)
    }

    /// Returns whether the badge size is large.
    pub fn is_large(&self) -> bool {
        matches!(self, BadgeSize::Large)
    }
}

/// Properties to be used for the badge component.
#[derive(Clone, Debug, PartialEq, Properties)]
pub struct BadgeProps<B>
where
    B: RowToBadge,
{
    pub badge: B,
    #[prop_or(false)]
    pub closable: bool,
    #[prop_or(BadgeSize::Large)]
    pub size: BadgeSize,
    #[prop_or_default]
    pub query: Option<String>,
    #[prop_or_default]
    pub on_close: Callback<()>,
    #[prop_or(false)]
    pub li: bool
}

/// A badge component that can be used to display a badge.
#[function_component(Badge)]
pub fn badge<B: RowToBadge>(props: &BadgeProps<B>) -> Html {
    let navigator = use_navigator().unwrap();

    let onclick = {
        let path = props.badge.path();
        Callback::from(move |e: MouseEvent| {
            e.prevent_default();
            e.stop_immediate_propagation();
            if let Some(path) = path {
                navigator.push(&path);
            }
        })
    };

    let badge_classes = format!(
        "badge{}{}{}{}",
        match props.size {
            BadgeSize::Small => " small",
            BadgeSize::Large => "",
        },
        match props.badge.path() {
            Some(_) => " clickable",
            None => "",
        },
        match props.badge.primary_image_url() {
            Some(_) => " has-image",
            None => "",
        },
        match props.badge.color() {
            Some(color) => format!(" {}", color),
            None => "".to_owned(),
        }
    );

    let tag = if props.li {
        "li"
    } else {
        "div"
    };

    html! {
        <@{tag} class={badge_classes} onclick={onclick}>
            <p class="badge-title">
                if let Some(icon) = props.badge.font_awesome_icon() {
                    <>
                        <i class={format!("fas fa-{}{}", icon, match props.badge.color() {
                            Some(color) => format!(" {}", color),
                            None => "".to_owned(),
                        })}></i>
                        {'\u{00A0}'}
                    </>
                }
                <strong>{props.badge.badge_title().maybe_format_match(props.query.clone())}</strong>
            </p>
            if props.size.is_large() {
                if let Some(description) = props.badge.description() {
                    <p class="badge-description">
                        {description.to_owned().maybe_format_match(props.query.clone())}
                    </p>
                }
            }
        </@>
    }
}
