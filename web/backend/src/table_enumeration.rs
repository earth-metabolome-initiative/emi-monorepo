//! This module contains the table names enumeration.
//!
//! This module is automatically generated. Do not write anything here.

use crate::models::*;
use crate::nested_models::*;
use diesel::r2d2::PooledConnection;
use diesel::r2d2::ConnectionManager;
use crate::new_variants::InsertRow;
use crate::update_variants::UpdateRow;

/// Trait providing the search method for the Table enum.
pub trait SearchableTable {
    /// Search the table by the query using the similarity method from PostgreSQL.
    ///
    /// # Arguments
    /// * `query` - The string to search for.
    /// * `limit` - The maximum number of results, by default `10`.
    /// * `connection` - The database connection.
    ///
    /// # Returns
    /// A serialized vector of the rows of the table, using bincode.
    fn similarity_search(
         &self,
         query: &str,
         limit: Option<i32>,
         connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>
) -> Result<Vec<Vec<u8>>, web_common::api::ApiError>;

    /// Search editables rows by the query using the similarity method from PostgreSQL.
    ///
    /// # Arguments
    /// * `user_id` - The user ID of the user performing the operation.
    /// * `query` - The string to search for.
    /// * `limit` - The maximum number of results, by default `10`.
    /// * `connection` - The database connection.
    ///
    /// # Returns
    /// A serialized vector of the rows of the table, using bincode.
    fn similarity_search_editables(
         &self,
         user_id: i32,
         query: &str,
         limit: Option<i32>,
         connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>
) -> Result<Vec<Vec<u8>>, web_common::api::ApiError>;

    /// Search the table by the query using the word_similarity method from PostgreSQL.
    ///
    /// # Arguments
    /// * `query` - The string to search for.
    /// * `limit` - The maximum number of results, by default `10`.
    /// * `connection` - The database connection.
    ///
    /// # Returns
    /// A serialized vector of the rows of the table, using bincode.
    fn word_similarity_search(
         &self,
         query: &str,
         limit: Option<i32>,
         connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>
) -> Result<Vec<Vec<u8>>, web_common::api::ApiError>;

    /// Search editables rows by the query using the word_similarity method from PostgreSQL.
    ///
    /// # Arguments
    /// * `user_id` - The user ID of the user performing the operation.
    /// * `query` - The string to search for.
    /// * `limit` - The maximum number of results, by default `10`.
    /// * `connection` - The database connection.
    ///
    /// # Returns
    /// A serialized vector of the rows of the table, using bincode.
    fn word_similarity_search_editables(
         &self,
         user_id: i32,
         query: &str,
         limit: Option<i32>,
         connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>
) -> Result<Vec<Vec<u8>>, web_common::api::ApiError>;

    /// Search the table by the query using the strict_word_similarity method from PostgreSQL.
    ///
    /// # Arguments
    /// * `query` - The string to search for.
    /// * `limit` - The maximum number of results, by default `10`.
    /// * `connection` - The database connection.
    ///
    /// # Returns
    /// A serialized vector of the rows of the table, using bincode.
    fn strict_word_similarity_search(
         &self,
         query: &str,
         limit: Option<i32>,
         connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>
) -> Result<Vec<Vec<u8>>, web_common::api::ApiError>;

    /// Search editables rows by the query using the strict_word_similarity method from PostgreSQL.
    ///
    /// # Arguments
    /// * `user_id` - The user ID of the user performing the operation.
    /// * `query` - The string to search for.
    /// * `limit` - The maximum number of results, by default `10`.
    /// * `connection` - The database connection.
    ///
    /// # Returns
    /// A serialized vector of the rows of the table, using bincode.
    fn strict_word_similarity_search_editables(
         &self,
         user_id: i32,
         query: &str,
         limit: Option<i32>,
         connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>
) -> Result<Vec<Vec<u8>>, web_common::api::ApiError>;

}

