//! This module contains the table names enumeration.
//!
//! This module is automatically generated. Do not write anything here.

use crate::database::*;
use std::rc::Rc;

pub trait Searchable<const EDIT: bool>: Filtrable {
    fn search_task(
        filter: Option<&Self::Filter>,
        query: String,
        limit: i64,
        offset: i64,
    ) -> super::Select;
}

pub trait SimilarityScore {
    /// Returns the similarity score of the struct with respect to the query.
    ///
    /// # Arguments
    /// * `lowercase_query` - The lowercase query string.
    ///
    /// # Implementative details
    /// The similarity score is a floating point number between 0 and 1.
    /// The higher the score, the more similar the struct is to the query.
    /// We obtain this score by averaging the Normalized Damerau-Levenshtein similarity
    /// score of each attribute of the struct that implements AsRef<str>, plus all of the
    /// similarity scores of the children structs that also implement SimilarityScore.
    fn similarity_score<S: AsRef<str>>(&self, lowercase_query: S) -> f64;
}

impl SimilarityScore for NestedDerivedSample {
    fn similarity_score<S: AsRef<str>>(&self, lowercase_query: S) -> f64 {
        let mut number_of_attributes = 0;
        let mut total_similarity_score = 0.0;
        number_of_attributes += 1;
        total_similarity_score += self.created_by.similarity_score(lowercase_query.as_ref());
        number_of_attributes += 1;
        total_similarity_score += self.updated_by.similarity_score(lowercase_query.as_ref());
        number_of_attributes += 1;
        total_similarity_score += self
            .parent_sample
            .similarity_score(lowercase_query.as_ref());
        number_of_attributes += 1;
        total_similarity_score += self.child_sample.similarity_score(lowercase_query.as_ref());
        number_of_attributes += 1;
        total_similarity_score += self.unit.similarity_score(lowercase_query.as_ref());
        total_similarity_score / number_of_attributes as f64
    }
}
impl Searchable<false> for NestedDerivedSample {
    fn search_task(
        filter: Option<&Self::Filter>,
        query: String,
        limit: i64,
        offset: i64,
    ) -> super::Select {
        super::Select::search(Table::DerivedSamples, filter, query, limit, offset)
    }
}
impl Searchable<true> for NestedDerivedSample {
    fn search_task(
        filter: Option<&Self::Filter>,
        query: String,
        limit: i64,
        offset: i64,
    ) -> super::Select {
        super::Select::search_updatables(Table::DerivedSamples, filter, query, limit, offset)
    }
}
impl SimilarityScore for NestedDocumentFormat {
    fn similarity_score<S: AsRef<str>>(&self, lowercase_query: S) -> f64 {
        let mut number_of_attributes = 0;
        let mut total_similarity_score = 0.0;
        number_of_attributes += 1;
        total_similarity_score += self.inner.similarity_score(lowercase_query.as_ref());
        number_of_attributes += 1;
        total_similarity_score += self.icon.similarity_score(lowercase_query.as_ref());
        number_of_attributes += 1;
        total_similarity_score += self.color.similarity_score(lowercase_query.as_ref());
        total_similarity_score / number_of_attributes as f64
    }
}
impl Searchable<false> for NestedDocumentFormat {
    fn search_task(
        filter: Option<&Self::Filter>,
        query: String,
        limit: i64,
        offset: i64,
    ) -> super::Select {
        super::Select::search(Table::DocumentFormats, filter, query, limit, offset)
    }
}
impl SimilarityScore for NestedLoginProvider {
    fn similarity_score<S: AsRef<str>>(&self, lowercase_query: S) -> f64 {
        let mut number_of_attributes = 0;
        let mut total_similarity_score = 0.0;
        number_of_attributes += 1;
        total_similarity_score += self.inner.similarity_score(lowercase_query.as_ref());
        number_of_attributes += 1;
        total_similarity_score += self.icon.similarity_score(lowercase_query.as_ref());
        number_of_attributes += 1;
        total_similarity_score += self.color.similarity_score(lowercase_query.as_ref());
        total_similarity_score / number_of_attributes as f64
    }
}
impl SimilarityScore for NestedMaterial {
    fn similarity_score<S: AsRef<str>>(&self, lowercase_query: S) -> f64 {
        let mut number_of_attributes = 0;
        let mut total_similarity_score = 0.0;
        number_of_attributes += 1;
        total_similarity_score += self.inner.similarity_score(lowercase_query.as_ref());
        number_of_attributes += 1;
        total_similarity_score += self.icon.similarity_score(lowercase_query.as_ref());
        number_of_attributes += 1;
        total_similarity_score += self.color.similarity_score(lowercase_query.as_ref());
        total_similarity_score / number_of_attributes as f64
    }
}
impl SimilarityScore for NestedNameplateCategory {
    fn similarity_score<S: AsRef<str>>(&self, lowercase_query: S) -> f64 {
        let mut number_of_attributes = 0;
        let mut total_similarity_score = 0.0;
        number_of_attributes += 1;
        total_similarity_score += self.inner.similarity_score(lowercase_query.as_ref());
        number_of_attributes += 1;
        total_similarity_score += self.permanence.similarity_score(lowercase_query.as_ref());
        number_of_attributes += 1;
        total_similarity_score += self.material.similarity_score(lowercase_query.as_ref());
        number_of_attributes += 1;
        total_similarity_score += self.icon.similarity_score(lowercase_query.as_ref());
        number_of_attributes += 1;
        total_similarity_score += self.color.similarity_score(lowercase_query.as_ref());
        total_similarity_score / number_of_attributes as f64
    }
}
impl Searchable<false> for NestedNameplateCategory {
    fn search_task(
        filter: Option<&Self::Filter>,
        query: String,
        limit: i64,
        offset: i64,
    ) -> super::Select {
        super::Select::search(Table::NameplateCategories, filter, query, limit, offset)
    }
}
impl SimilarityScore for NestedNameplate {
    fn similarity_score<S: AsRef<str>>(&self, lowercase_query: S) -> f64 {
        let mut number_of_attributes = 0;
        let mut total_similarity_score = 0.0;
        number_of_attributes += 1;
        total_similarity_score += self.inner.similarity_score(lowercase_query.as_ref());
        number_of_attributes += 1;
        total_similarity_score += self.project.similarity_score(lowercase_query.as_ref());
        number_of_attributes += 1;
        total_similarity_score += self.category.similarity_score(lowercase_query.as_ref());
        number_of_attributes += 1;
        total_similarity_score += self.created_by.similarity_score(lowercase_query.as_ref());
        number_of_attributes += 1;
        total_similarity_score += self.updated_by.similarity_score(lowercase_query.as_ref());
        total_similarity_score / number_of_attributes as f64
    }
}
impl Searchable<false> for NestedNameplate {
    fn search_task(
        filter: Option<&Self::Filter>,
        query: String,
        limit: i64,
        offset: i64,
    ) -> super::Select {
        super::Select::search(Table::Nameplates, filter, query, limit, offset)
    }
}
impl Searchable<true> for NestedNameplate {
    fn search_task(
        filter: Option<&Self::Filter>,
        query: String,
        limit: i64,
        offset: i64,
    ) -> super::Select {
        super::Select::search_updatables(Table::Nameplates, filter, query, limit, offset)
    }
}
impl SimilarityScore for NestedNotification {
    fn similarity_score<S: AsRef<str>>(&self, lowercase_query: S) -> f64 {
        let mut number_of_attributes = 0;
        let mut total_similarity_score = 0.0;
        number_of_attributes += 1;
        total_similarity_score += self.inner.similarity_score(lowercase_query.as_ref());
        number_of_attributes += 1;
        total_similarity_score += self.user.similarity_score(lowercase_query.as_ref());
        total_similarity_score / number_of_attributes as f64
    }
}
impl SimilarityScore for NestedObservationSubject {
    fn similarity_score<S: AsRef<str>>(&self, lowercase_query: S) -> f64 {
        let mut number_of_attributes = 0;
        let mut total_similarity_score = 0.0;
        number_of_attributes += 1;
        total_similarity_score += self.inner.similarity_score(lowercase_query.as_ref());
        number_of_attributes += 1;
        total_similarity_score += self.icon.similarity_score(lowercase_query.as_ref());
        number_of_attributes += 1;
        total_similarity_score += self.color.similarity_score(lowercase_query.as_ref());
        total_similarity_score / number_of_attributes as f64
    }
}
impl Searchable<false> for NestedObservationSubject {
    fn search_task(
        filter: Option<&Self::Filter>,
        query: String,
        limit: i64,
        offset: i64,
    ) -> super::Select {
        super::Select::search(Table::ObservationSubjects, filter, query, limit, offset)
    }
}
impl SimilarityScore for NestedObservation {
    fn similarity_score<S: AsRef<str>>(&self, lowercase_query: S) -> f64 {
        let mut number_of_attributes = 0;
        let mut total_similarity_score = 0.0;
        number_of_attributes += 1;
        total_similarity_score += self.inner.similarity_score(lowercase_query.as_ref());
        if let Some(parent_observation) = &self.parent_observation {
            number_of_attributes += 1;
            total_similarity_score += parent_observation.similarity_score(lowercase_query.as_ref());
        }
        number_of_attributes += 1;
        total_similarity_score += self.created_by.similarity_score(lowercase_query.as_ref());
        number_of_attributes += 1;
        total_similarity_score += self.updated_by.similarity_score(lowercase_query.as_ref());
        number_of_attributes += 1;
        total_similarity_score += self.project.similarity_score(lowercase_query.as_ref());
        if let Some(organism) = &self.organism {
            number_of_attributes += 1;
            total_similarity_score += organism.similarity_score(lowercase_query.as_ref());
        }
        if let Some(sample) = &self.sample {
            number_of_attributes += 1;
            total_similarity_score += sample.similarity_score(lowercase_query.as_ref());
        }
        number_of_attributes += 1;
        total_similarity_score += self.subject.similarity_score(lowercase_query.as_ref());
        total_similarity_score / number_of_attributes as f64
    }
}
impl Searchable<false> for NestedObservation {
    fn search_task(
        filter: Option<&Self::Filter>,
        query: String,
        limit: i64,
        offset: i64,
    ) -> super::Select {
        super::Select::search(Table::Observations, filter, query, limit, offset)
    }
}
impl Searchable<true> for NestedObservation {
    fn search_task(
        filter: Option<&Self::Filter>,
        query: String,
        limit: i64,
        offset: i64,
    ) -> super::Select {
        super::Select::search_updatables(Table::Observations, filter, query, limit, offset)
    }
}
impl SimilarityScore for NestedOrganismTaxon {
    fn similarity_score<S: AsRef<str>>(&self, lowercase_query: S) -> f64 {
        let mut number_of_attributes = 0;
        let mut total_similarity_score = 0.0;
        number_of_attributes += 1;
        total_similarity_score += self.created_by.similarity_score(lowercase_query.as_ref());
        number_of_attributes += 1;
        total_similarity_score += self.organism.similarity_score(lowercase_query.as_ref());
        number_of_attributes += 1;
        total_similarity_score += self.taxon.similarity_score(lowercase_query.as_ref());
        total_similarity_score / number_of_attributes as f64
    }
}
impl Searchable<false> for NestedOrganismTaxon {
    fn search_task(
        filter: Option<&Self::Filter>,
        query: String,
        limit: i64,
        offset: i64,
    ) -> super::Select {
        super::Select::search(Table::OrganismTaxa, filter, query, limit, offset)
    }
}
impl SimilarityScore for NestedOrganism {
    fn similarity_score<S: AsRef<str>>(&self, lowercase_query: S) -> f64 {
        let mut number_of_attributes = 0;
        let mut total_similarity_score = 0.0;
        number_of_attributes += 1;
        total_similarity_score += self.inner.similarity_score(lowercase_query.as_ref());
        if let Some(host_organism) = &self.host_organism {
            number_of_attributes += 1;
            total_similarity_score += host_organism.similarity_score(lowercase_query.as_ref());
        }
        if let Some(sample) = &self.sample {
            number_of_attributes += 1;
            total_similarity_score += sample.similarity_score(lowercase_query.as_ref());
        }
        number_of_attributes += 1;
        total_similarity_score += self.nameplate.similarity_score(lowercase_query.as_ref());
        number_of_attributes += 1;
        total_similarity_score += self.project.similarity_score(lowercase_query.as_ref());
        number_of_attributes += 1;
        total_similarity_score += self.created_by.similarity_score(lowercase_query.as_ref());
        number_of_attributes += 1;
        total_similarity_score += self.updated_by.similarity_score(lowercase_query.as_ref());
        total_similarity_score / number_of_attributes as f64
    }
}
impl Searchable<false> for NestedOrganism {
    fn search_task(
        filter: Option<&Self::Filter>,
        query: String,
        limit: i64,
        offset: i64,
    ) -> super::Select {
        super::Select::search(Table::Organisms, filter, query, limit, offset)
    }
}
impl Searchable<true> for NestedOrganism {
    fn search_task(
        filter: Option<&Self::Filter>,
        query: String,
        limit: i64,
        offset: i64,
    ) -> super::Select {
        super::Select::search_updatables(Table::Organisms, filter, query, limit, offset)
    }
}
impl SimilarityScore for NestedOrganization {
    fn similarity_score<S: AsRef<str>>(&self, lowercase_query: S) -> f64 {
        let mut number_of_attributes = 0;
        let mut total_similarity_score = 0.0;
        number_of_attributes += 1;
        total_similarity_score += self.inner.similarity_score(lowercase_query.as_ref());
        number_of_attributes += 1;
        total_similarity_score += self.country.similarity_score(lowercase_query.as_ref());
        total_similarity_score / number_of_attributes as f64
    }
}
impl Searchable<false> for NestedOrganization {
    fn search_task(
        filter: Option<&Self::Filter>,
        query: String,
        limit: i64,
        offset: i64,
    ) -> super::Select {
        super::Select::search(Table::Organizations, filter, query, limit, offset)
    }
}
impl SimilarityScore for NestedPermanenceCategory {
    fn similarity_score<S: AsRef<str>>(&self, lowercase_query: S) -> f64 {
        let mut number_of_attributes = 0;
        let mut total_similarity_score = 0.0;
        number_of_attributes += 1;
        total_similarity_score += self.inner.similarity_score(lowercase_query.as_ref());
        number_of_attributes += 1;
        total_similarity_score += self.icon.similarity_score(lowercase_query.as_ref());
        number_of_attributes += 1;
        total_similarity_score += self.color.similarity_score(lowercase_query.as_ref());
        total_similarity_score / number_of_attributes as f64
    }
}
impl SimilarityScore for NestedProjectState {
    fn similarity_score<S: AsRef<str>>(&self, lowercase_query: S) -> f64 {
        let mut number_of_attributes = 0;
        let mut total_similarity_score = 0.0;
        number_of_attributes += 1;
        total_similarity_score += self.inner.similarity_score(lowercase_query.as_ref());
        number_of_attributes += 1;
        total_similarity_score += self.icon.similarity_score(lowercase_query.as_ref());
        number_of_attributes += 1;
        total_similarity_score += self.color.similarity_score(lowercase_query.as_ref());
        total_similarity_score / number_of_attributes as f64
    }
}
impl Searchable<false> for NestedProjectState {
    fn search_task(
        filter: Option<&Self::Filter>,
        query: String,
        limit: i64,
        offset: i64,
    ) -> super::Select {
        super::Select::search(Table::ProjectStates, filter, query, limit, offset)
    }
}
impl SimilarityScore for NestedProject {
    fn similarity_score<S: AsRef<str>>(&self, lowercase_query: S) -> f64 {
        let mut number_of_attributes = 0;
        let mut total_similarity_score = 0.0;
        number_of_attributes += 1;
        total_similarity_score += self.inner.similarity_score(lowercase_query.as_ref());
        number_of_attributes += 1;
        total_similarity_score += self.state.similarity_score(lowercase_query.as_ref());
        number_of_attributes += 1;
        total_similarity_score += self.icon.similarity_score(lowercase_query.as_ref());
        number_of_attributes += 1;
        total_similarity_score += self.color.similarity_score(lowercase_query.as_ref());
        if let Some(parent_project) = &self.parent_project {
            number_of_attributes += 1;
            total_similarity_score += parent_project.similarity_score(lowercase_query.as_ref());
        }
        number_of_attributes += 1;
        total_similarity_score += self.created_by.similarity_score(lowercase_query.as_ref());
        number_of_attributes += 1;
        total_similarity_score += self.updated_by.similarity_score(lowercase_query.as_ref());
        total_similarity_score / number_of_attributes as f64
    }
}
impl Searchable<false> for NestedProject {
    fn search_task(
        filter: Option<&Self::Filter>,
        query: String,
        limit: i64,
        offset: i64,
    ) -> super::Select {
        super::Select::search(Table::Projects, filter, query, limit, offset)
    }
}
impl Searchable<true> for NestedProject {
    fn search_task(
        filter: Option<&Self::Filter>,
        query: String,
        limit: i64,
        offset: i64,
    ) -> super::Select {
        super::Select::search_updatables(Table::Projects, filter, query, limit, offset)
    }
}
impl SimilarityScore for NestedProjectsTeamsRoleInvitation {
    fn similarity_score<S: AsRef<str>>(&self, lowercase_query: S) -> f64 {
        let mut number_of_attributes = 0;
        let mut total_similarity_score = 0.0;
        number_of_attributes += 1;
        total_similarity_score += self.table.similarity_score(lowercase_query.as_ref());
        number_of_attributes += 1;
        total_similarity_score += self.team.similarity_score(lowercase_query.as_ref());
        number_of_attributes += 1;
        total_similarity_score += self.role.similarity_score(lowercase_query.as_ref());
        number_of_attributes += 1;
        total_similarity_score += self.created_by.similarity_score(lowercase_query.as_ref());
        total_similarity_score / number_of_attributes as f64
    }
}
impl Searchable<false> for NestedProjectsTeamsRoleInvitation {
    fn search_task(
        filter: Option<&Self::Filter>,
        query: String,
        limit: i64,
        offset: i64,
    ) -> super::Select {
        super::Select::search(
            Table::ProjectsTeamsRoleInvitations,
            filter,
            query,
            limit,
            offset,
        )
    }
}
impl SimilarityScore for NestedProjectsTeamsRoleRequest {
    fn similarity_score<S: AsRef<str>>(&self, lowercase_query: S) -> f64 {
        let mut number_of_attributes = 0;
        let mut total_similarity_score = 0.0;
        number_of_attributes += 1;
        total_similarity_score += self.table.similarity_score(lowercase_query.as_ref());
        number_of_attributes += 1;
        total_similarity_score += self.team.similarity_score(lowercase_query.as_ref());
        number_of_attributes += 1;
        total_similarity_score += self.role.similarity_score(lowercase_query.as_ref());
        number_of_attributes += 1;
        total_similarity_score += self.created_by.similarity_score(lowercase_query.as_ref());
        total_similarity_score / number_of_attributes as f64
    }
}
impl Searchable<false> for NestedProjectsTeamsRoleRequest {
    fn search_task(
        filter: Option<&Self::Filter>,
        query: String,
        limit: i64,
        offset: i64,
    ) -> super::Select {
        super::Select::search(
            Table::ProjectsTeamsRoleRequests,
            filter,
            query,
            limit,
            offset,
        )
    }
}
impl SimilarityScore for NestedProjectsTeamsRole {
    fn similarity_score<S: AsRef<str>>(&self, lowercase_query: S) -> f64 {
        let mut number_of_attributes = 0;
        let mut total_similarity_score = 0.0;
        number_of_attributes += 1;
        total_similarity_score += self.table.similarity_score(lowercase_query.as_ref());
        number_of_attributes += 1;
        total_similarity_score += self.team.similarity_score(lowercase_query.as_ref());
        number_of_attributes += 1;
        total_similarity_score += self.role.similarity_score(lowercase_query.as_ref());
        number_of_attributes += 1;
        total_similarity_score += self.created_by.similarity_score(lowercase_query.as_ref());
        total_similarity_score / number_of_attributes as f64
    }
}
impl Searchable<false> for NestedProjectsTeamsRole {
    fn search_task(
        filter: Option<&Self::Filter>,
        query: String,
        limit: i64,
        offset: i64,
    ) -> super::Select {
        super::Select::search(Table::ProjectsTeamsRoles, filter, query, limit, offset)
    }
}
impl SimilarityScore for NestedProjectsUsersRoleInvitation {
    fn similarity_score<S: AsRef<str>>(&self, lowercase_query: S) -> f64 {
        let mut number_of_attributes = 0;
        let mut total_similarity_score = 0.0;
        number_of_attributes += 1;
        total_similarity_score += self.table.similarity_score(lowercase_query.as_ref());
        number_of_attributes += 1;
        total_similarity_score += self.user.similarity_score(lowercase_query.as_ref());
        number_of_attributes += 1;
        total_similarity_score += self.role.similarity_score(lowercase_query.as_ref());
        number_of_attributes += 1;
        total_similarity_score += self.created_by.similarity_score(lowercase_query.as_ref());
        total_similarity_score / number_of_attributes as f64
    }
}
impl Searchable<false> for NestedProjectsUsersRoleInvitation {
    fn search_task(
        filter: Option<&Self::Filter>,
        query: String,
        limit: i64,
        offset: i64,
    ) -> super::Select {
        super::Select::search(
            Table::ProjectsUsersRoleInvitations,
            filter,
            query,
            limit,
            offset,
        )
    }
}
impl SimilarityScore for NestedProjectsUsersRoleRequest {
    fn similarity_score<S: AsRef<str>>(&self, lowercase_query: S) -> f64 {
        let mut number_of_attributes = 0;
        let mut total_similarity_score = 0.0;
        number_of_attributes += 1;
        total_similarity_score += self.table.similarity_score(lowercase_query.as_ref());
        number_of_attributes += 1;
        total_similarity_score += self.user.similarity_score(lowercase_query.as_ref());
        number_of_attributes += 1;
        total_similarity_score += self.role.similarity_score(lowercase_query.as_ref());
        number_of_attributes += 1;
        total_similarity_score += self.created_by.similarity_score(lowercase_query.as_ref());
        total_similarity_score / number_of_attributes as f64
    }
}
impl Searchable<false> for NestedProjectsUsersRoleRequest {
    fn search_task(
        filter: Option<&Self::Filter>,
        query: String,
        limit: i64,
        offset: i64,
    ) -> super::Select {
        super::Select::search(
            Table::ProjectsUsersRoleRequests,
            filter,
            query,
            limit,
            offset,
        )
    }
}
impl SimilarityScore for NestedProjectsUsersRole {
    fn similarity_score<S: AsRef<str>>(&self, lowercase_query: S) -> f64 {
        let mut number_of_attributes = 0;
        let mut total_similarity_score = 0.0;
        number_of_attributes += 1;
        total_similarity_score += self.table.similarity_score(lowercase_query.as_ref());
        number_of_attributes += 1;
        total_similarity_score += self.user.similarity_score(lowercase_query.as_ref());
        number_of_attributes += 1;
        total_similarity_score += self.role.similarity_score(lowercase_query.as_ref());
        number_of_attributes += 1;
        total_similarity_score += self.created_by.similarity_score(lowercase_query.as_ref());
        total_similarity_score / number_of_attributes as f64
    }
}
impl Searchable<false> for NestedProjectsUsersRole {
    fn search_task(
        filter: Option<&Self::Filter>,
        query: String,
        limit: i64,
        offset: i64,
    ) -> super::Select {
        super::Select::search(Table::ProjectsUsersRoles, filter, query, limit, offset)
    }
}
impl SimilarityScore for NestedRank {
    fn similarity_score<S: AsRef<str>>(&self, lowercase_query: S) -> f64 {
        let mut number_of_attributes = 0;
        let mut total_similarity_score = 0.0;
        number_of_attributes += 1;
        total_similarity_score += self.inner.similarity_score(lowercase_query.as_ref());
        number_of_attributes += 1;
        total_similarity_score += self.icon.similarity_score(lowercase_query.as_ref());
        number_of_attributes += 1;
        total_similarity_score += self.color.similarity_score(lowercase_query.as_ref());
        total_similarity_score / number_of_attributes as f64
    }
}
impl Searchable<false> for NestedRank {
    fn search_task(
        filter: Option<&Self::Filter>,
        query: String,
        limit: i64,
        offset: i64,
    ) -> super::Select {
        super::Select::search(Table::Ranks, filter, query, limit, offset)
    }
}
impl SimilarityScore for NestedRole {
    fn similarity_score<S: AsRef<str>>(&self, lowercase_query: S) -> f64 {
        let mut number_of_attributes = 0;
        let mut total_similarity_score = 0.0;
        number_of_attributes += 1;
        total_similarity_score += self.inner.similarity_score(lowercase_query.as_ref());
        number_of_attributes += 1;
        total_similarity_score += self.icon.similarity_score(lowercase_query.as_ref());
        number_of_attributes += 1;
        total_similarity_score += self.color.similarity_score(lowercase_query.as_ref());
        total_similarity_score / number_of_attributes as f64
    }
}
impl Searchable<false> for NestedRole {
    fn search_task(
        filter: Option<&Self::Filter>,
        query: String,
        limit: i64,
        offset: i64,
    ) -> super::Select {
        super::Select::search(Table::Roles, filter, query, limit, offset)
    }
}
impl SimilarityScore for NestedSampleContainerCategory {
    fn similarity_score<S: AsRef<str>>(&self, lowercase_query: S) -> f64 {
        let mut number_of_attributes = 0;
        let mut total_similarity_score = 0.0;
        number_of_attributes += 1;
        total_similarity_score += self.inner.similarity_score(lowercase_query.as_ref());
        number_of_attributes += 1;
        total_similarity_score += self.material.similarity_score(lowercase_query.as_ref());
        number_of_attributes += 1;
        total_similarity_score += self.icon.similarity_score(lowercase_query.as_ref());
        number_of_attributes += 1;
        total_similarity_score += self.color.similarity_score(lowercase_query.as_ref());
        total_similarity_score / number_of_attributes as f64
    }
}
impl Searchable<false> for NestedSampleContainerCategory {
    fn search_task(
        filter: Option<&Self::Filter>,
        query: String,
        limit: i64,
        offset: i64,
    ) -> super::Select {
        super::Select::search(
            Table::SampleContainerCategories,
            filter,
            query,
            limit,
            offset,
        )
    }
}
impl SimilarityScore for NestedSampleContainer {
    fn similarity_score<S: AsRef<str>>(&self, lowercase_query: S) -> f64 {
        let mut number_of_attributes = 0;
        let mut total_similarity_score = 0.0;
        number_of_attributes += 1;
        total_similarity_score += self.inner.similarity_score(lowercase_query.as_ref());
        number_of_attributes += 1;
        total_similarity_score += self.project.similarity_score(lowercase_query.as_ref());
        number_of_attributes += 1;
        total_similarity_score += self.category.similarity_score(lowercase_query.as_ref());
        number_of_attributes += 1;
        total_similarity_score += self.created_by.similarity_score(lowercase_query.as_ref());
        number_of_attributes += 1;
        total_similarity_score += self.updated_by.similarity_score(lowercase_query.as_ref());
        total_similarity_score / number_of_attributes as f64
    }
}
impl Searchable<false> for NestedSampleContainer {
    fn search_task(
        filter: Option<&Self::Filter>,
        query: String,
        limit: i64,
        offset: i64,
    ) -> super::Select {
        super::Select::search(Table::SampleContainers, filter, query, limit, offset)
    }
}
impl Searchable<true> for NestedSampleContainer {
    fn search_task(
        filter: Option<&Self::Filter>,
        query: String,
        limit: i64,
        offset: i64,
    ) -> super::Select {
        super::Select::search_updatables(Table::SampleContainers, filter, query, limit, offset)
    }
}
impl SimilarityScore for NestedSampleState {
    fn similarity_score<S: AsRef<str>>(&self, lowercase_query: S) -> f64 {
        let mut number_of_attributes = 0;
        let mut total_similarity_score = 0.0;
        number_of_attributes += 1;
        total_similarity_score += self.inner.similarity_score(lowercase_query.as_ref());
        number_of_attributes += 1;
        total_similarity_score += self.icon.similarity_score(lowercase_query.as_ref());
        number_of_attributes += 1;
        total_similarity_score += self.color.similarity_score(lowercase_query.as_ref());
        total_similarity_score / number_of_attributes as f64
    }
}
impl Searchable<false> for NestedSampleState {
    fn search_task(
        filter: Option<&Self::Filter>,
        query: String,
        limit: i64,
        offset: i64,
    ) -> super::Select {
        super::Select::search(Table::SampleStates, filter, query, limit, offset)
    }
}
impl SimilarityScore for NestedSampleTaxon {
    fn similarity_score<S: AsRef<str>>(&self, lowercase_query: S) -> f64 {
        let mut number_of_attributes = 0;
        let mut total_similarity_score = 0.0;
        number_of_attributes += 1;
        total_similarity_score += self.created_by.similarity_score(lowercase_query.as_ref());
        number_of_attributes += 1;
        total_similarity_score += self.sample.similarity_score(lowercase_query.as_ref());
        number_of_attributes += 1;
        total_similarity_score += self.taxon.similarity_score(lowercase_query.as_ref());
        total_similarity_score / number_of_attributes as f64
    }
}
impl Searchable<false> for NestedSampleTaxon {
    fn search_task(
        filter: Option<&Self::Filter>,
        query: String,
        limit: i64,
        offset: i64,
    ) -> super::Select {
        super::Select::search(Table::SampleTaxa, filter, query, limit, offset)
    }
}
impl SimilarityScore for NestedSample {
    fn similarity_score<S: AsRef<str>>(&self, lowercase_query: S) -> f64 {
        let mut number_of_attributes = 0;
        let mut total_similarity_score = 0.0;
        number_of_attributes += 1;
        total_similarity_score += self.inner.similarity_score(lowercase_query.as_ref());
        number_of_attributes += 1;
        total_similarity_score += self.container.similarity_score(lowercase_query.as_ref());
        number_of_attributes += 1;
        total_similarity_score += self.project.similarity_score(lowercase_query.as_ref());
        number_of_attributes += 1;
        total_similarity_score += self.created_by.similarity_score(lowercase_query.as_ref());
        number_of_attributes += 1;
        total_similarity_score += self.sampled_by.similarity_score(lowercase_query.as_ref());
        number_of_attributes += 1;
        total_similarity_score += self.updated_by.similarity_score(lowercase_query.as_ref());
        number_of_attributes += 1;
        total_similarity_score += self.state.similarity_score(lowercase_query.as_ref());
        total_similarity_score / number_of_attributes as f64
    }
}
impl Searchable<false> for NestedSample {
    fn search_task(
        filter: Option<&Self::Filter>,
        query: String,
        limit: i64,
        offset: i64,
    ) -> super::Select {
        super::Select::search(Table::Samples, filter, query, limit, offset)
    }
}
impl Searchable<true> for NestedSample {
    fn search_task(
        filter: Option<&Self::Filter>,
        query: String,
        limit: i64,
        offset: i64,
    ) -> super::Select {
        super::Select::search_updatables(Table::Samples, filter, query, limit, offset)
    }
}
impl SimilarityScore for NestedSpectrum {
    fn similarity_score<S: AsRef<str>>(&self, lowercase_query: S) -> f64 {
        let mut number_of_attributes = 0;
        let mut total_similarity_score = 0.0;
        number_of_attributes += 1;
        total_similarity_score += self
            .spectra_collection
            .similarity_score(lowercase_query.as_ref());
        total_similarity_score / number_of_attributes as f64
    }
}
impl SimilarityScore for NestedSpectraCollection {
    fn similarity_score<S: AsRef<str>>(&self, lowercase_query: S) -> f64 {
        let mut number_of_attributes = 0;
        let mut total_similarity_score = 0.0;
        number_of_attributes += 1;
        total_similarity_score += self.inner.similarity_score(lowercase_query.as_ref());
        number_of_attributes += 1;
        total_similarity_score += self.sample.similarity_score(lowercase_query.as_ref());
        number_of_attributes += 1;
        total_similarity_score += self.created_by.similarity_score(lowercase_query.as_ref());
        number_of_attributes += 1;
        total_similarity_score += self.updated_by.similarity_score(lowercase_query.as_ref());
        total_similarity_score / number_of_attributes as f64
    }
}
impl Searchable<false> for NestedSpectraCollection {
    fn search_task(
        filter: Option<&Self::Filter>,
        query: String,
        limit: i64,
        offset: i64,
    ) -> super::Select {
        super::Select::search(Table::SpectraCollections, filter, query, limit, offset)
    }
}
impl Searchable<true> for NestedSpectraCollection {
    fn search_task(
        filter: Option<&Self::Filter>,
        query: String,
        limit: i64,
        offset: i64,
    ) -> super::Select {
        super::Select::search_updatables(Table::SpectraCollections, filter, query, limit, offset)
    }
}
impl SimilarityScore for NestedTaxon {
    fn similarity_score<S: AsRef<str>>(&self, lowercase_query: S) -> f64 {
        let mut number_of_attributes = 0;
        let mut total_similarity_score = 0.0;
        number_of_attributes += 1;
        total_similarity_score += self.inner.similarity_score(lowercase_query.as_ref());
        number_of_attributes += 1;
        total_similarity_score += self.ott_rank.similarity_score(lowercase_query.as_ref());
        if let Some(domain) = &self.domain {
            number_of_attributes += 1;
            total_similarity_score += domain.similarity_score(lowercase_query.as_ref());
        }
        if let Some(kingdom) = &self.kingdom {
            number_of_attributes += 1;
            total_similarity_score += kingdom.similarity_score(lowercase_query.as_ref());
        }
        if let Some(phylum) = &self.phylum {
            number_of_attributes += 1;
            total_similarity_score += phylum.similarity_score(lowercase_query.as_ref());
        }
        if let Some(class) = &self.class {
            number_of_attributes += 1;
            total_similarity_score += class.similarity_score(lowercase_query.as_ref());
        }
        if let Some(order) = &self.order {
            number_of_attributes += 1;
            total_similarity_score += order.similarity_score(lowercase_query.as_ref());
        }
        if let Some(family) = &self.family {
            number_of_attributes += 1;
            total_similarity_score += family.similarity_score(lowercase_query.as_ref());
        }
        if let Some(genus) = &self.genus {
            number_of_attributes += 1;
            total_similarity_score += genus.similarity_score(lowercase_query.as_ref());
        }
        number_of_attributes += 1;
        total_similarity_score += self.parent.similarity_score(lowercase_query.as_ref());
        number_of_attributes += 1;
        total_similarity_score += self.icon.similarity_score(lowercase_query.as_ref());
        number_of_attributes += 1;
        total_similarity_score += self.color.similarity_score(lowercase_query.as_ref());
        total_similarity_score / number_of_attributes as f64
    }
}
impl Searchable<false> for NestedTaxon {
    fn search_task(
        filter: Option<&Self::Filter>,
        query: String,
        limit: i64,
        offset: i64,
    ) -> super::Select {
        super::Select::search(Table::Taxa, filter, query, limit, offset)
    }
}
impl SimilarityScore for NestedTeamState {
    fn similarity_score<S: AsRef<str>>(&self, lowercase_query: S) -> f64 {
        let mut number_of_attributes = 0;
        let mut total_similarity_score = 0.0;
        number_of_attributes += 1;
        total_similarity_score += self.inner.similarity_score(lowercase_query.as_ref());
        number_of_attributes += 1;
        total_similarity_score += self.icon.similarity_score(lowercase_query.as_ref());
        number_of_attributes += 1;
        total_similarity_score += self.color.similarity_score(lowercase_query.as_ref());
        total_similarity_score / number_of_attributes as f64
    }
}
impl Searchable<false> for NestedTeamState {
    fn search_task(
        filter: Option<&Self::Filter>,
        query: String,
        limit: i64,
        offset: i64,
    ) -> super::Select {
        super::Select::search(Table::TeamStates, filter, query, limit, offset)
    }
}
impl SimilarityScore for NestedTeam {
    fn similarity_score<S: AsRef<str>>(&self, lowercase_query: S) -> f64 {
        let mut number_of_attributes = 0;
        let mut total_similarity_score = 0.0;
        number_of_attributes += 1;
        total_similarity_score += self.inner.similarity_score(lowercase_query.as_ref());
        number_of_attributes += 1;
        total_similarity_score += self.icon.similarity_score(lowercase_query.as_ref());
        number_of_attributes += 1;
        total_similarity_score += self.color.similarity_score(lowercase_query.as_ref());
        number_of_attributes += 1;
        total_similarity_score += self.state.similarity_score(lowercase_query.as_ref());
        if let Some(parent_team) = &self.parent_team {
            number_of_attributes += 1;
            total_similarity_score += parent_team.similarity_score(lowercase_query.as_ref());
        }
        number_of_attributes += 1;
        total_similarity_score += self.created_by.similarity_score(lowercase_query.as_ref());
        number_of_attributes += 1;
        total_similarity_score += self.updated_by.similarity_score(lowercase_query.as_ref());
        total_similarity_score / number_of_attributes as f64
    }
}
impl Searchable<false> for NestedTeam {
    fn search_task(
        filter: Option<&Self::Filter>,
        query: String,
        limit: i64,
        offset: i64,
    ) -> super::Select {
        super::Select::search(Table::Teams, filter, query, limit, offset)
    }
}
impl Searchable<true> for NestedTeam {
    fn search_task(
        filter: Option<&Self::Filter>,
        query: String,
        limit: i64,
        offset: i64,
    ) -> super::Select {
        super::Select::search_updatables(Table::Teams, filter, query, limit, offset)
    }
}
impl SimilarityScore for NestedTeamsTeamsRoleInvitation {
    fn similarity_score<S: AsRef<str>>(&self, lowercase_query: S) -> f64 {
        let mut number_of_attributes = 0;
        let mut total_similarity_score = 0.0;
        number_of_attributes += 1;
        total_similarity_score += self.table.similarity_score(lowercase_query.as_ref());
        number_of_attributes += 1;
        total_similarity_score += self.team.similarity_score(lowercase_query.as_ref());
        number_of_attributes += 1;
        total_similarity_score += self.role.similarity_score(lowercase_query.as_ref());
        number_of_attributes += 1;
        total_similarity_score += self.created_by.similarity_score(lowercase_query.as_ref());
        total_similarity_score / number_of_attributes as f64
    }
}
impl Searchable<false> for NestedTeamsTeamsRoleInvitation {
    fn search_task(
        filter: Option<&Self::Filter>,
        query: String,
        limit: i64,
        offset: i64,
    ) -> super::Select {
        super::Select::search(
            Table::TeamsTeamsRoleInvitations,
            filter,
            query,
            limit,
            offset,
        )
    }
}
impl SimilarityScore for NestedTeamsUsersRoleInvitation {
    fn similarity_score<S: AsRef<str>>(&self, lowercase_query: S) -> f64 {
        let mut number_of_attributes = 0;
        let mut total_similarity_score = 0.0;
        number_of_attributes += 1;
        total_similarity_score += self.table.similarity_score(lowercase_query.as_ref());
        number_of_attributes += 1;
        total_similarity_score += self.user.similarity_score(lowercase_query.as_ref());
        number_of_attributes += 1;
        total_similarity_score += self.role.similarity_score(lowercase_query.as_ref());
        number_of_attributes += 1;
        total_similarity_score += self.created_by.similarity_score(lowercase_query.as_ref());
        total_similarity_score / number_of_attributes as f64
    }
}
impl Searchable<false> for NestedTeamsUsersRoleInvitation {
    fn search_task(
        filter: Option<&Self::Filter>,
        query: String,
        limit: i64,
        offset: i64,
    ) -> super::Select {
        super::Select::search(
            Table::TeamsUsersRoleInvitations,
            filter,
            query,
            limit,
            offset,
        )
    }
}
impl SimilarityScore for NestedTeamsUsersRoleRequest {
    fn similarity_score<S: AsRef<str>>(&self, lowercase_query: S) -> f64 {
        let mut number_of_attributes = 0;
        let mut total_similarity_score = 0.0;
        number_of_attributes += 1;
        total_similarity_score += self.table.similarity_score(lowercase_query.as_ref());
        number_of_attributes += 1;
        total_similarity_score += self.user.similarity_score(lowercase_query.as_ref());
        number_of_attributes += 1;
        total_similarity_score += self.role.similarity_score(lowercase_query.as_ref());
        number_of_attributes += 1;
        total_similarity_score += self.created_by.similarity_score(lowercase_query.as_ref());
        total_similarity_score / number_of_attributes as f64
    }
}
impl Searchable<false> for NestedTeamsUsersRoleRequest {
    fn search_task(
        filter: Option<&Self::Filter>,
        query: String,
        limit: i64,
        offset: i64,
    ) -> super::Select {
        super::Select::search(Table::TeamsUsersRoleRequests, filter, query, limit, offset)
    }
}
impl SimilarityScore for NestedTeamsUsersRole {
    fn similarity_score<S: AsRef<str>>(&self, lowercase_query: S) -> f64 {
        let mut number_of_attributes = 0;
        let mut total_similarity_score = 0.0;
        number_of_attributes += 1;
        total_similarity_score += self.table.similarity_score(lowercase_query.as_ref());
        number_of_attributes += 1;
        total_similarity_score += self.user.similarity_score(lowercase_query.as_ref());
        number_of_attributes += 1;
        total_similarity_score += self.role.similarity_score(lowercase_query.as_ref());
        number_of_attributes += 1;
        total_similarity_score += self.created_by.similarity_score(lowercase_query.as_ref());
        total_similarity_score / number_of_attributes as f64
    }
}
impl Searchable<false> for NestedTeamsUsersRole {
    fn search_task(
        filter: Option<&Self::Filter>,
        query: String,
        limit: i64,
        offset: i64,
    ) -> super::Select {
        super::Select::search(Table::TeamsUsersRoles, filter, query, limit, offset)
    }
}
impl SimilarityScore for NestedUnit {
    fn similarity_score<S: AsRef<str>>(&self, lowercase_query: S) -> f64 {
        let mut number_of_attributes = 0;
        let mut total_similarity_score = 0.0;
        number_of_attributes += 1;
        total_similarity_score += self.inner.similarity_score(lowercase_query.as_ref());
        number_of_attributes += 1;
        total_similarity_score += self.icon.similarity_score(lowercase_query.as_ref());
        number_of_attributes += 1;
        total_similarity_score += self.color.similarity_score(lowercase_query.as_ref());
        total_similarity_score / number_of_attributes as f64
    }
}
impl Searchable<false> for NestedUnit {
    fn search_task(
        filter: Option<&Self::Filter>,
        query: String,
        limit: i64,
        offset: i64,
    ) -> super::Select {
        super::Select::search(Table::Units, filter, query, limit, offset)
    }
}
impl SimilarityScore for NestedUserEmail {
    fn similarity_score<S: AsRef<str>>(&self, lowercase_query: S) -> f64 {
        let mut number_of_attributes = 0;
        let mut total_similarity_score = 0.0;
        number_of_attributes += 1;
        total_similarity_score += self.inner.similarity_score(lowercase_query.as_ref());
        number_of_attributes += 1;
        total_similarity_score += self.created_by.similarity_score(lowercase_query.as_ref());
        number_of_attributes += 1;
        total_similarity_score += self
            .login_provider
            .similarity_score(lowercase_query.as_ref());
        total_similarity_score / number_of_attributes as f64
    }
}
impl SimilarityScore for NestedUser {
    fn similarity_score<S: AsRef<str>>(&self, lowercase_query: S) -> f64 {
        let mut number_of_attributes = 0;
        let mut total_similarity_score = 0.0;
        number_of_attributes += 1;
        total_similarity_score += self.inner.similarity_score(lowercase_query.as_ref());
        if let Some(organization) = &self.organization {
            number_of_attributes += 1;
            total_similarity_score += organization.similarity_score(lowercase_query.as_ref());
        }
        total_similarity_score / number_of_attributes as f64
    }
}
impl Searchable<false> for NestedUser {
    fn search_task(
        filter: Option<&Self::Filter>,
        query: String,
        limit: i64,
        offset: i64,
    ) -> super::Select {
        super::Select::search(Table::Users, filter, query, limit, offset)
    }
}
impl Searchable<true> for NestedUser {
    fn search_task(
        filter: Option<&Self::Filter>,
        query: String,
        limit: i64,
        offset: i64,
    ) -> super::Select {
        super::Select::search_updatables(Table::Users, filter, query, limit, offset)
    }
}
impl SimilarityScore for NestedUsersUsersRoleInvitation {
    fn similarity_score<S: AsRef<str>>(&self, lowercase_query: S) -> f64 {
        let mut number_of_attributes = 0;
        let mut total_similarity_score = 0.0;
        number_of_attributes += 1;
        total_similarity_score += self.table.similarity_score(lowercase_query.as_ref());
        number_of_attributes += 1;
        total_similarity_score += self.user.similarity_score(lowercase_query.as_ref());
        number_of_attributes += 1;
        total_similarity_score += self.role.similarity_score(lowercase_query.as_ref());
        number_of_attributes += 1;
        total_similarity_score += self.created_by.similarity_score(lowercase_query.as_ref());
        total_similarity_score / number_of_attributes as f64
    }
}
impl Searchable<false> for NestedUsersUsersRoleInvitation {
    fn search_task(
        filter: Option<&Self::Filter>,
        query: String,
        limit: i64,
        offset: i64,
    ) -> super::Select {
        super::Select::search(
            Table::UsersUsersRoleInvitations,
            filter,
            query,
            limit,
            offset,
        )
    }
}
impl SimilarityScore for NestedUsersUsersRoleRequest {
    fn similarity_score<S: AsRef<str>>(&self, lowercase_query: S) -> f64 {
        let mut number_of_attributes = 0;
        let mut total_similarity_score = 0.0;
        number_of_attributes += 1;
        total_similarity_score += self.table.similarity_score(lowercase_query.as_ref());
        number_of_attributes += 1;
        total_similarity_score += self.user.similarity_score(lowercase_query.as_ref());
        number_of_attributes += 1;
        total_similarity_score += self.role.similarity_score(lowercase_query.as_ref());
        number_of_attributes += 1;
        total_similarity_score += self.created_by.similarity_score(lowercase_query.as_ref());
        total_similarity_score / number_of_attributes as f64
    }
}
impl Searchable<false> for NestedUsersUsersRoleRequest {
    fn search_task(
        filter: Option<&Self::Filter>,
        query: String,
        limit: i64,
        offset: i64,
    ) -> super::Select {
        super::Select::search(Table::UsersUsersRoleRequests, filter, query, limit, offset)
    }
}
impl SimilarityScore for NestedUsersUsersRole {
    fn similarity_score<S: AsRef<str>>(&self, lowercase_query: S) -> f64 {
        let mut number_of_attributes = 0;
        let mut total_similarity_score = 0.0;
        number_of_attributes += 1;
        total_similarity_score += self.table.similarity_score(lowercase_query.as_ref());
        number_of_attributes += 1;
        total_similarity_score += self.user.similarity_score(lowercase_query.as_ref());
        number_of_attributes += 1;
        total_similarity_score += self.role.similarity_score(lowercase_query.as_ref());
        number_of_attributes += 1;
        total_similarity_score += self.created_by.similarity_score(lowercase_query.as_ref());
        total_similarity_score / number_of_attributes as f64
    }
}
impl Searchable<false> for NestedUsersUsersRole {
    fn search_task(
        filter: Option<&Self::Filter>,
        query: String,
        limit: i64,
        offset: i64,
    ) -> super::Select {
        super::Select::search(Table::UsersUsersRoles, filter, query, limit, offset)
    }
}
impl SimilarityScore for Color {
    fn similarity_score<S: AsRef<str>>(&self, lowercase_query: S) -> f64 {
        let mut number_of_attributes = 0;
        let mut total_similarity_score = 0.0;
        number_of_attributes += 1;
        let name: &str = self.name.as_ref();
        total_similarity_score +=
            strsim::normalized_damerau_levenshtein(&name.to_lowercase(), lowercase_query.as_ref());
        number_of_attributes += 1;
        let hexadecimal_value: &str = self.hexadecimal_value.as_ref();
        total_similarity_score += strsim::normalized_damerau_levenshtein(
            &hexadecimal_value.to_lowercase(),
            lowercase_query.as_ref(),
        );
        number_of_attributes += 1;
        let description: &str = self.description.as_ref();
        total_similarity_score += strsim::normalized_damerau_levenshtein(
            &description.to_lowercase(),
            lowercase_query.as_ref(),
        );
        total_similarity_score / number_of_attributes as f64
    }
}
impl Searchable<false> for Color {
    fn search_task(
        filter: Option<&Self::Filter>,
        query: String,
        limit: i64,
        offset: i64,
    ) -> super::Select {
        super::Select::search(Table::Colors, filter, query, limit, offset)
    }
}
impl SimilarityScore for Country {
    fn similarity_score<S: AsRef<str>>(&self, lowercase_query: S) -> f64 {
        let mut number_of_attributes = 0;
        let mut total_similarity_score = 0.0;
        number_of_attributes += 1;
        let iso: &str = self.iso.as_ref();
        total_similarity_score +=
            strsim::normalized_damerau_levenshtein(&iso.to_lowercase(), lowercase_query.as_ref());
        number_of_attributes += 1;
        let emoji: &str = self.emoji.as_ref();
        total_similarity_score +=
            strsim::normalized_damerau_levenshtein(&emoji.to_lowercase(), lowercase_query.as_ref());
        number_of_attributes += 1;
        let unicode: &str = self.unicode.as_ref();
        total_similarity_score += strsim::normalized_damerau_levenshtein(
            &unicode.to_lowercase(),
            lowercase_query.as_ref(),
        );
        number_of_attributes += 1;
        let name: &str = self.name.as_ref();
        total_similarity_score +=
            strsim::normalized_damerau_levenshtein(&name.to_lowercase(), lowercase_query.as_ref());
        total_similarity_score / number_of_attributes as f64
    }
}
impl Searchable<false> for Country {
    fn search_task(
        filter: Option<&Self::Filter>,
        query: String,
        limit: i64,
        offset: i64,
    ) -> super::Select {
        super::Select::search(Table::Countries, filter, query, limit, offset)
    }
}
impl Searchable<false> for DerivedSample {
    fn search_task(
        filter: Option<&Self::Filter>,
        query: String,
        limit: i64,
        offset: i64,
    ) -> super::Select {
        super::Select::search(Table::DerivedSamples, filter, query, limit, offset)
    }
}
impl Searchable<true> for DerivedSample {
    fn search_task(
        filter: Option<&Self::Filter>,
        query: String,
        limit: i64,
        offset: i64,
    ) -> super::Select {
        super::Select::search_updatables(Table::DerivedSamples, filter, query, limit, offset)
    }
}
impl SimilarityScore for DocumentFormat {
    fn similarity_score<S: AsRef<str>>(&self, lowercase_query: S) -> f64 {
        let mut number_of_attributes = 0;
        let mut total_similarity_score = 0.0;
        number_of_attributes += 1;
        let extension: &str = self.extension.as_ref();
        total_similarity_score += strsim::normalized_damerau_levenshtein(
            &extension.to_lowercase(),
            lowercase_query.as_ref(),
        );
        number_of_attributes += 1;
        let mime_type: &str = self.mime_type.as_ref();
        total_similarity_score += strsim::normalized_damerau_levenshtein(
            &mime_type.to_lowercase(),
            lowercase_query.as_ref(),
        );
        number_of_attributes += 1;
        let description: &str = self.description.as_ref();
        total_similarity_score += strsim::normalized_damerau_levenshtein(
            &description.to_lowercase(),
            lowercase_query.as_ref(),
        );
        total_similarity_score / number_of_attributes as f64
    }
}
impl Searchable<false> for DocumentFormat {
    fn search_task(
        filter: Option<&Self::Filter>,
        query: String,
        limit: i64,
        offset: i64,
    ) -> super::Select {
        super::Select::search(Table::DocumentFormats, filter, query, limit, offset)
    }
}
impl SimilarityScore for FontAwesomeIcon {
    fn similarity_score<S: AsRef<str>>(&self, lowercase_query: S) -> f64 {
        let mut number_of_attributes = 0;
        let mut total_similarity_score = 0.0;
        number_of_attributes += 1;
        let name: &str = self.name.as_ref();
        total_similarity_score +=
            strsim::normalized_damerau_levenshtein(&name.to_lowercase(), lowercase_query.as_ref());
        number_of_attributes += 1;
        let description: &str = self.description.as_ref();
        total_similarity_score += strsim::normalized_damerau_levenshtein(
            &description.to_lowercase(),
            lowercase_query.as_ref(),
        );
        total_similarity_score / number_of_attributes as f64
    }
}
impl Searchable<false> for FontAwesomeIcon {
    fn search_task(
        filter: Option<&Self::Filter>,
        query: String,
        limit: i64,
        offset: i64,
    ) -> super::Select {
        super::Select::search(Table::FontAwesomeIcons, filter, query, limit, offset)
    }
}
impl SimilarityScore for LoginProvider {
    fn similarity_score<S: AsRef<str>>(&self, lowercase_query: S) -> f64 {
        let mut number_of_attributes = 0;
        let mut total_similarity_score = 0.0;
        number_of_attributes += 1;
        let name: &str = self.name.as_ref();
        total_similarity_score +=
            strsim::normalized_damerau_levenshtein(&name.to_lowercase(), lowercase_query.as_ref());
        number_of_attributes += 1;
        let client_id_var_name: &str = self.client_id_var_name.as_ref();
        total_similarity_score += strsim::normalized_damerau_levenshtein(
            &client_id_var_name.to_lowercase(),
            lowercase_query.as_ref(),
        );
        number_of_attributes += 1;
        let redirect_uri_var_name: &str = self.redirect_uri_var_name.as_ref();
        total_similarity_score += strsim::normalized_damerau_levenshtein(
            &redirect_uri_var_name.to_lowercase(),
            lowercase_query.as_ref(),
        );
        number_of_attributes += 1;
        let oauth_url: &str = self.oauth_url.as_ref();
        total_similarity_score += strsim::normalized_damerau_levenshtein(
            &oauth_url.to_lowercase(),
            lowercase_query.as_ref(),
        );
        number_of_attributes += 1;
        let scope: &str = self.scope.as_ref();
        total_similarity_score +=
            strsim::normalized_damerau_levenshtein(&scope.to_lowercase(), lowercase_query.as_ref());
        total_similarity_score / number_of_attributes as f64
    }
}
impl SimilarityScore for Material {
    fn similarity_score<S: AsRef<str>>(&self, lowercase_query: S) -> f64 {
        let mut number_of_attributes = 0;
        let mut total_similarity_score = 0.0;
        number_of_attributes += 1;
        let name: &str = self.name.as_ref();
        total_similarity_score +=
            strsim::normalized_damerau_levenshtein(&name.to_lowercase(), lowercase_query.as_ref());
        number_of_attributes += 1;
        let description: &str = self.description.as_ref();
        total_similarity_score += strsim::normalized_damerau_levenshtein(
            &description.to_lowercase(),
            lowercase_query.as_ref(),
        );
        total_similarity_score / number_of_attributes as f64
    }
}
impl SimilarityScore for NameplateCategory {
    fn similarity_score<S: AsRef<str>>(&self, lowercase_query: S) -> f64 {
        let mut number_of_attributes = 0;
        let mut total_similarity_score = 0.0;
        number_of_attributes += 1;
        let name: &str = self.name.as_ref();
        total_similarity_score +=
            strsim::normalized_damerau_levenshtein(&name.to_lowercase(), lowercase_query.as_ref());
        number_of_attributes += 1;
        let description: &str = self.description.as_ref();
        total_similarity_score += strsim::normalized_damerau_levenshtein(
            &description.to_lowercase(),
            lowercase_query.as_ref(),
        );
        total_similarity_score / number_of_attributes as f64
    }
}
impl Searchable<false> for NameplateCategory {
    fn search_task(
        filter: Option<&Self::Filter>,
        query: String,
        limit: i64,
        offset: i64,
    ) -> super::Select {
        super::Select::search(Table::NameplateCategories, filter, query, limit, offset)
    }
}
impl SimilarityScore for Nameplate {
    fn similarity_score<S: AsRef<str>>(&self, lowercase_query: S) -> f64 {
        let mut number_of_attributes = 0;
        let mut total_similarity_score = 0.0;
        number_of_attributes += 1;
        let barcode: &str = self.barcode.as_ref();
        total_similarity_score += strsim::normalized_damerau_levenshtein(
            &barcode.to_lowercase(),
            lowercase_query.as_ref(),
        );
        total_similarity_score / number_of_attributes as f64
    }
}
impl Searchable<false> for Nameplate {
    fn search_task(
        filter: Option<&Self::Filter>,
        query: String,
        limit: i64,
        offset: i64,
    ) -> super::Select {
        super::Select::search(Table::Nameplates, filter, query, limit, offset)
    }
}
impl Searchable<true> for Nameplate {
    fn search_task(
        filter: Option<&Self::Filter>,
        query: String,
        limit: i64,
        offset: i64,
    ) -> super::Select {
        super::Select::search_updatables(Table::Nameplates, filter, query, limit, offset)
    }
}
impl SimilarityScore for Notification {
    fn similarity_score<S: AsRef<str>>(&self, lowercase_query: S) -> f64 {
        let mut number_of_attributes = 0;
        let mut total_similarity_score = 0.0;
        number_of_attributes += 1;
        let operation: &str = self.operation.as_ref();
        total_similarity_score += strsim::normalized_damerau_levenshtein(
            &operation.to_lowercase(),
            lowercase_query.as_ref(),
        );
        number_of_attributes += 1;
        let table_name: &str = self.table_name.as_ref();
        total_similarity_score += strsim::normalized_damerau_levenshtein(
            &table_name.to_lowercase(),
            lowercase_query.as_ref(),
        );
        number_of_attributes += 1;
        let record: &str = self.record.as_ref();
        total_similarity_score += strsim::normalized_damerau_levenshtein(
            &record.to_lowercase(),
            lowercase_query.as_ref(),
        );
        total_similarity_score / number_of_attributes as f64
    }
}
impl SimilarityScore for ObservationSubject {
    fn similarity_score<S: AsRef<str>>(&self, lowercase_query: S) -> f64 {
        let mut number_of_attributes = 0;
        let mut total_similarity_score = 0.0;
        number_of_attributes += 1;
        let name: &str = self.name.as_ref();
        total_similarity_score +=
            strsim::normalized_damerau_levenshtein(&name.to_lowercase(), lowercase_query.as_ref());
        number_of_attributes += 1;
        let description: &str = self.description.as_ref();
        total_similarity_score += strsim::normalized_damerau_levenshtein(
            &description.to_lowercase(),
            lowercase_query.as_ref(),
        );
        total_similarity_score / number_of_attributes as f64
    }
}
impl Searchable<false> for ObservationSubject {
    fn search_task(
        filter: Option<&Self::Filter>,
        query: String,
        limit: i64,
        offset: i64,
    ) -> super::Select {
        super::Select::search(Table::ObservationSubjects, filter, query, limit, offset)
    }
}
impl SimilarityScore for Observation {
    fn similarity_score<S: AsRef<str>>(&self, lowercase_query: S) -> f64 {
        let mut number_of_attributes = 0;
        let mut total_similarity_score = 0.0;
        if let Some(notes) = &self.notes {
            number_of_attributes += 1;
            let notes: &str = notes.as_ref();
            total_similarity_score += strsim::normalized_damerau_levenshtein(
                &notes.to_lowercase(),
                lowercase_query.as_ref(),
            );
        }
        total_similarity_score / number_of_attributes as f64
    }
}
impl Searchable<false> for Observation {
    fn search_task(
        filter: Option<&Self::Filter>,
        query: String,
        limit: i64,
        offset: i64,
    ) -> super::Select {
        super::Select::search(Table::Observations, filter, query, limit, offset)
    }
}
impl Searchable<true> for Observation {
    fn search_task(
        filter: Option<&Self::Filter>,
        query: String,
        limit: i64,
        offset: i64,
    ) -> super::Select {
        super::Select::search_updatables(Table::Observations, filter, query, limit, offset)
    }
}
impl Searchable<false> for OrganismTaxon {
    fn search_task(
        filter: Option<&Self::Filter>,
        query: String,
        limit: i64,
        offset: i64,
    ) -> super::Select {
        super::Select::search(Table::OrganismTaxa, filter, query, limit, offset)
    }
}
impl SimilarityScore for Organism {
    fn similarity_score<S: AsRef<str>>(&self, lowercase_query: S) -> f64 {
        let mut number_of_attributes = 0;
        let mut total_similarity_score = 0.0;
        if let Some(notes) = &self.notes {
            number_of_attributes += 1;
            let notes: &str = notes.as_ref();
            total_similarity_score += strsim::normalized_damerau_levenshtein(
                &notes.to_lowercase(),
                lowercase_query.as_ref(),
            );
        }
        total_similarity_score / number_of_attributes as f64
    }
}
impl Searchable<false> for Organism {
    fn search_task(
        filter: Option<&Self::Filter>,
        query: String,
        limit: i64,
        offset: i64,
    ) -> super::Select {
        super::Select::search(Table::Organisms, filter, query, limit, offset)
    }
}
impl Searchable<true> for Organism {
    fn search_task(
        filter: Option<&Self::Filter>,
        query: String,
        limit: i64,
        offset: i64,
    ) -> super::Select {
        super::Select::search_updatables(Table::Organisms, filter, query, limit, offset)
    }
}
impl SimilarityScore for Organization {
    fn similarity_score<S: AsRef<str>>(&self, lowercase_query: S) -> f64 {
        let mut number_of_attributes = 0;
        let mut total_similarity_score = 0.0;
        number_of_attributes += 1;
        let name: &str = self.name.as_ref();
        total_similarity_score +=
            strsim::normalized_damerau_levenshtein(&name.to_lowercase(), lowercase_query.as_ref());
        number_of_attributes += 1;
        let url: &str = self.url.as_ref();
        total_similarity_score +=
            strsim::normalized_damerau_levenshtein(&url.to_lowercase(), lowercase_query.as_ref());
        if let Some(state_province) = &self.state_province {
            number_of_attributes += 1;
            let state_province: &str = state_province.as_ref();
            total_similarity_score += strsim::normalized_damerau_levenshtein(
                &state_province.to_lowercase(),
                lowercase_query.as_ref(),
            );
        }
        number_of_attributes += 1;
        let domain: &str = self.domain.as_ref();
        total_similarity_score += strsim::normalized_damerau_levenshtein(
            &domain.to_lowercase(),
            lowercase_query.as_ref(),
        );
        total_similarity_score / number_of_attributes as f64
    }
}
impl Searchable<false> for Organization {
    fn search_task(
        filter: Option<&Self::Filter>,
        query: String,
        limit: i64,
        offset: i64,
    ) -> super::Select {
        super::Select::search(Table::Organizations, filter, query, limit, offset)
    }
}
impl SimilarityScore for PermanenceCategory {
    fn similarity_score<S: AsRef<str>>(&self, lowercase_query: S) -> f64 {
        let mut number_of_attributes = 0;
        let mut total_similarity_score = 0.0;
        number_of_attributes += 1;
        let name: &str = self.name.as_ref();
        total_similarity_score +=
            strsim::normalized_damerau_levenshtein(&name.to_lowercase(), lowercase_query.as_ref());
        number_of_attributes += 1;
        let description: &str = self.description.as_ref();
        total_similarity_score += strsim::normalized_damerau_levenshtein(
            &description.to_lowercase(),
            lowercase_query.as_ref(),
        );
        total_similarity_score / number_of_attributes as f64
    }
}
impl SimilarityScore for ProjectState {
    fn similarity_score<S: AsRef<str>>(&self, lowercase_query: S) -> f64 {
        let mut number_of_attributes = 0;
        let mut total_similarity_score = 0.0;
        number_of_attributes += 1;
        let name: &str = self.name.as_ref();
        total_similarity_score +=
            strsim::normalized_damerau_levenshtein(&name.to_lowercase(), lowercase_query.as_ref());
        number_of_attributes += 1;
        let description: &str = self.description.as_ref();
        total_similarity_score += strsim::normalized_damerau_levenshtein(
            &description.to_lowercase(),
            lowercase_query.as_ref(),
        );
        total_similarity_score / number_of_attributes as f64
    }
}
impl Searchable<false> for ProjectState {
    fn search_task(
        filter: Option<&Self::Filter>,
        query: String,
        limit: i64,
        offset: i64,
    ) -> super::Select {
        super::Select::search(Table::ProjectStates, filter, query, limit, offset)
    }
}
impl SimilarityScore for Project {
    fn similarity_score<S: AsRef<str>>(&self, lowercase_query: S) -> f64 {
        let mut number_of_attributes = 0;
        let mut total_similarity_score = 0.0;
        number_of_attributes += 1;
        let name: &str = self.name.as_ref();
        total_similarity_score +=
            strsim::normalized_damerau_levenshtein(&name.to_lowercase(), lowercase_query.as_ref());
        number_of_attributes += 1;
        let description: &str = self.description.as_ref();
        total_similarity_score += strsim::normalized_damerau_levenshtein(
            &description.to_lowercase(),
            lowercase_query.as_ref(),
        );
        total_similarity_score / number_of_attributes as f64
    }
}
impl Searchable<false> for Project {
    fn search_task(
        filter: Option<&Self::Filter>,
        query: String,
        limit: i64,
        offset: i64,
    ) -> super::Select {
        super::Select::search(Table::Projects, filter, query, limit, offset)
    }
}
impl Searchable<true> for Project {
    fn search_task(
        filter: Option<&Self::Filter>,
        query: String,
        limit: i64,
        offset: i64,
    ) -> super::Select {
        super::Select::search_updatables(Table::Projects, filter, query, limit, offset)
    }
}
impl Searchable<false> for ProjectsTeamsRoleInvitation {
    fn search_task(
        filter: Option<&Self::Filter>,
        query: String,
        limit: i64,
        offset: i64,
    ) -> super::Select {
        super::Select::search(
            Table::ProjectsTeamsRoleInvitations,
            filter,
            query,
            limit,
            offset,
        )
    }
}
impl Searchable<false> for ProjectsTeamsRoleRequest {
    fn search_task(
        filter: Option<&Self::Filter>,
        query: String,
        limit: i64,
        offset: i64,
    ) -> super::Select {
        super::Select::search(
            Table::ProjectsTeamsRoleRequests,
            filter,
            query,
            limit,
            offset,
        )
    }
}
impl Searchable<false> for ProjectsTeamsRole {
    fn search_task(
        filter: Option<&Self::Filter>,
        query: String,
        limit: i64,
        offset: i64,
    ) -> super::Select {
        super::Select::search(Table::ProjectsTeamsRoles, filter, query, limit, offset)
    }
}
impl Searchable<false> for ProjectsUsersRoleInvitation {
    fn search_task(
        filter: Option<&Self::Filter>,
        query: String,
        limit: i64,
        offset: i64,
    ) -> super::Select {
        super::Select::search(
            Table::ProjectsUsersRoleInvitations,
            filter,
            query,
            limit,
            offset,
        )
    }
}
impl Searchable<false> for ProjectsUsersRoleRequest {
    fn search_task(
        filter: Option<&Self::Filter>,
        query: String,
        limit: i64,
        offset: i64,
    ) -> super::Select {
        super::Select::search(
            Table::ProjectsUsersRoleRequests,
            filter,
            query,
            limit,
            offset,
        )
    }
}
impl Searchable<false> for ProjectsUsersRole {
    fn search_task(
        filter: Option<&Self::Filter>,
        query: String,
        limit: i64,
        offset: i64,
    ) -> super::Select {
        super::Select::search(Table::ProjectsUsersRoles, filter, query, limit, offset)
    }
}
impl SimilarityScore for Rank {
    fn similarity_score<S: AsRef<str>>(&self, lowercase_query: S) -> f64 {
        let mut number_of_attributes = 0;
        let mut total_similarity_score = 0.0;
        number_of_attributes += 1;
        let name: &str = self.name.as_ref();
        total_similarity_score +=
            strsim::normalized_damerau_levenshtein(&name.to_lowercase(), lowercase_query.as_ref());
        number_of_attributes += 1;
        let description: &str = self.description.as_ref();
        total_similarity_score += strsim::normalized_damerau_levenshtein(
            &description.to_lowercase(),
            lowercase_query.as_ref(),
        );
        total_similarity_score / number_of_attributes as f64
    }
}
impl Searchable<false> for Rank {
    fn search_task(
        filter: Option<&Self::Filter>,
        query: String,
        limit: i64,
        offset: i64,
    ) -> super::Select {
        super::Select::search(Table::Ranks, filter, query, limit, offset)
    }
}
impl SimilarityScore for Role {
    fn similarity_score<S: AsRef<str>>(&self, lowercase_query: S) -> f64 {
        let mut number_of_attributes = 0;
        let mut total_similarity_score = 0.0;
        number_of_attributes += 1;
        let name: &str = self.name.as_ref();
        total_similarity_score +=
            strsim::normalized_damerau_levenshtein(&name.to_lowercase(), lowercase_query.as_ref());
        number_of_attributes += 1;
        let description: &str = self.description.as_ref();
        total_similarity_score += strsim::normalized_damerau_levenshtein(
            &description.to_lowercase(),
            lowercase_query.as_ref(),
        );
        total_similarity_score / number_of_attributes as f64
    }
}
impl Searchable<false> for Role {
    fn search_task(
        filter: Option<&Self::Filter>,
        query: String,
        limit: i64,
        offset: i64,
    ) -> super::Select {
        super::Select::search(Table::Roles, filter, query, limit, offset)
    }
}
impl SimilarityScore for SampleContainerCategory {
    fn similarity_score<S: AsRef<str>>(&self, lowercase_query: S) -> f64 {
        let mut number_of_attributes = 0;
        let mut total_similarity_score = 0.0;
        number_of_attributes += 1;
        let name: &str = self.name.as_ref();
        total_similarity_score +=
            strsim::normalized_damerau_levenshtein(&name.to_lowercase(), lowercase_query.as_ref());
        number_of_attributes += 1;
        let unit: &str = self.unit.as_ref();
        total_similarity_score +=
            strsim::normalized_damerau_levenshtein(&unit.to_lowercase(), lowercase_query.as_ref());
        number_of_attributes += 1;
        let description: &str = self.description.as_ref();
        total_similarity_score += strsim::normalized_damerau_levenshtein(
            &description.to_lowercase(),
            lowercase_query.as_ref(),
        );
        total_similarity_score / number_of_attributes as f64
    }
}
impl Searchable<false> for SampleContainerCategory {
    fn search_task(
        filter: Option<&Self::Filter>,
        query: String,
        limit: i64,
        offset: i64,
    ) -> super::Select {
        super::Select::search(
            Table::SampleContainerCategories,
            filter,
            query,
            limit,
            offset,
        )
    }
}
impl SimilarityScore for SampleContainer {
    fn similarity_score<S: AsRef<str>>(&self, lowercase_query: S) -> f64 {
        let mut number_of_attributes = 0;
        let mut total_similarity_score = 0.0;
        number_of_attributes += 1;
        let barcode: &str = self.barcode.as_ref();
        total_similarity_score += strsim::normalized_damerau_levenshtein(
            &barcode.to_lowercase(),
            lowercase_query.as_ref(),
        );
        total_similarity_score / number_of_attributes as f64
    }
}
impl Searchable<false> for SampleContainer {
    fn search_task(
        filter: Option<&Self::Filter>,
        query: String,
        limit: i64,
        offset: i64,
    ) -> super::Select {
        super::Select::search(Table::SampleContainers, filter, query, limit, offset)
    }
}
impl Searchable<true> for SampleContainer {
    fn search_task(
        filter: Option<&Self::Filter>,
        query: String,
        limit: i64,
        offset: i64,
    ) -> super::Select {
        super::Select::search_updatables(Table::SampleContainers, filter, query, limit, offset)
    }
}
impl SimilarityScore for SampleState {
    fn similarity_score<S: AsRef<str>>(&self, lowercase_query: S) -> f64 {
        let mut number_of_attributes = 0;
        let mut total_similarity_score = 0.0;
        number_of_attributes += 1;
        let name: &str = self.name.as_ref();
        total_similarity_score +=
            strsim::normalized_damerau_levenshtein(&name.to_lowercase(), lowercase_query.as_ref());
        number_of_attributes += 1;
        let description: &str = self.description.as_ref();
        total_similarity_score += strsim::normalized_damerau_levenshtein(
            &description.to_lowercase(),
            lowercase_query.as_ref(),
        );
        total_similarity_score / number_of_attributes as f64
    }
}
impl Searchable<false> for SampleState {
    fn search_task(
        filter: Option<&Self::Filter>,
        query: String,
        limit: i64,
        offset: i64,
    ) -> super::Select {
        super::Select::search(Table::SampleStates, filter, query, limit, offset)
    }
}
impl Searchable<false> for SampleTaxon {
    fn search_task(
        filter: Option<&Self::Filter>,
        query: String,
        limit: i64,
        offset: i64,
    ) -> super::Select {
        super::Select::search(Table::SampleTaxa, filter, query, limit, offset)
    }
}
impl SimilarityScore for Sample {
    fn similarity_score<S: AsRef<str>>(&self, lowercase_query: S) -> f64 {
        let mut number_of_attributes = 0;
        let mut total_similarity_score = 0.0;
        if let Some(notes) = &self.notes {
            number_of_attributes += 1;
            let notes: &str = notes.as_ref();
            total_similarity_score += strsim::normalized_damerau_levenshtein(
                &notes.to_lowercase(),
                lowercase_query.as_ref(),
            );
        }
        total_similarity_score / number_of_attributes as f64
    }
}
impl Searchable<false> for Sample {
    fn search_task(
        filter: Option<&Self::Filter>,
        query: String,
        limit: i64,
        offset: i64,
    ) -> super::Select {
        super::Select::search(Table::Samples, filter, query, limit, offset)
    }
}
impl Searchable<true> for Sample {
    fn search_task(
        filter: Option<&Self::Filter>,
        query: String,
        limit: i64,
        offset: i64,
    ) -> super::Select {
        super::Select::search_updatables(Table::Samples, filter, query, limit, offset)
    }
}
impl SimilarityScore for SpectraCollection {
    fn similarity_score<S: AsRef<str>>(&self, lowercase_query: S) -> f64 {
        let mut number_of_attributes = 0;
        let mut total_similarity_score = 0.0;
        if let Some(notes) = &self.notes {
            number_of_attributes += 1;
            let notes: &str = notes.as_ref();
            total_similarity_score += strsim::normalized_damerau_levenshtein(
                &notes.to_lowercase(),
                lowercase_query.as_ref(),
            );
        }
        total_similarity_score / number_of_attributes as f64
    }
}
impl Searchable<false> for SpectraCollection {
    fn search_task(
        filter: Option<&Self::Filter>,
        query: String,
        limit: i64,
        offset: i64,
    ) -> super::Select {
        super::Select::search(Table::SpectraCollections, filter, query, limit, offset)
    }
}
impl Searchable<true> for SpectraCollection {
    fn search_task(
        filter: Option<&Self::Filter>,
        query: String,
        limit: i64,
        offset: i64,
    ) -> super::Select {
        super::Select::search_updatables(Table::SpectraCollections, filter, query, limit, offset)
    }
}
impl SimilarityScore for Taxon {
    fn similarity_score<S: AsRef<str>>(&self, lowercase_query: S) -> f64 {
        let mut number_of_attributes = 0;
        let mut total_similarity_score = 0.0;
        number_of_attributes += 1;
        let name: &str = self.name.as_ref();
        total_similarity_score +=
            strsim::normalized_damerau_levenshtein(&name.to_lowercase(), lowercase_query.as_ref());
        total_similarity_score / number_of_attributes as f64
    }
}
impl Searchable<false> for Taxon {
    fn search_task(
        filter: Option<&Self::Filter>,
        query: String,
        limit: i64,
        offset: i64,
    ) -> super::Select {
        super::Select::search(Table::Taxa, filter, query, limit, offset)
    }
}
impl SimilarityScore for TeamState {
    fn similarity_score<S: AsRef<str>>(&self, lowercase_query: S) -> f64 {
        let mut number_of_attributes = 0;
        let mut total_similarity_score = 0.0;
        number_of_attributes += 1;
        let name: &str = self.name.as_ref();
        total_similarity_score +=
            strsim::normalized_damerau_levenshtein(&name.to_lowercase(), lowercase_query.as_ref());
        number_of_attributes += 1;
        let description: &str = self.description.as_ref();
        total_similarity_score += strsim::normalized_damerau_levenshtein(
            &description.to_lowercase(),
            lowercase_query.as_ref(),
        );
        total_similarity_score / number_of_attributes as f64
    }
}
impl Searchable<false> for TeamState {
    fn search_task(
        filter: Option<&Self::Filter>,
        query: String,
        limit: i64,
        offset: i64,
    ) -> super::Select {
        super::Select::search(Table::TeamStates, filter, query, limit, offset)
    }
}
impl SimilarityScore for Team {
    fn similarity_score<S: AsRef<str>>(&self, lowercase_query: S) -> f64 {
        let mut number_of_attributes = 0;
        let mut total_similarity_score = 0.0;
        number_of_attributes += 1;
        let name: &str = self.name.as_ref();
        total_similarity_score +=
            strsim::normalized_damerau_levenshtein(&name.to_lowercase(), lowercase_query.as_ref());
        number_of_attributes += 1;
        let description: &str = self.description.as_ref();
        total_similarity_score += strsim::normalized_damerau_levenshtein(
            &description.to_lowercase(),
            lowercase_query.as_ref(),
        );
        total_similarity_score / number_of_attributes as f64
    }
}
impl Searchable<false> for Team {
    fn search_task(
        filter: Option<&Self::Filter>,
        query: String,
        limit: i64,
        offset: i64,
    ) -> super::Select {
        super::Select::search(Table::Teams, filter, query, limit, offset)
    }
}
impl Searchable<true> for Team {
    fn search_task(
        filter: Option<&Self::Filter>,
        query: String,
        limit: i64,
        offset: i64,
    ) -> super::Select {
        super::Select::search_updatables(Table::Teams, filter, query, limit, offset)
    }
}
impl Searchable<false> for TeamsTeamsRoleInvitation {
    fn search_task(
        filter: Option<&Self::Filter>,
        query: String,
        limit: i64,
        offset: i64,
    ) -> super::Select {
        super::Select::search(
            Table::TeamsTeamsRoleInvitations,
            filter,
            query,
            limit,
            offset,
        )
    }
}
impl Searchable<false> for TeamsUsersRoleInvitation {
    fn search_task(
        filter: Option<&Self::Filter>,
        query: String,
        limit: i64,
        offset: i64,
    ) -> super::Select {
        super::Select::search(
            Table::TeamsUsersRoleInvitations,
            filter,
            query,
            limit,
            offset,
        )
    }
}
impl Searchable<false> for TeamsUsersRoleRequest {
    fn search_task(
        filter: Option<&Self::Filter>,
        query: String,
        limit: i64,
        offset: i64,
    ) -> super::Select {
        super::Select::search(Table::TeamsUsersRoleRequests, filter, query, limit, offset)
    }
}
impl Searchable<false> for TeamsUsersRole {
    fn search_task(
        filter: Option<&Self::Filter>,
        query: String,
        limit: i64,
        offset: i64,
    ) -> super::Select {
        super::Select::search(Table::TeamsUsersRoles, filter, query, limit, offset)
    }
}
impl SimilarityScore for Unit {
    fn similarity_score<S: AsRef<str>>(&self, lowercase_query: S) -> f64 {
        let mut number_of_attributes = 0;
        let mut total_similarity_score = 0.0;
        number_of_attributes += 1;
        let name: &str = self.name.as_ref();
        total_similarity_score +=
            strsim::normalized_damerau_levenshtein(&name.to_lowercase(), lowercase_query.as_ref());
        number_of_attributes += 1;
        let unit: &str = self.unit.as_ref();
        total_similarity_score +=
            strsim::normalized_damerau_levenshtein(&unit.to_lowercase(), lowercase_query.as_ref());
        total_similarity_score / number_of_attributes as f64
    }
}
impl Searchable<false> for Unit {
    fn search_task(
        filter: Option<&Self::Filter>,
        query: String,
        limit: i64,
        offset: i64,
    ) -> super::Select {
        super::Select::search(Table::Units, filter, query, limit, offset)
    }
}
impl SimilarityScore for UserEmail {
    fn similarity_score<S: AsRef<str>>(&self, lowercase_query: S) -> f64 {
        let mut number_of_attributes = 0;
        let mut total_similarity_score = 0.0;
        number_of_attributes += 1;
        let email: &str = self.email.as_ref();
        total_similarity_score +=
            strsim::normalized_damerau_levenshtein(&email.to_lowercase(), lowercase_query.as_ref());
        total_similarity_score / number_of_attributes as f64
    }
}
impl SimilarityScore for User {
    fn similarity_score<S: AsRef<str>>(&self, lowercase_query: S) -> f64 {
        let mut number_of_attributes = 0;
        let mut total_similarity_score = 0.0;
        number_of_attributes += 1;
        let first_name: &str = self.first_name.as_ref();
        total_similarity_score += strsim::normalized_damerau_levenshtein(
            &first_name.to_lowercase(),
            lowercase_query.as_ref(),
        );
        if let Some(middle_name) = &self.middle_name {
            number_of_attributes += 1;
            let middle_name: &str = middle_name.as_ref();
            total_similarity_score += strsim::normalized_damerau_levenshtein(
                &middle_name.to_lowercase(),
                lowercase_query.as_ref(),
            );
        }
        number_of_attributes += 1;
        let last_name: &str = self.last_name.as_ref();
        total_similarity_score += strsim::normalized_damerau_levenshtein(
            &last_name.to_lowercase(),
            lowercase_query.as_ref(),
        );
        if let Some(description) = &self.description {
            number_of_attributes += 1;
            let description: &str = description.as_ref();
            total_similarity_score += strsim::normalized_damerau_levenshtein(
                &description.to_lowercase(),
                lowercase_query.as_ref(),
            );
        }
        total_similarity_score / number_of_attributes as f64
    }
}
impl Searchable<false> for User {
    fn search_task(
        filter: Option<&Self::Filter>,
        query: String,
        limit: i64,
        offset: i64,
    ) -> super::Select {
        super::Select::search(Table::Users, filter, query, limit, offset)
    }
}
impl Searchable<true> for User {
    fn search_task(
        filter: Option<&Self::Filter>,
        query: String,
        limit: i64,
        offset: i64,
    ) -> super::Select {
        super::Select::search_updatables(Table::Users, filter, query, limit, offset)
    }
}
impl Searchable<false> for UsersUsersRoleInvitation {
    fn search_task(
        filter: Option<&Self::Filter>,
        query: String,
        limit: i64,
        offset: i64,
    ) -> super::Select {
        super::Select::search(
            Table::UsersUsersRoleInvitations,
            filter,
            query,
            limit,
            offset,
        )
    }
}
impl Searchable<false> for UsersUsersRoleRequest {
    fn search_task(
        filter: Option<&Self::Filter>,
        query: String,
        limit: i64,
        offset: i64,
    ) -> super::Select {
        super::Select::search(Table::UsersUsersRoleRequests, filter, query, limit, offset)
    }
}
impl Searchable<false> for UsersUsersRole {
    fn search_task(
        filter: Option<&Self::Filter>,
        query: String,
        limit: i64,
        offset: i64,
    ) -> super::Select {
        super::Select::search(Table::UsersUsersRoles, filter, query, limit, offset)
    }
}
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub enum SearchableStruct {
    NestedDerivedSample(Rc<NestedDerivedSample>),
    NestedDocumentFormat(Rc<NestedDocumentFormat>),
    NestedNameplateCategory(Rc<NestedNameplateCategory>),
    NestedNameplate(Rc<NestedNameplate>),
    NestedObservationSubject(Rc<NestedObservationSubject>),
    NestedObservation(Rc<NestedObservation>),
    NestedOrganismTaxon(Rc<NestedOrganismTaxon>),
    NestedOrganism(Rc<NestedOrganism>),
    NestedOrganization(Rc<NestedOrganization>),
    NestedProjectState(Rc<NestedProjectState>),
    NestedProject(Rc<NestedProject>),
    NestedProjectsTeamsRoleInvitation(Rc<NestedProjectsTeamsRoleInvitation>),
    NestedProjectsTeamsRoleRequest(Rc<NestedProjectsTeamsRoleRequest>),
    NestedProjectsTeamsRole(Rc<NestedProjectsTeamsRole>),
    NestedProjectsUsersRoleInvitation(Rc<NestedProjectsUsersRoleInvitation>),
    NestedProjectsUsersRoleRequest(Rc<NestedProjectsUsersRoleRequest>),
    NestedProjectsUsersRole(Rc<NestedProjectsUsersRole>),
    NestedRank(Rc<NestedRank>),
    NestedRole(Rc<NestedRole>),
    NestedSampleContainerCategory(Rc<NestedSampleContainerCategory>),
    NestedSampleContainer(Rc<NestedSampleContainer>),
    NestedSampleState(Rc<NestedSampleState>),
    NestedSampleTaxon(Rc<NestedSampleTaxon>),
    NestedSample(Rc<NestedSample>),
    NestedSpectraCollection(Rc<NestedSpectraCollection>),
    NestedTaxon(Rc<NestedTaxon>),
    NestedTeamState(Rc<NestedTeamState>),
    NestedTeam(Rc<NestedTeam>),
    NestedTeamsTeamsRoleInvitation(Rc<NestedTeamsTeamsRoleInvitation>),
    NestedTeamsUsersRoleInvitation(Rc<NestedTeamsUsersRoleInvitation>),
    NestedTeamsUsersRoleRequest(Rc<NestedTeamsUsersRoleRequest>),
    NestedTeamsUsersRole(Rc<NestedTeamsUsersRole>),
    NestedUnit(Rc<NestedUnit>),
    NestedUser(Rc<NestedUser>),
    NestedUsersUsersRoleInvitation(Rc<NestedUsersUsersRoleInvitation>),
    NestedUsersUsersRoleRequest(Rc<NestedUsersUsersRoleRequest>),
    NestedUsersUsersRole(Rc<NestedUsersUsersRole>),
    Color(Rc<Color>),
    Country(Rc<Country>),
    FontAwesomeIcon(Rc<FontAwesomeIcon>),
}

