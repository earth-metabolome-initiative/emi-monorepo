//! This module contains the new nested variants of the database models.
//!
//! This module is automatically generated. Do not write anything here.

use serde::{Deserialize, Serialize};
use super::*;

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Clone, Default)]
pub struct NestedNewBioOttRank {
    pub inner: NewBioOttRank,
    pub font_awesome_icon: FontAwesomeIcon,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Clone, Default)]
pub struct NestedNewBioOttTaxonItem {
    pub inner: NewBioOttTaxonItem,
    pub ott_rank: NestedBioOttRank,
    pub domain: Option<BioOttTaxonItem>,
    pub kingdom: Option<BioOttTaxonItem>,
    pub phylum: Option<BioOttTaxonItem>,
    pub class: Option<BioOttTaxonItem>,
    pub order: Option<BioOttTaxonItem>,
    pub family: Option<BioOttTaxonItem>,
    pub genus: Option<BioOttTaxonItem>,
    pub parent: BioOttTaxonItem,
    pub font_awesome_icon: FontAwesomeIcon,
    pub color: Color,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Clone, Default)]
pub struct NestedNewContainerHorizontalRule {
    pub inner: NewContainerHorizontalRule,
    pub item_type: NestedItemCategory,
    pub other_item_type: NestedItemCategory,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Clone, Default)]
pub struct NestedNewContainerVerticalRule {
    pub inner: NewContainerVerticalRule,
    pub container_item_type: NestedItemCategory,
    pub contained_item_type: NestedItemCategory,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Clone, Default)]
pub struct NestedNewItemCategory {
    pub inner: NewItemCategory,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Clone, Default)]
pub struct NestedNewItemCategoryRelationship {
    pub inner: NewItemCategoryRelationship,
    pub parent: NestedItemCategory,
    pub child: NestedItemCategory,
    pub added_by: User,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Clone, Default)]
pub struct NestedNewItemCategoryUnit {
    pub inner: NewItemCategoryUnit,
    pub item_category: NestedItemCategory,
    pub unit: Unit,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Clone, Default)]
pub struct NestedNewLoginProvider {
    pub inner: NewLoginProvider,
    pub font_awesome_icon: FontAwesomeIcon,
    pub color: Color,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Clone, Default)]
pub struct NestedNewManufacturedItemCategory {
    pub inner: NewManufacturedItemCategory,
    pub manifacturer: NestedOrganization,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Clone, Default)]
pub struct NestedNewNotification {
    pub inner: NewNotification,
    pub user: User,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Clone, Default)]
pub struct NestedNewOrganization {
    pub inner: NewOrganization,
    pub parent_organization: Option<Organization>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Clone, Default)]
pub struct NestedNewProcedure {
    pub inner: NewProcedure,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Clone, Default)]
pub struct NestedNewProjectRequirement {
    pub inner: NewProjectRequirement,
    pub project: NestedProject,
    pub item_category: NestedItemCategory,
    pub unit: Option<Unit>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Clone, Default)]
pub struct NestedNewProjectState {
    pub inner: NewProjectState,
    pub font_awesome_icon: FontAwesomeIcon,
    pub color: Color,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Clone, Default)]
pub struct NestedNewProject {
    pub inner: NewProject,
    pub state: NestedProjectState,
    pub parent_project: Option<Project>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Clone, Default)]
pub struct NestedNewSampleState {
    pub inner: NewSampleState,
    pub font_awesome_icon: FontAwesomeIcon,
    pub color: Color,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Clone, Default)]
pub struct NestedNewSample {
    pub inner: NewSample,
    pub sampled_by: User,
    pub procedure: NestedSamplingProcedure,
    pub state: NestedSampleState,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Clone, Default)]
pub struct NestedNewSamplingProcedure {
    pub inner: NewSamplingProcedure,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Clone, Default)]
pub struct NestedNewTeamState {
    pub inner: NewTeamState,
    pub font_awesome_icon: FontAwesomeIcon,
    pub color: Color,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Clone, Default)]
pub struct NestedNewTeam {
    pub inner: NewTeam,
    pub parent_team: Option<Team>,
}