impl SearchableTable for web_common::database::Table {
    fn similarity_search(&self, query: &str, limit: Option<i32>, connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>) -> Result<Vec<Vec<u8>>, web_common::api::ApiError> {
        match self {
            web_common::database::Table::BioOttRanks => NestedBioOttRank::similarity_search(query, limit, connection)?.iter().map(|row| bincode::serialize(row).map_err(web_common::api::ApiError::from)).collect(),
            web_common::database::Table::BioOttTaxonItems => NestedBioOttTaxonItem::similarity_search(query, limit, connection)?.iter().map(|row| bincode::serialize(row).map_err(web_common::api::ApiError::from)).collect(),
            web_common::database::Table::Colors => Color::similarity_search(query, limit, connection)?.iter().map(|row| bincode::serialize(row).map_err(web_common::api::ApiError::from)).collect(),
            web_common::database::Table::Countries => Country::similarity_search(query, limit, connection)?.iter().map(|row| bincode::serialize(row).map_err(web_common::api::ApiError::from)).collect(),
            web_common::database::Table::DerivedSamples => unimplemented!("Table `derived_samples` does not have a GIN similarity index."),
            web_common::database::Table::DocumentFormats => NestedDocumentFormat::similarity_search(query, limit, connection)?.iter().map(|row| bincode::serialize(row).map_err(web_common::api::ApiError::from)).collect(),
            web_common::database::Table::FontAwesomeIcons => FontAwesomeIcon::similarity_search(query, limit, connection)?.iter().map(|row| bincode::serialize(row).map_err(web_common::api::ApiError::from)).collect(),
            web_common::database::Table::LoginProviders => unimplemented!("Table `login_providers` does not have a GIN similarity index."),
            web_common::database::Table::Notifications => unimplemented!("Table `notifications` does not have a GIN similarity index."),
            web_common::database::Table::Organizations => NestedOrganization::similarity_search(query, limit, connection)?.iter().map(|row| bincode::serialize(row).map_err(web_common::api::ApiError::from)).collect(),
            web_common::database::Table::ProjectStates => NestedProjectState::similarity_search(query, limit, connection)?.iter().map(|row| bincode::serialize(row).map_err(web_common::api::ApiError::from)).collect(),
            web_common::database::Table::Projects => NestedProject::similarity_search(query, limit, connection)?.iter().map(|row| bincode::serialize(row).map_err(web_common::api::ApiError::from)).collect(),
            web_common::database::Table::ProjectsTeamsRoleInvitations => unimplemented!("Table `projects_teams_role_invitations` does not have a GIN similarity index."),
            web_common::database::Table::ProjectsTeamsRoleRequests => unimplemented!("Table `projects_teams_role_requests` does not have a GIN similarity index."),
            web_common::database::Table::ProjectsTeamsRoles => unimplemented!("Table `projects_teams_roles` does not have a GIN similarity index."),
            web_common::database::Table::ProjectsUsersRoleInvitations => unimplemented!("Table `projects_users_role_invitations` does not have a GIN similarity index."),
            web_common::database::Table::ProjectsUsersRoleRequests => unimplemented!("Table `projects_users_role_requests` does not have a GIN similarity index."),
            web_common::database::Table::ProjectsUsersRoles => unimplemented!("Table `projects_users_roles` does not have a GIN similarity index."),
            web_common::database::Table::Roles => NestedRole::similarity_search(query, limit, connection)?.iter().map(|row| bincode::serialize(row).map_err(web_common::api::ApiError::from)).collect(),
            web_common::database::Table::SampleBioOttTaxonItems => unimplemented!("Table `sample_bio_ott_taxon_items` does not have a GIN similarity index."),
            web_common::database::Table::SampleStates => NestedSampleState::similarity_search(query, limit, connection)?.iter().map(|row| bincode::serialize(row).map_err(web_common::api::ApiError::from)).collect(),
            web_common::database::Table::SampledIndividualBioOttTaxonItems => unimplemented!("Table `sampled_individual_bio_ott_taxon_items` does not have a GIN similarity index."),
            web_common::database::Table::SampledIndividuals => NestedSampledIndividual::similarity_search(query, limit, connection)?.iter().map(|row| bincode::serialize(row).map_err(web_common::api::ApiError::from)).collect(),
            web_common::database::Table::SampledIndividualsTeamsRoleInvitations => unimplemented!("Table `sampled_individuals_teams_role_invitations` does not have a GIN similarity index."),
            web_common::database::Table::SampledIndividualsTeamsRoleRequests => unimplemented!("Table `sampled_individuals_teams_role_requests` does not have a GIN similarity index."),
            web_common::database::Table::SampledIndividualsTeamsRoles => unimplemented!("Table `sampled_individuals_teams_roles` does not have a GIN similarity index."),
            web_common::database::Table::SampledIndividualsUsersRoleInvitations => unimplemented!("Table `sampled_individuals_users_role_invitations` does not have a GIN similarity index."),
            web_common::database::Table::SampledIndividualsUsersRoleRequests => unimplemented!("Table `sampled_individuals_users_role_requests` does not have a GIN similarity index."),
            web_common::database::Table::SampledIndividualsUsersRoles => unimplemented!("Table `sampled_individuals_users_roles` does not have a GIN similarity index."),
            web_common::database::Table::Samples => NestedSample::similarity_search(query, limit, connection)?.iter().map(|row| bincode::serialize(row).map_err(web_common::api::ApiError::from)).collect(),
            web_common::database::Table::SamplesTeamsRoleInvitations => unimplemented!("Table `samples_teams_role_invitations` does not have a GIN similarity index."),
            web_common::database::Table::SamplesTeamsRoleRequests => unimplemented!("Table `samples_teams_role_requests` does not have a GIN similarity index."),
            web_common::database::Table::SamplesTeamsRoles => unimplemented!("Table `samples_teams_roles` does not have a GIN similarity index."),
            web_common::database::Table::SamplesUsersRoleInvitations => unimplemented!("Table `samples_users_role_invitations` does not have a GIN similarity index."),
            web_common::database::Table::SamplesUsersRoleRequests => unimplemented!("Table `samples_users_role_requests` does not have a GIN similarity index."),
            web_common::database::Table::SamplesUsersRoles => unimplemented!("Table `samples_users_roles` does not have a GIN similarity index."),
            web_common::database::Table::Spectra => unimplemented!("Table `spectra` does not have a GIN similarity index."),
            web_common::database::Table::SpectraCollections => unimplemented!("Table `spectra_collections` does not have a GIN similarity index."),
            web_common::database::Table::SpectraCollectionsTeamsRoleInvitations => unimplemented!("Table `spectra_collections_teams_role_invitations` does not have a GIN similarity index."),
            web_common::database::Table::SpectraCollectionsTeamsRoleRequests => unimplemented!("Table `spectra_collections_teams_role_requests` does not have a GIN similarity index."),
            web_common::database::Table::SpectraCollectionsTeamsRoles => unimplemented!("Table `spectra_collections_teams_roles` does not have a GIN similarity index."),
            web_common::database::Table::SpectraCollectionsUsersRoleInvitations => unimplemented!("Table `spectra_collections_users_role_invitations` does not have a GIN similarity index."),
            web_common::database::Table::SpectraCollectionsUsersRoleRequests => unimplemented!("Table `spectra_collections_users_role_requests` does not have a GIN similarity index."),
            web_common::database::Table::SpectraCollectionsUsersRoles => unimplemented!("Table `spectra_collections_users_roles` does not have a GIN similarity index."),
            web_common::database::Table::TeamStates => NestedTeamState::similarity_search(query, limit, connection)?.iter().map(|row| bincode::serialize(row).map_err(web_common::api::ApiError::from)).collect(),
            web_common::database::Table::Teams => NestedTeam::similarity_search(query, limit, connection)?.iter().map(|row| bincode::serialize(row).map_err(web_common::api::ApiError::from)).collect(),
            web_common::database::Table::TeamsTeamsRoleInvitations => unimplemented!("Table `teams_teams_role_invitations` does not have a GIN similarity index."),
            web_common::database::Table::TeamsUsersRoleInvitations => unimplemented!("Table `teams_users_role_invitations` does not have a GIN similarity index."),
            web_common::database::Table::TeamsUsersRoleRequests => unimplemented!("Table `teams_users_role_requests` does not have a GIN similarity index."),
            web_common::database::Table::TeamsUsersRoles => unimplemented!("Table `teams_users_roles` does not have a GIN similarity index."),
            web_common::database::Table::Units => Unit::similarity_search(query, limit, connection)?.iter().map(|row| bincode::serialize(row).map_err(web_common::api::ApiError::from)).collect(),
            web_common::database::Table::UserEmails => unimplemented!("Table `user_emails` does not have a GIN similarity index."),
            web_common::database::Table::Users => User::similarity_search(query, limit, connection)?.iter().map(|row| bincode::serialize(row).map_err(web_common::api::ApiError::from)).collect(),
            web_common::database::Table::UsersUsersRoleInvitations => unimplemented!("Table `users_users_role_invitations` does not have a GIN similarity index."),
            web_common::database::Table::UsersUsersRoleRequests => unimplemented!("Table `users_users_role_requests` does not have a GIN similarity index."),
            web_common::database::Table::UsersUsersRoles => unimplemented!("Table `users_users_roles` does not have a GIN similarity index."),
        }
    }
    fn similarity_search_editables(&self, user_id: i32, query: &str, limit: Option<i32>, connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>) -> Result<Vec<Vec<u8>>, web_common::api::ApiError> {
        match self {
            web_common::database::Table::BioOttRanks => unimplemented!("Table `bio_ott_ranks` does not have associated roles."),
            web_common::database::Table::BioOttTaxonItems => unimplemented!("Table `bio_ott_taxon_items` does not have associated roles."),
            web_common::database::Table::Colors => unimplemented!("Table `colors` does not have associated roles."),
            web_common::database::Table::Countries => unimplemented!("Table `countries` does not have associated roles."),
            web_common::database::Table::DerivedSamples => unimplemented!("Table `derived_samples` does not have a GIN similarity index."),
            web_common::database::Table::DocumentFormats => unimplemented!("Table `document_formats` does not have associated roles."),
            web_common::database::Table::FontAwesomeIcons => unimplemented!("Table `font_awesome_icons` does not have associated roles."),
            web_common::database::Table::LoginProviders => unimplemented!("Table `login_providers` does not have a GIN similarity index."),
            web_common::database::Table::Notifications => unimplemented!("Table `notifications` does not have a GIN similarity index."),
            web_common::database::Table::Organizations => unimplemented!("Table `organizations` does not have associated roles."),
            web_common::database::Table::ProjectStates => unimplemented!("Table `project_states` does not have associated roles."),
            web_common::database::Table::Projects => NestedProject::similarity_search_editables(user_id, query, limit, connection)?.iter().map(|row| bincode::serialize(row).map_err(web_common::api::ApiError::from)).collect(),
            web_common::database::Table::ProjectsTeamsRoleInvitations => unimplemented!("Table `projects_teams_role_invitations` does not have a GIN similarity index."),
            web_common::database::Table::ProjectsTeamsRoleRequests => unimplemented!("Table `projects_teams_role_requests` does not have a GIN similarity index."),
            web_common::database::Table::ProjectsTeamsRoles => unimplemented!("Table `projects_teams_roles` does not have a GIN similarity index."),
            web_common::database::Table::ProjectsUsersRoleInvitations => unimplemented!("Table `projects_users_role_invitations` does not have a GIN similarity index."),
            web_common::database::Table::ProjectsUsersRoleRequests => unimplemented!("Table `projects_users_role_requests` does not have a GIN similarity index."),
            web_common::database::Table::ProjectsUsersRoles => unimplemented!("Table `projects_users_roles` does not have a GIN similarity index."),
            web_common::database::Table::Roles => unimplemented!("Table `roles` does not have associated roles."),
            web_common::database::Table::SampleBioOttTaxonItems => unimplemented!("Table `sample_bio_ott_taxon_items` does not have a GIN similarity index."),
            web_common::database::Table::SampleStates => unimplemented!("Table `sample_states` does not have associated roles."),
            web_common::database::Table::SampledIndividualBioOttTaxonItems => unimplemented!("Table `sampled_individual_bio_ott_taxon_items` does not have a GIN similarity index."),
            web_common::database::Table::SampledIndividuals => NestedSampledIndividual::similarity_search_editables(user_id, query, limit, connection)?.iter().map(|row| bincode::serialize(row).map_err(web_common::api::ApiError::from)).collect(),
            web_common::database::Table::SampledIndividualsTeamsRoleInvitations => unimplemented!("Table `sampled_individuals_teams_role_invitations` does not have a GIN similarity index."),
            web_common::database::Table::SampledIndividualsTeamsRoleRequests => unimplemented!("Table `sampled_individuals_teams_role_requests` does not have a GIN similarity index."),
            web_common::database::Table::SampledIndividualsTeamsRoles => unimplemented!("Table `sampled_individuals_teams_roles` does not have a GIN similarity index."),
            web_common::database::Table::SampledIndividualsUsersRoleInvitations => unimplemented!("Table `sampled_individuals_users_role_invitations` does not have a GIN similarity index."),
            web_common::database::Table::SampledIndividualsUsersRoleRequests => unimplemented!("Table `sampled_individuals_users_role_requests` does not have a GIN similarity index."),
            web_common::database::Table::SampledIndividualsUsersRoles => unimplemented!("Table `sampled_individuals_users_roles` does not have a GIN similarity index."),
            web_common::database::Table::Samples => NestedSample::similarity_search_editables(user_id, query, limit, connection)?.iter().map(|row| bincode::serialize(row).map_err(web_common::api::ApiError::from)).collect(),
            web_common::database::Table::SamplesTeamsRoleInvitations => unimplemented!("Table `samples_teams_role_invitations` does not have a GIN similarity index."),
            web_common::database::Table::SamplesTeamsRoleRequests => unimplemented!("Table `samples_teams_role_requests` does not have a GIN similarity index."),
            web_common::database::Table::SamplesTeamsRoles => unimplemented!("Table `samples_teams_roles` does not have a GIN similarity index."),
            web_common::database::Table::SamplesUsersRoleInvitations => unimplemented!("Table `samples_users_role_invitations` does not have a GIN similarity index."),
            web_common::database::Table::SamplesUsersRoleRequests => unimplemented!("Table `samples_users_role_requests` does not have a GIN similarity index."),
            web_common::database::Table::SamplesUsersRoles => unimplemented!("Table `samples_users_roles` does not have a GIN similarity index."),
            web_common::database::Table::Spectra => unimplemented!("Table `spectra` does not have a GIN similarity index."),
            web_common::database::Table::SpectraCollections => unimplemented!("Table `spectra_collections` does not have a GIN similarity index."),
            web_common::database::Table::SpectraCollectionsTeamsRoleInvitations => unimplemented!("Table `spectra_collections_teams_role_invitations` does not have a GIN similarity index."),
            web_common::database::Table::SpectraCollectionsTeamsRoleRequests => unimplemented!("Table `spectra_collections_teams_role_requests` does not have a GIN similarity index."),
            web_common::database::Table::SpectraCollectionsTeamsRoles => unimplemented!("Table `spectra_collections_teams_roles` does not have a GIN similarity index."),
            web_common::database::Table::SpectraCollectionsUsersRoleInvitations => unimplemented!("Table `spectra_collections_users_role_invitations` does not have a GIN similarity index."),
            web_common::database::Table::SpectraCollectionsUsersRoleRequests => unimplemented!("Table `spectra_collections_users_role_requests` does not have a GIN similarity index."),
            web_common::database::Table::SpectraCollectionsUsersRoles => unimplemented!("Table `spectra_collections_users_roles` does not have a GIN similarity index."),
            web_common::database::Table::TeamStates => unimplemented!("Table `team_states` does not have associated roles."),
            web_common::database::Table::Teams => NestedTeam::similarity_search_editables(user_id, query, limit, connection)?.iter().map(|row| bincode::serialize(row).map_err(web_common::api::ApiError::from)).collect(),
            web_common::database::Table::TeamsTeamsRoleInvitations => unimplemented!("Table `teams_teams_role_invitations` does not have a GIN similarity index."),
            web_common::database::Table::TeamsUsersRoleInvitations => unimplemented!("Table `teams_users_role_invitations` does not have a GIN similarity index."),
            web_common::database::Table::TeamsUsersRoleRequests => unimplemented!("Table `teams_users_role_requests` does not have a GIN similarity index."),
            web_common::database::Table::TeamsUsersRoles => unimplemented!("Table `teams_users_roles` does not have a GIN similarity index."),
            web_common::database::Table::Units => unimplemented!("Table `units` does not have associated roles."),
            web_common::database::Table::UserEmails => unimplemented!("Table `user_emails` does not have a GIN similarity index."),
            web_common::database::Table::Users => unimplemented!("Table `users` does not have associated roles."),
            web_common::database::Table::UsersUsersRoleInvitations => unimplemented!("Table `users_users_role_invitations` does not have a GIN similarity index."),
            web_common::database::Table::UsersUsersRoleRequests => unimplemented!("Table `users_users_role_requests` does not have a GIN similarity index."),
            web_common::database::Table::UsersUsersRoles => unimplemented!("Table `users_users_roles` does not have a GIN similarity index."),
        }
    }
    fn word_similarity_search(&self, query: &str, limit: Option<i32>, connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>) -> Result<Vec<Vec<u8>>, web_common::api::ApiError> {
        match self {
            web_common::database::Table::BioOttRanks => NestedBioOttRank::word_similarity_search(query, limit, connection)?.iter().map(|row| bincode::serialize(row).map_err(web_common::api::ApiError::from)).collect(),
            web_common::database::Table::BioOttTaxonItems => NestedBioOttTaxonItem::word_similarity_search(query, limit, connection)?.iter().map(|row| bincode::serialize(row).map_err(web_common::api::ApiError::from)).collect(),
            web_common::database::Table::Colors => Color::word_similarity_search(query, limit, connection)?.iter().map(|row| bincode::serialize(row).map_err(web_common::api::ApiError::from)).collect(),
            web_common::database::Table::Countries => Country::word_similarity_search(query, limit, connection)?.iter().map(|row| bincode::serialize(row).map_err(web_common::api::ApiError::from)).collect(),
            web_common::database::Table::DerivedSamples => unimplemented!("Table `derived_samples` does not have a GIN similarity index."),
            web_common::database::Table::DocumentFormats => NestedDocumentFormat::word_similarity_search(query, limit, connection)?.iter().map(|row| bincode::serialize(row).map_err(web_common::api::ApiError::from)).collect(),
            web_common::database::Table::FontAwesomeIcons => FontAwesomeIcon::word_similarity_search(query, limit, connection)?.iter().map(|row| bincode::serialize(row).map_err(web_common::api::ApiError::from)).collect(),
            web_common::database::Table::LoginProviders => unimplemented!("Table `login_providers` does not have a GIN similarity index."),
            web_common::database::Table::Notifications => unimplemented!("Table `notifications` does not have a GIN similarity index."),
            web_common::database::Table::Organizations => NestedOrganization::word_similarity_search(query, limit, connection)?.iter().map(|row| bincode::serialize(row).map_err(web_common::api::ApiError::from)).collect(),
            web_common::database::Table::ProjectStates => NestedProjectState::word_similarity_search(query, limit, connection)?.iter().map(|row| bincode::serialize(row).map_err(web_common::api::ApiError::from)).collect(),
            web_common::database::Table::Projects => NestedProject::word_similarity_search(query, limit, connection)?.iter().map(|row| bincode::serialize(row).map_err(web_common::api::ApiError::from)).collect(),
            web_common::database::Table::ProjectsTeamsRoleInvitations => unimplemented!("Table `projects_teams_role_invitations` does not have a GIN similarity index."),
            web_common::database::Table::ProjectsTeamsRoleRequests => unimplemented!("Table `projects_teams_role_requests` does not have a GIN similarity index."),
            web_common::database::Table::ProjectsTeamsRoles => unimplemented!("Table `projects_teams_roles` does not have a GIN similarity index."),
            web_common::database::Table::ProjectsUsersRoleInvitations => unimplemented!("Table `projects_users_role_invitations` does not have a GIN similarity index."),
            web_common::database::Table::ProjectsUsersRoleRequests => unimplemented!("Table `projects_users_role_requests` does not have a GIN similarity index."),
            web_common::database::Table::ProjectsUsersRoles => unimplemented!("Table `projects_users_roles` does not have a GIN similarity index."),
            web_common::database::Table::Roles => NestedRole::word_similarity_search(query, limit, connection)?.iter().map(|row| bincode::serialize(row).map_err(web_common::api::ApiError::from)).collect(),
            web_common::database::Table::SampleBioOttTaxonItems => unimplemented!("Table `sample_bio_ott_taxon_items` does not have a GIN similarity index."),
            web_common::database::Table::SampleStates => NestedSampleState::word_similarity_search(query, limit, connection)?.iter().map(|row| bincode::serialize(row).map_err(web_common::api::ApiError::from)).collect(),
            web_common::database::Table::SampledIndividualBioOttTaxonItems => unimplemented!("Table `sampled_individual_bio_ott_taxon_items` does not have a GIN similarity index."),
            web_common::database::Table::SampledIndividuals => NestedSampledIndividual::word_similarity_search(query, limit, connection)?.iter().map(|row| bincode::serialize(row).map_err(web_common::api::ApiError::from)).collect(),
            web_common::database::Table::SampledIndividualsTeamsRoleInvitations => unimplemented!("Table `sampled_individuals_teams_role_invitations` does not have a GIN similarity index."),
            web_common::database::Table::SampledIndividualsTeamsRoleRequests => unimplemented!("Table `sampled_individuals_teams_role_requests` does not have a GIN similarity index."),
            web_common::database::Table::SampledIndividualsTeamsRoles => unimplemented!("Table `sampled_individuals_teams_roles` does not have a GIN similarity index."),
            web_common::database::Table::SampledIndividualsUsersRoleInvitations => unimplemented!("Table `sampled_individuals_users_role_invitations` does not have a GIN similarity index."),
            web_common::database::Table::SampledIndividualsUsersRoleRequests => unimplemented!("Table `sampled_individuals_users_role_requests` does not have a GIN similarity index."),
            web_common::database::Table::SampledIndividualsUsersRoles => unimplemented!("Table `sampled_individuals_users_roles` does not have a GIN similarity index."),
            web_common::database::Table::Samples => NestedSample::word_similarity_search(query, limit, connection)?.iter().map(|row| bincode::serialize(row).map_err(web_common::api::ApiError::from)).collect(),
            web_common::database::Table::SamplesTeamsRoleInvitations => unimplemented!("Table `samples_teams_role_invitations` does not have a GIN similarity index."),
            web_common::database::Table::SamplesTeamsRoleRequests => unimplemented!("Table `samples_teams_role_requests` does not have a GIN similarity index."),
            web_common::database::Table::SamplesTeamsRoles => unimplemented!("Table `samples_teams_roles` does not have a GIN similarity index."),
            web_common::database::Table::SamplesUsersRoleInvitations => unimplemented!("Table `samples_users_role_invitations` does not have a GIN similarity index."),
            web_common::database::Table::SamplesUsersRoleRequests => unimplemented!("Table `samples_users_role_requests` does not have a GIN similarity index."),
            web_common::database::Table::SamplesUsersRoles => unimplemented!("Table `samples_users_roles` does not have a GIN similarity index."),
            web_common::database::Table::Spectra => unimplemented!("Table `spectra` does not have a GIN similarity index."),
            web_common::database::Table::SpectraCollections => unimplemented!("Table `spectra_collections` does not have a GIN similarity index."),
            web_common::database::Table::SpectraCollectionsTeamsRoleInvitations => unimplemented!("Table `spectra_collections_teams_role_invitations` does not have a GIN similarity index."),
            web_common::database::Table::SpectraCollectionsTeamsRoleRequests => unimplemented!("Table `spectra_collections_teams_role_requests` does not have a GIN similarity index."),
            web_common::database::Table::SpectraCollectionsTeamsRoles => unimplemented!("Table `spectra_collections_teams_roles` does not have a GIN similarity index."),
            web_common::database::Table::SpectraCollectionsUsersRoleInvitations => unimplemented!("Table `spectra_collections_users_role_invitations` does not have a GIN similarity index."),
            web_common::database::Table::SpectraCollectionsUsersRoleRequests => unimplemented!("Table `spectra_collections_users_role_requests` does not have a GIN similarity index."),
            web_common::database::Table::SpectraCollectionsUsersRoles => unimplemented!("Table `spectra_collections_users_roles` does not have a GIN similarity index."),
            web_common::database::Table::TeamStates => NestedTeamState::word_similarity_search(query, limit, connection)?.iter().map(|row| bincode::serialize(row).map_err(web_common::api::ApiError::from)).collect(),
            web_common::database::Table::Teams => NestedTeam::word_similarity_search(query, limit, connection)?.iter().map(|row| bincode::serialize(row).map_err(web_common::api::ApiError::from)).collect(),
            web_common::database::Table::TeamsTeamsRoleInvitations => unimplemented!("Table `teams_teams_role_invitations` does not have a GIN similarity index."),
            web_common::database::Table::TeamsUsersRoleInvitations => unimplemented!("Table `teams_users_role_invitations` does not have a GIN similarity index."),
            web_common::database::Table::TeamsUsersRoleRequests => unimplemented!("Table `teams_users_role_requests` does not have a GIN similarity index."),
            web_common::database::Table::TeamsUsersRoles => unimplemented!("Table `teams_users_roles` does not have a GIN similarity index."),
            web_common::database::Table::Units => Unit::word_similarity_search(query, limit, connection)?.iter().map(|row| bincode::serialize(row).map_err(web_common::api::ApiError::from)).collect(),
            web_common::database::Table::UserEmails => unimplemented!("Table `user_emails` does not have a GIN similarity index."),
            web_common::database::Table::Users => User::word_similarity_search(query, limit, connection)?.iter().map(|row| bincode::serialize(row).map_err(web_common::api::ApiError::from)).collect(),
            web_common::database::Table::UsersUsersRoleInvitations => unimplemented!("Table `users_users_role_invitations` does not have a GIN similarity index."),
            web_common::database::Table::UsersUsersRoleRequests => unimplemented!("Table `users_users_role_requests` does not have a GIN similarity index."),
            web_common::database::Table::UsersUsersRoles => unimplemented!("Table `users_users_roles` does not have a GIN similarity index."),
        }
    }
    fn word_similarity_search_editables(&self, user_id: i32, query: &str, limit: Option<i32>, connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>) -> Result<Vec<Vec<u8>>, web_common::api::ApiError> {
        match self {
            web_common::database::Table::BioOttRanks => unimplemented!("Table `bio_ott_ranks` does not have associated roles."),
            web_common::database::Table::BioOttTaxonItems => unimplemented!("Table `bio_ott_taxon_items` does not have associated roles."),
            web_common::database::Table::Colors => unimplemented!("Table `colors` does not have associated roles."),
            web_common::database::Table::Countries => unimplemented!("Table `countries` does not have associated roles."),
            web_common::database::Table::DerivedSamples => unimplemented!("Table `derived_samples` does not have a GIN similarity index."),
            web_common::database::Table::DocumentFormats => unimplemented!("Table `document_formats` does not have associated roles."),
            web_common::database::Table::FontAwesomeIcons => unimplemented!("Table `font_awesome_icons` does not have associated roles."),
            web_common::database::Table::LoginProviders => unimplemented!("Table `login_providers` does not have a GIN similarity index."),
            web_common::database::Table::Notifications => unimplemented!("Table `notifications` does not have a GIN similarity index."),
            web_common::database::Table::Organizations => unimplemented!("Table `organizations` does not have associated roles."),
            web_common::database::Table::ProjectStates => unimplemented!("Table `project_states` does not have associated roles."),
            web_common::database::Table::Projects => NestedProject::word_similarity_search_editables(user_id, query, limit, connection)?.iter().map(|row| bincode::serialize(row).map_err(web_common::api::ApiError::from)).collect(),
            web_common::database::Table::ProjectsTeamsRoleInvitations => unimplemented!("Table `projects_teams_role_invitations` does not have a GIN similarity index."),
            web_common::database::Table::ProjectsTeamsRoleRequests => unimplemented!("Table `projects_teams_role_requests` does not have a GIN similarity index."),
            web_common::database::Table::ProjectsTeamsRoles => unimplemented!("Table `projects_teams_roles` does not have a GIN similarity index."),
            web_common::database::Table::ProjectsUsersRoleInvitations => unimplemented!("Table `projects_users_role_invitations` does not have a GIN similarity index."),
            web_common::database::Table::ProjectsUsersRoleRequests => unimplemented!("Table `projects_users_role_requests` does not have a GIN similarity index."),
            web_common::database::Table::ProjectsUsersRoles => unimplemented!("Table `projects_users_roles` does not have a GIN similarity index."),
            web_common::database::Table::Roles => unimplemented!("Table `roles` does not have associated roles."),
            web_common::database::Table::SampleBioOttTaxonItems => unimplemented!("Table `sample_bio_ott_taxon_items` does not have a GIN similarity index."),
            web_common::database::Table::SampleStates => unimplemented!("Table `sample_states` does not have associated roles."),
            web_common::database::Table::SampledIndividualBioOttTaxonItems => unimplemented!("Table `sampled_individual_bio_ott_taxon_items` does not have a GIN similarity index."),
            web_common::database::Table::SampledIndividuals => NestedSampledIndividual::word_similarity_search_editables(user_id, query, limit, connection)?.iter().map(|row| bincode::serialize(row).map_err(web_common::api::ApiError::from)).collect(),
            web_common::database::Table::SampledIndividualsTeamsRoleInvitations => unimplemented!("Table `sampled_individuals_teams_role_invitations` does not have a GIN similarity index."),
            web_common::database::Table::SampledIndividualsTeamsRoleRequests => unimplemented!("Table `sampled_individuals_teams_role_requests` does not have a GIN similarity index."),
            web_common::database::Table::SampledIndividualsTeamsRoles => unimplemented!("Table `sampled_individuals_teams_roles` does not have a GIN similarity index."),
            web_common::database::Table::SampledIndividualsUsersRoleInvitations => unimplemented!("Table `sampled_individuals_users_role_invitations` does not have a GIN similarity index."),
            web_common::database::Table::SampledIndividualsUsersRoleRequests => unimplemented!("Table `sampled_individuals_users_role_requests` does not have a GIN similarity index."),
            web_common::database::Table::SampledIndividualsUsersRoles => unimplemented!("Table `sampled_individuals_users_roles` does not have a GIN similarity index."),
            web_common::database::Table::Samples => NestedSample::word_similarity_search_editables(user_id, query, limit, connection)?.iter().map(|row| bincode::serialize(row).map_err(web_common::api::ApiError::from)).collect(),
            web_common::database::Table::SamplesTeamsRoleInvitations => unimplemented!("Table `samples_teams_role_invitations` does not have a GIN similarity index."),
            web_common::database::Table::SamplesTeamsRoleRequests => unimplemented!("Table `samples_teams_role_requests` does not have a GIN similarity index."),
            web_common::database::Table::SamplesTeamsRoles => unimplemented!("Table `samples_teams_roles` does not have a GIN similarity index."),
            web_common::database::Table::SamplesUsersRoleInvitations => unimplemented!("Table `samples_users_role_invitations` does not have a GIN similarity index."),
            web_common::database::Table::SamplesUsersRoleRequests => unimplemented!("Table `samples_users_role_requests` does not have a GIN similarity index."),
            web_common::database::Table::SamplesUsersRoles => unimplemented!("Table `samples_users_roles` does not have a GIN similarity index."),
            web_common::database::Table::Spectra => unimplemented!("Table `spectra` does not have a GIN similarity index."),
            web_common::database::Table::SpectraCollections => unimplemented!("Table `spectra_collections` does not have a GIN similarity index."),
            web_common::database::Table::SpectraCollectionsTeamsRoleInvitations => unimplemented!("Table `spectra_collections_teams_role_invitations` does not have a GIN similarity index."),
            web_common::database::Table::SpectraCollectionsTeamsRoleRequests => unimplemented!("Table `spectra_collections_teams_role_requests` does not have a GIN similarity index."),
            web_common::database::Table::SpectraCollectionsTeamsRoles => unimplemented!("Table `spectra_collections_teams_roles` does not have a GIN similarity index."),
            web_common::database::Table::SpectraCollectionsUsersRoleInvitations => unimplemented!("Table `spectra_collections_users_role_invitations` does not have a GIN similarity index."),
            web_common::database::Table::SpectraCollectionsUsersRoleRequests => unimplemented!("Table `spectra_collections_users_role_requests` does not have a GIN similarity index."),
            web_common::database::Table::SpectraCollectionsUsersRoles => unimplemented!("Table `spectra_collections_users_roles` does not have a GIN similarity index."),
            web_common::database::Table::TeamStates => unimplemented!("Table `team_states` does not have associated roles."),
            web_common::database::Table::Teams => NestedTeam::word_similarity_search_editables(user_id, query, limit, connection)?.iter().map(|row| bincode::serialize(row).map_err(web_common::api::ApiError::from)).collect(),
            web_common::database::Table::TeamsTeamsRoleInvitations => unimplemented!("Table `teams_teams_role_invitations` does not have a GIN similarity index."),
            web_common::database::Table::TeamsUsersRoleInvitations => unimplemented!("Table `teams_users_role_invitations` does not have a GIN similarity index."),
            web_common::database::Table::TeamsUsersRoleRequests => unimplemented!("Table `teams_users_role_requests` does not have a GIN similarity index."),
            web_common::database::Table::TeamsUsersRoles => unimplemented!("Table `teams_users_roles` does not have a GIN similarity index."),
            web_common::database::Table::Units => unimplemented!("Table `units` does not have associated roles."),
            web_common::database::Table::UserEmails => unimplemented!("Table `user_emails` does not have a GIN similarity index."),
            web_common::database::Table::Users => unimplemented!("Table `users` does not have associated roles."),
            web_common::database::Table::UsersUsersRoleInvitations => unimplemented!("Table `users_users_role_invitations` does not have a GIN similarity index."),
            web_common::database::Table::UsersUsersRoleRequests => unimplemented!("Table `users_users_role_requests` does not have a GIN similarity index."),
            web_common::database::Table::UsersUsersRoles => unimplemented!("Table `users_users_roles` does not have a GIN similarity index."),
        }
    }
    fn strict_word_similarity_search(&self, query: &str, limit: Option<i32>, connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>) -> Result<Vec<Vec<u8>>, web_common::api::ApiError> {
        match self {
            web_common::database::Table::BioOttRanks => NestedBioOttRank::strict_word_similarity_search(query, limit, connection)?.iter().map(|row| bincode::serialize(row).map_err(web_common::api::ApiError::from)).collect(),
            web_common::database::Table::BioOttTaxonItems => NestedBioOttTaxonItem::strict_word_similarity_search(query, limit, connection)?.iter().map(|row| bincode::serialize(row).map_err(web_common::api::ApiError::from)).collect(),
            web_common::database::Table::Colors => Color::strict_word_similarity_search(query, limit, connection)?.iter().map(|row| bincode::serialize(row).map_err(web_common::api::ApiError::from)).collect(),
            web_common::database::Table::Countries => Country::strict_word_similarity_search(query, limit, connection)?.iter().map(|row| bincode::serialize(row).map_err(web_common::api::ApiError::from)).collect(),
            web_common::database::Table::DerivedSamples => unimplemented!("Table `derived_samples` does not have a GIN similarity index."),
            web_common::database::Table::DocumentFormats => NestedDocumentFormat::strict_word_similarity_search(query, limit, connection)?.iter().map(|row| bincode::serialize(row).map_err(web_common::api::ApiError::from)).collect(),
            web_common::database::Table::FontAwesomeIcons => FontAwesomeIcon::strict_word_similarity_search(query, limit, connection)?.iter().map(|row| bincode::serialize(row).map_err(web_common::api::ApiError::from)).collect(),
            web_common::database::Table::LoginProviders => unimplemented!("Table `login_providers` does not have a GIN similarity index."),
            web_common::database::Table::Notifications => unimplemented!("Table `notifications` does not have a GIN similarity index."),
            web_common::database::Table::Organizations => NestedOrganization::strict_word_similarity_search(query, limit, connection)?.iter().map(|row| bincode::serialize(row).map_err(web_common::api::ApiError::from)).collect(),
            web_common::database::Table::ProjectStates => NestedProjectState::strict_word_similarity_search(query, limit, connection)?.iter().map(|row| bincode::serialize(row).map_err(web_common::api::ApiError::from)).collect(),
            web_common::database::Table::Projects => NestedProject::strict_word_similarity_search(query, limit, connection)?.iter().map(|row| bincode::serialize(row).map_err(web_common::api::ApiError::from)).collect(),
            web_common::database::Table::ProjectsTeamsRoleInvitations => unimplemented!("Table `projects_teams_role_invitations` does not have a GIN similarity index."),
            web_common::database::Table::ProjectsTeamsRoleRequests => unimplemented!("Table `projects_teams_role_requests` does not have a GIN similarity index."),
            web_common::database::Table::ProjectsTeamsRoles => unimplemented!("Table `projects_teams_roles` does not have a GIN similarity index."),
            web_common::database::Table::ProjectsUsersRoleInvitations => unimplemented!("Table `projects_users_role_invitations` does not have a GIN similarity index."),
            web_common::database::Table::ProjectsUsersRoleRequests => unimplemented!("Table `projects_users_role_requests` does not have a GIN similarity index."),
            web_common::database::Table::ProjectsUsersRoles => unimplemented!("Table `projects_users_roles` does not have a GIN similarity index."),
            web_common::database::Table::Roles => NestedRole::strict_word_similarity_search(query, limit, connection)?.iter().map(|row| bincode::serialize(row).map_err(web_common::api::ApiError::from)).collect(),
            web_common::database::Table::SampleBioOttTaxonItems => unimplemented!("Table `sample_bio_ott_taxon_items` does not have a GIN similarity index."),
            web_common::database::Table::SampleStates => NestedSampleState::strict_word_similarity_search(query, limit, connection)?.iter().map(|row| bincode::serialize(row).map_err(web_common::api::ApiError::from)).collect(),
            web_common::database::Table::SampledIndividualBioOttTaxonItems => unimplemented!("Table `sampled_individual_bio_ott_taxon_items` does not have a GIN similarity index."),
            web_common::database::Table::SampledIndividuals => NestedSampledIndividual::strict_word_similarity_search(query, limit, connection)?.iter().map(|row| bincode::serialize(row).map_err(web_common::api::ApiError::from)).collect(),
            web_common::database::Table::SampledIndividualsTeamsRoleInvitations => unimplemented!("Table `sampled_individuals_teams_role_invitations` does not have a GIN similarity index."),
            web_common::database::Table::SampledIndividualsTeamsRoleRequests => unimplemented!("Table `sampled_individuals_teams_role_requests` does not have a GIN similarity index."),
            web_common::database::Table::SampledIndividualsTeamsRoles => unimplemented!("Table `sampled_individuals_teams_roles` does not have a GIN similarity index."),
            web_common::database::Table::SampledIndividualsUsersRoleInvitations => unimplemented!("Table `sampled_individuals_users_role_invitations` does not have a GIN similarity index."),
            web_common::database::Table::SampledIndividualsUsersRoleRequests => unimplemented!("Table `sampled_individuals_users_role_requests` does not have a GIN similarity index."),
            web_common::database::Table::SampledIndividualsUsersRoles => unimplemented!("Table `sampled_individuals_users_roles` does not have a GIN similarity index."),
            web_common::database::Table::Samples => NestedSample::strict_word_similarity_search(query, limit, connection)?.iter().map(|row| bincode::serialize(row).map_err(web_common::api::ApiError::from)).collect(),
            web_common::database::Table::SamplesTeamsRoleInvitations => unimplemented!("Table `samples_teams_role_invitations` does not have a GIN similarity index."),
            web_common::database::Table::SamplesTeamsRoleRequests => unimplemented!("Table `samples_teams_role_requests` does not have a GIN similarity index."),
            web_common::database::Table::SamplesTeamsRoles => unimplemented!("Table `samples_teams_roles` does not have a GIN similarity index."),
            web_common::database::Table::SamplesUsersRoleInvitations => unimplemented!("Table `samples_users_role_invitations` does not have a GIN similarity index."),
            web_common::database::Table::SamplesUsersRoleRequests => unimplemented!("Table `samples_users_role_requests` does not have a GIN similarity index."),
            web_common::database::Table::SamplesUsersRoles => unimplemented!("Table `samples_users_roles` does not have a GIN similarity index."),
            web_common::database::Table::Spectra => unimplemented!("Table `spectra` does not have a GIN similarity index."),
            web_common::database::Table::SpectraCollections => unimplemented!("Table `spectra_collections` does not have a GIN similarity index."),
            web_common::database::Table::SpectraCollectionsTeamsRoleInvitations => unimplemented!("Table `spectra_collections_teams_role_invitations` does not have a GIN similarity index."),
            web_common::database::Table::SpectraCollectionsTeamsRoleRequests => unimplemented!("Table `spectra_collections_teams_role_requests` does not have a GIN similarity index."),
            web_common::database::Table::SpectraCollectionsTeamsRoles => unimplemented!("Table `spectra_collections_teams_roles` does not have a GIN similarity index."),
            web_common::database::Table::SpectraCollectionsUsersRoleInvitations => unimplemented!("Table `spectra_collections_users_role_invitations` does not have a GIN similarity index."),
            web_common::database::Table::SpectraCollectionsUsersRoleRequests => unimplemented!("Table `spectra_collections_users_role_requests` does not have a GIN similarity index."),
            web_common::database::Table::SpectraCollectionsUsersRoles => unimplemented!("Table `spectra_collections_users_roles` does not have a GIN similarity index."),
            web_common::database::Table::TeamStates => NestedTeamState::strict_word_similarity_search(query, limit, connection)?.iter().map(|row| bincode::serialize(row).map_err(web_common::api::ApiError::from)).collect(),
            web_common::database::Table::Teams => NestedTeam::strict_word_similarity_search(query, limit, connection)?.iter().map(|row| bincode::serialize(row).map_err(web_common::api::ApiError::from)).collect(),
            web_common::database::Table::TeamsTeamsRoleInvitations => unimplemented!("Table `teams_teams_role_invitations` does not have a GIN similarity index."),
            web_common::database::Table::TeamsUsersRoleInvitations => unimplemented!("Table `teams_users_role_invitations` does not have a GIN similarity index."),
            web_common::database::Table::TeamsUsersRoleRequests => unimplemented!("Table `teams_users_role_requests` does not have a GIN similarity index."),
            web_common::database::Table::TeamsUsersRoles => unimplemented!("Table `teams_users_roles` does not have a GIN similarity index."),
            web_common::database::Table::Units => Unit::strict_word_similarity_search(query, limit, connection)?.iter().map(|row| bincode::serialize(row).map_err(web_common::api::ApiError::from)).collect(),
            web_common::database::Table::UserEmails => unimplemented!("Table `user_emails` does not have a GIN similarity index."),
            web_common::database::Table::Users => User::strict_word_similarity_search(query, limit, connection)?.iter().map(|row| bincode::serialize(row).map_err(web_common::api::ApiError::from)).collect(),
            web_common::database::Table::UsersUsersRoleInvitations => unimplemented!("Table `users_users_role_invitations` does not have a GIN similarity index."),
            web_common::database::Table::UsersUsersRoleRequests => unimplemented!("Table `users_users_role_requests` does not have a GIN similarity index."),
            web_common::database::Table::UsersUsersRoles => unimplemented!("Table `users_users_roles` does not have a GIN similarity index."),
        }
    }
    fn strict_word_similarity_search_editables(&self, user_id: i32, query: &str, limit: Option<i32>, connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>) -> Result<Vec<Vec<u8>>, web_common::api::ApiError> {
        match self {
            web_common::database::Table::BioOttRanks => unimplemented!("Table `bio_ott_ranks` does not have associated roles."),
            web_common::database::Table::BioOttTaxonItems => unimplemented!("Table `bio_ott_taxon_items` does not have associated roles."),
            web_common::database::Table::Colors => unimplemented!("Table `colors` does not have associated roles."),
            web_common::database::Table::Countries => unimplemented!("Table `countries` does not have associated roles."),
            web_common::database::Table::DerivedSamples => unimplemented!("Table `derived_samples` does not have a GIN similarity index."),
            web_common::database::Table::DocumentFormats => unimplemented!("Table `document_formats` does not have associated roles."),
            web_common::database::Table::FontAwesomeIcons => unimplemented!("Table `font_awesome_icons` does not have associated roles."),
            web_common::database::Table::LoginProviders => unimplemented!("Table `login_providers` does not have a GIN similarity index."),
            web_common::database::Table::Notifications => unimplemented!("Table `notifications` does not have a GIN similarity index."),
            web_common::database::Table::Organizations => unimplemented!("Table `organizations` does not have associated roles."),
            web_common::database::Table::ProjectStates => unimplemented!("Table `project_states` does not have associated roles."),
            web_common::database::Table::Projects => NestedProject::strict_word_similarity_search_editables(user_id, query, limit, connection)?.iter().map(|row| bincode::serialize(row).map_err(web_common::api::ApiError::from)).collect(),
            web_common::database::Table::ProjectsTeamsRoleInvitations => unimplemented!("Table `projects_teams_role_invitations` does not have a GIN similarity index."),
            web_common::database::Table::ProjectsTeamsRoleRequests => unimplemented!("Table `projects_teams_role_requests` does not have a GIN similarity index."),
            web_common::database::Table::ProjectsTeamsRoles => unimplemented!("Table `projects_teams_roles` does not have a GIN similarity index."),
            web_common::database::Table::ProjectsUsersRoleInvitations => unimplemented!("Table `projects_users_role_invitations` does not have a GIN similarity index."),
            web_common::database::Table::ProjectsUsersRoleRequests => unimplemented!("Table `projects_users_role_requests` does not have a GIN similarity index."),
            web_common::database::Table::ProjectsUsersRoles => unimplemented!("Table `projects_users_roles` does not have a GIN similarity index."),
            web_common::database::Table::Roles => unimplemented!("Table `roles` does not have associated roles."),
            web_common::database::Table::SampleBioOttTaxonItems => unimplemented!("Table `sample_bio_ott_taxon_items` does not have a GIN similarity index."),
            web_common::database::Table::SampleStates => unimplemented!("Table `sample_states` does not have associated roles."),
            web_common::database::Table::SampledIndividualBioOttTaxonItems => unimplemented!("Table `sampled_individual_bio_ott_taxon_items` does not have a GIN similarity index."),
            web_common::database::Table::SampledIndividuals => NestedSampledIndividual::strict_word_similarity_search_editables(user_id, query, limit, connection)?.iter().map(|row| bincode::serialize(row).map_err(web_common::api::ApiError::from)).collect(),
            web_common::database::Table::SampledIndividualsTeamsRoleInvitations => unimplemented!("Table `sampled_individuals_teams_role_invitations` does not have a GIN similarity index."),
            web_common::database::Table::SampledIndividualsTeamsRoleRequests => unimplemented!("Table `sampled_individuals_teams_role_requests` does not have a GIN similarity index."),
            web_common::database::Table::SampledIndividualsTeamsRoles => unimplemented!("Table `sampled_individuals_teams_roles` does not have a GIN similarity index."),
            web_common::database::Table::SampledIndividualsUsersRoleInvitations => unimplemented!("Table `sampled_individuals_users_role_invitations` does not have a GIN similarity index."),
            web_common::database::Table::SampledIndividualsUsersRoleRequests => unimplemented!("Table `sampled_individuals_users_role_requests` does not have a GIN similarity index."),
            web_common::database::Table::SampledIndividualsUsersRoles => unimplemented!("Table `sampled_individuals_users_roles` does not have a GIN similarity index."),
            web_common::database::Table::Samples => NestedSample::strict_word_similarity_search_editables(user_id, query, limit, connection)?.iter().map(|row| bincode::serialize(row).map_err(web_common::api::ApiError::from)).collect(),
            web_common::database::Table::SamplesTeamsRoleInvitations => unimplemented!("Table `samples_teams_role_invitations` does not have a GIN similarity index."),
            web_common::database::Table::SamplesTeamsRoleRequests => unimplemented!("Table `samples_teams_role_requests` does not have a GIN similarity index."),
            web_common::database::Table::SamplesTeamsRoles => unimplemented!("Table `samples_teams_roles` does not have a GIN similarity index."),
            web_common::database::Table::SamplesUsersRoleInvitations => unimplemented!("Table `samples_users_role_invitations` does not have a GIN similarity index."),
            web_common::database::Table::SamplesUsersRoleRequests => unimplemented!("Table `samples_users_role_requests` does not have a GIN similarity index."),
            web_common::database::Table::SamplesUsersRoles => unimplemented!("Table `samples_users_roles` does not have a GIN similarity index."),
            web_common::database::Table::Spectra => unimplemented!("Table `spectra` does not have a GIN similarity index."),
            web_common::database::Table::SpectraCollections => unimplemented!("Table `spectra_collections` does not have a GIN similarity index."),
            web_common::database::Table::SpectraCollectionsTeamsRoleInvitations => unimplemented!("Table `spectra_collections_teams_role_invitations` does not have a GIN similarity index."),
            web_common::database::Table::SpectraCollectionsTeamsRoleRequests => unimplemented!("Table `spectra_collections_teams_role_requests` does not have a GIN similarity index."),
            web_common::database::Table::SpectraCollectionsTeamsRoles => unimplemented!("Table `spectra_collections_teams_roles` does not have a GIN similarity index."),
            web_common::database::Table::SpectraCollectionsUsersRoleInvitations => unimplemented!("Table `spectra_collections_users_role_invitations` does not have a GIN similarity index."),
            web_common::database::Table::SpectraCollectionsUsersRoleRequests => unimplemented!("Table `spectra_collections_users_role_requests` does not have a GIN similarity index."),
            web_common::database::Table::SpectraCollectionsUsersRoles => unimplemented!("Table `spectra_collections_users_roles` does not have a GIN similarity index."),
            web_common::database::Table::TeamStates => unimplemented!("Table `team_states` does not have associated roles."),
            web_common::database::Table::Teams => NestedTeam::strict_word_similarity_search_editables(user_id, query, limit, connection)?.iter().map(|row| bincode::serialize(row).map_err(web_common::api::ApiError::from)).collect(),
            web_common::database::Table::TeamsTeamsRoleInvitations => unimplemented!("Table `teams_teams_role_invitations` does not have a GIN similarity index."),
            web_common::database::Table::TeamsUsersRoleInvitations => unimplemented!("Table `teams_users_role_invitations` does not have a GIN similarity index."),
            web_common::database::Table::TeamsUsersRoleRequests => unimplemented!("Table `teams_users_role_requests` does not have a GIN similarity index."),
            web_common::database::Table::TeamsUsersRoles => unimplemented!("Table `teams_users_roles` does not have a GIN similarity index."),
            web_common::database::Table::Units => unimplemented!("Table `units` does not have associated roles."),
            web_common::database::Table::UserEmails => unimplemented!("Table `user_emails` does not have a GIN similarity index."),
            web_common::database::Table::Users => unimplemented!("Table `users` does not have associated roles."),
            web_common::database::Table::UsersUsersRoleInvitations => unimplemented!("Table `users_users_role_invitations` does not have a GIN similarity index."),
            web_common::database::Table::UsersUsersRoleRequests => unimplemented!("Table `users_users_role_requests` does not have a GIN similarity index."),
            web_common::database::Table::UsersUsersRoles => unimplemented!("Table `users_users_roles` does not have a GIN similarity index."),
        }
    }
}
/// Trait providing the get method for the Table enum.
pub trait IdentifiableTable {
    /// Get the row from the table by the primary key.
    ///
    /// # Arguments
    /// * `primary_key` - The primary key of the row.
    /// * `connection` - The database connection.
    ///
    /// # Returns
    /// A serialized version of the row of the table, using bincode.
    fn get(
         &self,
         primary_key: web_common::database::operations::PrimaryKey,
         connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>
) -> Result<Vec<u8>, web_common::api::ApiError>;
}