impl Filtrable for SearchableStruct {
    type Filter = EmptyFilter;
}
impl Searchable<false> for SearchableStruct {
    fn search_task(
        _filter: Option<&Self::Filter>,
        query: String,
        limit: i64,
        _offset: i64,
    ) -> super::Select {
        super::Select::search_all(query, limit)
    }
}
impl From<NestedDerivedSample> for SearchableStruct {
    fn from(value: NestedDerivedSample) -> Self {
        SearchableStruct::NestedDerivedSample(Rc::from(value))
    }
}
impl From<NestedDocumentFormat> for SearchableStruct {
    fn from(value: NestedDocumentFormat) -> Self {
        SearchableStruct::NestedDocumentFormat(Rc::from(value))
    }
}
impl From<NestedNameplateCategory> for SearchableStruct {
    fn from(value: NestedNameplateCategory) -> Self {
        SearchableStruct::NestedNameplateCategory(Rc::from(value))
    }
}
impl From<NestedNameplate> for SearchableStruct {
    fn from(value: NestedNameplate) -> Self {
        SearchableStruct::NestedNameplate(Rc::from(value))
    }
}
impl From<NestedObservationSubject> for SearchableStruct {
    fn from(value: NestedObservationSubject) -> Self {
        SearchableStruct::NestedObservationSubject(Rc::from(value))
    }
}
impl From<NestedObservation> for SearchableStruct {
    fn from(value: NestedObservation) -> Self {
        SearchableStruct::NestedObservation(Rc::from(value))
    }
}
impl From<NestedOrganismTaxon> for SearchableStruct {
    fn from(value: NestedOrganismTaxon) -> Self {
        SearchableStruct::NestedOrganismTaxon(Rc::from(value))
    }
}
impl From<NestedOrganism> for SearchableStruct {
    fn from(value: NestedOrganism) -> Self {
        SearchableStruct::NestedOrganism(Rc::from(value))
    }
}
impl From<NestedOrganization> for SearchableStruct {
    fn from(value: NestedOrganization) -> Self {
        SearchableStruct::NestedOrganization(Rc::from(value))
    }
}
impl From<NestedProjectState> for SearchableStruct {
    fn from(value: NestedProjectState) -> Self {
        SearchableStruct::NestedProjectState(Rc::from(value))
    }
}
impl From<NestedProject> for SearchableStruct {
    fn from(value: NestedProject) -> Self {
        SearchableStruct::NestedProject(Rc::from(value))
    }
}
impl From<NestedProjectsTeamsRoleInvitation> for SearchableStruct {
    fn from(value: NestedProjectsTeamsRoleInvitation) -> Self {
        SearchableStruct::NestedProjectsTeamsRoleInvitation(Rc::from(value))
    }
}
impl From<NestedProjectsTeamsRoleRequest> for SearchableStruct {
    fn from(value: NestedProjectsTeamsRoleRequest) -> Self {
        SearchableStruct::NestedProjectsTeamsRoleRequest(Rc::from(value))
    }
}
impl From<NestedProjectsTeamsRole> for SearchableStruct {
    fn from(value: NestedProjectsTeamsRole) -> Self {
        SearchableStruct::NestedProjectsTeamsRole(Rc::from(value))
    }
}
impl From<NestedProjectsUsersRoleInvitation> for SearchableStruct {
    fn from(value: NestedProjectsUsersRoleInvitation) -> Self {
        SearchableStruct::NestedProjectsUsersRoleInvitation(Rc::from(value))
    }
}
impl From<NestedProjectsUsersRoleRequest> for SearchableStruct {
    fn from(value: NestedProjectsUsersRoleRequest) -> Self {
        SearchableStruct::NestedProjectsUsersRoleRequest(Rc::from(value))
    }
}
impl From<NestedProjectsUsersRole> for SearchableStruct {
    fn from(value: NestedProjectsUsersRole) -> Self {
        SearchableStruct::NestedProjectsUsersRole(Rc::from(value))
    }
}
impl From<NestedRank> for SearchableStruct {
    fn from(value: NestedRank) -> Self {
        SearchableStruct::NestedRank(Rc::from(value))
    }
}
impl From<NestedRole> for SearchableStruct {
    fn from(value: NestedRole) -> Self {
        SearchableStruct::NestedRole(Rc::from(value))
    }
}
impl From<NestedSampleContainerCategory> for SearchableStruct {
    fn from(value: NestedSampleContainerCategory) -> Self {
        SearchableStruct::NestedSampleContainerCategory(Rc::from(value))
    }
}
impl From<NestedSampleContainer> for SearchableStruct {
    fn from(value: NestedSampleContainer) -> Self {
        SearchableStruct::NestedSampleContainer(Rc::from(value))
    }
}
impl From<NestedSampleState> for SearchableStruct {
    fn from(value: NestedSampleState) -> Self {
        SearchableStruct::NestedSampleState(Rc::from(value))
    }
}
impl From<NestedSampleTaxon> for SearchableStruct {
    fn from(value: NestedSampleTaxon) -> Self {
        SearchableStruct::NestedSampleTaxon(Rc::from(value))
    }
}
impl From<NestedSample> for SearchableStruct {
    fn from(value: NestedSample) -> Self {
        SearchableStruct::NestedSample(Rc::from(value))
    }
}
impl From<NestedSpectraCollection> for SearchableStruct {
    fn from(value: NestedSpectraCollection) -> Self {
        SearchableStruct::NestedSpectraCollection(Rc::from(value))
    }
}
impl From<NestedTaxon> for SearchableStruct {
    fn from(value: NestedTaxon) -> Self {
        SearchableStruct::NestedTaxon(Rc::from(value))
    }
}
impl From<NestedTeamState> for SearchableStruct {
    fn from(value: NestedTeamState) -> Self {
        SearchableStruct::NestedTeamState(Rc::from(value))
    }
}
impl From<NestedTeam> for SearchableStruct {
    fn from(value: NestedTeam) -> Self {
        SearchableStruct::NestedTeam(Rc::from(value))
    }
}
impl From<NestedTeamsTeamsRoleInvitation> for SearchableStruct {
    fn from(value: NestedTeamsTeamsRoleInvitation) -> Self {
        SearchableStruct::NestedTeamsTeamsRoleInvitation(Rc::from(value))
    }
}
impl From<NestedTeamsUsersRoleInvitation> for SearchableStruct {
    fn from(value: NestedTeamsUsersRoleInvitation) -> Self {
        SearchableStruct::NestedTeamsUsersRoleInvitation(Rc::from(value))
    }
}
impl From<NestedTeamsUsersRoleRequest> for SearchableStruct {
    fn from(value: NestedTeamsUsersRoleRequest) -> Self {
        SearchableStruct::NestedTeamsUsersRoleRequest(Rc::from(value))
    }
}
impl From<NestedTeamsUsersRole> for SearchableStruct {
    fn from(value: NestedTeamsUsersRole) -> Self {
        SearchableStruct::NestedTeamsUsersRole(Rc::from(value))
    }
}
impl From<NestedUnit> for SearchableStruct {
    fn from(value: NestedUnit) -> Self {
        SearchableStruct::NestedUnit(Rc::from(value))
    }
}
impl From<NestedUser> for SearchableStruct {
    fn from(value: NestedUser) -> Self {
        SearchableStruct::NestedUser(Rc::from(value))
    }
}
impl From<NestedUsersUsersRoleInvitation> for SearchableStruct {
    fn from(value: NestedUsersUsersRoleInvitation) -> Self {
        SearchableStruct::NestedUsersUsersRoleInvitation(Rc::from(value))
    }
}
impl From<NestedUsersUsersRoleRequest> for SearchableStruct {
    fn from(value: NestedUsersUsersRoleRequest) -> Self {
        SearchableStruct::NestedUsersUsersRoleRequest(Rc::from(value))
    }
}
impl From<NestedUsersUsersRole> for SearchableStruct {
    fn from(value: NestedUsersUsersRole) -> Self {
        SearchableStruct::NestedUsersUsersRole(Rc::from(value))
    }
}
impl From<Color> for SearchableStruct {
    fn from(value: Color) -> Self {
        SearchableStruct::Color(Rc::from(value))
    }
}
impl From<Country> for SearchableStruct {
    fn from(value: Country) -> Self {
        SearchableStruct::Country(Rc::from(value))
    }
}
impl From<FontAwesomeIcon> for SearchableStruct {
    fn from(value: FontAwesomeIcon) -> Self {
        SearchableStruct::FontAwesomeIcon(Rc::from(value))
    }
}
impl Describable for SearchableStruct {
    fn description(&self) -> Option<&str> {
        match self {
            SearchableStruct::NestedDerivedSample(value) => value.description(),
            SearchableStruct::NestedDocumentFormat(value) => value.description(),
            SearchableStruct::NestedNameplateCategory(value) => value.description(),
            SearchableStruct::NestedNameplate(value) => value.description(),
            SearchableStruct::NestedObservationSubject(value) => value.description(),
            SearchableStruct::NestedObservation(value) => value.description(),
            SearchableStruct::NestedOrganismTaxon(value) => value.description(),
            SearchableStruct::NestedOrganism(value) => value.description(),
            SearchableStruct::NestedOrganization(value) => value.description(),
            SearchableStruct::NestedProjectState(value) => value.description(),
            SearchableStruct::NestedProject(value) => value.description(),
            SearchableStruct::NestedProjectsTeamsRoleInvitation(value) => value.description(),
            SearchableStruct::NestedProjectsTeamsRoleRequest(value) => value.description(),
            SearchableStruct::NestedProjectsTeamsRole(value) => value.description(),
            SearchableStruct::NestedProjectsUsersRoleInvitation(value) => value.description(),
            SearchableStruct::NestedProjectsUsersRoleRequest(value) => value.description(),
            SearchableStruct::NestedProjectsUsersRole(value) => value.description(),
            SearchableStruct::NestedRank(value) => value.description(),
            SearchableStruct::NestedRole(value) => value.description(),
            SearchableStruct::NestedSampleContainerCategory(value) => value.description(),
            SearchableStruct::NestedSampleContainer(value) => value.description(),
            SearchableStruct::NestedSampleState(value) => value.description(),
            SearchableStruct::NestedSampleTaxon(value) => value.description(),
            SearchableStruct::NestedSample(value) => value.description(),
            SearchableStruct::NestedSpectraCollection(value) => value.description(),
            SearchableStruct::NestedTaxon(value) => value.description(),
            SearchableStruct::NestedTeamState(value) => value.description(),
            SearchableStruct::NestedTeam(value) => value.description(),
            SearchableStruct::NestedTeamsTeamsRoleInvitation(value) => value.description(),
            SearchableStruct::NestedTeamsUsersRoleInvitation(value) => value.description(),
            SearchableStruct::NestedTeamsUsersRoleRequest(value) => value.description(),
            SearchableStruct::NestedTeamsUsersRole(value) => value.description(),
            SearchableStruct::NestedUnit(value) => value.description(),
            SearchableStruct::NestedUser(value) => value.description(),
            SearchableStruct::NestedUsersUsersRoleInvitation(value) => value.description(),
            SearchableStruct::NestedUsersUsersRoleRequest(value) => value.description(),
            SearchableStruct::NestedUsersUsersRole(value) => value.description(),
            SearchableStruct::Color(value) => value.description(),
            SearchableStruct::Country(value) => value.description(),
            SearchableStruct::FontAwesomeIcon(value) => value.description(),
        }
    }
}
impl Colorable for SearchableStruct {
    fn color(&self) -> Option<&str> {
        match self {
            SearchableStruct::NestedDerivedSample(value) => value.color(),
            SearchableStruct::NestedDocumentFormat(value) => value.color(),
            SearchableStruct::NestedNameplateCategory(value) => value.color(),
            SearchableStruct::NestedNameplate(value) => value.color(),
            SearchableStruct::NestedObservationSubject(value) => value.color(),
            SearchableStruct::NestedObservation(value) => value.color(),
            SearchableStruct::NestedOrganismTaxon(value) => value.color(),
            SearchableStruct::NestedOrganism(value) => value.color(),
            SearchableStruct::NestedOrganization(value) => value.color(),
            SearchableStruct::NestedProjectState(value) => value.color(),
            SearchableStruct::NestedProject(value) => value.color(),
            SearchableStruct::NestedProjectsTeamsRoleInvitation(value) => value.color(),
            SearchableStruct::NestedProjectsTeamsRoleRequest(value) => value.color(),
            SearchableStruct::NestedProjectsTeamsRole(value) => value.color(),
            SearchableStruct::NestedProjectsUsersRoleInvitation(value) => value.color(),
            SearchableStruct::NestedProjectsUsersRoleRequest(value) => value.color(),
            SearchableStruct::NestedProjectsUsersRole(value) => value.color(),
            SearchableStruct::NestedRank(value) => value.color(),
            SearchableStruct::NestedRole(value) => value.color(),
            SearchableStruct::NestedSampleContainerCategory(value) => value.color(),
            SearchableStruct::NestedSampleContainer(value) => value.color(),
            SearchableStruct::NestedSampleState(value) => value.color(),
            SearchableStruct::NestedSampleTaxon(value) => value.color(),
            SearchableStruct::NestedSample(value) => value.color(),
            SearchableStruct::NestedSpectraCollection(value) => value.color(),
            SearchableStruct::NestedTaxon(value) => value.color(),
            SearchableStruct::NestedTeamState(value) => value.color(),
            SearchableStruct::NestedTeam(value) => value.color(),
            SearchableStruct::NestedTeamsTeamsRoleInvitation(value) => value.color(),
            SearchableStruct::NestedTeamsUsersRoleInvitation(value) => value.color(),
            SearchableStruct::NestedTeamsUsersRoleRequest(value) => value.color(),
            SearchableStruct::NestedTeamsUsersRole(value) => value.color(),
            SearchableStruct::NestedUnit(value) => value.color(),
            SearchableStruct::NestedUser(value) => value.color(),
            SearchableStruct::NestedUsersUsersRoleInvitation(value) => value.color(),
            SearchableStruct::NestedUsersUsersRoleRequest(value) => value.color(),
            SearchableStruct::NestedUsersUsersRole(value) => value.color(),
            SearchableStruct::Color(value) => value.color(),
            SearchableStruct::Country(value) => value.color(),
            SearchableStruct::FontAwesomeIcon(value) => value.color(),
        }
    }
}
impl SimilarityScore for SearchableStruct {
    fn similarity_score<S: AsRef<str>>(&self, lowercase_query: S) -> f64 {
        match self {
            SearchableStruct::NestedDerivedSample(value) => value.similarity_score(lowercase_query),
            SearchableStruct::NestedDocumentFormat(value) => {
                value.similarity_score(lowercase_query)
            }
            SearchableStruct::NestedNameplateCategory(value) => {
                value.similarity_score(lowercase_query)
            }
            SearchableStruct::NestedNameplate(value) => value.similarity_score(lowercase_query),
            SearchableStruct::NestedObservationSubject(value) => {
                value.similarity_score(lowercase_query)
            }
            SearchableStruct::NestedObservation(value) => value.similarity_score(lowercase_query),
            SearchableStruct::NestedOrganismTaxon(value) => value.similarity_score(lowercase_query),
            SearchableStruct::NestedOrganism(value) => value.similarity_score(lowercase_query),
            SearchableStruct::NestedOrganization(value) => value.similarity_score(lowercase_query),
            SearchableStruct::NestedProjectState(value) => value.similarity_score(lowercase_query),
            SearchableStruct::NestedProject(value) => value.similarity_score(lowercase_query),
            SearchableStruct::NestedProjectsTeamsRoleInvitation(value) => {
                value.similarity_score(lowercase_query)
            }
            SearchableStruct::NestedProjectsTeamsRoleRequest(value) => {
                value.similarity_score(lowercase_query)
            }
            SearchableStruct::NestedProjectsTeamsRole(value) => {
                value.similarity_score(lowercase_query)
            }
            SearchableStruct::NestedProjectsUsersRoleInvitation(value) => {
                value.similarity_score(lowercase_query)
            }
            SearchableStruct::NestedProjectsUsersRoleRequest(value) => {
                value.similarity_score(lowercase_query)
            }
            SearchableStruct::NestedProjectsUsersRole(value) => {
                value.similarity_score(lowercase_query)
            }
            SearchableStruct::NestedRank(value) => value.similarity_score(lowercase_query),
            SearchableStruct::NestedRole(value) => value.similarity_score(lowercase_query),
            SearchableStruct::NestedSampleContainerCategory(value) => {
                value.similarity_score(lowercase_query)
            }
            SearchableStruct::NestedSampleContainer(value) => {
                value.similarity_score(lowercase_query)
            }
            SearchableStruct::NestedSampleState(value) => value.similarity_score(lowercase_query),
            SearchableStruct::NestedSampleTaxon(value) => value.similarity_score(lowercase_query),
            SearchableStruct::NestedSample(value) => value.similarity_score(lowercase_query),
            SearchableStruct::NestedSpectraCollection(value) => {
                value.similarity_score(lowercase_query)
            }
            SearchableStruct::NestedTaxon(value) => value.similarity_score(lowercase_query),
            SearchableStruct::NestedTeamState(value) => value.similarity_score(lowercase_query),
            SearchableStruct::NestedTeam(value) => value.similarity_score(lowercase_query),
            SearchableStruct::NestedTeamsTeamsRoleInvitation(value) => {
                value.similarity_score(lowercase_query)
            }
            SearchableStruct::NestedTeamsUsersRoleInvitation(value) => {
                value.similarity_score(lowercase_query)
            }
            SearchableStruct::NestedTeamsUsersRoleRequest(value) => {
                value.similarity_score(lowercase_query)
            }
            SearchableStruct::NestedTeamsUsersRole(value) => {
                value.similarity_score(lowercase_query)
            }
            SearchableStruct::NestedUnit(value) => value.similarity_score(lowercase_query),
            SearchableStruct::NestedUser(value) => value.similarity_score(lowercase_query),
            SearchableStruct::NestedUsersUsersRoleInvitation(value) => {
                value.similarity_score(lowercase_query)
            }
            SearchableStruct::NestedUsersUsersRoleRequest(value) => {
                value.similarity_score(lowercase_query)
            }
            SearchableStruct::NestedUsersUsersRole(value) => {
                value.similarity_score(lowercase_query)
            }
            SearchableStruct::Color(value) => value.similarity_score(lowercase_query),
            SearchableStruct::Country(value) => value.similarity_score(lowercase_query),
            SearchableStruct::FontAwesomeIcon(value) => value.similarity_score(lowercase_query),
        }
    }
}
