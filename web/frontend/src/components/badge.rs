//! Defines a badge component that can be used to display a badge.

use crate::router::AppRoute;
use crate::traits::format_match::FormatMatch;
use core::fmt::Debug;
use std::rc::Rc;
use web_common::database::{Colorable, Describable, SimilarityScore};
use yew::prelude::*;
use yew_router::prelude::*;

pub mod bio_ott_ranks;
pub mod bio_ott_taxon_items;
pub mod colors;
pub mod countries;
pub mod derived_samples;
pub mod document_formats;
pub mod font_awesome_icons;
pub mod materials;
pub mod nameplate_categories;
pub mod nameplates;
pub mod observation_subjects;
pub mod observations;
pub mod organism_bio_ott_taxon_items;
pub mod organisms;
pub mod organizations;
pub mod permanence_categories;
pub mod project_states;
pub mod projects;
pub mod projects_teams_role_invitations;
pub mod projects_teams_role_requests;
pub mod projects_teams_roles;
pub mod projects_users_role_invitations;
pub mod projects_users_role_requests;
pub mod projects_users_roles;
pub mod roles;
pub mod sample_bio_ott_taxon_items;
pub mod sample_container_categories;
pub mod sample_containers;
pub mod sample_states;
pub mod samples;
pub mod searchable_struct;
pub mod spectra_collections;
pub mod team_states;
pub mod teams;
pub mod teams_teams_role_invitations;
pub mod teams_users_role_invitations;
pub mod teams_users_role_requests;
pub mod teams_users_roles;
pub mod units;
pub mod users;
pub mod users_users_role_invitations;
pub mod users_users_role_requests;
pub mod users_users_roles;

/// Trait for types that can be converted to a badge.
pub trait RowToBadge:
    Colorable + Describable + Sized + Clone + PartialEq + Debug + SimilarityScore
{
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

    /// Returns the child badges of the implementing type.
    ///
    /// # Arguments
    /// * `props` - The properties to be used for the child badge component.
    fn children(&self, _props: &BadgeProps<Self>) -> Option<Html> {
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
    pub badge: Rc<B>,
    #[prop_or(false)]
    pub closable: bool,
    #[prop_or(BadgeSize::Large)]
    pub size: BadgeSize,
    #[prop_or_default]
    pub query: Option<Rc<String>>,
    #[prop_or_default]
    pub onclick: Option<Callback<MouseEvent>>,
    #[prop_or(false)]
    pub li: bool,
}

impl<B: RowToBadge> BadgeProps<B> {
    pub fn to_child_props<C: RowToBadge>(&self, child: Rc<C>) -> BadgeProps<C> {
        BadgeProps {
            badge: child,
            closable: self.closable,
            size: BadgeSize::Small,
            query: self.query.clone(),
            onclick: self.onclick.clone(),
            li: self.li,
        }
    }
}

#[function_component(Badge)]
pub fn badge<B: RowToBadge>(props: &BadgeProps<B>) -> Html {
    let navigator = use_navigator().unwrap();
    let onclick = {
        let onclick: Option<Callback<MouseEvent>> = props.onclick.clone();
        let path = props.badge.path();
        Callback::from(move |e: MouseEvent| {
            e.prevent_default();
            e.stop_immediate_propagation();
            if let Some(onclick) = onclick.as_ref() {
                onclick.emit(e);
            } else if let Some(path) = path {
                navigator.push(&path);
            }
        })
    };

    let badge_classes = format!(
        "badge{}{}{}{}{}",
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
        },
        match props.closable {
            true => " closable",
            false => "",
        }
    );

    let tag = if props.li { "li" } else { "div" };

    html! {
        <@{tag} class={badge_classes} onclick={onclick}>
            if let Some(image_url) = props.badge.primary_image_url() {
                <img class="badge-image" src={image_url} />
            }
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
                <span>{props.badge.badge_title().maybe_format_match(props.query.as_deref())}</span>
            </p>
            if props.size.is_large() {
                if let Some(description) = props.badge.description() {
                    <p class="badge-description">
                        {description.to_owned().maybe_format_match(props.query.as_deref())}
                    </p>
                }
                if let Some(children) = props.badge.children(props) {
                    <ul class="badges-list">
                        {children}
                    </ul>
                }
            }
        </@>
    }
}