impl IdentifiableTable for web_common::database::Table {

    fn get(
        &self,
        primary_key: web_common::database::operations::PrimaryKey,
        connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>
    ) -> Result<Vec<u8>, web_common::api::ApiError> {
        Ok(match self {
            web_common::database::Table::BioOttRanks => bincode::serialize(&NestedBioOttRank::get(primary_key.into(), connection)?)?,
            web_common::database::Table::BioOttTaxonItems => bincode::serialize(&NestedBioOttTaxonItem::get(primary_key.into(), connection)?)?,
            web_common::database::Table::Colors => bincode::serialize(&Color::get(primary_key.into(), connection)?)?,
            web_common::database::Table::Countries => bincode::serialize(&Country::get(primary_key.into(), connection)?)?,
            web_common::database::Table::DerivedSamples => bincode::serialize(&NestedDerivedSample::get(primary_key.into(), connection)?)?,
            web_common::database::Table::DocumentFormats => bincode::serialize(&NestedDocumentFormat::get(primary_key.into(), connection)?)?,
            web_common::database::Table::FontAwesomeIcons => bincode::serialize(&FontAwesomeIcon::get(primary_key.into(), connection)?)?,
            web_common::database::Table::LoginProviders => bincode::serialize(&NestedLoginProvider::get(primary_key.into(), connection)?)?,
            web_common::database::Table::Notifications => bincode::serialize(&NestedNotification::get(primary_key.into(), connection)?)?,
            web_common::database::Table::Organizations => bincode::serialize(&NestedOrganization::get(primary_key.into(), connection)?)?,
            web_common::database::Table::ProjectStates => bincode::serialize(&NestedProjectState::get(primary_key.into(), connection)?)?,
            web_common::database::Table::Projects => bincode::serialize(&NestedProject::get(primary_key.into(), connection)?)?,
            web_common::database::Table::ProjectsTeamsRoleInvitations => bincode::serialize(&NestedProjectsTeamsRoleInvitation::get(primary_key.into(), connection)?)?,
            web_common::database::Table::ProjectsTeamsRoleRequests => bincode::serialize(&NestedProjectsTeamsRoleRequest::get(primary_key.into(), connection)?)?,
            web_common::database::Table::ProjectsTeamsRoles => bincode::serialize(&NestedProjectsTeamsRole::get(primary_key.into(), connection)?)?,
            web_common::database::Table::ProjectsUsersRoleInvitations => bincode::serialize(&NestedProjectsUsersRoleInvitation::get(primary_key.into(), connection)?)?,
            web_common::database::Table::ProjectsUsersRoleRequests => bincode::serialize(&NestedProjectsUsersRoleRequest::get(primary_key.into(), connection)?)?,
            web_common::database::Table::ProjectsUsersRoles => bincode::serialize(&NestedProjectsUsersRole::get(primary_key.into(), connection)?)?,
            web_common::database::Table::Roles => bincode::serialize(&NestedRole::get(primary_key.into(), connection)?)?,
            web_common::database::Table::SampleBioOttTaxonItems => bincode::serialize(&NestedSampleBioOttTaxonItem::get(primary_key.into(), connection)?)?,
            web_common::database::Table::SampleStates => bincode::serialize(&NestedSampleState::get(primary_key.into(), connection)?)?,
            web_common::database::Table::SampledIndividualBioOttTaxonItems => bincode::serialize(&NestedSampledIndividualBioOttTaxonItem::get(primary_key.into(), connection)?)?,
            web_common::database::Table::SampledIndividuals => bincode::serialize(&NestedSampledIndividual::get(primary_key.into(), connection)?)?,
            web_common::database::Table::SampledIndividualsTeamsRoleInvitations => bincode::serialize(&NestedSampledIndividualsTeamsRoleInvitation::get(primary_key.into(), connection)?)?,
            web_common::database::Table::SampledIndividualsTeamsRoleRequests => bincode::serialize(&NestedSampledIndividualsTeamsRoleRequest::get(primary_key.into(), connection)?)?,
            web_common::database::Table::SampledIndividualsTeamsRoles => bincode::serialize(&NestedSampledIndividualsTeamsRole::get(primary_key.into(), connection)?)?,
            web_common::database::Table::SampledIndividualsUsersRoleInvitations => bincode::serialize(&NestedSampledIndividualsUsersRoleInvitation::get(primary_key.into(), connection)?)?,
            web_common::database::Table::SampledIndividualsUsersRoleRequests => bincode::serialize(&NestedSampledIndividualsUsersRoleRequest::get(primary_key.into(), connection)?)?,
            web_common::database::Table::SampledIndividualsUsersRoles => bincode::serialize(&NestedSampledIndividualsUsersRole::get(primary_key.into(), connection)?)?,
            web_common::database::Table::Samples => bincode::serialize(&NestedSample::get(primary_key.into(), connection)?)?,
            web_common::database::Table::SamplesTeamsRoleInvitations => bincode::serialize(&NestedSamplesTeamsRoleInvitation::get(primary_key.into(), connection)?)?,
            web_common::database::Table::SamplesTeamsRoleRequests => bincode::serialize(&NestedSamplesTeamsRoleRequest::get(primary_key.into(), connection)?)?,
            web_common::database::Table::SamplesTeamsRoles => bincode::serialize(&NestedSamplesTeamsRole::get(primary_key.into(), connection)?)?,
            web_common::database::Table::SamplesUsersRoleInvitations => bincode::serialize(&NestedSamplesUsersRoleInvitation::get(primary_key.into(), connection)?)?,
            web_common::database::Table::SamplesUsersRoleRequests => bincode::serialize(&NestedSamplesUsersRoleRequest::get(primary_key.into(), connection)?)?,
            web_common::database::Table::SamplesUsersRoles => bincode::serialize(&NestedSamplesUsersRole::get(primary_key.into(), connection)?)?,
            web_common::database::Table::Spectra => bincode::serialize(&NestedSpectra::get(primary_key.into(), connection)?)?,
            web_common::database::Table::SpectraCollections => bincode::serialize(&NestedSpectraCollection::get(primary_key.into(), connection)?)?,
            web_common::database::Table::SpectraCollectionsTeamsRoleInvitations => bincode::serialize(&NestedSpectraCollectionsTeamsRoleInvitation::get(primary_key.into(), connection)?)?,
            web_common::database::Table::SpectraCollectionsTeamsRoleRequests => bincode::serialize(&NestedSpectraCollectionsTeamsRoleRequest::get(primary_key.into(), connection)?)?,
            web_common::database::Table::SpectraCollectionsTeamsRoles => bincode::serialize(&NestedSpectraCollectionsTeamsRole::get(primary_key.into(), connection)?)?,
            web_common::database::Table::SpectraCollectionsUsersRoleInvitations => bincode::serialize(&NestedSpectraCollectionsUsersRoleInvitation::get(primary_key.into(), connection)?)?,
            web_common::database::Table::SpectraCollectionsUsersRoleRequests => bincode::serialize(&NestedSpectraCollectionsUsersRoleRequest::get(primary_key.into(), connection)?)?,
            web_common::database::Table::SpectraCollectionsUsersRoles => bincode::serialize(&NestedSpectraCollectionsUsersRole::get(primary_key.into(), connection)?)?,
            web_common::database::Table::TeamStates => bincode::serialize(&NestedTeamState::get(primary_key.into(), connection)?)?,
            web_common::database::Table::Teams => bincode::serialize(&NestedTeam::get(primary_key.into(), connection)?)?,
            web_common::database::Table::TeamsTeamsRoleInvitations => bincode::serialize(&NestedTeamsTeamsRoleInvitation::get(primary_key.into(), connection)?)?,
            web_common::database::Table::TeamsUsersRoleInvitations => bincode::serialize(&NestedTeamsUsersRoleInvitation::get(primary_key.into(), connection)?)?,
            web_common::database::Table::TeamsUsersRoleRequests => bincode::serialize(&NestedTeamsUsersRoleRequest::get(primary_key.into(), connection)?)?,
            web_common::database::Table::TeamsUsersRoles => bincode::serialize(&NestedTeamsUsersRole::get(primary_key.into(), connection)?)?,
            web_common::database::Table::Units => bincode::serialize(&Unit::get(primary_key.into(), connection)?)?,
            web_common::database::Table::UserEmails => bincode::serialize(&NestedUserEmail::get(primary_key.into(), connection)?)?,
            web_common::database::Table::Users => bincode::serialize(&User::get(primary_key.into(), connection)?)?,
            web_common::database::Table::UsersUsersRoleInvitations => bincode::serialize(&NestedUsersUsersRoleInvitation::get(primary_key.into(), connection)?)?,
            web_common::database::Table::UsersUsersRoleRequests => bincode::serialize(&NestedUsersUsersRoleRequest::get(primary_key.into(), connection)?)?,
            web_common::database::Table::UsersUsersRoles => bincode::serialize(&NestedUsersUsersRole::get(primary_key.into(), connection)?)?,
        })
    }
}
/// Trait providing the can_view method for the Table enum.
pub trait ViewableTable {
    /// Check whether the user can view the row.
    ///
    /// # Arguments
    /// * `primary_key` - The primary key of the row.
    /// * `user_id` - The user ID of the user performing the operation.
    /// * `connection` - The database connection.
    ///
    /// # Returns
    /// A boolean indicating whether the user can view the row.
    fn can_view(
         &self,
         primary_key: web_common::database::operations::PrimaryKey,
         user_id: Option<i32>,
         connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>
) -> Result<bool, web_common::api::ApiError>;
}

