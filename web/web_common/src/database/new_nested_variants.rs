//! This module contains the new nested variants of the database models.
//!
//! This module is automatically generated. Do not write anything here.

use serde::{Deserialize, Serialize};
use super::*;

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
pub struct NestedNewProject {
    pub inner: NewProject,
    pub state: NestedProjectState,
    pub parent_project: Option<Project>,
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
pub struct NestedNewTeam {
    pub inner: NewTeam,
    pub parent_team: Option<Team>,
}