impl ViewableTable for web_common::database::Table {

    fn can_view(
        &self,
        primary_key: web_common::database::operations::PrimaryKey,
        user_id: Option<i32>,
        connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>
    ) -> Result<bool, web_common::api::ApiError> {
        Ok(match self {
            web_common::database::Table::BioOttRanks => true,
            web_common::database::Table::BioOttTaxonItems => true,
            web_common::database::Table::Colors => true,
            web_common::database::Table::Countries => true,
            web_common::database::Table::DerivedSamples => true,
            web_common::database::Table::DocumentFormats => true,
            web_common::database::Table::FontAwesomeIcons => true,
            web_common::database::Table::LoginProviders => true,
            web_common::database::Table::Notifications => true,
            web_common::database::Table::Organizations => true,
            web_common::database::Table::ProjectStates => true,
            web_common::database::Table::Projects => {
                Project::get(primary_key.into(), connection)?.public ||
                user_id.map_or(Ok(false), |user_id| Project::is_viewer_by_id(primary_key.into(), user_id, connection))?
            },
            web_common::database::Table::ProjectsTeamsRoleInvitations => true,
            web_common::database::Table::ProjectsTeamsRoleRequests => true,
            web_common::database::Table::ProjectsTeamsRoles => true,
            web_common::database::Table::ProjectsUsersRoleInvitations => true,
            web_common::database::Table::ProjectsUsersRoleRequests => true,
            web_common::database::Table::ProjectsUsersRoles => true,
            web_common::database::Table::Roles => true,
            web_common::database::Table::SampleBioOttTaxonItems => true,
            web_common::database::Table::SampleStates => true,
            web_common::database::Table::SampledIndividualBioOttTaxonItems => true,
            web_common::database::Table::SampledIndividuals => true,
            web_common::database::Table::SampledIndividualsTeamsRoleInvitations => true,
            web_common::database::Table::SampledIndividualsTeamsRoleRequests => true,
            web_common::database::Table::SampledIndividualsTeamsRoles => true,
            web_common::database::Table::SampledIndividualsUsersRoleInvitations => true,
            web_common::database::Table::SampledIndividualsUsersRoleRequests => true,
            web_common::database::Table::SampledIndividualsUsersRoles => true,
            web_common::database::Table::Samples => true,
            web_common::database::Table::SamplesTeamsRoleInvitations => true,
            web_common::database::Table::SamplesTeamsRoleRequests => true,
            web_common::database::Table::SamplesTeamsRoles => true,
            web_common::database::Table::SamplesUsersRoleInvitations => true,
            web_common::database::Table::SamplesUsersRoleRequests => true,
            web_common::database::Table::SamplesUsersRoles => true,
            web_common::database::Table::Spectra => true,
            web_common::database::Table::SpectraCollections => true,
            web_common::database::Table::SpectraCollectionsTeamsRoleInvitations => true,
            web_common::database::Table::SpectraCollectionsTeamsRoleRequests => true,
            web_common::database::Table::SpectraCollectionsTeamsRoles => true,
            web_common::database::Table::SpectraCollectionsUsersRoleInvitations => true,
            web_common::database::Table::SpectraCollectionsUsersRoleRequests => true,
            web_common::database::Table::SpectraCollectionsUsersRoles => true,
            web_common::database::Table::TeamStates => true,
            web_common::database::Table::Teams => true,
            web_common::database::Table::TeamsTeamsRoleInvitations => true,
            web_common::database::Table::TeamsUsersRoleInvitations => true,
            web_common::database::Table::TeamsUsersRoleRequests => true,
            web_common::database::Table::TeamsUsersRoles => true,
            web_common::database::Table::Units => true,
            web_common::database::Table::UserEmails => true,
            web_common::database::Table::Users => true,
            web_common::database::Table::UsersUsersRoleInvitations => true,
            web_common::database::Table::UsersUsersRoleRequests => true,
            web_common::database::Table::UsersUsersRoles => true,
        })
    }
}
/// Trait providing the can_update method for the Table enum.
pub trait EditableTable {
    /// Check whether the user can edit the row.
    ///
    /// # Arguments
    /// * `primary_key` - The primary key of the row.
    /// * `user_id` - The user ID of the user performing the operation.
    /// * `connection` - The database connection.
    ///
    /// # Returns
    /// A boolean indicating whether the user can edit the row.
    fn can_update(
         &self,
         primary_key: web_common::database::operations::PrimaryKey,
         user_id: i32,
         connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>
) -> Result<bool, web_common::api::ApiError>;
}

impl EditableTable for web_common::database::Table {

    fn can_update(
        &self,
        primary_key: web_common::database::operations::PrimaryKey,
        user_id: i32,
        connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>
    ) -> Result<bool, web_common::api::ApiError> {
        Ok(match self {
            web_common::database::Table::BioOttRanks => false,
            web_common::database::Table::BioOttTaxonItems => false,
            web_common::database::Table::Colors => false,
            web_common::database::Table::Countries => false,
            web_common::database::Table::DerivedSamples => false,
            web_common::database::Table::DocumentFormats => false,
            web_common::database::Table::FontAwesomeIcons => false,
            web_common::database::Table::LoginProviders => false,
            web_common::database::Table::Notifications => false,
            web_common::database::Table::Organizations => false,
            web_common::database::Table::ProjectStates => false,
            web_common::database::Table::Projects => Project::is_editor_by_id(primary_key.into(), user_id, connection)?,
            web_common::database::Table::ProjectsTeamsRoleInvitations => false,
            web_common::database::Table::ProjectsTeamsRoleRequests => false,
            web_common::database::Table::ProjectsTeamsRoles => false,
            web_common::database::Table::ProjectsUsersRoleInvitations => false,
            web_common::database::Table::ProjectsUsersRoleRequests => false,
            web_common::database::Table::ProjectsUsersRoles => false,
            web_common::database::Table::Roles => false,
            web_common::database::Table::SampleBioOttTaxonItems => false,
            web_common::database::Table::SampleStates => false,
            web_common::database::Table::SampledIndividualBioOttTaxonItems => false,
            web_common::database::Table::SampledIndividuals => SampledIndividual::is_editor_by_id(primary_key.into(), user_id, connection)?,
            web_common::database::Table::SampledIndividualsTeamsRoleInvitations => false,
            web_common::database::Table::SampledIndividualsTeamsRoleRequests => false,
            web_common::database::Table::SampledIndividualsTeamsRoles => false,
            web_common::database::Table::SampledIndividualsUsersRoleInvitations => false,
            web_common::database::Table::SampledIndividualsUsersRoleRequests => false,
            web_common::database::Table::SampledIndividualsUsersRoles => false,
            web_common::database::Table::Samples => Sample::is_editor_by_id(primary_key.into(), user_id, connection)?,
            web_common::database::Table::SamplesTeamsRoleInvitations => false,
            web_common::database::Table::SamplesTeamsRoleRequests => false,
            web_common::database::Table::SamplesTeamsRoles => false,
            web_common::database::Table::SamplesUsersRoleInvitations => false,
            web_common::database::Table::SamplesUsersRoleRequests => false,
            web_common::database::Table::SamplesUsersRoles => false,
            web_common::database::Table::Spectra => false,
            web_common::database::Table::SpectraCollections => SpectraCollection::is_editor_by_id(primary_key.into(), user_id, connection)?,
            web_common::database::Table::SpectraCollectionsTeamsRoleInvitations => false,
            web_common::database::Table::SpectraCollectionsTeamsRoleRequests => false,
            web_common::database::Table::SpectraCollectionsTeamsRoles => false,
            web_common::database::Table::SpectraCollectionsUsersRoleInvitations => false,
            web_common::database::Table::SpectraCollectionsUsersRoleRequests => false,
            web_common::database::Table::SpectraCollectionsUsersRoles => false,
            web_common::database::Table::TeamStates => false,
            web_common::database::Table::Teams => Team::is_editor_by_id(primary_key.into(), user_id, connection)?,
            web_common::database::Table::TeamsTeamsRoleInvitations => false,
            web_common::database::Table::TeamsUsersRoleInvitations => false,
            web_common::database::Table::TeamsUsersRoleRequests => false,
            web_common::database::Table::TeamsUsersRoles => false,
            web_common::database::Table::Units => false,
            web_common::database::Table::UserEmails => false,
            web_common::database::Table::Users => {
                let primary_key: i32 = primary_key.into();
                primary_key == user_id
            },
            web_common::database::Table::UsersUsersRoleInvitations => false,
            web_common::database::Table::UsersUsersRoleRequests => false,
            web_common::database::Table::UsersUsersRoles => false,
        })
    }
}
/// Trait providing the can_delete method for the Table enum.
pub trait DeletableTable {
    /// Check whether the user can delete the row.
    ///
    /// # Arguments
    /// * `primary_key` - The primary key of the row.
    /// * `user_id` - The user ID of the user performing the operation.
    /// * `connection` - The database connection.
    ///
    /// # Returns
    /// A boolean indicating whether the user can delete the row.
    fn can_delete(
         &self,
         primary_key: web_common::database::operations::PrimaryKey,
         user_id: i32,
         connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>
) -> Result<bool, web_common::api::ApiError>;

    /// Delete the row from the table by the primary key.
    ///
    /// # Arguments
    /// * `primary_key` - The primary key of the row.
    /// * `author_user_id` - The user ID of the user performing the operation.
    /// * `connection` - The database connection.
    ///
    /// # Returns
    /// The number of rows deleted.
    fn delete(
         &self,
         primary_key: web_common::database::operations::PrimaryKey,
         author_user_id: i32,
         connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>
) -> Result<usize, web_common::api::ApiError>;
}

impl DeletableTable for web_common::database::Table {

    fn can_delete(
        &self,
        primary_key: web_common::database::operations::PrimaryKey,
        user_id: i32,
        connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>
    ) -> Result<bool, web_common::api::ApiError> {
        Ok(match self {
            web_common::database::Table::BioOttRanks => false,
            web_common::database::Table::BioOttTaxonItems => false,
            web_common::database::Table::Colors => false,
            web_common::database::Table::Countries => false,
            web_common::database::Table::DerivedSamples => false,
            web_common::database::Table::DocumentFormats => false,
            web_common::database::Table::FontAwesomeIcons => false,
            web_common::database::Table::LoginProviders => false,
            web_common::database::Table::Notifications => false,
            web_common::database::Table::Organizations => false,
            web_common::database::Table::ProjectStates => false,
            web_common::database::Table::Projects => Project::is_admin_by_id(primary_key.into(), user_id, connection)?,
            web_common::database::Table::ProjectsTeamsRoleInvitations => false,
            web_common::database::Table::ProjectsTeamsRoleRequests => false,
            web_common::database::Table::ProjectsTeamsRoles => false,
            web_common::database::Table::ProjectsUsersRoleInvitations => false,
            web_common::database::Table::ProjectsUsersRoleRequests => false,
            web_common::database::Table::ProjectsUsersRoles => false,
            web_common::database::Table::Roles => false,
            web_common::database::Table::SampleBioOttTaxonItems => false,
            web_common::database::Table::SampleStates => false,
            web_common::database::Table::SampledIndividualBioOttTaxonItems => false,
            web_common::database::Table::SampledIndividuals => SampledIndividual::is_admin_by_id(primary_key.into(), user_id, connection)?,
            web_common::database::Table::SampledIndividualsTeamsRoleInvitations => false,
            web_common::database::Table::SampledIndividualsTeamsRoleRequests => false,
            web_common::database::Table::SampledIndividualsTeamsRoles => false,
            web_common::database::Table::SampledIndividualsUsersRoleInvitations => false,
            web_common::database::Table::SampledIndividualsUsersRoleRequests => false,
            web_common::database::Table::SampledIndividualsUsersRoles => false,
            web_common::database::Table::Samples => Sample::is_admin_by_id(primary_key.into(), user_id, connection)?,
            web_common::database::Table::SamplesTeamsRoleInvitations => false,
            web_common::database::Table::SamplesTeamsRoleRequests => false,
            web_common::database::Table::SamplesTeamsRoles => false,
            web_common::database::Table::SamplesUsersRoleInvitations => false,
            web_common::database::Table::SamplesUsersRoleRequests => false,
            web_common::database::Table::SamplesUsersRoles => false,
            web_common::database::Table::Spectra => false,
            web_common::database::Table::SpectraCollections => SpectraCollection::is_admin_by_id(primary_key.into(), user_id, connection)?,
            web_common::database::Table::SpectraCollectionsTeamsRoleInvitations => false,
            web_common::database::Table::SpectraCollectionsTeamsRoleRequests => false,
            web_common::database::Table::SpectraCollectionsTeamsRoles => false,
            web_common::database::Table::SpectraCollectionsUsersRoleInvitations => false,
            web_common::database::Table::SpectraCollectionsUsersRoleRequests => false,
            web_common::database::Table::SpectraCollectionsUsersRoles => false,
            web_common::database::Table::TeamStates => false,
            web_common::database::Table::Teams => Team::is_admin_by_id(primary_key.into(), user_id, connection)?,
            web_common::database::Table::TeamsTeamsRoleInvitations => false,
            web_common::database::Table::TeamsUsersRoleInvitations => false,
            web_common::database::Table::TeamsUsersRoleRequests => false,
            web_common::database::Table::TeamsUsersRoles => false,
            web_common::database::Table::Units => false,
            web_common::database::Table::UserEmails => false,
            web_common::database::Table::Users => false,
            web_common::database::Table::UsersUsersRoleInvitations => false,
            web_common::database::Table::UsersUsersRoleRequests => false,
            web_common::database::Table::UsersUsersRoles => false,
        })
    }
    fn delete(
        &self,
        primary_key: web_common::database::operations::PrimaryKey,
        author_user_id: i32,
        connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>
    ) -> Result<usize, web_common::api::ApiError> {
        if !self.can_delete(primary_key, author_user_id, connection)? {
            return Err(web_common::api::ApiError::unauthorized());
        }
        Ok(match self {
            web_common::database::Table::BioOttRanks => unimplemented!("Delete not implemented for bio_ott_ranks."),
            web_common::database::Table::BioOttTaxonItems => unimplemented!("Delete not implemented for bio_ott_taxon_items."),
            web_common::database::Table::Colors => unimplemented!("Delete not implemented for colors."),
            web_common::database::Table::Countries => unimplemented!("Delete not implemented for countries."),
            web_common::database::Table::DerivedSamples => unimplemented!("Delete not implemented for derived_samples."),
            web_common::database::Table::DocumentFormats => unimplemented!("Delete not implemented for document_formats."),
            web_common::database::Table::FontAwesomeIcons => unimplemented!("Delete not implemented for font_awesome_icons."),
            web_common::database::Table::LoginProviders => unimplemented!("Delete not implemented for login_providers."),
            web_common::database::Table::Notifications => unimplemented!("Delete not implemented for notifications."),
            web_common::database::Table::Organizations => unimplemented!("Delete not implemented for organizations."),
            web_common::database::Table::ProjectStates => unimplemented!("Delete not implemented for project_states."),
            web_common::database::Table::Projects => Project::delete_by_id(primary_key.into(), author_user_id, connection)?,
            web_common::database::Table::ProjectsTeamsRoleInvitations => unimplemented!("Delete not implemented for projects_teams_role_invitations."),
            web_common::database::Table::ProjectsTeamsRoleRequests => unimplemented!("Delete not implemented for projects_teams_role_requests."),
            web_common::database::Table::ProjectsTeamsRoles => unimplemented!("Delete not implemented for projects_teams_roles."),
            web_common::database::Table::ProjectsUsersRoleInvitations => unimplemented!("Delete not implemented for projects_users_role_invitations."),
            web_common::database::Table::ProjectsUsersRoleRequests => unimplemented!("Delete not implemented for projects_users_role_requests."),
            web_common::database::Table::ProjectsUsersRoles => unimplemented!("Delete not implemented for projects_users_roles."),
            web_common::database::Table::Roles => unimplemented!("Delete not implemented for roles."),
            web_common::database::Table::SampleBioOttTaxonItems => unimplemented!("Delete not implemented for sample_bio_ott_taxon_items."),
            web_common::database::Table::SampleStates => unimplemented!("Delete not implemented for sample_states."),
            web_common::database::Table::SampledIndividualBioOttTaxonItems => unimplemented!("Delete not implemented for sampled_individual_bio_ott_taxon_items."),
            web_common::database::Table::SampledIndividuals => SampledIndividual::delete_by_id(primary_key.into(), author_user_id, connection)?,
            web_common::database::Table::SampledIndividualsTeamsRoleInvitations => unimplemented!("Delete not implemented for sampled_individuals_teams_role_invitations."),
            web_common::database::Table::SampledIndividualsTeamsRoleRequests => unimplemented!("Delete not implemented for sampled_individuals_teams_role_requests."),
            web_common::database::Table::SampledIndividualsTeamsRoles => unimplemented!("Delete not implemented for sampled_individuals_teams_roles."),
            web_common::database::Table::SampledIndividualsUsersRoleInvitations => unimplemented!("Delete not implemented for sampled_individuals_users_role_invitations."),
            web_common::database::Table::SampledIndividualsUsersRoleRequests => unimplemented!("Delete not implemented for sampled_individuals_users_role_requests."),
            web_common::database::Table::SampledIndividualsUsersRoles => unimplemented!("Delete not implemented for sampled_individuals_users_roles."),
            web_common::database::Table::Samples => Sample::delete_by_id(primary_key.into(), author_user_id, connection)?,
            web_common::database::Table::SamplesTeamsRoleInvitations => unimplemented!("Delete not implemented for samples_teams_role_invitations."),
            web_common::database::Table::SamplesTeamsRoleRequests => unimplemented!("Delete not implemented for samples_teams_role_requests."),
            web_common::database::Table::SamplesTeamsRoles => unimplemented!("Delete not implemented for samples_teams_roles."),
            web_common::database::Table::SamplesUsersRoleInvitations => unimplemented!("Delete not implemented for samples_users_role_invitations."),
            web_common::database::Table::SamplesUsersRoleRequests => unimplemented!("Delete not implemented for samples_users_role_requests."),
            web_common::database::Table::SamplesUsersRoles => unimplemented!("Delete not implemented for samples_users_roles."),
            web_common::database::Table::Spectra => unimplemented!("Delete not implemented for spectra."),
            web_common::database::Table::SpectraCollections => SpectraCollection::delete_by_id(primary_key.into(), author_user_id, connection)?,
            web_common::database::Table::SpectraCollectionsTeamsRoleInvitations => unimplemented!("Delete not implemented for spectra_collections_teams_role_invitations."),
            web_common::database::Table::SpectraCollectionsTeamsRoleRequests => unimplemented!("Delete not implemented for spectra_collections_teams_role_requests."),
            web_common::database::Table::SpectraCollectionsTeamsRoles => unimplemented!("Delete not implemented for spectra_collections_teams_roles."),
            web_common::database::Table::SpectraCollectionsUsersRoleInvitations => unimplemented!("Delete not implemented for spectra_collections_users_role_invitations."),
            web_common::database::Table::SpectraCollectionsUsersRoleRequests => unimplemented!("Delete not implemented for spectra_collections_users_role_requests."),
            web_common::database::Table::SpectraCollectionsUsersRoles => unimplemented!("Delete not implemented for spectra_collections_users_roles."),
            web_common::database::Table::TeamStates => unimplemented!("Delete not implemented for team_states."),
            web_common::database::Table::Teams => Team::delete_by_id(primary_key.into(), author_user_id, connection)?,
            web_common::database::Table::TeamsTeamsRoleInvitations => unimplemented!("Delete not implemented for teams_teams_role_invitations."),
            web_common::database::Table::TeamsUsersRoleInvitations => unimplemented!("Delete not implemented for teams_users_role_invitations."),
            web_common::database::Table::TeamsUsersRoleRequests => unimplemented!("Delete not implemented for teams_users_role_requests."),
            web_common::database::Table::TeamsUsersRoles => unimplemented!("Delete not implemented for teams_users_roles."),
            web_common::database::Table::Units => unimplemented!("Delete not implemented for units."),
            web_common::database::Table::UserEmails => unimplemented!("Delete not implemented for user_emails."),
            web_common::database::Table::Users => unimplemented!("Delete not implemented for users."),
            web_common::database::Table::UsersUsersRoleInvitations => unimplemented!("Delete not implemented for users_users_role_invitations."),
            web_common::database::Table::UsersUsersRoleRequests => unimplemented!("Delete not implemented for users_users_role_requests."),
            web_common::database::Table::UsersUsersRoles => unimplemented!("Delete not implemented for users_users_roles."),
        })
    }
}
/// Trait providing the all method for the Table enum.
pub trait AllTable {
    /// Get all the rows from the table.
    ///
    /// # Arguments
    /// * `filter` - The filter to apply to the rows.
    /// * `limit` - The maximum number of rows to return.
    /// * `offset` - The number of rows to skip.
    /// * `connection` - The database connection.
    ///
    /// # Returns
    /// A vector of the rows of the table.
    fn all(
         &self,
         filter: Option<Vec<u8>>,
         limit: Option<i64>,
         offset: Option<i64>,
         connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>
) -> Result<Vec<Vec<u8>>, web_common::api::ApiError>;
}

impl AllTable for web_common::database::Table {

    fn all(
        &self,
        filter: Option<Vec<u8>>,
        limit: Option<i64>,
        offset: Option<i64>,
        connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>
    ) -> Result<Vec<Vec<u8>>, web_common::api::ApiError> {
        match self {
            web_common::database::Table::BioOttRanks => {let filter: Option<web_common::database::BioOttRankFilter> = filter.map(|filter| bincode::deserialize::<web_common::database::BioOttRankFilter>(&filter).map_err(web_common::api::ApiError::from)).transpose()?;
NestedBioOttRank::all(filter.as_ref(), limit, offset, connection)?.iter().map(|row| bincode::serialize(row).map_err(web_common::api::ApiError::from)).collect()
},
            web_common::database::Table::BioOttTaxonItems => {let filter: Option<web_common::database::BioOttTaxonItemFilter> = filter.map(|filter| bincode::deserialize::<web_common::database::BioOttTaxonItemFilter>(&filter).map_err(web_common::api::ApiError::from)).transpose()?;
NestedBioOttTaxonItem::all(filter.as_ref(), limit, offset, connection)?.iter().map(|row| bincode::serialize(row).map_err(web_common::api::ApiError::from)).collect()
},
            web_common::database::Table::Colors => {assert!(filter.is_none(), "Filter not implemented for colors.");
Color::all(limit, offset, connection)?.iter().map(|row| bincode::serialize(row).map_err(web_common::api::ApiError::from)).collect()
},
            web_common::database::Table::Countries => {assert!(filter.is_none(), "Filter not implemented for countries.");
Country::all(limit, offset, connection)?.iter().map(|row| bincode::serialize(row).map_err(web_common::api::ApiError::from)).collect()
},
            web_common::database::Table::DerivedSamples => {let filter: Option<web_common::database::DerivedSampleFilter> = filter.map(|filter| bincode::deserialize::<web_common::database::DerivedSampleFilter>(&filter).map_err(web_common::api::ApiError::from)).transpose()?;
NestedDerivedSample::all(filter.as_ref(), limit, offset, connection)?.iter().map(|row| bincode::serialize(row).map_err(web_common::api::ApiError::from)).collect()
},
            web_common::database::Table::DocumentFormats => {let filter: Option<web_common::database::DocumentFormatFilter> = filter.map(|filter| bincode::deserialize::<web_common::database::DocumentFormatFilter>(&filter).map_err(web_common::api::ApiError::from)).transpose()?;
NestedDocumentFormat::all(filter.as_ref(), limit, offset, connection)?.iter().map(|row| bincode::serialize(row).map_err(web_common::api::ApiError::from)).collect()
},
            web_common::database::Table::FontAwesomeIcons => {assert!(filter.is_none(), "Filter not implemented for font_awesome_icons.");
FontAwesomeIcon::all(limit, offset, connection)?.iter().map(|row| bincode::serialize(row).map_err(web_common::api::ApiError::from)).collect()
},
            web_common::database::Table::LoginProviders => {let filter: Option<web_common::database::LoginProviderFilter> = filter.map(|filter| bincode::deserialize::<web_common::database::LoginProviderFilter>(&filter).map_err(web_common::api::ApiError::from)).transpose()?;
NestedLoginProvider::all(filter.as_ref(), limit, offset, connection)?.iter().map(|row| bincode::serialize(row).map_err(web_common::api::ApiError::from)).collect()
},
            web_common::database::Table::Notifications => {let filter: Option<web_common::database::NotificationFilter> = filter.map(|filter| bincode::deserialize::<web_common::database::NotificationFilter>(&filter).map_err(web_common::api::ApiError::from)).transpose()?;
NestedNotification::all(filter.as_ref(), limit, offset, connection)?.iter().map(|row| bincode::serialize(row).map_err(web_common::api::ApiError::from)).collect()
},
            web_common::database::Table::Organizations => {let filter: Option<web_common::database::OrganizationFilter> = filter.map(|filter| bincode::deserialize::<web_common::database::OrganizationFilter>(&filter).map_err(web_common::api::ApiError::from)).transpose()?;
NestedOrganization::all(filter.as_ref(), limit, offset, connection)?.iter().map(|row| bincode::serialize(row).map_err(web_common::api::ApiError::from)).collect()
},
            web_common::database::Table::ProjectStates => {let filter: Option<web_common::database::ProjectStateFilter> = filter.map(|filter| bincode::deserialize::<web_common::database::ProjectStateFilter>(&filter).map_err(web_common::api::ApiError::from)).transpose()?;
NestedProjectState::all(filter.as_ref(), limit, offset, connection)?.iter().map(|row| bincode::serialize(row).map_err(web_common::api::ApiError::from)).collect()
},
            web_common::database::Table::Projects => {let filter: Option<web_common::database::ProjectFilter> = filter.map(|filter| bincode::deserialize::<web_common::database::ProjectFilter>(&filter).map_err(web_common::api::ApiError::from)).transpose()?;
NestedProject::all(filter.as_ref(), limit, offset, connection)?.iter().map(|row| bincode::serialize(row).map_err(web_common::api::ApiError::from)).collect()
},
            web_common::database::Table::ProjectsTeamsRoleInvitations => {let filter: Option<web_common::database::ProjectsTeamsRoleInvitationFilter> = filter.map(|filter| bincode::deserialize::<web_common::database::ProjectsTeamsRoleInvitationFilter>(&filter).map_err(web_common::api::ApiError::from)).transpose()?;
NestedProjectsTeamsRoleInvitation::all(filter.as_ref(), limit, offset, connection)?.iter().map(|row| bincode::serialize(row).map_err(web_common::api::ApiError::from)).collect()
},
            web_common::database::Table::ProjectsTeamsRoleRequests => {let filter: Option<web_common::database::ProjectsTeamsRoleRequestFilter> = filter.map(|filter| bincode::deserialize::<web_common::database::ProjectsTeamsRoleRequestFilter>(&filter).map_err(web_common::api::ApiError::from)).transpose()?;
NestedProjectsTeamsRoleRequest::all(filter.as_ref(), limit, offset, connection)?.iter().map(|row| bincode::serialize(row).map_err(web_common::api::ApiError::from)).collect()
},
            web_common::database::Table::ProjectsTeamsRoles => {let filter: Option<web_common::database::ProjectsTeamsRoleFilter> = filter.map(|filter| bincode::deserialize::<web_common::database::ProjectsTeamsRoleFilter>(&filter).map_err(web_common::api::ApiError::from)).transpose()?;
NestedProjectsTeamsRole::all(filter.as_ref(), limit, offset, connection)?.iter().map(|row| bincode::serialize(row).map_err(web_common::api::ApiError::from)).collect()
},
            web_common::database::Table::ProjectsUsersRoleInvitations => {let filter: Option<web_common::database::ProjectsUsersRoleInvitationFilter> = filter.map(|filter| bincode::deserialize::<web_common::database::ProjectsUsersRoleInvitationFilter>(&filter).map_err(web_common::api::ApiError::from)).transpose()?;
NestedProjectsUsersRoleInvitation::all(filter.as_ref(), limit, offset, connection)?.iter().map(|row| bincode::serialize(row).map_err(web_common::api::ApiError::from)).collect()
},
            web_common::database::Table::ProjectsUsersRoleRequests => {let filter: Option<web_common::database::ProjectsUsersRoleRequestFilter> = filter.map(|filter| bincode::deserialize::<web_common::database::ProjectsUsersRoleRequestFilter>(&filter).map_err(web_common::api::ApiError::from)).transpose()?;
NestedProjectsUsersRoleRequest::all(filter.as_ref(), limit, offset, connection)?.iter().map(|row| bincode::serialize(row).map_err(web_common::api::ApiError::from)).collect()
},
            web_common::database::Table::ProjectsUsersRoles => {let filter: Option<web_common::database::ProjectsUsersRoleFilter> = filter.map(|filter| bincode::deserialize::<web_common::database::ProjectsUsersRoleFilter>(&filter).map_err(web_common::api::ApiError::from)).transpose()?;
NestedProjectsUsersRole::all(filter.as_ref(), limit, offset, connection)?.iter().map(|row| bincode::serialize(row).map_err(web_common::api::ApiError::from)).collect()
},
            web_common::database::Table::Roles => {let filter: Option<web_common::database::RoleFilter> = filter.map(|filter| bincode::deserialize::<web_common::database::RoleFilter>(&filter).map_err(web_common::api::ApiError::from)).transpose()?;
NestedRole::all(filter.as_ref(), limit, offset, connection)?.iter().map(|row| bincode::serialize(row).map_err(web_common::api::ApiError::from)).collect()
},
            web_common::database::Table::SampleBioOttTaxonItems => {let filter: Option<web_common::database::SampleBioOttTaxonItemFilter> = filter.map(|filter| bincode::deserialize::<web_common::database::SampleBioOttTaxonItemFilter>(&filter).map_err(web_common::api::ApiError::from)).transpose()?;
NestedSampleBioOttTaxonItem::all(filter.as_ref(), limit, offset, connection)?.iter().map(|row| bincode::serialize(row).map_err(web_common::api::ApiError::from)).collect()
},
            web_common::database::Table::SampleStates => {let filter: Option<web_common::database::SampleStateFilter> = filter.map(|filter| bincode::deserialize::<web_common::database::SampleStateFilter>(&filter).map_err(web_common::api::ApiError::from)).transpose()?;
NestedSampleState::all(filter.as_ref(), limit, offset, connection)?.iter().map(|row| bincode::serialize(row).map_err(web_common::api::ApiError::from)).collect()
},
            web_common::database::Table::SampledIndividualBioOttTaxonItems => {let filter: Option<web_common::database::SampledIndividualBioOttTaxonItemFilter> = filter.map(|filter| bincode::deserialize::<web_common::database::SampledIndividualBioOttTaxonItemFilter>(&filter).map_err(web_common::api::ApiError::from)).transpose()?;
NestedSampledIndividualBioOttTaxonItem::all(filter.as_ref(), limit, offset, connection)?.iter().map(|row| bincode::serialize(row).map_err(web_common::api::ApiError::from)).collect()
},
            web_common::database::Table::SampledIndividuals => {let filter: Option<web_common::database::SampledIndividualFilter> = filter.map(|filter| bincode::deserialize::<web_common::database::SampledIndividualFilter>(&filter).map_err(web_common::api::ApiError::from)).transpose()?;
NestedSampledIndividual::all(filter.as_ref(), limit, offset, connection)?.iter().map(|row| bincode::serialize(row).map_err(web_common::api::ApiError::from)).collect()
},
            web_common::database::Table::SampledIndividualsTeamsRoleInvitations => {let filter: Option<web_common::database::SampledIndividualsTeamsRoleInvitationFilter> = filter.map(|filter| bincode::deserialize::<web_common::database::SampledIndividualsTeamsRoleInvitationFilter>(&filter).map_err(web_common::api::ApiError::from)).transpose()?;
NestedSampledIndividualsTeamsRoleInvitation::all(filter.as_ref(), limit, offset, connection)?.iter().map(|row| bincode::serialize(row).map_err(web_common::api::ApiError::from)).collect()
},
            web_common::database::Table::SampledIndividualsTeamsRoleRequests => {let filter: Option<web_common::database::SampledIndividualsTeamsRoleRequestFilter> = filter.map(|filter| bincode::deserialize::<web_common::database::SampledIndividualsTeamsRoleRequestFilter>(&filter).map_err(web_common::api::ApiError::from)).transpose()?;
NestedSampledIndividualsTeamsRoleRequest::all(filter.as_ref(), limit, offset, connection)?.iter().map(|row| bincode::serialize(row).map_err(web_common::api::ApiError::from)).collect()
},
            web_common::database::Table::SampledIndividualsTeamsRoles => {let filter: Option<web_common::database::SampledIndividualsTeamsRoleFilter> = filter.map(|filter| bincode::deserialize::<web_common::database::SampledIndividualsTeamsRoleFilter>(&filter).map_err(web_common::api::ApiError::from)).transpose()?;
NestedSampledIndividualsTeamsRole::all(filter.as_ref(), limit, offset, connection)?.iter().map(|row| bincode::serialize(row).map_err(web_common::api::ApiError::from)).collect()
},
            web_common::database::Table::SampledIndividualsUsersRoleInvitations => {let filter: Option<web_common::database::SampledIndividualsUsersRoleInvitationFilter> = filter.map(|filter| bincode::deserialize::<web_common::database::SampledIndividualsUsersRoleInvitationFilter>(&filter).map_err(web_common::api::ApiError::from)).transpose()?;
NestedSampledIndividualsUsersRoleInvitation::all(filter.as_ref(), limit, offset, connection)?.iter().map(|row| bincode::serialize(row).map_err(web_common::api::ApiError::from)).collect()
},
            web_common::database::Table::SampledIndividualsUsersRoleRequests => {let filter: Option<web_common::database::SampledIndividualsUsersRoleRequestFilter> = filter.map(|filter| bincode::deserialize::<web_common::database::SampledIndividualsUsersRoleRequestFilter>(&filter).map_err(web_common::api::ApiError::from)).transpose()?;
NestedSampledIndividualsUsersRoleRequest::all(filter.as_ref(), limit, offset, connection)?.iter().map(|row| bincode::serialize(row).map_err(web_common::api::ApiError::from)).collect()
},
            web_common::database::Table::SampledIndividualsUsersRoles => {let filter: Option<web_common::database::SampledIndividualsUsersRoleFilter> = filter.map(|filter| bincode::deserialize::<web_common::database::SampledIndividualsUsersRoleFilter>(&filter).map_err(web_common::api::ApiError::from)).transpose()?;
NestedSampledIndividualsUsersRole::all(filter.as_ref(), limit, offset, connection)?.iter().map(|row| bincode::serialize(row).map_err(web_common::api::ApiError::from)).collect()
},
            web_common::database::Table::Samples => {let filter: Option<web_common::database::SampleFilter> = filter.map(|filter| bincode::deserialize::<web_common::database::SampleFilter>(&filter).map_err(web_common::api::ApiError::from)).transpose()?;
NestedSample::all(filter.as_ref(), limit, offset, connection)?.iter().map(|row| bincode::serialize(row).map_err(web_common::api::ApiError::from)).collect()
},
            web_common::database::Table::SamplesTeamsRoleInvitations => {let filter: Option<web_common::database::SamplesTeamsRoleInvitationFilter> = filter.map(|filter| bincode::deserialize::<web_common::database::SamplesTeamsRoleInvitationFilter>(&filter).map_err(web_common::api::ApiError::from)).transpose()?;
NestedSamplesTeamsRoleInvitation::all(filter.as_ref(), limit, offset, connection)?.iter().map(|row| bincode::serialize(row).map_err(web_common::api::ApiError::from)).collect()
},
            web_common::database::Table::SamplesTeamsRoleRequests => {let filter: Option<web_common::database::SamplesTeamsRoleRequestFilter> = filter.map(|filter| bincode::deserialize::<web_common::database::SamplesTeamsRoleRequestFilter>(&filter).map_err(web_common::api::ApiError::from)).transpose()?;
NestedSamplesTeamsRoleRequest::all(filter.as_ref(), limit, offset, connection)?.iter().map(|row| bincode::serialize(row).map_err(web_common::api::ApiError::from)).collect()
},
            web_common::database::Table::SamplesTeamsRoles => {let filter: Option<web_common::database::SamplesTeamsRoleFilter> = filter.map(|filter| bincode::deserialize::<web_common::database::SamplesTeamsRoleFilter>(&filter).map_err(web_common::api::ApiError::from)).transpose()?;
NestedSamplesTeamsRole::all(filter.as_ref(), limit, offset, connection)?.iter().map(|row| bincode::serialize(row).map_err(web_common::api::ApiError::from)).collect()
},
            web_common::database::Table::SamplesUsersRoleInvitations => {let filter: Option<web_common::database::SamplesUsersRoleInvitationFilter> = filter.map(|filter| bincode::deserialize::<web_common::database::SamplesUsersRoleInvitationFilter>(&filter).map_err(web_common::api::ApiError::from)).transpose()?;
NestedSamplesUsersRoleInvitation::all(filter.as_ref(), limit, offset, connection)?.iter().map(|row| bincode::serialize(row).map_err(web_common::api::ApiError::from)).collect()
},
            web_common::database::Table::SamplesUsersRoleRequests => {let filter: Option<web_common::database::SamplesUsersRoleRequestFilter> = filter.map(|filter| bincode::deserialize::<web_common::database::SamplesUsersRoleRequestFilter>(&filter).map_err(web_common::api::ApiError::from)).transpose()?;
NestedSamplesUsersRoleRequest::all(filter.as_ref(), limit, offset, connection)?.iter().map(|row| bincode::serialize(row).map_err(web_common::api::ApiError::from)).collect()
},
            web_common::database::Table::SamplesUsersRoles => {let filter: Option<web_common::database::SamplesUsersRoleFilter> = filter.map(|filter| bincode::deserialize::<web_common::database::SamplesUsersRoleFilter>(&filter).map_err(web_common::api::ApiError::from)).transpose()?;
NestedSamplesUsersRole::all(filter.as_ref(), limit, offset, connection)?.iter().map(|row| bincode::serialize(row).map_err(web_common::api::ApiError::from)).collect()
},
            web_common::database::Table::Spectra => {let filter: Option<web_common::database::SpectraFilter> = filter.map(|filter| bincode::deserialize::<web_common::database::SpectraFilter>(&filter).map_err(web_common::api::ApiError::from)).transpose()?;
NestedSpectra::all(filter.as_ref(), limit, offset, connection)?.iter().map(|row| bincode::serialize(row).map_err(web_common::api::ApiError::from)).collect()
},
            web_common::database::Table::SpectraCollections => {let filter: Option<web_common::database::SpectraCollectionFilter> = filter.map(|filter| bincode::deserialize::<web_common::database::SpectraCollectionFilter>(&filter).map_err(web_common::api::ApiError::from)).transpose()?;
NestedSpectraCollection::all(filter.as_ref(), limit, offset, connection)?.iter().map(|row| bincode::serialize(row).map_err(web_common::api::ApiError::from)).collect()
},
            web_common::database::Table::SpectraCollectionsTeamsRoleInvitations => {let filter: Option<web_common::database::SpectraCollectionsTeamsRoleInvitationFilter> = filter.map(|filter| bincode::deserialize::<web_common::database::SpectraCollectionsTeamsRoleInvitationFilter>(&filter).map_err(web_common::api::ApiError::from)).transpose()?;
NestedSpectraCollectionsTeamsRoleInvitation::all(filter.as_ref(), limit, offset, connection)?.iter().map(|row| bincode::serialize(row).map_err(web_common::api::ApiError::from)).collect()
},
            web_common::database::Table::SpectraCollectionsTeamsRoleRequests => {let filter: Option<web_common::database::SpectraCollectionsTeamsRoleRequestFilter> = filter.map(|filter| bincode::deserialize::<web_common::database::SpectraCollectionsTeamsRoleRequestFilter>(&filter).map_err(web_common::api::ApiError::from)).transpose()?;
NestedSpectraCollectionsTeamsRoleRequest::all(filter.as_ref(), limit, offset, connection)?.iter().map(|row| bincode::serialize(row).map_err(web_common::api::ApiError::from)).collect()
},
            web_common::database::Table::SpectraCollectionsTeamsRoles => {let filter: Option<web_common::database::SpectraCollectionsTeamsRoleFilter> = filter.map(|filter| bincode::deserialize::<web_common::database::SpectraCollectionsTeamsRoleFilter>(&filter).map_err(web_common::api::ApiError::from)).transpose()?;
NestedSpectraCollectionsTeamsRole::all(filter.as_ref(), limit, offset, connection)?.iter().map(|row| bincode::serialize(row).map_err(web_common::api::ApiError::from)).collect()
},
            web_common::database::Table::SpectraCollectionsUsersRoleInvitations => {let filter: Option<web_common::database::SpectraCollectionsUsersRoleInvitationFilter> = filter.map(|filter| bincode::deserialize::<web_common::database::SpectraCollectionsUsersRoleInvitationFilter>(&filter).map_err(web_common::api::ApiError::from)).transpose()?;
NestedSpectraCollectionsUsersRoleInvitation::all(filter.as_ref(), limit, offset, connection)?.iter().map(|row| bincode::serialize(row).map_err(web_common::api::ApiError::from)).collect()
},
            web_common::database::Table::SpectraCollectionsUsersRoleRequests => {let filter: Option<web_common::database::SpectraCollectionsUsersRoleRequestFilter> = filter.map(|filter| bincode::deserialize::<web_common::database::SpectraCollectionsUsersRoleRequestFilter>(&filter).map_err(web_common::api::ApiError::from)).transpose()?;
NestedSpectraCollectionsUsersRoleRequest::all(filter.as_ref(), limit, offset, connection)?.iter().map(|row| bincode::serialize(row).map_err(web_common::api::ApiError::from)).collect()
},
            web_common::database::Table::SpectraCollectionsUsersRoles => {let filter: Option<web_common::database::SpectraCollectionsUsersRoleFilter> = filter.map(|filter| bincode::deserialize::<web_common::database::SpectraCollectionsUsersRoleFilter>(&filter).map_err(web_common::api::ApiError::from)).transpose()?;
NestedSpectraCollectionsUsersRole::all(filter.as_ref(), limit, offset, connection)?.iter().map(|row| bincode::serialize(row).map_err(web_common::api::ApiError::from)).collect()
},
            web_common::database::Table::TeamStates => {let filter: Option<web_common::database::TeamStateFilter> = filter.map(|filter| bincode::deserialize::<web_common::database::TeamStateFilter>(&filter).map_err(web_common::api::ApiError::from)).transpose()?;
NestedTeamState::all(filter.as_ref(), limit, offset, connection)?.iter().map(|row| bincode::serialize(row).map_err(web_common::api::ApiError::from)).collect()
},
            web_common::database::Table::Teams => {let filter: Option<web_common::database::TeamFilter> = filter.map(|filter| bincode::deserialize::<web_common::database::TeamFilter>(&filter).map_err(web_common::api::ApiError::from)).transpose()?;
NestedTeam::all(filter.as_ref(), limit, offset, connection)?.iter().map(|row| bincode::serialize(row).map_err(web_common::api::ApiError::from)).collect()
},
            web_common::database::Table::TeamsTeamsRoleInvitations => {let filter: Option<web_common::database::TeamsTeamsRoleInvitationFilter> = filter.map(|filter| bincode::deserialize::<web_common::database::TeamsTeamsRoleInvitationFilter>(&filter).map_err(web_common::api::ApiError::from)).transpose()?;
NestedTeamsTeamsRoleInvitation::all(filter.as_ref(), limit, offset, connection)?.iter().map(|row| bincode::serialize(row).map_err(web_common::api::ApiError::from)).collect()
},
            web_common::database::Table::TeamsUsersRoleInvitations => {let filter: Option<web_common::database::TeamsUsersRoleInvitationFilter> = filter.map(|filter| bincode::deserialize::<web_common::database::TeamsUsersRoleInvitationFilter>(&filter).map_err(web_common::api::ApiError::from)).transpose()?;
NestedTeamsUsersRoleInvitation::all(filter.as_ref(), limit, offset, connection)?.iter().map(|row| bincode::serialize(row).map_err(web_common::api::ApiError::from)).collect()
},
            web_common::database::Table::TeamsUsersRoleRequests => {let filter: Option<web_common::database::TeamsUsersRoleRequestFilter> = filter.map(|filter| bincode::deserialize::<web_common::database::TeamsUsersRoleRequestFilter>(&filter).map_err(web_common::api::ApiError::from)).transpose()?;
NestedTeamsUsersRoleRequest::all(filter.as_ref(), limit, offset, connection)?.iter().map(|row| bincode::serialize(row).map_err(web_common::api::ApiError::from)).collect()
},
            web_common::database::Table::TeamsUsersRoles => {let filter: Option<web_common::database::TeamsUsersRoleFilter> = filter.map(|filter| bincode::deserialize::<web_common::database::TeamsUsersRoleFilter>(&filter).map_err(web_common::api::ApiError::from)).transpose()?;
NestedTeamsUsersRole::all(filter.as_ref(), limit, offset, connection)?.iter().map(|row| bincode::serialize(row).map_err(web_common::api::ApiError::from)).collect()
},
            web_common::database::Table::Units => {assert!(filter.is_none(), "Filter not implemented for units.");
Unit::all(limit, offset, connection)?.iter().map(|row| bincode::serialize(row).map_err(web_common::api::ApiError::from)).collect()
},
            web_common::database::Table::UserEmails => {let filter: Option<web_common::database::UserEmailFilter> = filter.map(|filter| bincode::deserialize::<web_common::database::UserEmailFilter>(&filter).map_err(web_common::api::ApiError::from)).transpose()?;
NestedUserEmail::all(filter.as_ref(), limit, offset, connection)?.iter().map(|row| bincode::serialize(row).map_err(web_common::api::ApiError::from)).collect()
},
            web_common::database::Table::Users => {assert!(filter.is_none(), "Filter not implemented for users.");
User::all(limit, offset, connection)?.iter().map(|row| bincode::serialize(row).map_err(web_common::api::ApiError::from)).collect()
},
            web_common::database::Table::UsersUsersRoleInvitations => {let filter: Option<web_common::database::UsersUsersRoleInvitationFilter> = filter.map(|filter| bincode::deserialize::<web_common::database::UsersUsersRoleInvitationFilter>(&filter).map_err(web_common::api::ApiError::from)).transpose()?;
NestedUsersUsersRoleInvitation::all(filter.as_ref(), limit, offset, connection)?.iter().map(|row| bincode::serialize(row).map_err(web_common::api::ApiError::from)).collect()
},
            web_common::database::Table::UsersUsersRoleRequests => {let filter: Option<web_common::database::UsersUsersRoleRequestFilter> = filter.map(|filter| bincode::deserialize::<web_common::database::UsersUsersRoleRequestFilter>(&filter).map_err(web_common::api::ApiError::from)).transpose()?;
NestedUsersUsersRoleRequest::all(filter.as_ref(), limit, offset, connection)?.iter().map(|row| bincode::serialize(row).map_err(web_common::api::ApiError::from)).collect()
},
            web_common::database::Table::UsersUsersRoles => {let filter: Option<web_common::database::UsersUsersRoleFilter> = filter.map(|filter| bincode::deserialize::<web_common::database::UsersUsersRoleFilter>(&filter).map_err(web_common::api::ApiError::from)).transpose()?;
NestedUsersUsersRole::all(filter.as_ref(), limit, offset, connection)?.iter().map(|row| bincode::serialize(row).map_err(web_common::api::ApiError::from)).collect()
},
        }
    }
}
/// Trait providing the all_by_updated_at method for the Table enum.
pub trait AllByUpdatedAtTable {
    /// Get all the rows from the table ordered by the `updated_at` column.
    ///
    /// # Arguments
    /// * `filter` - The filter to apply to the rows.
    /// * `limit` - The maximum number of rows to return.
    /// * `offset` - The number of rows to skip.
    /// * `connection` - The database connection.
    ///
    /// # Returns
    /// A vector of the rows of the table.
    fn all_by_updated_at(
         &self,
         filter: Option<Vec<u8>>,
         limit: Option<i64>,
         offset: Option<i64>,
         connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>
) -> Result<Vec<Vec<u8>>, web_common::api::ApiError>;
}

impl AllByUpdatedAtTable for web_common::database::Table {

    fn all_by_updated_at(
        &self,
        filter: Option<Vec<u8>>,
        limit: Option<i64>,
        offset: Option<i64>,
        connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>
    ) -> Result<Vec<Vec<u8>>, web_common::api::ApiError> {
        match self {
            web_common::database::Table::BioOttRanks => unimplemented!("all_by_updated_at not implemented for bio_ott_ranks."),
            web_common::database::Table::BioOttTaxonItems => unimplemented!("all_by_updated_at not implemented for bio_ott_taxon_items."),
            web_common::database::Table::Colors => unimplemented!("all_by_updated_at not implemented for colors."),
            web_common::database::Table::Countries => unimplemented!("all_by_updated_at not implemented for countries."),
            web_common::database::Table::DerivedSamples => unimplemented!("all_by_updated_at not implemented for derived_samples."),
            web_common::database::Table::DocumentFormats => unimplemented!("all_by_updated_at not implemented for document_formats."),
            web_common::database::Table::FontAwesomeIcons => unimplemented!("all_by_updated_at not implemented for font_awesome_icons."),
            web_common::database::Table::LoginProviders => unimplemented!("all_by_updated_at not implemented for login_providers."),
            web_common::database::Table::Notifications => unimplemented!("all_by_updated_at not implemented for notifications."),
            web_common::database::Table::Organizations => unimplemented!("all_by_updated_at not implemented for organizations."),
            web_common::database::Table::ProjectStates => unimplemented!("all_by_updated_at not implemented for project_states."),
            web_common::database::Table::Projects => {let filter: Option<web_common::database::ProjectFilter> = filter.map(|filter| bincode::deserialize::<web_common::database::ProjectFilter>(&filter).map_err(web_common::api::ApiError::from)).transpose()?;
NestedProject::all_by_updated_at(filter.as_ref(), limit, offset, connection)?.iter().map(|row| bincode::serialize(row).map_err(web_common::api::ApiError::from)).collect()
},
            web_common::database::Table::ProjectsTeamsRoleInvitations => unimplemented!("all_by_updated_at not implemented for projects_teams_role_invitations."),
            web_common::database::Table::ProjectsTeamsRoleRequests => unimplemented!("all_by_updated_at not implemented for projects_teams_role_requests."),
            web_common::database::Table::ProjectsTeamsRoles => unimplemented!("all_by_updated_at not implemented for projects_teams_roles."),
            web_common::database::Table::ProjectsUsersRoleInvitations => unimplemented!("all_by_updated_at not implemented for projects_users_role_invitations."),
            web_common::database::Table::ProjectsUsersRoleRequests => unimplemented!("all_by_updated_at not implemented for projects_users_role_requests."),
            web_common::database::Table::ProjectsUsersRoles => unimplemented!("all_by_updated_at not implemented for projects_users_roles."),
            web_common::database::Table::Roles => unimplemented!("all_by_updated_at not implemented for roles."),
            web_common::database::Table::SampleBioOttTaxonItems => unimplemented!("all_by_updated_at not implemented for sample_bio_ott_taxon_items."),
            web_common::database::Table::SampleStates => unimplemented!("all_by_updated_at not implemented for sample_states."),
            web_common::database::Table::SampledIndividualBioOttTaxonItems => unimplemented!("all_by_updated_at not implemented for sampled_individual_bio_ott_taxon_items."),
            web_common::database::Table::SampledIndividuals => {let filter: Option<web_common::database::SampledIndividualFilter> = filter.map(|filter| bincode::deserialize::<web_common::database::SampledIndividualFilter>(&filter).map_err(web_common::api::ApiError::from)).transpose()?;
NestedSampledIndividual::all_by_updated_at(filter.as_ref(), limit, offset, connection)?.iter().map(|row| bincode::serialize(row).map_err(web_common::api::ApiError::from)).collect()
},
            web_common::database::Table::SampledIndividualsTeamsRoleInvitations => unimplemented!("all_by_updated_at not implemented for sampled_individuals_teams_role_invitations."),
            web_common::database::Table::SampledIndividualsTeamsRoleRequests => unimplemented!("all_by_updated_at not implemented for sampled_individuals_teams_role_requests."),
            web_common::database::Table::SampledIndividualsTeamsRoles => unimplemented!("all_by_updated_at not implemented for sampled_individuals_teams_roles."),
            web_common::database::Table::SampledIndividualsUsersRoleInvitations => unimplemented!("all_by_updated_at not implemented for sampled_individuals_users_role_invitations."),
            web_common::database::Table::SampledIndividualsUsersRoleRequests => unimplemented!("all_by_updated_at not implemented for sampled_individuals_users_role_requests."),
            web_common::database::Table::SampledIndividualsUsersRoles => unimplemented!("all_by_updated_at not implemented for sampled_individuals_users_roles."),
            web_common::database::Table::Samples => {let filter: Option<web_common::database::SampleFilter> = filter.map(|filter| bincode::deserialize::<web_common::database::SampleFilter>(&filter).map_err(web_common::api::ApiError::from)).transpose()?;
NestedSample::all_by_updated_at(filter.as_ref(), limit, offset, connection)?.iter().map(|row| bincode::serialize(row).map_err(web_common::api::ApiError::from)).collect()
},
            web_common::database::Table::SamplesTeamsRoleInvitations => unimplemented!("all_by_updated_at not implemented for samples_teams_role_invitations."),
            web_common::database::Table::SamplesTeamsRoleRequests => unimplemented!("all_by_updated_at not implemented for samples_teams_role_requests."),
            web_common::database::Table::SamplesTeamsRoles => unimplemented!("all_by_updated_at not implemented for samples_teams_roles."),
            web_common::database::Table::SamplesUsersRoleInvitations => unimplemented!("all_by_updated_at not implemented for samples_users_role_invitations."),
            web_common::database::Table::SamplesUsersRoleRequests => unimplemented!("all_by_updated_at not implemented for samples_users_role_requests."),
            web_common::database::Table::SamplesUsersRoles => unimplemented!("all_by_updated_at not implemented for samples_users_roles."),
            web_common::database::Table::Spectra => unimplemented!("all_by_updated_at not implemented for spectra."),
            web_common::database::Table::SpectraCollections => {let filter: Option<web_common::database::SpectraCollectionFilter> = filter.map(|filter| bincode::deserialize::<web_common::database::SpectraCollectionFilter>(&filter).map_err(web_common::api::ApiError::from)).transpose()?;
NestedSpectraCollection::all_by_updated_at(filter.as_ref(), limit, offset, connection)?.iter().map(|row| bincode::serialize(row).map_err(web_common::api::ApiError::from)).collect()
},
            web_common::database::Table::SpectraCollectionsTeamsRoleInvitations => unimplemented!("all_by_updated_at not implemented for spectra_collections_teams_role_invitations."),
            web_common::database::Table::SpectraCollectionsTeamsRoleRequests => unimplemented!("all_by_updated_at not implemented for spectra_collections_teams_role_requests."),
            web_common::database::Table::SpectraCollectionsTeamsRoles => unimplemented!("all_by_updated_at not implemented for spectra_collections_teams_roles."),
            web_common::database::Table::SpectraCollectionsUsersRoleInvitations => unimplemented!("all_by_updated_at not implemented for spectra_collections_users_role_invitations."),
            web_common::database::Table::SpectraCollectionsUsersRoleRequests => unimplemented!("all_by_updated_at not implemented for spectra_collections_users_role_requests."),
            web_common::database::Table::SpectraCollectionsUsersRoles => unimplemented!("all_by_updated_at not implemented for spectra_collections_users_roles."),
            web_common::database::Table::TeamStates => unimplemented!("all_by_updated_at not implemented for team_states."),
            web_common::database::Table::Teams => {let filter: Option<web_common::database::TeamFilter> = filter.map(|filter| bincode::deserialize::<web_common::database::TeamFilter>(&filter).map_err(web_common::api::ApiError::from)).transpose()?;
NestedTeam::all_by_updated_at(filter.as_ref(), limit, offset, connection)?.iter().map(|row| bincode::serialize(row).map_err(web_common::api::ApiError::from)).collect()
},
            web_common::database::Table::TeamsTeamsRoleInvitations => unimplemented!("all_by_updated_at not implemented for teams_teams_role_invitations."),
            web_common::database::Table::TeamsUsersRoleInvitations => unimplemented!("all_by_updated_at not implemented for teams_users_role_invitations."),
            web_common::database::Table::TeamsUsersRoleRequests => unimplemented!("all_by_updated_at not implemented for teams_users_role_requests."),
            web_common::database::Table::TeamsUsersRoles => unimplemented!("all_by_updated_at not implemented for teams_users_roles."),
            web_common::database::Table::Units => unimplemented!("all_by_updated_at not implemented for units."),
            web_common::database::Table::UserEmails => unimplemented!("all_by_updated_at not implemented for user_emails."),
            web_common::database::Table::Users => {assert!(filter.is_none(), "Filter not implemented for users.");
User::all_by_updated_at(limit, offset, connection)?.iter().map(|row| bincode::serialize(row).map_err(web_common::api::ApiError::from)).collect()
},
            web_common::database::Table::UsersUsersRoleInvitations => unimplemented!("all_by_updated_at not implemented for users_users_role_invitations."),
            web_common::database::Table::UsersUsersRoleRequests => unimplemented!("all_by_updated_at not implemented for users_users_role_requests."),
            web_common::database::Table::UsersUsersRoles => unimplemented!("all_by_updated_at not implemented for users_users_roles."),
        }
    }
}
/// Trait providing the insert method for the Table enum.
pub trait InsertableTable {
    /// Insert a new row into the table.
    ///
    /// # Arguments
    /// * `row` - The bincode-serialized row of the table.
    /// * `user_id` - The id of the user inserting the row.
    /// * `connection` - The database connection.
    ///
    /// # Returns
    /// The bincode-serialized row of the table.
    fn insert(
         &self,
         row: Vec<u8>,
         user_id: i32,
         connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>
) -> Result<Vec<u8>, web_common::api::ApiError>;
}

impl InsertableTable for web_common::database::Table {

    fn insert(
        &self,
        row: Vec<u8>,
        user_id: i32,
        connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>
    ) -> Result<Vec<u8>, web_common::api::ApiError> {
        Ok(match self {
            web_common::database::Table::BioOttRanks => unreachable!("Table `bio_ott_ranks` is not insertable as it does not have a known column associated to a creator user id."),
            web_common::database::Table::BioOttTaxonItems => unreachable!("Table `bio_ott_taxon_items` is not insertable as it does not have a known column associated to a creator user id."),
            web_common::database::Table::Colors => unreachable!("Table `colors` is not insertable as it does not have a known column associated to a creator user id."),
            web_common::database::Table::Countries => unreachable!("Table `countries` is not insertable as it does not have a known column associated to a creator user id."),
            web_common::database::Table::DerivedSamples => unreachable!("Table `derived_samples` is not insertable as it does not have a known column associated to a creator user id."),
            web_common::database::Table::DocumentFormats => unreachable!("Table `document_formats` is not insertable as it does not have a known column associated to a creator user id."),
            web_common::database::Table::FontAwesomeIcons => unreachable!("Table `font_awesome_icons` is not insertable as it does not have a known column associated to a creator user id."),
            web_common::database::Table::LoginProviders => unreachable!("Table `login_providers` is not insertable as it does not have a known column associated to a creator user id."),
            web_common::database::Table::Notifications => unreachable!("Table `notifications` is not insertable as it does not have a known column associated to a creator user id."),
            web_common::database::Table::Organizations => unreachable!("Table `organizations` is not insertable as it does not have a known column associated to a creator user id."),
            web_common::database::Table::ProjectStates => unreachable!("Table `project_states` is not insertable as it does not have a known column associated to a creator user id."),
            web_common::database::Table::Projects => {
                let row: web_common::database::NewProject = bincode::deserialize::<web_common::database::NewProject>(&row).map_err(web_common::api::ApiError::from)?;
                let inserted_row: crate::models::Project = <web_common::database::NewProject as InsertRow>::insert(row, user_id, connection)?;
                let nested_row = crate::nested_models::NestedProject::from_flat(inserted_row, connection)?;
                 bincode::serialize(&nested_row).map_err(web_common::api::ApiError::from)?
            },
            web_common::database::Table::ProjectsTeamsRoleInvitations => unreachable!("Table `projects_teams_role_invitations` is not insertable as it does not have a known column associated to a creator user id."),
            web_common::database::Table::ProjectsTeamsRoleRequests => unreachable!("Table `projects_teams_role_requests` is not insertable as it does not have a known column associated to a creator user id."),
            web_common::database::Table::ProjectsTeamsRoles => unreachable!("Table `projects_teams_roles` is not insertable as it does not have a known column associated to a creator user id."),
            web_common::database::Table::ProjectsUsersRoleInvitations => unreachable!("Table `projects_users_role_invitations` is not insertable as it does not have a known column associated to a creator user id."),
            web_common::database::Table::ProjectsUsersRoleRequests => unreachable!("Table `projects_users_role_requests` is not insertable as it does not have a known column associated to a creator user id."),
            web_common::database::Table::ProjectsUsersRoles => unreachable!("Table `projects_users_roles` is not insertable as it does not have a known column associated to a creator user id."),
            web_common::database::Table::Roles => unreachable!("Table `roles` is not insertable as it does not have a known column associated to a creator user id."),
            web_common::database::Table::SampleBioOttTaxonItems => unreachable!("Table `sample_bio_ott_taxon_items` is not insertable as it does not have a known column associated to a creator user id."),
            web_common::database::Table::SampleStates => unreachable!("Table `sample_states` is not insertable as it does not have a known column associated to a creator user id."),
            web_common::database::Table::SampledIndividualBioOttTaxonItems => unreachable!("Table `sampled_individual_bio_ott_taxon_items` is not insertable as it does not have a known column associated to a creator user id."),
            web_common::database::Table::SampledIndividuals => {
                let row: web_common::database::NewSampledIndividual = bincode::deserialize::<web_common::database::NewSampledIndividual>(&row).map_err(web_common::api::ApiError::from)?;
                let inserted_row: crate::models::SampledIndividual = <web_common::database::NewSampledIndividual as InsertRow>::insert(row, user_id, connection)?;
                let nested_row = crate::nested_models::NestedSampledIndividual::from_flat(inserted_row, connection)?;
                 bincode::serialize(&nested_row).map_err(web_common::api::ApiError::from)?
            },
            web_common::database::Table::SampledIndividualsTeamsRoleInvitations => unreachable!("Table `sampled_individuals_teams_role_invitations` is not insertable as it does not have a known column associated to a creator user id."),
            web_common::database::Table::SampledIndividualsTeamsRoleRequests => unreachable!("Table `sampled_individuals_teams_role_requests` is not insertable as it does not have a known column associated to a creator user id."),
            web_common::database::Table::SampledIndividualsTeamsRoles => unreachable!("Table `sampled_individuals_teams_roles` is not insertable as it does not have a known column associated to a creator user id."),
            web_common::database::Table::SampledIndividualsUsersRoleInvitations => unreachable!("Table `sampled_individuals_users_role_invitations` is not insertable as it does not have a known column associated to a creator user id."),
            web_common::database::Table::SampledIndividualsUsersRoleRequests => unreachable!("Table `sampled_individuals_users_role_requests` is not insertable as it does not have a known column associated to a creator user id."),
            web_common::database::Table::SampledIndividualsUsersRoles => unreachable!("Table `sampled_individuals_users_roles` is not insertable as it does not have a known column associated to a creator user id."),
            web_common::database::Table::Samples => {
                let row: web_common::database::NewSample = bincode::deserialize::<web_common::database::NewSample>(&row).map_err(web_common::api::ApiError::from)?;
                let inserted_row: crate::models::Sample = <web_common::database::NewSample as InsertRow>::insert(row, user_id, connection)?;
                let nested_row = crate::nested_models::NestedSample::from_flat(inserted_row, connection)?;
                 bincode::serialize(&nested_row).map_err(web_common::api::ApiError::from)?
            },
            web_common::database::Table::SamplesTeamsRoleInvitations => unreachable!("Table `samples_teams_role_invitations` is not insertable as it does not have a known column associated to a creator user id."),
            web_common::database::Table::SamplesTeamsRoleRequests => unreachable!("Table `samples_teams_role_requests` is not insertable as it does not have a known column associated to a creator user id."),
            web_common::database::Table::SamplesTeamsRoles => unreachable!("Table `samples_teams_roles` is not insertable as it does not have a known column associated to a creator user id."),
            web_common::database::Table::SamplesUsersRoleInvitations => unreachable!("Table `samples_users_role_invitations` is not insertable as it does not have a known column associated to a creator user id."),
            web_common::database::Table::SamplesUsersRoleRequests => unreachable!("Table `samples_users_role_requests` is not insertable as it does not have a known column associated to a creator user id."),
            web_common::database::Table::SamplesUsersRoles => unreachable!("Table `samples_users_roles` is not insertable as it does not have a known column associated to a creator user id."),
            web_common::database::Table::Spectra => unreachable!("Table `spectra` is not insertable as it does not have a known column associated to a creator user id."),
            web_common::database::Table::SpectraCollections => {
                let row: web_common::database::NewSpectraCollection = bincode::deserialize::<web_common::database::NewSpectraCollection>(&row).map_err(web_common::api::ApiError::from)?;
                let inserted_row: crate::models::SpectraCollection = <web_common::database::NewSpectraCollection as InsertRow>::insert(row, user_id, connection)?;
                let nested_row = crate::nested_models::NestedSpectraCollection::from_flat(inserted_row, connection)?;
                 bincode::serialize(&nested_row).map_err(web_common::api::ApiError::from)?
            },
            web_common::database::Table::SpectraCollectionsTeamsRoleInvitations => unreachable!("Table `spectra_collections_teams_role_invitations` is not insertable as it does not have a known column associated to a creator user id."),
            web_common::database::Table::SpectraCollectionsTeamsRoleRequests => unreachable!("Table `spectra_collections_teams_role_requests` is not insertable as it does not have a known column associated to a creator user id."),
            web_common::database::Table::SpectraCollectionsTeamsRoles => unreachable!("Table `spectra_collections_teams_roles` is not insertable as it does not have a known column associated to a creator user id."),
            web_common::database::Table::SpectraCollectionsUsersRoleInvitations => unreachable!("Table `spectra_collections_users_role_invitations` is not insertable as it does not have a known column associated to a creator user id."),
            web_common::database::Table::SpectraCollectionsUsersRoleRequests => unreachable!("Table `spectra_collections_users_role_requests` is not insertable as it does not have a known column associated to a creator user id."),
            web_common::database::Table::SpectraCollectionsUsersRoles => unreachable!("Table `spectra_collections_users_roles` is not insertable as it does not have a known column associated to a creator user id."),
            web_common::database::Table::TeamStates => unreachable!("Table `team_states` is not insertable as it does not have a known column associated to a creator user id."),
            web_common::database::Table::Teams => {
                let row: web_common::database::NewTeam = bincode::deserialize::<web_common::database::NewTeam>(&row).map_err(web_common::api::ApiError::from)?;
                let inserted_row: crate::models::Team = <web_common::database::NewTeam as InsertRow>::insert(row, user_id, connection)?;
                let nested_row = crate::nested_models::NestedTeam::from_flat(inserted_row, connection)?;
                 bincode::serialize(&nested_row).map_err(web_common::api::ApiError::from)?
            },
            web_common::database::Table::TeamsTeamsRoleInvitations => unreachable!("Table `teams_teams_role_invitations` is not insertable as it does not have a known column associated to a creator user id."),
            web_common::database::Table::TeamsUsersRoleInvitations => unreachable!("Table `teams_users_role_invitations` is not insertable as it does not have a known column associated to a creator user id."),
            web_common::database::Table::TeamsUsersRoleRequests => unreachable!("Table `teams_users_role_requests` is not insertable as it does not have a known column associated to a creator user id."),
            web_common::database::Table::TeamsUsersRoles => unreachable!("Table `teams_users_roles` is not insertable as it does not have a known column associated to a creator user id."),
            web_common::database::Table::Units => unreachable!("Table `units` is not insertable as it does not have a known column associated to a creator user id."),
            web_common::database::Table::UserEmails => {
                let row: web_common::database::NewUserEmail = bincode::deserialize::<web_common::database::NewUserEmail>(&row).map_err(web_common::api::ApiError::from)?;
                let inserted_row: crate::models::UserEmail = <web_common::database::NewUserEmail as InsertRow>::insert(row, user_id, connection)?;
                let nested_row = crate::nested_models::NestedUserEmail::from_flat(inserted_row, connection)?;
                 bincode::serialize(&nested_row).map_err(web_common::api::ApiError::from)?
            },
            web_common::database::Table::Users => {
                let row: web_common::database::NewUser = bincode::deserialize::<web_common::database::NewUser>(&row).map_err(web_common::api::ApiError::from)?;
                let inserted_row: crate::models::User = <web_common::database::NewUser as InsertRow>::insert(row, user_id, connection)?;
                 bincode::serialize(&inserted_row).map_err(web_common::api::ApiError::from)?
            },
            web_common::database::Table::UsersUsersRoleInvitations => unreachable!("Table `users_users_role_invitations` is not insertable as it does not have a known column associated to a creator user id."),
            web_common::database::Table::UsersUsersRoleRequests => unreachable!("Table `users_users_role_requests` is not insertable as it does not have a known column associated to a creator user id."),
            web_common::database::Table::UsersUsersRoles => unreachable!("Table `users_users_roles` is not insertable as it does not have a known column associated to a creator user id."),
})
    }
}
/// Trait providing the update method for the Table enum.
pub trait UpdatableTable {
    /// Update a row in the table.
    ///
    /// # Arguments
    /// * `row` - The bincode-serialized row of the table.
    /// * `user_id` - The id of the user updating the row.
    /// * `connection` - The database connection.
    ///
    /// # Returns
    /// The bincode-serialized row of the table.
    fn update(
         &self,
         row: Vec<u8>,
         user_id: i32,
         connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>
) -> Result<Vec<u8>, web_common::api::ApiError>;
}

impl UpdatableTable for web_common::database::Table {

    fn update(
        &self,
        row: Vec<u8>,
        user_id: i32,
        connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>
    ) -> Result<Vec<u8>, web_common::api::ApiError> {
        Ok(match self {
            web_common::database::Table::BioOttRanks => unreachable!("Table `bio_ott_ranks` is not updatable as it does not have a known column associated to an updater user id."),
            web_common::database::Table::BioOttTaxonItems => unreachable!("Table `bio_ott_taxon_items` is not updatable as it does not have a known column associated to an updater user id."),
            web_common::database::Table::Colors => unreachable!("Table `colors` is not updatable as it does not have a known column associated to an updater user id."),
            web_common::database::Table::Countries => unreachable!("Table `countries` is not updatable as it does not have a known column associated to an updater user id."),
            web_common::database::Table::DerivedSamples => unreachable!("Table `derived_samples` is not updatable as it does not have a known column associated to an updater user id."),
            web_common::database::Table::DocumentFormats => unreachable!("Table `document_formats` is not updatable as it does not have a known column associated to an updater user id."),
            web_common::database::Table::FontAwesomeIcons => unreachable!("Table `font_awesome_icons` is not updatable as it does not have a known column associated to an updater user id."),
            web_common::database::Table::LoginProviders => unreachable!("Table `login_providers` is not updatable as it does not have a known column associated to an updater user id."),
            web_common::database::Table::Notifications => unreachable!("Table `notifications` is not updatable as it does not have a known column associated to an updater user id."),
            web_common::database::Table::Organizations => unreachable!("Table `organizations` is not updatable as it does not have a known column associated to an updater user id."),
            web_common::database::Table::ProjectStates => unreachable!("Table `project_states` is not updatable as it does not have a known column associated to an updater user id."),
            web_common::database::Table::Projects => {
                let row: web_common::database::UpdateProject = bincode::deserialize::<web_common::database::UpdateProject>(&row).map_err(web_common::api::ApiError::from)?;
                let updated_row: crate::models::Project = <web_common::database::UpdateProject as UpdateRow>::update(row, user_id, connection)?;
                let nested_row = crate::nested_models::NestedProject::from_flat(updated_row, connection)?;
                 bincode::serialize(&nested_row).map_err(web_common::api::ApiError::from)?
            },
            web_common::database::Table::ProjectsTeamsRoleInvitations => unreachable!("Table `projects_teams_role_invitations` is not updatable as it does not have a known column associated to an updater user id."),
            web_common::database::Table::ProjectsTeamsRoleRequests => unreachable!("Table `projects_teams_role_requests` is not updatable as it does not have a known column associated to an updater user id."),
            web_common::database::Table::ProjectsTeamsRoles => unreachable!("Table `projects_teams_roles` is not updatable as it does not have a known column associated to an updater user id."),
            web_common::database::Table::ProjectsUsersRoleInvitations => unreachable!("Table `projects_users_role_invitations` is not updatable as it does not have a known column associated to an updater user id."),
            web_common::database::Table::ProjectsUsersRoleRequests => unreachable!("Table `projects_users_role_requests` is not updatable as it does not have a known column associated to an updater user id."),
            web_common::database::Table::ProjectsUsersRoles => unreachable!("Table `projects_users_roles` is not updatable as it does not have a known column associated to an updater user id."),
            web_common::database::Table::Roles => unreachable!("Table `roles` is not updatable as it does not have a known column associated to an updater user id."),
            web_common::database::Table::SampleBioOttTaxonItems => unreachable!("Table `sample_bio_ott_taxon_items` is not updatable as it does not have a known column associated to an updater user id."),
            web_common::database::Table::SampleStates => unreachable!("Table `sample_states` is not updatable as it does not have a known column associated to an updater user id."),
            web_common::database::Table::SampledIndividualBioOttTaxonItems => unreachable!("Table `sampled_individual_bio_ott_taxon_items` is not updatable as it does not have a known column associated to an updater user id."),
            web_common::database::Table::SampledIndividuals => {
                let row: web_common::database::NewSampledIndividual = bincode::deserialize::<web_common::database::NewSampledIndividual>(&row).map_err(web_common::api::ApiError::from)?;
                let updated_row: crate::models::SampledIndividual = <web_common::database::NewSampledIndividual as UpdateRow>::update(row, user_id, connection)?;
                let nested_row = crate::nested_models::NestedSampledIndividual::from_flat(updated_row, connection)?;
                 bincode::serialize(&nested_row).map_err(web_common::api::ApiError::from)?
            },
            web_common::database::Table::SampledIndividualsTeamsRoleInvitations => unreachable!("Table `sampled_individuals_teams_role_invitations` is not updatable as it does not have a known column associated to an updater user id."),
            web_common::database::Table::SampledIndividualsTeamsRoleRequests => unreachable!("Table `sampled_individuals_teams_role_requests` is not updatable as it does not have a known column associated to an updater user id."),
            web_common::database::Table::SampledIndividualsTeamsRoles => unreachable!("Table `sampled_individuals_teams_roles` is not updatable as it does not have a known column associated to an updater user id."),
            web_common::database::Table::SampledIndividualsUsersRoleInvitations => unreachable!("Table `sampled_individuals_users_role_invitations` is not updatable as it does not have a known column associated to an updater user id."),
            web_common::database::Table::SampledIndividualsUsersRoleRequests => unreachable!("Table `sampled_individuals_users_role_requests` is not updatable as it does not have a known column associated to an updater user id."),
            web_common::database::Table::SampledIndividualsUsersRoles => unreachable!("Table `sampled_individuals_users_roles` is not updatable as it does not have a known column associated to an updater user id."),
            web_common::database::Table::Samples => {
                let row: web_common::database::NewSample = bincode::deserialize::<web_common::database::NewSample>(&row).map_err(web_common::api::ApiError::from)?;
                let updated_row: crate::models::Sample = <web_common::database::NewSample as UpdateRow>::update(row, user_id, connection)?;
                let nested_row = crate::nested_models::NestedSample::from_flat(updated_row, connection)?;
                 bincode::serialize(&nested_row).map_err(web_common::api::ApiError::from)?
            },
            web_common::database::Table::SamplesTeamsRoleInvitations => unreachable!("Table `samples_teams_role_invitations` is not updatable as it does not have a known column associated to an updater user id."),
            web_common::database::Table::SamplesTeamsRoleRequests => unreachable!("Table `samples_teams_role_requests` is not updatable as it does not have a known column associated to an updater user id."),
            web_common::database::Table::SamplesTeamsRoles => unreachable!("Table `samples_teams_roles` is not updatable as it does not have a known column associated to an updater user id."),
            web_common::database::Table::SamplesUsersRoleInvitations => unreachable!("Table `samples_users_role_invitations` is not updatable as it does not have a known column associated to an updater user id."),
            web_common::database::Table::SamplesUsersRoleRequests => unreachable!("Table `samples_users_role_requests` is not updatable as it does not have a known column associated to an updater user id."),
            web_common::database::Table::SamplesUsersRoles => unreachable!("Table `samples_users_roles` is not updatable as it does not have a known column associated to an updater user id."),
            web_common::database::Table::Spectra => unreachable!("Table `spectra` is not updatable as it does not have a known column associated to an updater user id."),
            web_common::database::Table::SpectraCollections => {
                let row: web_common::database::UpdateSpectraCollection = bincode::deserialize::<web_common::database::UpdateSpectraCollection>(&row).map_err(web_common::api::ApiError::from)?;
                let updated_row: crate::models::SpectraCollection = <web_common::database::UpdateSpectraCollection as UpdateRow>::update(row, user_id, connection)?;
                let nested_row = crate::nested_models::NestedSpectraCollection::from_flat(updated_row, connection)?;
                 bincode::serialize(&nested_row).map_err(web_common::api::ApiError::from)?
            },
            web_common::database::Table::SpectraCollectionsTeamsRoleInvitations => unreachable!("Table `spectra_collections_teams_role_invitations` is not updatable as it does not have a known column associated to an updater user id."),
            web_common::database::Table::SpectraCollectionsTeamsRoleRequests => unreachable!("Table `spectra_collections_teams_role_requests` is not updatable as it does not have a known column associated to an updater user id."),
            web_common::database::Table::SpectraCollectionsTeamsRoles => unreachable!("Table `spectra_collections_teams_roles` is not updatable as it does not have a known column associated to an updater user id."),
            web_common::database::Table::SpectraCollectionsUsersRoleInvitations => unreachable!("Table `spectra_collections_users_role_invitations` is not updatable as it does not have a known column associated to an updater user id."),
            web_common::database::Table::SpectraCollectionsUsersRoleRequests => unreachable!("Table `spectra_collections_users_role_requests` is not updatable as it does not have a known column associated to an updater user id."),
            web_common::database::Table::SpectraCollectionsUsersRoles => unreachable!("Table `spectra_collections_users_roles` is not updatable as it does not have a known column associated to an updater user id."),
            web_common::database::Table::TeamStates => unreachable!("Table `team_states` is not updatable as it does not have a known column associated to an updater user id."),
            web_common::database::Table::Teams => {
                let row: web_common::database::UpdateTeam = bincode::deserialize::<web_common::database::UpdateTeam>(&row).map_err(web_common::api::ApiError::from)?;
                let updated_row: crate::models::Team = <web_common::database::UpdateTeam as UpdateRow>::update(row, user_id, connection)?;
                let nested_row = crate::nested_models::NestedTeam::from_flat(updated_row, connection)?;
                 bincode::serialize(&nested_row).map_err(web_common::api::ApiError::from)?
            },
            web_common::database::Table::TeamsTeamsRoleInvitations => unreachable!("Table `teams_teams_role_invitations` is not updatable as it does not have a known column associated to an updater user id."),
            web_common::database::Table::TeamsUsersRoleInvitations => unreachable!("Table `teams_users_role_invitations` is not updatable as it does not have a known column associated to an updater user id."),
            web_common::database::Table::TeamsUsersRoleRequests => unreachable!("Table `teams_users_role_requests` is not updatable as it does not have a known column associated to an updater user id."),
            web_common::database::Table::TeamsUsersRoles => unreachable!("Table `teams_users_roles` is not updatable as it does not have a known column associated to an updater user id."),
            web_common::database::Table::Units => unreachable!("Table `units` is not updatable as it does not have a known column associated to an updater user id."),
            web_common::database::Table::UserEmails => unreachable!("Table `user_emails` is not updatable as it does not have a known column associated to an updater user id."),
            web_common::database::Table::Users => {
                let row: web_common::database::UpdateUser = bincode::deserialize::<web_common::database::UpdateUser>(&row).map_err(web_common::api::ApiError::from)?;
                let updated_row: crate::models::User = <web_common::database::UpdateUser as UpdateRow>::update(row, user_id, connection)?;
                 bincode::serialize(&updated_row).map_err(web_common::api::ApiError::from)?
            },
            web_common::database::Table::UsersUsersRoleInvitations => unreachable!("Table `users_users_role_invitations` is not updatable as it does not have a known column associated to an updater user id."),
            web_common::database::Table::UsersUsersRoleRequests => unreachable!("Table `users_users_role_requests` is not updatable as it does not have a known column associated to an updater user id."),
            web_common::database::Table::UsersUsersRoles => unreachable!("Table `users_users_roles` is not updatable as it does not have a known column associated to an updater user id."),
})
    }
}
/// Trait providing the from_flat_str method for the Table enum.
pub trait FromFlatStrTable {
    /// Convert a JSON serialized row of the flat variant of the table to the richest struct.
    ///
    /// # Arguments
    /// * `row` - The JSON serialized row of the table.
    /// * `connection` - The database connection.
    ///
    /// # Returns
    /// The bincode-serialized row of the table.
    fn from_flat_str(
         &self,
         row: &str,
         connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>
) -> Result<Vec<u8>, web_common::api::ApiError>;
}

impl FromFlatStrTable for web_common::database::Table {

    fn from_flat_str(
        &self,
        row: &str,
        connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>
    ) -> Result<Vec<u8>, web_common::api::ApiError> {
        Ok(match self {
            web_common::database::Table::BioOttRanks => {
                let flat_row: crate::models::BioOttRank = serde_json::from_str::<crate::models::BioOttRank>(row).map_err(web_common::api::ApiError::from)?;
                let richest_row = crate::nested_models::NestedBioOttRank::from_flat(flat_row, connection)?;
                 bincode::serialize(&richest_row).map_err(web_common::api::ApiError::from)?
            },
            web_common::database::Table::BioOttTaxonItems => {
                let flat_row: crate::models::BioOttTaxonItem = serde_json::from_str::<crate::models::BioOttTaxonItem>(row).map_err(web_common::api::ApiError::from)?;
                let richest_row = crate::nested_models::NestedBioOttTaxonItem::from_flat(flat_row, connection)?;
                 bincode::serialize(&richest_row).map_err(web_common::api::ApiError::from)?
            },
            web_common::database::Table::Colors => bincode::serialize(&serde_json::from_str::<crate::models::Color>(row).map_err(web_common::api::ApiError::from)?).map_err(web_common::api::ApiError::from)?,
            web_common::database::Table::Countries => bincode::serialize(&serde_json::from_str::<crate::models::Country>(row).map_err(web_common::api::ApiError::from)?).map_err(web_common::api::ApiError::from)?,
            web_common::database::Table::DerivedSamples => {
                let flat_row: crate::models::DerivedSample = serde_json::from_str::<crate::models::DerivedSample>(row).map_err(web_common::api::ApiError::from)?;
                let richest_row = crate::nested_models::NestedDerivedSample::from_flat(flat_row, connection)?;
                 bincode::serialize(&richest_row).map_err(web_common::api::ApiError::from)?
            },
            web_common::database::Table::DocumentFormats => {
                let flat_row: crate::models::DocumentFormat = serde_json::from_str::<crate::models::DocumentFormat>(row).map_err(web_common::api::ApiError::from)?;
                let richest_row = crate::nested_models::NestedDocumentFormat::from_flat(flat_row, connection)?;
                 bincode::serialize(&richest_row).map_err(web_common::api::ApiError::from)?
            },
            web_common::database::Table::FontAwesomeIcons => bincode::serialize(&serde_json::from_str::<crate::models::FontAwesomeIcon>(row).map_err(web_common::api::ApiError::from)?).map_err(web_common::api::ApiError::from)?,
            web_common::database::Table::LoginProviders => {
                let flat_row: crate::models::LoginProvider = serde_json::from_str::<crate::models::LoginProvider>(row).map_err(web_common::api::ApiError::from)?;
                let richest_row = crate::nested_models::NestedLoginProvider::from_flat(flat_row, connection)?;
                 bincode::serialize(&richest_row).map_err(web_common::api::ApiError::from)?
            },
            web_common::database::Table::Notifications => {
                let flat_row: crate::models::Notification = serde_json::from_str::<crate::models::Notification>(row).map_err(web_common::api::ApiError::from)?;
                let richest_row = crate::nested_models::NestedNotification::from_flat(flat_row, connection)?;
                 bincode::serialize(&richest_row).map_err(web_common::api::ApiError::from)?
            },
            web_common::database::Table::Organizations => {
                let flat_row: crate::models::Organization = serde_json::from_str::<crate::models::Organization>(row).map_err(web_common::api::ApiError::from)?;
                let richest_row = crate::nested_models::NestedOrganization::from_flat(flat_row, connection)?;
                 bincode::serialize(&richest_row).map_err(web_common::api::ApiError::from)?
            },
            web_common::database::Table::ProjectStates => {
                let flat_row: crate::models::ProjectState = serde_json::from_str::<crate::models::ProjectState>(row).map_err(web_common::api::ApiError::from)?;
                let richest_row = crate::nested_models::NestedProjectState::from_flat(flat_row, connection)?;
                 bincode::serialize(&richest_row).map_err(web_common::api::ApiError::from)?
            },
            web_common::database::Table::Projects => {
                let flat_row: crate::models::Project = serde_json::from_str::<crate::models::Project>(row).map_err(web_common::api::ApiError::from)?;
                let richest_row = crate::nested_models::NestedProject::from_flat(flat_row, connection)?;
                 bincode::serialize(&richest_row).map_err(web_common::api::ApiError::from)?
            },
            web_common::database::Table::ProjectsTeamsRoleInvitations => {
                let flat_row: crate::models::ProjectsTeamsRoleInvitation = serde_json::from_str::<crate::models::ProjectsTeamsRoleInvitation>(row).map_err(web_common::api::ApiError::from)?;
                let richest_row = crate::nested_models::NestedProjectsTeamsRoleInvitation::from_flat(flat_row, connection)?;
                 bincode::serialize(&richest_row).map_err(web_common::api::ApiError::from)?
            },
            web_common::database::Table::ProjectsTeamsRoleRequests => {
                let flat_row: crate::models::ProjectsTeamsRoleRequest = serde_json::from_str::<crate::models::ProjectsTeamsRoleRequest>(row).map_err(web_common::api::ApiError::from)?;
                let richest_row = crate::nested_models::NestedProjectsTeamsRoleRequest::from_flat(flat_row, connection)?;
                 bincode::serialize(&richest_row).map_err(web_common::api::ApiError::from)?
            },
            web_common::database::Table::ProjectsTeamsRoles => {
                let flat_row: crate::models::ProjectsTeamsRole = serde_json::from_str::<crate::models::ProjectsTeamsRole>(row).map_err(web_common::api::ApiError::from)?;
                let richest_row = crate::nested_models::NestedProjectsTeamsRole::from_flat(flat_row, connection)?;
                 bincode::serialize(&richest_row).map_err(web_common::api::ApiError::from)?
            },
            web_common::database::Table::ProjectsUsersRoleInvitations => {
                let flat_row: crate::models::ProjectsUsersRoleInvitation = serde_json::from_str::<crate::models::ProjectsUsersRoleInvitation>(row).map_err(web_common::api::ApiError::from)?;
                let richest_row = crate::nested_models::NestedProjectsUsersRoleInvitation::from_flat(flat_row, connection)?;
                 bincode::serialize(&richest_row).map_err(web_common::api::ApiError::from)?
            },
            web_common::database::Table::ProjectsUsersRoleRequests => {
                let flat_row: crate::models::ProjectsUsersRoleRequest = serde_json::from_str::<crate::models::ProjectsUsersRoleRequest>(row).map_err(web_common::api::ApiError::from)?;
                let richest_row = crate::nested_models::NestedProjectsUsersRoleRequest::from_flat(flat_row, connection)?;
                 bincode::serialize(&richest_row).map_err(web_common::api::ApiError::from)?
            },
            web_common::database::Table::ProjectsUsersRoles => {
                let flat_row: crate::models::ProjectsUsersRole = serde_json::from_str::<crate::models::ProjectsUsersRole>(row).map_err(web_common::api::ApiError::from)?;
                let richest_row = crate::nested_models::NestedProjectsUsersRole::from_flat(flat_row, connection)?;
                 bincode::serialize(&richest_row).map_err(web_common::api::ApiError::from)?
            },
            web_common::database::Table::Roles => {
                let flat_row: crate::models::Role = serde_json::from_str::<crate::models::Role>(row).map_err(web_common::api::ApiError::from)?;
                let richest_row = crate::nested_models::NestedRole::from_flat(flat_row, connection)?;
                 bincode::serialize(&richest_row).map_err(web_common::api::ApiError::from)?
            },
            web_common::database::Table::SampleBioOttTaxonItems => {
                let flat_row: crate::models::SampleBioOttTaxonItem = serde_json::from_str::<crate::models::SampleBioOttTaxonItem>(row).map_err(web_common::api::ApiError::from)?;
                let richest_row = crate::nested_models::NestedSampleBioOttTaxonItem::from_flat(flat_row, connection)?;
                 bincode::serialize(&richest_row).map_err(web_common::api::ApiError::from)?
            },
            web_common::database::Table::SampleStates => {
                let flat_row: crate::models::SampleState = serde_json::from_str::<crate::models::SampleState>(row).map_err(web_common::api::ApiError::from)?;
                let richest_row = crate::nested_models::NestedSampleState::from_flat(flat_row, connection)?;
                 bincode::serialize(&richest_row).map_err(web_common::api::ApiError::from)?
            },
            web_common::database::Table::SampledIndividualBioOttTaxonItems => {
                let flat_row: crate::models::SampledIndividualBioOttTaxonItem = serde_json::from_str::<crate::models::SampledIndividualBioOttTaxonItem>(row).map_err(web_common::api::ApiError::from)?;
                let richest_row = crate::nested_models::NestedSampledIndividualBioOttTaxonItem::from_flat(flat_row, connection)?;
                 bincode::serialize(&richest_row).map_err(web_common::api::ApiError::from)?
            },
            web_common::database::Table::SampledIndividuals => {
                let flat_row: crate::models::SampledIndividual = serde_json::from_str::<crate::models::SampledIndividual>(row).map_err(web_common::api::ApiError::from)?;
                let richest_row = crate::nested_models::NestedSampledIndividual::from_flat(flat_row, connection)?;
                 bincode::serialize(&richest_row).map_err(web_common::api::ApiError::from)?
            },
            web_common::database::Table::SampledIndividualsTeamsRoleInvitations => {
                let flat_row: crate::models::SampledIndividualsTeamsRoleInvitation = serde_json::from_str::<crate::models::SampledIndividualsTeamsRoleInvitation>(row).map_err(web_common::api::ApiError::from)?;
                let richest_row = crate::nested_models::NestedSampledIndividualsTeamsRoleInvitation::from_flat(flat_row, connection)?;
                 bincode::serialize(&richest_row).map_err(web_common::api::ApiError::from)?
            },
            web_common::database::Table::SampledIndividualsTeamsRoleRequests => {
                let flat_row: crate::models::SampledIndividualsTeamsRoleRequest = serde_json::from_str::<crate::models::SampledIndividualsTeamsRoleRequest>(row).map_err(web_common::api::ApiError::from)?;
                let richest_row = crate::nested_models::NestedSampledIndividualsTeamsRoleRequest::from_flat(flat_row, connection)?;
                 bincode::serialize(&richest_row).map_err(web_common::api::ApiError::from)?
            },
            web_common::database::Table::SampledIndividualsTeamsRoles => {
                let flat_row: crate::models::SampledIndividualsTeamsRole = serde_json::from_str::<crate::models::SampledIndividualsTeamsRole>(row).map_err(web_common::api::ApiError::from)?;
                let richest_row = crate::nested_models::NestedSampledIndividualsTeamsRole::from_flat(flat_row, connection)?;
                 bincode::serialize(&richest_row).map_err(web_common::api::ApiError::from)?
            },
            web_common::database::Table::SampledIndividualsUsersRoleInvitations => {
                let flat_row: crate::models::SampledIndividualsUsersRoleInvitation = serde_json::from_str::<crate::models::SampledIndividualsUsersRoleInvitation>(row).map_err(web_common::api::ApiError::from)?;
                let richest_row = crate::nested_models::NestedSampledIndividualsUsersRoleInvitation::from_flat(flat_row, connection)?;
                 bincode::serialize(&richest_row).map_err(web_common::api::ApiError::from)?
            },
            web_common::database::Table::SampledIndividualsUsersRoleRequests => {
                let flat_row: crate::models::SampledIndividualsUsersRoleRequest = serde_json::from_str::<crate::models::SampledIndividualsUsersRoleRequest>(row).map_err(web_common::api::ApiError::from)?;
                let richest_row = crate::nested_models::NestedSampledIndividualsUsersRoleRequest::from_flat(flat_row, connection)?;
                 bincode::serialize(&richest_row).map_err(web_common::api::ApiError::from)?
            },
            web_common::database::Table::SampledIndividualsUsersRoles => {
                let flat_row: crate::models::SampledIndividualsUsersRole = serde_json::from_str::<crate::models::SampledIndividualsUsersRole>(row).map_err(web_common::api::ApiError::from)?;
                let richest_row = crate::nested_models::NestedSampledIndividualsUsersRole::from_flat(flat_row, connection)?;
                 bincode::serialize(&richest_row).map_err(web_common::api::ApiError::from)?
            },
            web_common::database::Table::Samples => {
                let flat_row: crate::models::Sample = serde_json::from_str::<crate::models::Sample>(row).map_err(web_common::api::ApiError::from)?;
                let richest_row = crate::nested_models::NestedSample::from_flat(flat_row, connection)?;
                 bincode::serialize(&richest_row).map_err(web_common::api::ApiError::from)?
            },
            web_common::database::Table::SamplesTeamsRoleInvitations => {
                let flat_row: crate::models::SamplesTeamsRoleInvitation = serde_json::from_str::<crate::models::SamplesTeamsRoleInvitation>(row).map_err(web_common::api::ApiError::from)?;
                let richest_row = crate::nested_models::NestedSamplesTeamsRoleInvitation::from_flat(flat_row, connection)?;
                 bincode::serialize(&richest_row).map_err(web_common::api::ApiError::from)?
            },
            web_common::database::Table::SamplesTeamsRoleRequests => {
                let flat_row: crate::models::SamplesTeamsRoleRequest = serde_json::from_str::<crate::models::SamplesTeamsRoleRequest>(row).map_err(web_common::api::ApiError::from)?;
                let richest_row = crate::nested_models::NestedSamplesTeamsRoleRequest::from_flat(flat_row, connection)?;
                 bincode::serialize(&richest_row).map_err(web_common::api::ApiError::from)?
            },
            web_common::database::Table::SamplesTeamsRoles => {
                let flat_row: crate::models::SamplesTeamsRole = serde_json::from_str::<crate::models::SamplesTeamsRole>(row).map_err(web_common::api::ApiError::from)?;
                let richest_row = crate::nested_models::NestedSamplesTeamsRole::from_flat(flat_row, connection)?;
                 bincode::serialize(&richest_row).map_err(web_common::api::ApiError::from)?
            },
            web_common::database::Table::SamplesUsersRoleInvitations => {
                let flat_row: crate::models::SamplesUsersRoleInvitation = serde_json::from_str::<crate::models::SamplesUsersRoleInvitation>(row).map_err(web_common::api::ApiError::from)?;
                let richest_row = crate::nested_models::NestedSamplesUsersRoleInvitation::from_flat(flat_row, connection)?;
                 bincode::serialize(&richest_row).map_err(web_common::api::ApiError::from)?
            },
            web_common::database::Table::SamplesUsersRoleRequests => {
                let flat_row: crate::models::SamplesUsersRoleRequest = serde_json::from_str::<crate::models::SamplesUsersRoleRequest>(row).map_err(web_common::api::ApiError::from)?;
                let richest_row = crate::nested_models::NestedSamplesUsersRoleRequest::from_flat(flat_row, connection)?;
                 bincode::serialize(&richest_row).map_err(web_common::api::ApiError::from)?
            },
            web_common::database::Table::SamplesUsersRoles => {
                let flat_row: crate::models::SamplesUsersRole = serde_json::from_str::<crate::models::SamplesUsersRole>(row).map_err(web_common::api::ApiError::from)?;
                let richest_row = crate::nested_models::NestedSamplesUsersRole::from_flat(flat_row, connection)?;
                 bincode::serialize(&richest_row).map_err(web_common::api::ApiError::from)?
            },
            web_common::database::Table::Spectra => {
                let flat_row: crate::models::Spectra = serde_json::from_str::<crate::models::Spectra>(row).map_err(web_common::api::ApiError::from)?;
                let richest_row = crate::nested_models::NestedSpectra::from_flat(flat_row, connection)?;
                 bincode::serialize(&richest_row).map_err(web_common::api::ApiError::from)?
            },
            web_common::database::Table::SpectraCollections => {
                let flat_row: crate::models::SpectraCollection = serde_json::from_str::<crate::models::SpectraCollection>(row).map_err(web_common::api::ApiError::from)?;
                let richest_row = crate::nested_models::NestedSpectraCollection::from_flat(flat_row, connection)?;
                 bincode::serialize(&richest_row).map_err(web_common::api::ApiError::from)?
            },
            web_common::database::Table::SpectraCollectionsTeamsRoleInvitations => {
                let flat_row: crate::models::SpectraCollectionsTeamsRoleInvitation = serde_json::from_str::<crate::models::SpectraCollectionsTeamsRoleInvitation>(row).map_err(web_common::api::ApiError::from)?;
                let richest_row = crate::nested_models::NestedSpectraCollectionsTeamsRoleInvitation::from_flat(flat_row, connection)?;
                 bincode::serialize(&richest_row).map_err(web_common::api::ApiError::from)?
            },
            web_common::database::Table::SpectraCollectionsTeamsRoleRequests => {
                let flat_row: crate::models::SpectraCollectionsTeamsRoleRequest = serde_json::from_str::<crate::models::SpectraCollectionsTeamsRoleRequest>(row).map_err(web_common::api::ApiError::from)?;
                let richest_row = crate::nested_models::NestedSpectraCollectionsTeamsRoleRequest::from_flat(flat_row, connection)?;
                 bincode::serialize(&richest_row).map_err(web_common::api::ApiError::from)?
            },
            web_common::database::Table::SpectraCollectionsTeamsRoles => {
                let flat_row: crate::models::SpectraCollectionsTeamsRole = serde_json::from_str::<crate::models::SpectraCollectionsTeamsRole>(row).map_err(web_common::api::ApiError::from)?;
                let richest_row = crate::nested_models::NestedSpectraCollectionsTeamsRole::from_flat(flat_row, connection)?;
                 bincode::serialize(&richest_row).map_err(web_common::api::ApiError::from)?
            },
            web_common::database::Table::SpectraCollectionsUsersRoleInvitations => {
                let flat_row: crate::models::SpectraCollectionsUsersRoleInvitation = serde_json::from_str::<crate::models::SpectraCollectionsUsersRoleInvitation>(row).map_err(web_common::api::ApiError::from)?;
                let richest_row = crate::nested_models::NestedSpectraCollectionsUsersRoleInvitation::from_flat(flat_row, connection)?;
                 bincode::serialize(&richest_row).map_err(web_common::api::ApiError::from)?
            },
            web_common::database::Table::SpectraCollectionsUsersRoleRequests => {
                let flat_row: crate::models::SpectraCollectionsUsersRoleRequest = serde_json::from_str::<crate::models::SpectraCollectionsUsersRoleRequest>(row).map_err(web_common::api::ApiError::from)?;
                let richest_row = crate::nested_models::NestedSpectraCollectionsUsersRoleRequest::from_flat(flat_row, connection)?;
                 bincode::serialize(&richest_row).map_err(web_common::api::ApiError::from)?
            },
            web_common::database::Table::SpectraCollectionsUsersRoles => {
                let flat_row: crate::models::SpectraCollectionsUsersRole = serde_json::from_str::<crate::models::SpectraCollectionsUsersRole>(row).map_err(web_common::api::ApiError::from)?;
                let richest_row = crate::nested_models::NestedSpectraCollectionsUsersRole::from_flat(flat_row, connection)?;
                 bincode::serialize(&richest_row).map_err(web_common::api::ApiError::from)?
            },
            web_common::database::Table::TeamStates => {
                let flat_row: crate::models::TeamState = serde_json::from_str::<crate::models::TeamState>(row).map_err(web_common::api::ApiError::from)?;
                let richest_row = crate::nested_models::NestedTeamState::from_flat(flat_row, connection)?;
                 bincode::serialize(&richest_row).map_err(web_common::api::ApiError::from)?
            },
            web_common::database::Table::Teams => {
                let flat_row: crate::models::Team = serde_json::from_str::<crate::models::Team>(row).map_err(web_common::api::ApiError::from)?;
                let richest_row = crate::nested_models::NestedTeam::from_flat(flat_row, connection)?;
                 bincode::serialize(&richest_row).map_err(web_common::api::ApiError::from)?
            },
            web_common::database::Table::TeamsTeamsRoleInvitations => {
                let flat_row: crate::models::TeamsTeamsRoleInvitation = serde_json::from_str::<crate::models::TeamsTeamsRoleInvitation>(row).map_err(web_common::api::ApiError::from)?;
                let richest_row = crate::nested_models::NestedTeamsTeamsRoleInvitation::from_flat(flat_row, connection)?;
                 bincode::serialize(&richest_row).map_err(web_common::api::ApiError::from)?
            },
            web_common::database::Table::TeamsUsersRoleInvitations => {
                let flat_row: crate::models::TeamsUsersRoleInvitation = serde_json::from_str::<crate::models::TeamsUsersRoleInvitation>(row).map_err(web_common::api::ApiError::from)?;
                let richest_row = crate::nested_models::NestedTeamsUsersRoleInvitation::from_flat(flat_row, connection)?;
                 bincode::serialize(&richest_row).map_err(web_common::api::ApiError::from)?
            },
            web_common::database::Table::TeamsUsersRoleRequests => {
                let flat_row: crate::models::TeamsUsersRoleRequest = serde_json::from_str::<crate::models::TeamsUsersRoleRequest>(row).map_err(web_common::api::ApiError::from)?;
                let richest_row = crate::nested_models::NestedTeamsUsersRoleRequest::from_flat(flat_row, connection)?;
                 bincode::serialize(&richest_row).map_err(web_common::api::ApiError::from)?
            },
            web_common::database::Table::TeamsUsersRoles => {
                let flat_row: crate::models::TeamsUsersRole = serde_json::from_str::<crate::models::TeamsUsersRole>(row).map_err(web_common::api::ApiError::from)?;
                let richest_row = crate::nested_models::NestedTeamsUsersRole::from_flat(flat_row, connection)?;
                 bincode::serialize(&richest_row).map_err(web_common::api::ApiError::from)?
            },
            web_common::database::Table::Units => bincode::serialize(&serde_json::from_str::<crate::models::Unit>(row).map_err(web_common::api::ApiError::from)?).map_err(web_common::api::ApiError::from)?,
            web_common::database::Table::UserEmails => {
                let flat_row: crate::models::UserEmail = serde_json::from_str::<crate::models::UserEmail>(row).map_err(web_common::api::ApiError::from)?;
                let richest_row = crate::nested_models::NestedUserEmail::from_flat(flat_row, connection)?;
                 bincode::serialize(&richest_row).map_err(web_common::api::ApiError::from)?
            },
            web_common::database::Table::Users => bincode::serialize(&serde_json::from_str::<crate::models::User>(row).map_err(web_common::api::ApiError::from)?).map_err(web_common::api::ApiError::from)?,
            web_common::database::Table::UsersUsersRoleInvitations => {
                let flat_row: crate::models::UsersUsersRoleInvitation = serde_json::from_str::<crate::models::UsersUsersRoleInvitation>(row).map_err(web_common::api::ApiError::from)?;
                let richest_row = crate::nested_models::NestedUsersUsersRoleInvitation::from_flat(flat_row, connection)?;
                 bincode::serialize(&richest_row).map_err(web_common::api::ApiError::from)?
            },
            web_common::database::Table::UsersUsersRoleRequests => {
                let flat_row: crate::models::UsersUsersRoleRequest = serde_json::from_str::<crate::models::UsersUsersRoleRequest>(row).map_err(web_common::api::ApiError::from)?;
                let richest_row = crate::nested_models::NestedUsersUsersRoleRequest::from_flat(flat_row, connection)?;
                 bincode::serialize(&richest_row).map_err(web_common::api::ApiError::from)?
            },
            web_common::database::Table::UsersUsersRoles => {
                let flat_row: crate::models::UsersUsersRole = serde_json::from_str::<crate::models::UsersUsersRole>(row).map_err(web_common::api::ApiError::from)?;
                let richest_row = crate::nested_models::NestedUsersUsersRole::from_flat(flat_row, connection)?;
                 bincode::serialize(&richest_row).map_err(web_common::api::ApiError::from)?
            },
        })
    }
}
