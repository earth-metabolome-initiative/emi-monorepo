//! This module contains the table names enumeration.
//!
//! This module is automatically generated. Do not write anything here.

use crate::models::*;
use crate::nested_models::*;
use web_common::database::filter_structs::*;
use web_common::database::PrimaryKey;
use diesel::r2d2::PooledConnection;
use diesel::r2d2::ConnectionManager;
use crate::new_variants::InsertRow;
use crate::update_variants::UpdateRow;

/// Trait providing the backend implementations for the Table enumeration
pub trait BackendTable {
    /// Check whether the user can view the struct associated to the provided ids.
    ///
    /// * `primary_key` - The primary key(s) of the struct to check.
    /// * `author_user_id` - The ID of the user to check.
    /// * `connection` - The connection to the database.
    ///
     fn can_view_by_id(
        &self,
primary_key: PrimaryKey,
author_user_id: Option<i32>,
connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>,
) -> Result<bool, web_common::api::ApiError>;

    /// Get all of the viewable structs from the database.
    ///
    /// * `filter` - The optional filter to apply to the query.
    /// * `author_user_id` - The ID of the user who is performing the search.
    /// * `limit` - The maximum number of results to return.
    /// * `offset` - The number of results to skip.
    /// * `connection` - The connection to the database.
    ///
     fn all_viewable(
        &self,
filter: Option<Vec<u8>>,
author_user_id: Option<i32>,
limit: Option<i64>,
offset: Option<i64>,
connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>,
) -> Result<Vec<u8>, web_common::api::ApiError>;

    /// Get all of the sorted viewable structs from the database.
    ///
    /// * `filter` - The optional filter to apply to the query.
    /// * `author_user_id` - The ID of the user who is performing the search.
    /// * `limit` - The maximum number of results to return.
    /// * `offset` - The number of results to skip.
    /// * `connection` - The connection to the database.
    ///
     fn all_viewable_sorted(
        &self,
filter: Option<Vec<u8>>,
author_user_id: Option<i32>,
limit: Option<i64>,
offset: Option<i64>,
connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>,
) -> Result<Vec<u8>, web_common::api::ApiError>;

    /// Get the struct from the database by its ID.
    ///
    /// * `primary_key` - The primary key(s) of the struct to get.
    /// * `author_user_id` - The ID of the user who is performing the search.
    /// * `connection` - The connection to the database.
    ///
     fn get(
        &self,
primary_key: PrimaryKey,
author_user_id: Option<i32>,
connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>,
) -> Result<Vec<u8>, web_common::api::ApiError>;

    /// Search for the viewable structs by a given string by Postgres's `similarity`.
    ///
    /// * `filter` - The optional filter to apply to the query.
    /// * `author_user_id` - The ID of the user who is performing the search.
    /// * `query` - The string to search for.
    /// * `limit` - The maximum number of results to return.
    /// * `offset` - The number of results to skip.
    /// * `connection` - The connection to the database.
    ///
     fn similarity_search_viewable(
        &self,
filter: Option<Vec<u8>>,
author_user_id: Option<i32>,
query: &str,
limit: Option<i64>,
offset: Option<i64>,
connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>,
) -> Result<Vec<u8>, web_common::api::ApiError>;

    /// Search for the viewable structs by a given string by Postgres's `word_similarity`.
    ///
    /// * `filter` - The optional filter to apply to the query.
    /// * `author_user_id` - The ID of the user who is performing the search.
    /// * `query` - The string to search for.
    /// * `limit` - The maximum number of results to return.
    /// * `offset` - The number of results to skip.
    /// * `connection` - The connection to the database.
    ///
     fn word_similarity_search_viewable(
        &self,
filter: Option<Vec<u8>>,
author_user_id: Option<i32>,
query: &str,
limit: Option<i64>,
offset: Option<i64>,
connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>,
) -> Result<Vec<u8>, web_common::api::ApiError>;

    /// Search for the viewable structs by a given string by Postgres's `strict_word_similarity`.
    ///
    /// * `filter` - The optional filter to apply to the query.
    /// * `author_user_id` - The ID of the user who is performing the search.
    /// * `query` - The string to search for.
    /// * `limit` - The maximum number of results to return.
    /// * `offset` - The number of results to skip.
    /// * `connection` - The connection to the database.
    ///
     fn strict_word_similarity_search_viewable(
        &self,
filter: Option<Vec<u8>>,
author_user_id: Option<i32>,
query: &str,
limit: Option<i64>,
offset: Option<i64>,
connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>,
) -> Result<Vec<u8>, web_common::api::ApiError>;

    /// Check whether the user can update the struct associated to the provided ids.
    ///
    /// * `primary_key` - The primary key(s) of the struct to check.
    /// * `author_user_id` - The ID of the user to check.
    /// * `connection` - The connection to the database.
    ///
     fn can_update_by_id(
        &self,
primary_key: PrimaryKey,
author_user_id: i32,
connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>,
) -> Result<bool, web_common::api::ApiError>;

    /// Get all of the updatable structs from the database.
    ///
    /// * `filter` - The optional filter to apply to the query.
    /// * `author_user_id` - The ID of the user who is performing the search.
    /// * `limit` - The maximum number of results to return.
    /// * `offset` - The number of results to skip.
    /// * `connection` - The connection to the database.
    ///
     fn all_updatable(
        &self,
filter: Option<Vec<u8>>,
author_user_id: i32,
limit: Option<i64>,
offset: Option<i64>,
connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>,
) -> Result<Vec<u8>, web_common::api::ApiError>;

    /// Get all of the sorted updatable structs from the database.
    ///
    /// * `filter` - The optional filter to apply to the query.
    /// * `author_user_id` - The ID of the user who is performing the search.
    /// * `limit` - The maximum number of results to return.
    /// * `offset` - The number of results to skip.
    /// * `connection` - The connection to the database.
    ///
     fn all_updatable_sorted(
        &self,
filter: Option<Vec<u8>>,
author_user_id: i32,
limit: Option<i64>,
offset: Option<i64>,
connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>,
) -> Result<Vec<u8>, web_common::api::ApiError>;

    /// Search for the updatable structs by a given string by Postgres's `similarity`.
    ///
    /// * `filter` - The optional filter to apply to the query.
    /// * `author_user_id` - The ID of the user who is performing the search.
    /// * `query` - The string to search for.
    /// * `limit` - The maximum number of results to return.
    /// * `offset` - The number of results to skip.
    /// * `connection` - The connection to the database.
    ///
     fn similarity_search_updatable(
        &self,
filter: Option<Vec<u8>>,
author_user_id: i32,
query: &str,
limit: Option<i64>,
offset: Option<i64>,
connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>,
) -> Result<Vec<u8>, web_common::api::ApiError>;

    /// Search for the updatable structs by a given string by Postgres's `word_similarity`.
    ///
    /// * `filter` - The optional filter to apply to the query.
    /// * `author_user_id` - The ID of the user who is performing the search.
    /// * `query` - The string to search for.
    /// * `limit` - The maximum number of results to return.
    /// * `offset` - The number of results to skip.
    /// * `connection` - The connection to the database.
    ///
     fn word_similarity_search_updatable(
        &self,
filter: Option<Vec<u8>>,
author_user_id: i32,
query: &str,
limit: Option<i64>,
offset: Option<i64>,
connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>,
) -> Result<Vec<u8>, web_common::api::ApiError>;

    /// Search for the updatable structs by a given string by Postgres's `strict_word_similarity`.
    ///
    /// * `filter` - The optional filter to apply to the query.
    /// * `author_user_id` - The ID of the user who is performing the search.
    /// * `query` - The string to search for.
    /// * `limit` - The maximum number of results to return.
    /// * `offset` - The number of results to skip.
    /// * `connection` - The connection to the database.
    ///
     fn strict_word_similarity_search_updatable(
        &self,
filter: Option<Vec<u8>>,
author_user_id: i32,
query: &str,
limit: Option<i64>,
offset: Option<i64>,
connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>,
) -> Result<Vec<u8>, web_common::api::ApiError>;

    /// Check whether the user can admin the struct associated to the provided ids.
    ///
    /// * `primary_key` - The primary key(s) of the struct to check.
    /// * `author_user_id` - The ID of the user to check.
    /// * `connection` - The connection to the database.
    ///
     fn can_admin_by_id(
        &self,
primary_key: PrimaryKey,
author_user_id: i32,
connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>,
) -> Result<bool, web_common::api::ApiError>;

    /// Get all of the administrable structs from the database.
    ///
    /// * `filter` - The optional filter to apply to the query.
    /// * `author_user_id` - The ID of the user who is performing the search.
    /// * `limit` - The maximum number of results to return.
    /// * `offset` - The number of results to skip.
    /// * `connection` - The connection to the database.
    ///
     fn all_administrable(
        &self,
filter: Option<Vec<u8>>,
author_user_id: i32,
limit: Option<i64>,
offset: Option<i64>,
connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>,
) -> Result<Vec<u8>, web_common::api::ApiError>;

    /// Get all of the sorted administrable structs from the database.
    ///
    /// * `filter` - The optional filter to apply to the query.
    /// * `author_user_id` - The ID of the user who is performing the search.
    /// * `limit` - The maximum number of results to return.
    /// * `offset` - The number of results to skip.
    /// * `connection` - The connection to the database.
    ///
     fn all_administrable_sorted(
        &self,
filter: Option<Vec<u8>>,
author_user_id: i32,
limit: Option<i64>,
offset: Option<i64>,
connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>,
) -> Result<Vec<u8>, web_common::api::ApiError>;

    /// Search for the administrable structs by a given string by Postgres's `similarity`.
    ///
    /// * `filter` - The optional filter to apply to the query.
    /// * `author_user_id` - The ID of the user who is performing the search.
    /// * `query` - The string to search for.
    /// * `limit` - The maximum number of results to return.
    /// * `offset` - The number of results to skip.
    /// * `connection` - The connection to the database.
    ///
     fn similarity_search_administrable(
        &self,
filter: Option<Vec<u8>>,
author_user_id: i32,
query: &str,
limit: Option<i64>,
offset: Option<i64>,
connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>,
) -> Result<Vec<u8>, web_common::api::ApiError>;

    /// Search for the administrable structs by a given string by Postgres's `word_similarity`.
    ///
    /// * `filter` - The optional filter to apply to the query.
    /// * `author_user_id` - The ID of the user who is performing the search.
    /// * `query` - The string to search for.
    /// * `limit` - The maximum number of results to return.
    /// * `offset` - The number of results to skip.
    /// * `connection` - The connection to the database.
    ///
     fn word_similarity_search_administrable(
        &self,
filter: Option<Vec<u8>>,
author_user_id: i32,
query: &str,
limit: Option<i64>,
offset: Option<i64>,
connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>,
) -> Result<Vec<u8>, web_common::api::ApiError>;

    /// Search for the administrable structs by a given string by Postgres's `strict_word_similarity`.
    ///
    /// * `filter` - The optional filter to apply to the query.
    /// * `author_user_id` - The ID of the user who is performing the search.
    /// * `query` - The string to search for.
    /// * `limit` - The maximum number of results to return.
    /// * `offset` - The number of results to skip.
    /// * `connection` - The connection to the database.
    ///
     fn strict_word_similarity_search_administrable(
        &self,
filter: Option<Vec<u8>>,
author_user_id: i32,
query: &str,
limit: Option<i64>,
offset: Option<i64>,
connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>,
) -> Result<Vec<u8>, web_common::api::ApiError>;

    /// Delete the struct from the database by its ID.
    ///
    /// * `primary_key` - The primary key(s) of the struct to delete.
    /// * `author_user_id` - The ID of the user who is deleting the struct.
    /// * `connection` - The connection to the database.
    ///
     fn delete_by_id(
        &self,
primary_key: PrimaryKey,
author_user_id: i32,
connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>,
) -> Result<usize, web_common::api::ApiError>;

}

impl BackendTable for web_common::database::Table {
    /// Check whether the user can view the struct associated to the provided ids.
    ///
    /// * `primary_key` - The primary key(s) of the struct to check.
    /// * `author_user_id` - The ID of the user to check.
    /// * `connection` - The connection to the database.
    ///
     fn can_view_by_id(
        &self,
primary_key: PrimaryKey,
author_user_id: Option<i32>,
connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>,
) -> Result<bool, web_common::api::ApiError> {
        Ok(match self {
            web_common::database::Table::BioOttRanks => {
NestedBioOttRank::can_view_by_id(
)?            },
            web_common::database::Table::BioOttTaxonItems => {
NestedBioOttTaxonItem::can_view_by_id(
)?            },
            web_common::database::Table::Colors => {
Color::can_view_by_id(
)?            },
            web_common::database::Table::Countries => {
Country::can_view_by_id(
)?            },
            web_common::database::Table::DerivedSamples => {
NestedDerivedSample::can_view_by_id(
primary_key.into(),
author_user_id,
connection)?            },
            web_common::database::Table::DocumentFormats => {
NestedDocumentFormat::can_view_by_id(
)?            },
            web_common::database::Table::FontAwesomeIcons => {
FontAwesomeIcon::can_view_by_id(
)?            },
            web_common::database::Table::LoginProviders => {
NestedLoginProvider::can_view_by_id(
)?            },
            web_common::database::Table::Materials => {
NestedMaterial::can_view_by_id(
)?            },
            web_common::database::Table::NameplateCategories => {
NestedNameplateCategory::can_view_by_id(
)?            },
            web_common::database::Table::Nameplates => {
NestedNameplate::can_view_by_id(
primary_key.into(),
author_user_id,
connection)?            },
            web_common::database::Table::Notifications => {
NestedNotification::can_view_by_id(
)?            },
            web_common::database::Table::ObservationSubjects => {
NestedObservationSubject::can_view_by_id(
)?            },
            web_common::database::Table::Observations => {
NestedObservation::can_view_by_id(
primary_key.into(),
author_user_id,
connection)?            },
            web_common::database::Table::OrganismBioOttTaxonItems => {
NestedOrganismBioOttTaxonItem::can_view_by_id(
primary_key.into(),
author_user_id,
connection)?            },
            web_common::database::Table::Organisms => {
NestedOrganism::can_view_by_id(
primary_key.into(),
author_user_id,
connection)?            },
            web_common::database::Table::Organizations => {
NestedOrganization::can_view_by_id(
)?            },
            web_common::database::Table::PermanenceCategories => {
NestedPermanenceCategory::can_view_by_id(
)?            },
            web_common::database::Table::ProjectStates => {
NestedProjectState::can_view_by_id(
)?            },
            web_common::database::Table::Projects => {
NestedProject::can_view_by_id(
primary_key.into(),
author_user_id,
connection)?            },
            web_common::database::Table::ProjectsTeamsRoleInvitations => {
NestedProjectsTeamsRoleInvitation::can_view_by_id(
primary_key.into(),
author_user_id,
connection)?            },
            web_common::database::Table::ProjectsTeamsRoleRequests => {
NestedProjectsTeamsRoleRequest::can_view_by_id(
primary_key.into(),
author_user_id,
connection)?            },
            web_common::database::Table::ProjectsTeamsRoles => {
NestedProjectsTeamsRole::can_view_by_id(
primary_key.into(),
author_user_id,
connection)?            },
            web_common::database::Table::ProjectsUsersRoleInvitations => {
NestedProjectsUsersRoleInvitation::can_view_by_id(
primary_key.into(),
author_user_id,
connection)?            },
            web_common::database::Table::ProjectsUsersRoleRequests => {
NestedProjectsUsersRoleRequest::can_view_by_id(
primary_key.into(),
author_user_id,
connection)?            },
            web_common::database::Table::ProjectsUsersRoles => {
NestedProjectsUsersRole::can_view_by_id(
primary_key.into(),
author_user_id,
connection)?            },
            web_common::database::Table::Roles => {
NestedRole::can_view_by_id(
)?            },
            web_common::database::Table::SampleBioOttTaxonItems => {
NestedSampleBioOttTaxonItem::can_view_by_id(
primary_key.into(),
author_user_id,
connection)?            },
            web_common::database::Table::SampleContainerCategories => {
NestedSampleContainerCategory::can_view_by_id(
)?            },
            web_common::database::Table::SampleContainers => {
NestedSampleContainer::can_view_by_id(
primary_key.into(),
author_user_id,
connection)?            },
            web_common::database::Table::SampleStates => {
NestedSampleState::can_view_by_id(
)?            },
            web_common::database::Table::Samples => {
NestedSample::can_view_by_id(
primary_key.into(),
author_user_id,
connection)?            },
            web_common::database::Table::Spectra => {
NestedSpectra::can_view_by_id(
primary_key.into(),
author_user_id,
connection)?            },
            web_common::database::Table::SpectraCollections => {
NestedSpectraCollection::can_view_by_id(
primary_key.into(),
author_user_id,
connection)?            },
            web_common::database::Table::TeamStates => {
NestedTeamState::can_view_by_id(
)?            },
            web_common::database::Table::Teams => {
NestedTeam::can_view_by_id(
)?            },
            web_common::database::Table::TeamsTeamsRoleInvitations => {
NestedTeamsTeamsRoleInvitation::can_view_by_id(
primary_key.into(),
author_user_id,
connection)?            },
            web_common::database::Table::TeamsUsersRoleInvitations => {
NestedTeamsUsersRoleInvitation::can_view_by_id(
primary_key.into(),
author_user_id,
connection)?            },
            web_common::database::Table::TeamsUsersRoleRequests => {
NestedTeamsUsersRoleRequest::can_view_by_id(
primary_key.into(),
author_user_id,
connection)?            },
            web_common::database::Table::TeamsUsersRoles => {
NestedTeamsUsersRole::can_view_by_id(
)?            },
            web_common::database::Table::Units => {
Unit::can_view_by_id(
)?            },
            web_common::database::Table::UserEmails => {
NestedUserEmail::can_view_by_id(
primary_key.into(),
author_user_id,
connection)?            },
            web_common::database::Table::Users => {
User::can_view_by_id(
)?            },
            web_common::database::Table::UsersUsersRoleInvitations => {
NestedUsersUsersRoleInvitation::can_view_by_id(
primary_key.into(),
author_user_id,
connection)?            },
            web_common::database::Table::UsersUsersRoleRequests => {
NestedUsersUsersRoleRequest::can_view_by_id(
primary_key.into(),
author_user_id,
connection)?            },
            web_common::database::Table::UsersUsersRoles => {
NestedUsersUsersRole::can_view_by_id(
)?            },
        })
    }

    /// Get all of the viewable structs from the database.
    ///
    /// * `filter` - The optional filter to apply to the query.
    /// * `author_user_id` - The ID of the user who is performing the search.
    /// * `limit` - The maximum number of results to return.
    /// * `offset` - The number of results to skip.
    /// * `connection` - The connection to the database.
    ///
     fn all_viewable(
        &self,
filter: Option<Vec<u8>>,
author_user_id: Option<i32>,
limit: Option<i64>,
offset: Option<i64>,
connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>,
) -> Result<Vec<u8>, web_common::api::ApiError> {
        Ok(match self {
            web_common::database::Table::BioOttRanks => {
bincode::serialize(&NestedBioOttRank::all_viewable(
filter.map(|filter| bincode::deserialize::<BioOttRankFilter>(&filter)).transpose()?.as_ref(),
limit,
offset,
connection)?)?
            },
            web_common::database::Table::BioOttTaxonItems => {
bincode::serialize(&NestedBioOttTaxonItem::all_viewable(
filter.map(|filter| bincode::deserialize::<BioOttTaxonItemFilter>(&filter)).transpose()?.as_ref(),
limit,
offset,
connection)?)?
            },
            web_common::database::Table::Colors => {
bincode::serialize(&Color::all_viewable(
limit,
offset,
connection)?)?
            },
            web_common::database::Table::Countries => {
bincode::serialize(&Country::all_viewable(
limit,
offset,
connection)?)?
            },
            web_common::database::Table::DerivedSamples => {
bincode::serialize(&NestedDerivedSample::all_viewable(
filter.map(|filter| bincode::deserialize::<DerivedSampleFilter>(&filter)).transpose()?.as_ref(),
author_user_id,
limit,
offset,
connection)?)?
            },
            web_common::database::Table::DocumentFormats => {
bincode::serialize(&NestedDocumentFormat::all_viewable(
filter.map(|filter| bincode::deserialize::<DocumentFormatFilter>(&filter)).transpose()?.as_ref(),
limit,
offset,
connection)?)?
            },
            web_common::database::Table::FontAwesomeIcons => {
bincode::serialize(&FontAwesomeIcon::all_viewable(
limit,
offset,
connection)?)?
            },
            web_common::database::Table::LoginProviders => {
bincode::serialize(&NestedLoginProvider::all_viewable(
filter.map(|filter| bincode::deserialize::<LoginProviderFilter>(&filter)).transpose()?.as_ref(),
limit,
offset,
connection)?)?
            },
            web_common::database::Table::Materials => {
bincode::serialize(&NestedMaterial::all_viewable(
filter.map(|filter| bincode::deserialize::<MaterialFilter>(&filter)).transpose()?.as_ref(),
limit,
offset,
connection)?)?
            },
            web_common::database::Table::NameplateCategories => {
bincode::serialize(&NestedNameplateCategory::all_viewable(
filter.map(|filter| bincode::deserialize::<NameplateCategoryFilter>(&filter)).transpose()?.as_ref(),
limit,
offset,
connection)?)?
            },
            web_common::database::Table::Nameplates => {
bincode::serialize(&NestedNameplate::all_viewable(
filter.map(|filter| bincode::deserialize::<NameplateFilter>(&filter)).transpose()?.as_ref(),
author_user_id,
limit,
offset,
connection)?)?
            },
            web_common::database::Table::Notifications => {
bincode::serialize(&NestedNotification::all_viewable(
filter.map(|filter| bincode::deserialize::<NotificationFilter>(&filter)).transpose()?.as_ref(),
limit,
offset,
connection)?)?
            },
            web_common::database::Table::ObservationSubjects => {
bincode::serialize(&NestedObservationSubject::all_viewable(
filter.map(|filter| bincode::deserialize::<ObservationSubjectFilter>(&filter)).transpose()?.as_ref(),
limit,
offset,
connection)?)?
            },
            web_common::database::Table::Observations => {
bincode::serialize(&NestedObservation::all_viewable(
filter.map(|filter| bincode::deserialize::<ObservationFilter>(&filter)).transpose()?.as_ref(),
author_user_id,
limit,
offset,
connection)?)?
            },
            web_common::database::Table::OrganismBioOttTaxonItems => {
bincode::serialize(&NestedOrganismBioOttTaxonItem::all_viewable(
filter.map(|filter| bincode::deserialize::<OrganismBioOttTaxonItemFilter>(&filter)).transpose()?.as_ref(),
author_user_id,
limit,
offset,
connection)?)?
            },
            web_common::database::Table::Organisms => {
bincode::serialize(&NestedOrganism::all_viewable(
filter.map(|filter| bincode::deserialize::<OrganismFilter>(&filter)).transpose()?.as_ref(),
author_user_id,
limit,
offset,
connection)?)?
            },
            web_common::database::Table::Organizations => {
bincode::serialize(&NestedOrganization::all_viewable(
filter.map(|filter| bincode::deserialize::<OrganizationFilter>(&filter)).transpose()?.as_ref(),
limit,
offset,
connection)?)?
            },
            web_common::database::Table::PermanenceCategories => {
bincode::serialize(&NestedPermanenceCategory::all_viewable(
filter.map(|filter| bincode::deserialize::<PermanenceCategoryFilter>(&filter)).transpose()?.as_ref(),
limit,
offset,
connection)?)?
            },
            web_common::database::Table::ProjectStates => {
bincode::serialize(&NestedProjectState::all_viewable(
filter.map(|filter| bincode::deserialize::<ProjectStateFilter>(&filter)).transpose()?.as_ref(),
limit,
offset,
connection)?)?
            },
            web_common::database::Table::Projects => {
bincode::serialize(&NestedProject::all_viewable(
filter.map(|filter| bincode::deserialize::<ProjectFilter>(&filter)).transpose()?.as_ref(),
author_user_id,
limit,
offset,
connection)?)?
            },
            web_common::database::Table::ProjectsTeamsRoleInvitations => {
bincode::serialize(&NestedProjectsTeamsRoleInvitation::all_viewable(
filter.map(|filter| bincode::deserialize::<ProjectsTeamsRoleInvitationFilter>(&filter)).transpose()?.as_ref(),
author_user_id,
limit,
offset,
connection)?)?
            },
            web_common::database::Table::ProjectsTeamsRoleRequests => {
bincode::serialize(&NestedProjectsTeamsRoleRequest::all_viewable(
filter.map(|filter| bincode::deserialize::<ProjectsTeamsRoleRequestFilter>(&filter)).transpose()?.as_ref(),
author_user_id,
limit,
offset,
connection)?)?
            },
            web_common::database::Table::ProjectsTeamsRoles => {
bincode::serialize(&NestedProjectsTeamsRole::all_viewable(
filter.map(|filter| bincode::deserialize::<ProjectsTeamsRoleFilter>(&filter)).transpose()?.as_ref(),
author_user_id,
limit,
offset,
connection)?)?
            },
            web_common::database::Table::ProjectsUsersRoleInvitations => {
bincode::serialize(&NestedProjectsUsersRoleInvitation::all_viewable(
filter.map(|filter| bincode::deserialize::<ProjectsUsersRoleInvitationFilter>(&filter)).transpose()?.as_ref(),
author_user_id,
limit,
offset,
connection)?)?
            },
            web_common::database::Table::ProjectsUsersRoleRequests => {
bincode::serialize(&NestedProjectsUsersRoleRequest::all_viewable(
filter.map(|filter| bincode::deserialize::<ProjectsUsersRoleRequestFilter>(&filter)).transpose()?.as_ref(),
author_user_id,
limit,
offset,
connection)?)?
            },
            web_common::database::Table::ProjectsUsersRoles => {
bincode::serialize(&NestedProjectsUsersRole::all_viewable(
filter.map(|filter| bincode::deserialize::<ProjectsUsersRoleFilter>(&filter)).transpose()?.as_ref(),
author_user_id,
limit,
offset,
connection)?)?
            },
            web_common::database::Table::Roles => {
bincode::serialize(&NestedRole::all_viewable(
filter.map(|filter| bincode::deserialize::<RoleFilter>(&filter)).transpose()?.as_ref(),
limit,
offset,
connection)?)?
            },
            web_common::database::Table::SampleBioOttTaxonItems => {
bincode::serialize(&NestedSampleBioOttTaxonItem::all_viewable(
filter.map(|filter| bincode::deserialize::<SampleBioOttTaxonItemFilter>(&filter)).transpose()?.as_ref(),
author_user_id,
limit,
offset,
connection)?)?
            },
            web_common::database::Table::SampleContainerCategories => {
bincode::serialize(&NestedSampleContainerCategory::all_viewable(
filter.map(|filter| bincode::deserialize::<SampleContainerCategoryFilter>(&filter)).transpose()?.as_ref(),
limit,
offset,
connection)?)?
            },
            web_common::database::Table::SampleContainers => {
bincode::serialize(&NestedSampleContainer::all_viewable(
filter.map(|filter| bincode::deserialize::<SampleContainerFilter>(&filter)).transpose()?.as_ref(),
author_user_id,
limit,
offset,
connection)?)?
            },
            web_common::database::Table::SampleStates => {
bincode::serialize(&NestedSampleState::all_viewable(
filter.map(|filter| bincode::deserialize::<SampleStateFilter>(&filter)).transpose()?.as_ref(),
limit,
offset,
connection)?)?
            },
            web_common::database::Table::Samples => {
bincode::serialize(&NestedSample::all_viewable(
filter.map(|filter| bincode::deserialize::<SampleFilter>(&filter)).transpose()?.as_ref(),
author_user_id,
limit,
offset,
connection)?)?
            },
            web_common::database::Table::Spectra => {
bincode::serialize(&NestedSpectra::all_viewable(
filter.map(|filter| bincode::deserialize::<SpectraFilter>(&filter)).transpose()?.as_ref(),
author_user_id,
limit,
offset,
connection)?)?
            },
            web_common::database::Table::SpectraCollections => {
bincode::serialize(&NestedSpectraCollection::all_viewable(
filter.map(|filter| bincode::deserialize::<SpectraCollectionFilter>(&filter)).transpose()?.as_ref(),
author_user_id,
limit,
offset,
connection)?)?
            },
            web_common::database::Table::TeamStates => {
bincode::serialize(&NestedTeamState::all_viewable(
filter.map(|filter| bincode::deserialize::<TeamStateFilter>(&filter)).transpose()?.as_ref(),
limit,
offset,
connection)?)?
            },
            web_common::database::Table::Teams => {
bincode::serialize(&NestedTeam::all_viewable(
filter.map(|filter| bincode::deserialize::<TeamFilter>(&filter)).transpose()?.as_ref(),
limit,
offset,
connection)?)?
            },
            web_common::database::Table::TeamsTeamsRoleInvitations => {
bincode::serialize(&NestedTeamsTeamsRoleInvitation::all_viewable(
filter.map(|filter| bincode::deserialize::<TeamsTeamsRoleInvitationFilter>(&filter)).transpose()?.as_ref(),
author_user_id,
limit,
offset,
connection)?)?
            },
            web_common::database::Table::TeamsUsersRoleInvitations => {
bincode::serialize(&NestedTeamsUsersRoleInvitation::all_viewable(
filter.map(|filter| bincode::deserialize::<TeamsUsersRoleInvitationFilter>(&filter)).transpose()?.as_ref(),
author_user_id,
limit,
offset,
connection)?)?
            },
            web_common::database::Table::TeamsUsersRoleRequests => {
bincode::serialize(&NestedTeamsUsersRoleRequest::all_viewable(
filter.map(|filter| bincode::deserialize::<TeamsUsersRoleRequestFilter>(&filter)).transpose()?.as_ref(),
author_user_id,
limit,
offset,
connection)?)?
            },
            web_common::database::Table::TeamsUsersRoles => {
bincode::serialize(&NestedTeamsUsersRole::all_viewable(
filter.map(|filter| bincode::deserialize::<TeamsUsersRoleFilter>(&filter)).transpose()?.as_ref(),
limit,
offset,
connection)?)?
            },
            web_common::database::Table::Units => {
bincode::serialize(&Unit::all_viewable(
limit,
offset,
connection)?)?
            },
            web_common::database::Table::UserEmails => {
bincode::serialize(&NestedUserEmail::all_viewable(
filter.map(|filter| bincode::deserialize::<UserEmailFilter>(&filter)).transpose()?.as_ref(),
author_user_id,
limit,
offset,
connection)?)?
            },
            web_common::database::Table::Users => {
bincode::serialize(&User::all_viewable(
limit,
offset,
connection)?)?
            },
            web_common::database::Table::UsersUsersRoleInvitations => {
bincode::serialize(&NestedUsersUsersRoleInvitation::all_viewable(
filter.map(|filter| bincode::deserialize::<UsersUsersRoleInvitationFilter>(&filter)).transpose()?.as_ref(),
author_user_id,
limit,
offset,
connection)?)?
            },
            web_common::database::Table::UsersUsersRoleRequests => {
bincode::serialize(&NestedUsersUsersRoleRequest::all_viewable(
filter.map(|filter| bincode::deserialize::<UsersUsersRoleRequestFilter>(&filter)).transpose()?.as_ref(),
author_user_id,
limit,
offset,
connection)?)?
            },
            web_common::database::Table::UsersUsersRoles => {
bincode::serialize(&NestedUsersUsersRole::all_viewable(
filter.map(|filter| bincode::deserialize::<UsersUsersRoleFilter>(&filter)).transpose()?.as_ref(),
limit,
offset,
connection)?)?
            },
        })
    }

    /// Get all of the sorted viewable structs from the database.
    ///
    /// * `filter` - The optional filter to apply to the query.
    /// * `author_user_id` - The ID of the user who is performing the search.
    /// * `limit` - The maximum number of results to return.
    /// * `offset` - The number of results to skip.
    /// * `connection` - The connection to the database.
    ///
     fn all_viewable_sorted(
        &self,
filter: Option<Vec<u8>>,
author_user_id: Option<i32>,
limit: Option<i64>,
offset: Option<i64>,
connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>,
) -> Result<Vec<u8>, web_common::api::ApiError> {
        Ok(match self {
            web_common::database::Table::BioOttRanks => {
bincode::serialize(&NestedBioOttRank::all_viewable_sorted(
filter.map(|filter| bincode::deserialize::<BioOttRankFilter>(&filter)).transpose()?.as_ref(),
limit,
offset,
connection)?)?
            },
            web_common::database::Table::BioOttTaxonItems => {
bincode::serialize(&NestedBioOttTaxonItem::all_viewable_sorted(
filter.map(|filter| bincode::deserialize::<BioOttTaxonItemFilter>(&filter)).transpose()?.as_ref(),
limit,
offset,
connection)?)?
            },
            web_common::database::Table::Colors => {
bincode::serialize(&Color::all_viewable_sorted(
limit,
offset,
connection)?)?
            },
            web_common::database::Table::Countries => {
bincode::serialize(&Country::all_viewable_sorted(
limit,
offset,
connection)?)?
            },
            web_common::database::Table::DerivedSamples => {
bincode::serialize(&NestedDerivedSample::all_viewable_sorted(
filter.map(|filter| bincode::deserialize::<DerivedSampleFilter>(&filter)).transpose()?.as_ref(),
author_user_id,
limit,
offset,
connection)?)?
            },
            web_common::database::Table::DocumentFormats => {
bincode::serialize(&NestedDocumentFormat::all_viewable_sorted(
filter.map(|filter| bincode::deserialize::<DocumentFormatFilter>(&filter)).transpose()?.as_ref(),
limit,
offset,
connection)?)?
            },
            web_common::database::Table::FontAwesomeIcons => {
bincode::serialize(&FontAwesomeIcon::all_viewable_sorted(
limit,
offset,
connection)?)?
            },
            web_common::database::Table::LoginProviders => {
bincode::serialize(&NestedLoginProvider::all_viewable_sorted(
filter.map(|filter| bincode::deserialize::<LoginProviderFilter>(&filter)).transpose()?.as_ref(),
limit,
offset,
connection)?)?
            },
            web_common::database::Table::Materials => {
bincode::serialize(&NestedMaterial::all_viewable_sorted(
filter.map(|filter| bincode::deserialize::<MaterialFilter>(&filter)).transpose()?.as_ref(),
limit,
offset,
connection)?)?
            },
            web_common::database::Table::NameplateCategories => {
bincode::serialize(&NestedNameplateCategory::all_viewable_sorted(
filter.map(|filter| bincode::deserialize::<NameplateCategoryFilter>(&filter)).transpose()?.as_ref(),
limit,
offset,
connection)?)?
            },
            web_common::database::Table::Nameplates => {
bincode::serialize(&NestedNameplate::all_viewable_sorted(
filter.map(|filter| bincode::deserialize::<NameplateFilter>(&filter)).transpose()?.as_ref(),
author_user_id,
limit,
offset,
connection)?)?
            },
            web_common::database::Table::Notifications => {
bincode::serialize(&NestedNotification::all_viewable_sorted(
filter.map(|filter| bincode::deserialize::<NotificationFilter>(&filter)).transpose()?.as_ref(),
limit,
offset,
connection)?)?
            },
            web_common::database::Table::ObservationSubjects => {
bincode::serialize(&NestedObservationSubject::all_viewable_sorted(
filter.map(|filter| bincode::deserialize::<ObservationSubjectFilter>(&filter)).transpose()?.as_ref(),
limit,
offset,
connection)?)?
            },
            web_common::database::Table::Observations => {
bincode::serialize(&NestedObservation::all_viewable_sorted(
filter.map(|filter| bincode::deserialize::<ObservationFilter>(&filter)).transpose()?.as_ref(),
author_user_id,
limit,
offset,
connection)?)?
            },
            web_common::database::Table::OrganismBioOttTaxonItems => {
bincode::serialize(&NestedOrganismBioOttTaxonItem::all_viewable_sorted(
filter.map(|filter| bincode::deserialize::<OrganismBioOttTaxonItemFilter>(&filter)).transpose()?.as_ref(),
author_user_id,
limit,
offset,
connection)?)?
            },
            web_common::database::Table::Organisms => {
bincode::serialize(&NestedOrganism::all_viewable_sorted(
filter.map(|filter| bincode::deserialize::<OrganismFilter>(&filter)).transpose()?.as_ref(),
author_user_id,
limit,
offset,
connection)?)?
            },
            web_common::database::Table::Organizations => {
bincode::serialize(&NestedOrganization::all_viewable_sorted(
filter.map(|filter| bincode::deserialize::<OrganizationFilter>(&filter)).transpose()?.as_ref(),
limit,
offset,
connection)?)?
            },
            web_common::database::Table::PermanenceCategories => {
bincode::serialize(&NestedPermanenceCategory::all_viewable_sorted(
filter.map(|filter| bincode::deserialize::<PermanenceCategoryFilter>(&filter)).transpose()?.as_ref(),
limit,
offset,
connection)?)?
            },
            web_common::database::Table::ProjectStates => {
bincode::serialize(&NestedProjectState::all_viewable_sorted(
filter.map(|filter| bincode::deserialize::<ProjectStateFilter>(&filter)).transpose()?.as_ref(),
limit,
offset,
connection)?)?
            },
            web_common::database::Table::Projects => {
bincode::serialize(&NestedProject::all_viewable_sorted(
filter.map(|filter| bincode::deserialize::<ProjectFilter>(&filter)).transpose()?.as_ref(),
author_user_id,
limit,
offset,
connection)?)?
            },
            web_common::database::Table::ProjectsTeamsRoleInvitations => {
bincode::serialize(&NestedProjectsTeamsRoleInvitation::all_viewable_sorted(
filter.map(|filter| bincode::deserialize::<ProjectsTeamsRoleInvitationFilter>(&filter)).transpose()?.as_ref(),
author_user_id,
limit,
offset,
connection)?)?
            },
            web_common::database::Table::ProjectsTeamsRoleRequests => {
bincode::serialize(&NestedProjectsTeamsRoleRequest::all_viewable_sorted(
filter.map(|filter| bincode::deserialize::<ProjectsTeamsRoleRequestFilter>(&filter)).transpose()?.as_ref(),
author_user_id,
limit,
offset,
connection)?)?
            },
            web_common::database::Table::ProjectsTeamsRoles => {
bincode::serialize(&NestedProjectsTeamsRole::all_viewable_sorted(
filter.map(|filter| bincode::deserialize::<ProjectsTeamsRoleFilter>(&filter)).transpose()?.as_ref(),
author_user_id,
limit,
offset,
connection)?)?
            },
            web_common::database::Table::ProjectsUsersRoleInvitations => {
bincode::serialize(&NestedProjectsUsersRoleInvitation::all_viewable_sorted(
filter.map(|filter| bincode::deserialize::<ProjectsUsersRoleInvitationFilter>(&filter)).transpose()?.as_ref(),
author_user_id,
limit,
offset,
connection)?)?
            },
            web_common::database::Table::ProjectsUsersRoleRequests => {
bincode::serialize(&NestedProjectsUsersRoleRequest::all_viewable_sorted(
filter.map(|filter| bincode::deserialize::<ProjectsUsersRoleRequestFilter>(&filter)).transpose()?.as_ref(),
author_user_id,
limit,
offset,
connection)?)?
            },
            web_common::database::Table::ProjectsUsersRoles => {
bincode::serialize(&NestedProjectsUsersRole::all_viewable_sorted(
filter.map(|filter| bincode::deserialize::<ProjectsUsersRoleFilter>(&filter)).transpose()?.as_ref(),
author_user_id,
limit,
offset,
connection)?)?
            },
            web_common::database::Table::Roles => {
bincode::serialize(&NestedRole::all_viewable_sorted(
filter.map(|filter| bincode::deserialize::<RoleFilter>(&filter)).transpose()?.as_ref(),
limit,
offset,
connection)?)?
            },
            web_common::database::Table::SampleBioOttTaxonItems => {
bincode::serialize(&NestedSampleBioOttTaxonItem::all_viewable_sorted(
filter.map(|filter| bincode::deserialize::<SampleBioOttTaxonItemFilter>(&filter)).transpose()?.as_ref(),
author_user_id,
limit,
offset,
connection)?)?
            },
            web_common::database::Table::SampleContainerCategories => {
bincode::serialize(&NestedSampleContainerCategory::all_viewable_sorted(
filter.map(|filter| bincode::deserialize::<SampleContainerCategoryFilter>(&filter)).transpose()?.as_ref(),
limit,
offset,
connection)?)?
            },
            web_common::database::Table::SampleContainers => {
bincode::serialize(&NestedSampleContainer::all_viewable_sorted(
filter.map(|filter| bincode::deserialize::<SampleContainerFilter>(&filter)).transpose()?.as_ref(),
author_user_id,
limit,
offset,
connection)?)?
            },
            web_common::database::Table::SampleStates => {
bincode::serialize(&NestedSampleState::all_viewable_sorted(
filter.map(|filter| bincode::deserialize::<SampleStateFilter>(&filter)).transpose()?.as_ref(),
limit,
offset,
connection)?)?
            },
            web_common::database::Table::Samples => {
bincode::serialize(&NestedSample::all_viewable_sorted(
filter.map(|filter| bincode::deserialize::<SampleFilter>(&filter)).transpose()?.as_ref(),
author_user_id,
limit,
offset,
connection)?)?
            },
            web_common::database::Table::Spectra => {
bincode::serialize(&NestedSpectra::all_viewable_sorted(
filter.map(|filter| bincode::deserialize::<SpectraFilter>(&filter)).transpose()?.as_ref(),
author_user_id,
limit,
offset,
connection)?)?
            },
            web_common::database::Table::SpectraCollections => {
bincode::serialize(&NestedSpectraCollection::all_viewable_sorted(
filter.map(|filter| bincode::deserialize::<SpectraCollectionFilter>(&filter)).transpose()?.as_ref(),
author_user_id,
limit,
offset,
connection)?)?
            },
            web_common::database::Table::TeamStates => {
bincode::serialize(&NestedTeamState::all_viewable_sorted(
filter.map(|filter| bincode::deserialize::<TeamStateFilter>(&filter)).transpose()?.as_ref(),
limit,
offset,
connection)?)?
            },
            web_common::database::Table::Teams => {
bincode::serialize(&NestedTeam::all_viewable_sorted(
filter.map(|filter| bincode::deserialize::<TeamFilter>(&filter)).transpose()?.as_ref(),
limit,
offset,
connection)?)?
            },
            web_common::database::Table::TeamsTeamsRoleInvitations => {
bincode::serialize(&NestedTeamsTeamsRoleInvitation::all_viewable_sorted(
filter.map(|filter| bincode::deserialize::<TeamsTeamsRoleInvitationFilter>(&filter)).transpose()?.as_ref(),
author_user_id,
limit,
offset,
connection)?)?
            },
            web_common::database::Table::TeamsUsersRoleInvitations => {
bincode::serialize(&NestedTeamsUsersRoleInvitation::all_viewable_sorted(
filter.map(|filter| bincode::deserialize::<TeamsUsersRoleInvitationFilter>(&filter)).transpose()?.as_ref(),
author_user_id,
limit,
offset,
connection)?)?
            },
            web_common::database::Table::TeamsUsersRoleRequests => {
bincode::serialize(&NestedTeamsUsersRoleRequest::all_viewable_sorted(
filter.map(|filter| bincode::deserialize::<TeamsUsersRoleRequestFilter>(&filter)).transpose()?.as_ref(),
author_user_id,
limit,
offset,
connection)?)?
            },
            web_common::database::Table::TeamsUsersRoles => {
bincode::serialize(&NestedTeamsUsersRole::all_viewable_sorted(
filter.map(|filter| bincode::deserialize::<TeamsUsersRoleFilter>(&filter)).transpose()?.as_ref(),
limit,
offset,
connection)?)?
            },
            web_common::database::Table::Units => {
bincode::serialize(&Unit::all_viewable_sorted(
limit,
offset,
connection)?)?
            },
            web_common::database::Table::UserEmails => {
bincode::serialize(&NestedUserEmail::all_viewable_sorted(
filter.map(|filter| bincode::deserialize::<UserEmailFilter>(&filter)).transpose()?.as_ref(),
author_user_id,
limit,
offset,
connection)?)?
            },
            web_common::database::Table::Users => {
bincode::serialize(&User::all_viewable_sorted(
limit,
offset,
connection)?)?
            },
            web_common::database::Table::UsersUsersRoleInvitations => {
bincode::serialize(&NestedUsersUsersRoleInvitation::all_viewable_sorted(
filter.map(|filter| bincode::deserialize::<UsersUsersRoleInvitationFilter>(&filter)).transpose()?.as_ref(),
author_user_id,
limit,
offset,
connection)?)?
            },
            web_common::database::Table::UsersUsersRoleRequests => {
bincode::serialize(&NestedUsersUsersRoleRequest::all_viewable_sorted(
filter.map(|filter| bincode::deserialize::<UsersUsersRoleRequestFilter>(&filter)).transpose()?.as_ref(),
author_user_id,
limit,
offset,
connection)?)?
            },
            web_common::database::Table::UsersUsersRoles => {
bincode::serialize(&NestedUsersUsersRole::all_viewable_sorted(
filter.map(|filter| bincode::deserialize::<UsersUsersRoleFilter>(&filter)).transpose()?.as_ref(),
limit,
offset,
connection)?)?
            },
        })
    }

    /// Get the struct from the database by its ID.
    ///
    /// * `primary_key` - The primary key(s) of the struct to get.
    /// * `author_user_id` - The ID of the user who is performing the search.
    /// * `connection` - The connection to the database.
    ///
     fn get(
        &self,
primary_key: PrimaryKey,
author_user_id: Option<i32>,
connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>,
) -> Result<Vec<u8>, web_common::api::ApiError> {
        Ok(match self {
            web_common::database::Table::BioOttRanks => {
bincode::serialize(&NestedBioOttRank::get(
primary_key.into(),
connection)?)?
            },
            web_common::database::Table::BioOttTaxonItems => {
bincode::serialize(&NestedBioOttTaxonItem::get(
primary_key.into(),
connection)?)?
            },
            web_common::database::Table::Colors => {
bincode::serialize(&Color::get(
primary_key.into(),
connection)?)?
            },
            web_common::database::Table::Countries => {
bincode::serialize(&Country::get(
primary_key.into(),
connection)?)?
            },
            web_common::database::Table::DerivedSamples => {
bincode::serialize(&NestedDerivedSample::get(
primary_key.into(),
author_user_id,
connection)?)?
            },
            web_common::database::Table::DocumentFormats => {
bincode::serialize(&NestedDocumentFormat::get(
primary_key.into(),
connection)?)?
            },
            web_common::database::Table::FontAwesomeIcons => {
bincode::serialize(&FontAwesomeIcon::get(
primary_key.into(),
connection)?)?
            },
            web_common::database::Table::LoginProviders => {
bincode::serialize(&NestedLoginProvider::get(
primary_key.into(),
connection)?)?
            },
            web_common::database::Table::Materials => {
bincode::serialize(&NestedMaterial::get(
primary_key.into(),
connection)?)?
            },
            web_common::database::Table::NameplateCategories => {
bincode::serialize(&NestedNameplateCategory::get(
primary_key.into(),
connection)?)?
            },
            web_common::database::Table::Nameplates => {
bincode::serialize(&NestedNameplate::get(
primary_key.into(),
author_user_id,
connection)?)?
            },
            web_common::database::Table::Notifications => {
bincode::serialize(&NestedNotification::get(
primary_key.into(),
connection)?)?
            },
            web_common::database::Table::ObservationSubjects => {
bincode::serialize(&NestedObservationSubject::get(
primary_key.into(),
connection)?)?
            },
            web_common::database::Table::Observations => {
bincode::serialize(&NestedObservation::get(
primary_key.into(),
author_user_id,
connection)?)?
            },
            web_common::database::Table::OrganismBioOttTaxonItems => {
bincode::serialize(&NestedOrganismBioOttTaxonItem::get(
primary_key.into(),
author_user_id,
connection)?)?
            },
            web_common::database::Table::Organisms => {
bincode::serialize(&NestedOrganism::get(
primary_key.into(),
author_user_id,
connection)?)?
            },
            web_common::database::Table::Organizations => {
bincode::serialize(&NestedOrganization::get(
primary_key.into(),
connection)?)?
            },
            web_common::database::Table::PermanenceCategories => {
bincode::serialize(&NestedPermanenceCategory::get(
primary_key.into(),
connection)?)?
            },
            web_common::database::Table::ProjectStates => {
bincode::serialize(&NestedProjectState::get(
primary_key.into(),
connection)?)?
            },
            web_common::database::Table::Projects => {
bincode::serialize(&NestedProject::get(
primary_key.into(),
author_user_id,
connection)?)?
            },
            web_common::database::Table::ProjectsTeamsRoleInvitations => {
bincode::serialize(&NestedProjectsTeamsRoleInvitation::get(
primary_key.into(),
author_user_id,
connection)?)?
            },
            web_common::database::Table::ProjectsTeamsRoleRequests => {
bincode::serialize(&NestedProjectsTeamsRoleRequest::get(
primary_key.into(),
author_user_id,
connection)?)?
            },
            web_common::database::Table::ProjectsTeamsRoles => {
bincode::serialize(&NestedProjectsTeamsRole::get(
primary_key.into(),
author_user_id,
connection)?)?
            },
            web_common::database::Table::ProjectsUsersRoleInvitations => {
bincode::serialize(&NestedProjectsUsersRoleInvitation::get(
primary_key.into(),
author_user_id,
connection)?)?
            },
            web_common::database::Table::ProjectsUsersRoleRequests => {
bincode::serialize(&NestedProjectsUsersRoleRequest::get(
primary_key.into(),
author_user_id,
connection)?)?
            },
            web_common::database::Table::ProjectsUsersRoles => {
bincode::serialize(&NestedProjectsUsersRole::get(
primary_key.into(),
author_user_id,
connection)?)?
            },
            web_common::database::Table::Roles => {
bincode::serialize(&NestedRole::get(
primary_key.into(),
connection)?)?
            },
            web_common::database::Table::SampleBioOttTaxonItems => {
bincode::serialize(&NestedSampleBioOttTaxonItem::get(
primary_key.into(),
author_user_id,
connection)?)?
            },
            web_common::database::Table::SampleContainerCategories => {
bincode::serialize(&NestedSampleContainerCategory::get(
primary_key.into(),
connection)?)?
            },
            web_common::database::Table::SampleContainers => {
bincode::serialize(&NestedSampleContainer::get(
primary_key.into(),
author_user_id,
connection)?)?
            },
            web_common::database::Table::SampleStates => {
bincode::serialize(&NestedSampleState::get(
primary_key.into(),
connection)?)?
            },
            web_common::database::Table::Samples => {
bincode::serialize(&NestedSample::get(
primary_key.into(),
author_user_id,
connection)?)?
            },
            web_common::database::Table::Spectra => {
bincode::serialize(&NestedSpectra::get(
primary_key.into(),
author_user_id,
connection)?)?
            },
            web_common::database::Table::SpectraCollections => {
bincode::serialize(&NestedSpectraCollection::get(
primary_key.into(),
author_user_id,
connection)?)?
            },
            web_common::database::Table::TeamStates => {
bincode::serialize(&NestedTeamState::get(
primary_key.into(),
connection)?)?
            },
            web_common::database::Table::Teams => {
bincode::serialize(&NestedTeam::get(
primary_key.into(),
connection)?)?
            },
            web_common::database::Table::TeamsTeamsRoleInvitations => {
bincode::serialize(&NestedTeamsTeamsRoleInvitation::get(
primary_key.into(),
author_user_id,
connection)?)?
            },
            web_common::database::Table::TeamsUsersRoleInvitations => {
bincode::serialize(&NestedTeamsUsersRoleInvitation::get(
primary_key.into(),
author_user_id,
connection)?)?
            },
            web_common::database::Table::TeamsUsersRoleRequests => {
bincode::serialize(&NestedTeamsUsersRoleRequest::get(
primary_key.into(),
author_user_id,
connection)?)?
            },
            web_common::database::Table::TeamsUsersRoles => {
bincode::serialize(&NestedTeamsUsersRole::get(
primary_key.into(),
connection)?)?
            },
            web_common::database::Table::Units => {
bincode::serialize(&Unit::get(
primary_key.into(),
connection)?)?
            },
            web_common::database::Table::UserEmails => {
bincode::serialize(&NestedUserEmail::get(
primary_key.into(),
author_user_id,
connection)?)?
            },
            web_common::database::Table::Users => {
bincode::serialize(&User::get(
primary_key.into(),
connection)?)?
            },
            web_common::database::Table::UsersUsersRoleInvitations => {
bincode::serialize(&NestedUsersUsersRoleInvitation::get(
primary_key.into(),
author_user_id,
connection)?)?
            },
            web_common::database::Table::UsersUsersRoleRequests => {
bincode::serialize(&NestedUsersUsersRoleRequest::get(
primary_key.into(),
author_user_id,
connection)?)?
            },
            web_common::database::Table::UsersUsersRoles => {
bincode::serialize(&NestedUsersUsersRole::get(
primary_key.into(),
connection)?)?
            },
        })
    }

    /// Search for the viewable structs by a given string by Postgres's `similarity`.
    ///
    /// * `filter` - The optional filter to apply to the query.
    /// * `author_user_id` - The ID of the user who is performing the search.
    /// * `query` - The string to search for.
    /// * `limit` - The maximum number of results to return.
    /// * `offset` - The number of results to skip.
    /// * `connection` - The connection to the database.
    ///
     fn similarity_search_viewable(
        &self,
filter: Option<Vec<u8>>,
author_user_id: Option<i32>,
query: &str,
limit: Option<i64>,
offset: Option<i64>,
connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>,
) -> Result<Vec<u8>, web_common::api::ApiError> {
        Ok(match self {
            web_common::database::Table::BioOttRanks => {
bincode::serialize(&NestedBioOttRank::similarity_search_viewable(
filter.map(|filter| bincode::deserialize::<BioOttRankFilter>(&filter)).transpose()?.as_ref(),
query,
limit,
offset,
connection)?)?
            },
            web_common::database::Table::BioOttTaxonItems => {
bincode::serialize(&NestedBioOttTaxonItem::similarity_search_viewable(
filter.map(|filter| bincode::deserialize::<BioOttTaxonItemFilter>(&filter)).transpose()?.as_ref(),
query,
limit,
offset,
connection)?)?
            },
            web_common::database::Table::Colors => {
bincode::serialize(&Color::similarity_search_viewable(
query,
limit,
offset,
connection)?)?
            },
            web_common::database::Table::Countries => {
bincode::serialize(&Country::similarity_search_viewable(
query,
limit,
offset,
connection)?)?
            },
            web_common::database::Table::DerivedSamples => {
bincode::serialize(&NestedDerivedSample::similarity_search_viewable(
filter.map(|filter| bincode::deserialize::<DerivedSampleFilter>(&filter)).transpose()?.as_ref(),
author_user_id,
query,
limit,
offset,
connection)?)?
            },
            web_common::database::Table::DocumentFormats => {
bincode::serialize(&NestedDocumentFormat::similarity_search_viewable(
filter.map(|filter| bincode::deserialize::<DocumentFormatFilter>(&filter)).transpose()?.as_ref(),
query,
limit,
offset,
connection)?)?
            },
            web_common::database::Table::FontAwesomeIcons => {
bincode::serialize(&FontAwesomeIcon::similarity_search_viewable(
query,
limit,
offset,
connection)?)?
            },
            web_common::database::Table::LoginProviders => unimplemented!("Method similarity_search_viewable not implemented for table login_providers."),
            web_common::database::Table::Materials => unimplemented!("Method similarity_search_viewable not implemented for table materials."),
            web_common::database::Table::NameplateCategories => {
bincode::serialize(&NestedNameplateCategory::similarity_search_viewable(
filter.map(|filter| bincode::deserialize::<NameplateCategoryFilter>(&filter)).transpose()?.as_ref(),
query,
limit,
offset,
connection)?)?
            },
            web_common::database::Table::Nameplates => {
bincode::serialize(&NestedNameplate::similarity_search_viewable(
filter.map(|filter| bincode::deserialize::<NameplateFilter>(&filter)).transpose()?.as_ref(),
author_user_id,
query,
limit,
offset,
connection)?)?
            },
            web_common::database::Table::Notifications => unimplemented!("Method similarity_search_viewable not implemented for table notifications."),
            web_common::database::Table::ObservationSubjects => unimplemented!("Method similarity_search_viewable not implemented for table observation_subjects."),
            web_common::database::Table::Observations => {
bincode::serialize(&NestedObservation::similarity_search_viewable(
filter.map(|filter| bincode::deserialize::<ObservationFilter>(&filter)).transpose()?.as_ref(),
author_user_id,
query,
limit,
offset,
connection)?)?
            },
            web_common::database::Table::OrganismBioOttTaxonItems => {
bincode::serialize(&NestedOrganismBioOttTaxonItem::similarity_search_viewable(
filter.map(|filter| bincode::deserialize::<OrganismBioOttTaxonItemFilter>(&filter)).transpose()?.as_ref(),
author_user_id,
query,
limit,
offset,
connection)?)?
            },
            web_common::database::Table::Organisms => {
bincode::serialize(&NestedOrganism::similarity_search_viewable(
filter.map(|filter| bincode::deserialize::<OrganismFilter>(&filter)).transpose()?.as_ref(),
author_user_id,
query,
limit,
offset,
connection)?)?
            },
            web_common::database::Table::Organizations => {
bincode::serialize(&NestedOrganization::similarity_search_viewable(
filter.map(|filter| bincode::deserialize::<OrganizationFilter>(&filter)).transpose()?.as_ref(),
query,
limit,
offset,
connection)?)?
            },
            web_common::database::Table::PermanenceCategories => unimplemented!("Method similarity_search_viewable not implemented for table permanence_categories."),
            web_common::database::Table::ProjectStates => {
bincode::serialize(&NestedProjectState::similarity_search_viewable(
filter.map(|filter| bincode::deserialize::<ProjectStateFilter>(&filter)).transpose()?.as_ref(),
query,
limit,
offset,
connection)?)?
            },
            web_common::database::Table::Projects => {
bincode::serialize(&NestedProject::similarity_search_viewable(
filter.map(|filter| bincode::deserialize::<ProjectFilter>(&filter)).transpose()?.as_ref(),
author_user_id,
query,
limit,
offset,
connection)?)?
            },
            web_common::database::Table::ProjectsTeamsRoleInvitations => {
bincode::serialize(&NestedProjectsTeamsRoleInvitation::similarity_search_viewable(
filter.map(|filter| bincode::deserialize::<ProjectsTeamsRoleInvitationFilter>(&filter)).transpose()?.as_ref(),
author_user_id,
query,
limit,
offset,
connection)?)?
            },
            web_common::database::Table::ProjectsTeamsRoleRequests => {
bincode::serialize(&NestedProjectsTeamsRoleRequest::similarity_search_viewable(
filter.map(|filter| bincode::deserialize::<ProjectsTeamsRoleRequestFilter>(&filter)).transpose()?.as_ref(),
author_user_id,
query,
limit,
offset,
connection)?)?
            },
            web_common::database::Table::ProjectsTeamsRoles => {
bincode::serialize(&NestedProjectsTeamsRole::similarity_search_viewable(
filter.map(|filter| bincode::deserialize::<ProjectsTeamsRoleFilter>(&filter)).transpose()?.as_ref(),
author_user_id,
query,
limit,
offset,
connection)?)?
            },
            web_common::database::Table::ProjectsUsersRoleInvitations => {
bincode::serialize(&NestedProjectsUsersRoleInvitation::similarity_search_viewable(
filter.map(|filter| bincode::deserialize::<ProjectsUsersRoleInvitationFilter>(&filter)).transpose()?.as_ref(),
author_user_id,
query,
limit,
offset,
connection)?)?
            },
            web_common::database::Table::ProjectsUsersRoleRequests => {
bincode::serialize(&NestedProjectsUsersRoleRequest::similarity_search_viewable(
filter.map(|filter| bincode::deserialize::<ProjectsUsersRoleRequestFilter>(&filter)).transpose()?.as_ref(),
author_user_id,
query,
limit,
offset,
connection)?)?
            },
            web_common::database::Table::ProjectsUsersRoles => {
bincode::serialize(&NestedProjectsUsersRole::similarity_search_viewable(
filter.map(|filter| bincode::deserialize::<ProjectsUsersRoleFilter>(&filter)).transpose()?.as_ref(),
author_user_id,
query,
limit,
offset,
connection)?)?
            },
            web_common::database::Table::Roles => {
bincode::serialize(&NestedRole::similarity_search_viewable(
filter.map(|filter| bincode::deserialize::<RoleFilter>(&filter)).transpose()?.as_ref(),
query,
limit,
offset,
connection)?)?
            },
            web_common::database::Table::SampleBioOttTaxonItems => {
bincode::serialize(&NestedSampleBioOttTaxonItem::similarity_search_viewable(
filter.map(|filter| bincode::deserialize::<SampleBioOttTaxonItemFilter>(&filter)).transpose()?.as_ref(),
author_user_id,
query,
limit,
offset,
connection)?)?
            },
            web_common::database::Table::SampleContainerCategories => {
bincode::serialize(&NestedSampleContainerCategory::similarity_search_viewable(
filter.map(|filter| bincode::deserialize::<SampleContainerCategoryFilter>(&filter)).transpose()?.as_ref(),
query,
limit,
offset,
connection)?)?
            },
            web_common::database::Table::SampleContainers => {
bincode::serialize(&NestedSampleContainer::similarity_search_viewable(
filter.map(|filter| bincode::deserialize::<SampleContainerFilter>(&filter)).transpose()?.as_ref(),
author_user_id,
query,
limit,
offset,
connection)?)?
            },
            web_common::database::Table::SampleStates => {
bincode::serialize(&NestedSampleState::similarity_search_viewable(
filter.map(|filter| bincode::deserialize::<SampleStateFilter>(&filter)).transpose()?.as_ref(),
query,
limit,
offset,
connection)?)?
            },
            web_common::database::Table::Samples => {
bincode::serialize(&NestedSample::similarity_search_viewable(
filter.map(|filter| bincode::deserialize::<SampleFilter>(&filter)).transpose()?.as_ref(),
author_user_id,
query,
limit,
offset,
connection)?)?
            },
            web_common::database::Table::Spectra => unimplemented!("Method similarity_search_viewable not implemented for table spectra."),
            web_common::database::Table::SpectraCollections => {
bincode::serialize(&NestedSpectraCollection::similarity_search_viewable(
filter.map(|filter| bincode::deserialize::<SpectraCollectionFilter>(&filter)).transpose()?.as_ref(),
author_user_id,
query,
limit,
offset,
connection)?)?
            },
            web_common::database::Table::TeamStates => {
bincode::serialize(&NestedTeamState::similarity_search_viewable(
filter.map(|filter| bincode::deserialize::<TeamStateFilter>(&filter)).transpose()?.as_ref(),
query,
limit,
offset,
connection)?)?
            },
            web_common::database::Table::Teams => {
bincode::serialize(&NestedTeam::similarity_search_viewable(
filter.map(|filter| bincode::deserialize::<TeamFilter>(&filter)).transpose()?.as_ref(),
query,
limit,
offset,
connection)?)?
            },
            web_common::database::Table::TeamsTeamsRoleInvitations => {
bincode::serialize(&NestedTeamsTeamsRoleInvitation::similarity_search_viewable(
filter.map(|filter| bincode::deserialize::<TeamsTeamsRoleInvitationFilter>(&filter)).transpose()?.as_ref(),
author_user_id,
query,
limit,
offset,
connection)?)?
            },
            web_common::database::Table::TeamsUsersRoleInvitations => {
bincode::serialize(&NestedTeamsUsersRoleInvitation::similarity_search_viewable(
filter.map(|filter| bincode::deserialize::<TeamsUsersRoleInvitationFilter>(&filter)).transpose()?.as_ref(),
author_user_id,
query,
limit,
offset,
connection)?)?
            },
            web_common::database::Table::TeamsUsersRoleRequests => {
bincode::serialize(&NestedTeamsUsersRoleRequest::similarity_search_viewable(
filter.map(|filter| bincode::deserialize::<TeamsUsersRoleRequestFilter>(&filter)).transpose()?.as_ref(),
author_user_id,
query,
limit,
offset,
connection)?)?
            },
            web_common::database::Table::TeamsUsersRoles => {
bincode::serialize(&NestedTeamsUsersRole::similarity_search_viewable(
filter.map(|filter| bincode::deserialize::<TeamsUsersRoleFilter>(&filter)).transpose()?.as_ref(),
query,
limit,
offset,
connection)?)?
            },
            web_common::database::Table::Units => {
bincode::serialize(&Unit::similarity_search_viewable(
query,
limit,
offset,
connection)?)?
            },
            web_common::database::Table::UserEmails => unimplemented!("Method similarity_search_viewable not implemented for table user_emails."),
            web_common::database::Table::Users => {
bincode::serialize(&User::similarity_search_viewable(
query,
limit,
offset,
connection)?)?
            },
            web_common::database::Table::UsersUsersRoleInvitations => {
bincode::serialize(&NestedUsersUsersRoleInvitation::similarity_search_viewable(
filter.map(|filter| bincode::deserialize::<UsersUsersRoleInvitationFilter>(&filter)).transpose()?.as_ref(),
author_user_id,
query,
limit,
offset,
connection)?)?
            },
            web_common::database::Table::UsersUsersRoleRequests => {
bincode::serialize(&NestedUsersUsersRoleRequest::similarity_search_viewable(
filter.map(|filter| bincode::deserialize::<UsersUsersRoleRequestFilter>(&filter)).transpose()?.as_ref(),
author_user_id,
query,
limit,
offset,
connection)?)?
            },
            web_common::database::Table::UsersUsersRoles => {
bincode::serialize(&NestedUsersUsersRole::similarity_search_viewable(
filter.map(|filter| bincode::deserialize::<UsersUsersRoleFilter>(&filter)).transpose()?.as_ref(),
query,
limit,
offset,
connection)?)?
            },
        })
    }

    /// Search for the viewable structs by a given string by Postgres's `word_similarity`.
    ///
    /// * `filter` - The optional filter to apply to the query.
    /// * `author_user_id` - The ID of the user who is performing the search.
    /// * `query` - The string to search for.
    /// * `limit` - The maximum number of results to return.
    /// * `offset` - The number of results to skip.
    /// * `connection` - The connection to the database.
    ///
     fn word_similarity_search_viewable(
        &self,
filter: Option<Vec<u8>>,
author_user_id: Option<i32>,
query: &str,
limit: Option<i64>,
offset: Option<i64>,
connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>,
) -> Result<Vec<u8>, web_common::api::ApiError> {
        Ok(match self {
            web_common::database::Table::BioOttRanks => {
bincode::serialize(&NestedBioOttRank::word_similarity_search_viewable(
filter.map(|filter| bincode::deserialize::<BioOttRankFilter>(&filter)).transpose()?.as_ref(),
query,
limit,
offset,
connection)?)?
            },
            web_common::database::Table::BioOttTaxonItems => {
bincode::serialize(&NestedBioOttTaxonItem::word_similarity_search_viewable(
filter.map(|filter| bincode::deserialize::<BioOttTaxonItemFilter>(&filter)).transpose()?.as_ref(),
query,
limit,
offset,
connection)?)?
            },
            web_common::database::Table::Colors => {
bincode::serialize(&Color::word_similarity_search_viewable(
query,
limit,
offset,
connection)?)?
            },
            web_common::database::Table::Countries => {
bincode::serialize(&Country::word_similarity_search_viewable(
query,
limit,
offset,
connection)?)?
            },
            web_common::database::Table::DerivedSamples => {
bincode::serialize(&NestedDerivedSample::word_similarity_search_viewable(
filter.map(|filter| bincode::deserialize::<DerivedSampleFilter>(&filter)).transpose()?.as_ref(),
author_user_id,
query,
limit,
offset,
connection)?)?
            },
            web_common::database::Table::DocumentFormats => {
bincode::serialize(&NestedDocumentFormat::word_similarity_search_viewable(
filter.map(|filter| bincode::deserialize::<DocumentFormatFilter>(&filter)).transpose()?.as_ref(),
query,
limit,
offset,
connection)?)?
            },
            web_common::database::Table::FontAwesomeIcons => {
bincode::serialize(&FontAwesomeIcon::word_similarity_search_viewable(
query,
limit,
offset,
connection)?)?
            },
            web_common::database::Table::LoginProviders => unimplemented!("Method word_similarity_search_viewable not implemented for table login_providers."),
            web_common::database::Table::Materials => unimplemented!("Method word_similarity_search_viewable not implemented for table materials."),
            web_common::database::Table::NameplateCategories => {
bincode::serialize(&NestedNameplateCategory::word_similarity_search_viewable(
filter.map(|filter| bincode::deserialize::<NameplateCategoryFilter>(&filter)).transpose()?.as_ref(),
query,
limit,
offset,
connection)?)?
            },
            web_common::database::Table::Nameplates => {
bincode::serialize(&NestedNameplate::word_similarity_search_viewable(
filter.map(|filter| bincode::deserialize::<NameplateFilter>(&filter)).transpose()?.as_ref(),
author_user_id,
query,
limit,
offset,
connection)?)?
            },
            web_common::database::Table::Notifications => unimplemented!("Method word_similarity_search_viewable not implemented for table notifications."),
            web_common::database::Table::ObservationSubjects => unimplemented!("Method word_similarity_search_viewable not implemented for table observation_subjects."),
            web_common::database::Table::Observations => {
bincode::serialize(&NestedObservation::word_similarity_search_viewable(
filter.map(|filter| bincode::deserialize::<ObservationFilter>(&filter)).transpose()?.as_ref(),
author_user_id,
query,
limit,
offset,
connection)?)?
            },
            web_common::database::Table::OrganismBioOttTaxonItems => {
bincode::serialize(&NestedOrganismBioOttTaxonItem::word_similarity_search_viewable(
filter.map(|filter| bincode::deserialize::<OrganismBioOttTaxonItemFilter>(&filter)).transpose()?.as_ref(),
author_user_id,
query,
limit,
offset,
connection)?)?
            },
            web_common::database::Table::Organisms => {
bincode::serialize(&NestedOrganism::word_similarity_search_viewable(
filter.map(|filter| bincode::deserialize::<OrganismFilter>(&filter)).transpose()?.as_ref(),
author_user_id,
query,
limit,
offset,
connection)?)?
            },
            web_common::database::Table::Organizations => {
bincode::serialize(&NestedOrganization::word_similarity_search_viewable(
filter.map(|filter| bincode::deserialize::<OrganizationFilter>(&filter)).transpose()?.as_ref(),
query,
limit,
offset,
connection)?)?
            },
            web_common::database::Table::PermanenceCategories => unimplemented!("Method word_similarity_search_viewable not implemented for table permanence_categories."),
            web_common::database::Table::ProjectStates => {
bincode::serialize(&NestedProjectState::word_similarity_search_viewable(
filter.map(|filter| bincode::deserialize::<ProjectStateFilter>(&filter)).transpose()?.as_ref(),
query,
limit,
offset,
connection)?)?
            },
            web_common::database::Table::Projects => {
bincode::serialize(&NestedProject::word_similarity_search_viewable(
filter.map(|filter| bincode::deserialize::<ProjectFilter>(&filter)).transpose()?.as_ref(),
author_user_id,
query,
limit,
offset,
connection)?)?
            },
            web_common::database::Table::ProjectsTeamsRoleInvitations => {
bincode::serialize(&NestedProjectsTeamsRoleInvitation::word_similarity_search_viewable(
filter.map(|filter| bincode::deserialize::<ProjectsTeamsRoleInvitationFilter>(&filter)).transpose()?.as_ref(),
author_user_id,
query,
limit,
offset,
connection)?)?
            },
            web_common::database::Table::ProjectsTeamsRoleRequests => {
bincode::serialize(&NestedProjectsTeamsRoleRequest::word_similarity_search_viewable(
filter.map(|filter| bincode::deserialize::<ProjectsTeamsRoleRequestFilter>(&filter)).transpose()?.as_ref(),
author_user_id,
query,
limit,
offset,
connection)?)?
            },
            web_common::database::Table::ProjectsTeamsRoles => {
bincode::serialize(&NestedProjectsTeamsRole::word_similarity_search_viewable(
filter.map(|filter| bincode::deserialize::<ProjectsTeamsRoleFilter>(&filter)).transpose()?.as_ref(),
author_user_id,
query,
limit,
offset,
connection)?)?
            },
            web_common::database::Table::ProjectsUsersRoleInvitations => {
bincode::serialize(&NestedProjectsUsersRoleInvitation::word_similarity_search_viewable(
filter.map(|filter| bincode::deserialize::<ProjectsUsersRoleInvitationFilter>(&filter)).transpose()?.as_ref(),
author_user_id,
query,
limit,
offset,
connection)?)?
            },
            web_common::database::Table::ProjectsUsersRoleRequests => {
bincode::serialize(&NestedProjectsUsersRoleRequest::word_similarity_search_viewable(
filter.map(|filter| bincode::deserialize::<ProjectsUsersRoleRequestFilter>(&filter)).transpose()?.as_ref(),
author_user_id,
query,
limit,
offset,
connection)?)?
            },
            web_common::database::Table::ProjectsUsersRoles => {
bincode::serialize(&NestedProjectsUsersRole::word_similarity_search_viewable(
filter.map(|filter| bincode::deserialize::<ProjectsUsersRoleFilter>(&filter)).transpose()?.as_ref(),
author_user_id,
query,
limit,
offset,
connection)?)?
            },
            web_common::database::Table::Roles => {
bincode::serialize(&NestedRole::word_similarity_search_viewable(
filter.map(|filter| bincode::deserialize::<RoleFilter>(&filter)).transpose()?.as_ref(),
query,
limit,
offset,
connection)?)?
            },
            web_common::database::Table::SampleBioOttTaxonItems => {
bincode::serialize(&NestedSampleBioOttTaxonItem::word_similarity_search_viewable(
filter.map(|filter| bincode::deserialize::<SampleBioOttTaxonItemFilter>(&filter)).transpose()?.as_ref(),
author_user_id,
query,
limit,
offset,
connection)?)?
            },
            web_common::database::Table::SampleContainerCategories => {
bincode::serialize(&NestedSampleContainerCategory::word_similarity_search_viewable(
filter.map(|filter| bincode::deserialize::<SampleContainerCategoryFilter>(&filter)).transpose()?.as_ref(),
query,
limit,
offset,
connection)?)?
            },
            web_common::database::Table::SampleContainers => {
bincode::serialize(&NestedSampleContainer::word_similarity_search_viewable(
filter.map(|filter| bincode::deserialize::<SampleContainerFilter>(&filter)).transpose()?.as_ref(),
author_user_id,
query,
limit,
offset,
connection)?)?
            },
            web_common::database::Table::SampleStates => {
bincode::serialize(&NestedSampleState::word_similarity_search_viewable(
filter.map(|filter| bincode::deserialize::<SampleStateFilter>(&filter)).transpose()?.as_ref(),
query,
limit,
offset,
connection)?)?
            },
            web_common::database::Table::Samples => {
bincode::serialize(&NestedSample::word_similarity_search_viewable(
filter.map(|filter| bincode::deserialize::<SampleFilter>(&filter)).transpose()?.as_ref(),
author_user_id,
query,
limit,
offset,
connection)?)?
            },
            web_common::database::Table::Spectra => unimplemented!("Method word_similarity_search_viewable not implemented for table spectra."),
            web_common::database::Table::SpectraCollections => {
bincode::serialize(&NestedSpectraCollection::word_similarity_search_viewable(
filter.map(|filter| bincode::deserialize::<SpectraCollectionFilter>(&filter)).transpose()?.as_ref(),
author_user_id,
query,
limit,
offset,
connection)?)?
            },
            web_common::database::Table::TeamStates => {
bincode::serialize(&NestedTeamState::word_similarity_search_viewable(
filter.map(|filter| bincode::deserialize::<TeamStateFilter>(&filter)).transpose()?.as_ref(),
query,
limit,
offset,
connection)?)?
            },
            web_common::database::Table::Teams => {
bincode::serialize(&NestedTeam::word_similarity_search_viewable(
filter.map(|filter| bincode::deserialize::<TeamFilter>(&filter)).transpose()?.as_ref(),
query,
limit,
offset,
connection)?)?
            },
            web_common::database::Table::TeamsTeamsRoleInvitations => {
bincode::serialize(&NestedTeamsTeamsRoleInvitation::word_similarity_search_viewable(
filter.map(|filter| bincode::deserialize::<TeamsTeamsRoleInvitationFilter>(&filter)).transpose()?.as_ref(),
author_user_id,
query,
limit,
offset,
connection)?)?
            },
            web_common::database::Table::TeamsUsersRoleInvitations => {
bincode::serialize(&NestedTeamsUsersRoleInvitation::word_similarity_search_viewable(
filter.map(|filter| bincode::deserialize::<TeamsUsersRoleInvitationFilter>(&filter)).transpose()?.as_ref(),
author_user_id,
query,
limit,
offset,
connection)?)?
            },
            web_common::database::Table::TeamsUsersRoleRequests => {
bincode::serialize(&NestedTeamsUsersRoleRequest::word_similarity_search_viewable(
filter.map(|filter| bincode::deserialize::<TeamsUsersRoleRequestFilter>(&filter)).transpose()?.as_ref(),
author_user_id,
query,
limit,
offset,
connection)?)?
            },
            web_common::database::Table::TeamsUsersRoles => {
bincode::serialize(&NestedTeamsUsersRole::word_similarity_search_viewable(
filter.map(|filter| bincode::deserialize::<TeamsUsersRoleFilter>(&filter)).transpose()?.as_ref(),
query,
limit,
offset,
connection)?)?
            },
            web_common::database::Table::Units => {
bincode::serialize(&Unit::word_similarity_search_viewable(
query,
limit,
offset,
connection)?)?
            },
            web_common::database::Table::UserEmails => unimplemented!("Method word_similarity_search_viewable not implemented for table user_emails."),
            web_common::database::Table::Users => {
bincode::serialize(&User::word_similarity_search_viewable(
query,
limit,
offset,
connection)?)?
            },
            web_common::database::Table::UsersUsersRoleInvitations => {
bincode::serialize(&NestedUsersUsersRoleInvitation::word_similarity_search_viewable(
filter.map(|filter| bincode::deserialize::<UsersUsersRoleInvitationFilter>(&filter)).transpose()?.as_ref(),
author_user_id,
query,
limit,
offset,
connection)?)?
            },
            web_common::database::Table::UsersUsersRoleRequests => {
bincode::serialize(&NestedUsersUsersRoleRequest::word_similarity_search_viewable(
filter.map(|filter| bincode::deserialize::<UsersUsersRoleRequestFilter>(&filter)).transpose()?.as_ref(),
author_user_id,
query,
limit,
offset,
connection)?)?
            },
            web_common::database::Table::UsersUsersRoles => {
bincode::serialize(&NestedUsersUsersRole::word_similarity_search_viewable(
filter.map(|filter| bincode::deserialize::<UsersUsersRoleFilter>(&filter)).transpose()?.as_ref(),
query,
limit,
offset,
connection)?)?
            },
        })
    }

    /// Search for the viewable structs by a given string by Postgres's `strict_word_similarity`.
    ///
    /// * `filter` - The optional filter to apply to the query.
    /// * `author_user_id` - The ID of the user who is performing the search.
    /// * `query` - The string to search for.
    /// * `limit` - The maximum number of results to return.
    /// * `offset` - The number of results to skip.
    /// * `connection` - The connection to the database.
    ///
     fn strict_word_similarity_search_viewable(
        &self,
filter: Option<Vec<u8>>,
author_user_id: Option<i32>,
query: &str,
limit: Option<i64>,
offset: Option<i64>,
connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>,
) -> Result<Vec<u8>, web_common::api::ApiError> {
        Ok(match self {
            web_common::database::Table::BioOttRanks => {
bincode::serialize(&NestedBioOttRank::strict_word_similarity_search_viewable(
filter.map(|filter| bincode::deserialize::<BioOttRankFilter>(&filter)).transpose()?.as_ref(),
query,
limit,
offset,
connection)?)?
            },
            web_common::database::Table::BioOttTaxonItems => {
bincode::serialize(&NestedBioOttTaxonItem::strict_word_similarity_search_viewable(
filter.map(|filter| bincode::deserialize::<BioOttTaxonItemFilter>(&filter)).transpose()?.as_ref(),
query,
limit,
offset,
connection)?)?
            },
            web_common::database::Table::Colors => {
bincode::serialize(&Color::strict_word_similarity_search_viewable(
query,
limit,
offset,
connection)?)?
            },
            web_common::database::Table::Countries => {
bincode::serialize(&Country::strict_word_similarity_search_viewable(
query,
limit,
offset,
connection)?)?
            },
            web_common::database::Table::DerivedSamples => {
bincode::serialize(&NestedDerivedSample::strict_word_similarity_search_viewable(
filter.map(|filter| bincode::deserialize::<DerivedSampleFilter>(&filter)).transpose()?.as_ref(),
author_user_id,
query,
limit,
offset,
connection)?)?
            },
            web_common::database::Table::DocumentFormats => {
bincode::serialize(&NestedDocumentFormat::strict_word_similarity_search_viewable(
filter.map(|filter| bincode::deserialize::<DocumentFormatFilter>(&filter)).transpose()?.as_ref(),
query,
limit,
offset,
connection)?)?
            },
            web_common::database::Table::FontAwesomeIcons => {
bincode::serialize(&FontAwesomeIcon::strict_word_similarity_search_viewable(
query,
limit,
offset,
connection)?)?
            },
            web_common::database::Table::LoginProviders => unimplemented!("Method strict_word_similarity_search_viewable not implemented for table login_providers."),
            web_common::database::Table::Materials => unimplemented!("Method strict_word_similarity_search_viewable not implemented for table materials."),
            web_common::database::Table::NameplateCategories => {
bincode::serialize(&NestedNameplateCategory::strict_word_similarity_search_viewable(
filter.map(|filter| bincode::deserialize::<NameplateCategoryFilter>(&filter)).transpose()?.as_ref(),
query,
limit,
offset,
connection)?)?
            },
            web_common::database::Table::Nameplates => {
bincode::serialize(&NestedNameplate::strict_word_similarity_search_viewable(
filter.map(|filter| bincode::deserialize::<NameplateFilter>(&filter)).transpose()?.as_ref(),
author_user_id,
query,
limit,
offset,
connection)?)?
            },
            web_common::database::Table::Notifications => unimplemented!("Method strict_word_similarity_search_viewable not implemented for table notifications."),
            web_common::database::Table::ObservationSubjects => unimplemented!("Method strict_word_similarity_search_viewable not implemented for table observation_subjects."),
            web_common::database::Table::Observations => {
bincode::serialize(&NestedObservation::strict_word_similarity_search_viewable(
filter.map(|filter| bincode::deserialize::<ObservationFilter>(&filter)).transpose()?.as_ref(),
author_user_id,
query,
limit,
offset,
connection)?)?
            },
            web_common::database::Table::OrganismBioOttTaxonItems => {
bincode::serialize(&NestedOrganismBioOttTaxonItem::strict_word_similarity_search_viewable(
filter.map(|filter| bincode::deserialize::<OrganismBioOttTaxonItemFilter>(&filter)).transpose()?.as_ref(),
author_user_id,
query,
limit,
offset,
connection)?)?
            },
            web_common::database::Table::Organisms => {
bincode::serialize(&NestedOrganism::strict_word_similarity_search_viewable(
filter.map(|filter| bincode::deserialize::<OrganismFilter>(&filter)).transpose()?.as_ref(),
author_user_id,
query,
limit,
offset,
connection)?)?
            },
            web_common::database::Table::Organizations => {
bincode::serialize(&NestedOrganization::strict_word_similarity_search_viewable(
filter.map(|filter| bincode::deserialize::<OrganizationFilter>(&filter)).transpose()?.as_ref(),
query,
limit,
offset,
connection)?)?
            },
            web_common::database::Table::PermanenceCategories => unimplemented!("Method strict_word_similarity_search_viewable not implemented for table permanence_categories."),
            web_common::database::Table::ProjectStates => {
bincode::serialize(&NestedProjectState::strict_word_similarity_search_viewable(
filter.map(|filter| bincode::deserialize::<ProjectStateFilter>(&filter)).transpose()?.as_ref(),
query,
limit,
offset,
connection)?)?
            },
            web_common::database::Table::Projects => {
bincode::serialize(&NestedProject::strict_word_similarity_search_viewable(
filter.map(|filter| bincode::deserialize::<ProjectFilter>(&filter)).transpose()?.as_ref(),
author_user_id,
query,
limit,
offset,
connection)?)?
            },
            web_common::database::Table::ProjectsTeamsRoleInvitations => {
bincode::serialize(&NestedProjectsTeamsRoleInvitation::strict_word_similarity_search_viewable(
filter.map(|filter| bincode::deserialize::<ProjectsTeamsRoleInvitationFilter>(&filter)).transpose()?.as_ref(),
author_user_id,
query,
limit,
offset,
connection)?)?
            },
            web_common::database::Table::ProjectsTeamsRoleRequests => {
bincode::serialize(&NestedProjectsTeamsRoleRequest::strict_word_similarity_search_viewable(
filter.map(|filter| bincode::deserialize::<ProjectsTeamsRoleRequestFilter>(&filter)).transpose()?.as_ref(),
author_user_id,
query,
limit,
offset,
connection)?)?
            },
            web_common::database::Table::ProjectsTeamsRoles => {
bincode::serialize(&NestedProjectsTeamsRole::strict_word_similarity_search_viewable(
filter.map(|filter| bincode::deserialize::<ProjectsTeamsRoleFilter>(&filter)).transpose()?.as_ref(),
author_user_id,
query,
limit,
offset,
connection)?)?
            },
            web_common::database::Table::ProjectsUsersRoleInvitations => {
bincode::serialize(&NestedProjectsUsersRoleInvitation::strict_word_similarity_search_viewable(
filter.map(|filter| bincode::deserialize::<ProjectsUsersRoleInvitationFilter>(&filter)).transpose()?.as_ref(),
author_user_id,
query,
limit,
offset,
connection)?)?
            },
            web_common::database::Table::ProjectsUsersRoleRequests => {
bincode::serialize(&NestedProjectsUsersRoleRequest::strict_word_similarity_search_viewable(
filter.map(|filter| bincode::deserialize::<ProjectsUsersRoleRequestFilter>(&filter)).transpose()?.as_ref(),
author_user_id,
query,
limit,
offset,
connection)?)?
            },
            web_common::database::Table::ProjectsUsersRoles => {
bincode::serialize(&NestedProjectsUsersRole::strict_word_similarity_search_viewable(
filter.map(|filter| bincode::deserialize::<ProjectsUsersRoleFilter>(&filter)).transpose()?.as_ref(),
author_user_id,
query,
limit,
offset,
connection)?)?
            },
            web_common::database::Table::Roles => {
bincode::serialize(&NestedRole::strict_word_similarity_search_viewable(
filter.map(|filter| bincode::deserialize::<RoleFilter>(&filter)).transpose()?.as_ref(),
query,
limit,
offset,
connection)?)?
            },
            web_common::database::Table::SampleBioOttTaxonItems => {
bincode::serialize(&NestedSampleBioOttTaxonItem::strict_word_similarity_search_viewable(
filter.map(|filter| bincode::deserialize::<SampleBioOttTaxonItemFilter>(&filter)).transpose()?.as_ref(),
author_user_id,
query,
limit,
offset,
connection)?)?
            },
            web_common::database::Table::SampleContainerCategories => {
bincode::serialize(&NestedSampleContainerCategory::strict_word_similarity_search_viewable(
filter.map(|filter| bincode::deserialize::<SampleContainerCategoryFilter>(&filter)).transpose()?.as_ref(),
query,
limit,
offset,
connection)?)?
            },
            web_common::database::Table::SampleContainers => {
bincode::serialize(&NestedSampleContainer::strict_word_similarity_search_viewable(
filter.map(|filter| bincode::deserialize::<SampleContainerFilter>(&filter)).transpose()?.as_ref(),
author_user_id,
query,
limit,
offset,
connection)?)?
            },
            web_common::database::Table::SampleStates => {
bincode::serialize(&NestedSampleState::strict_word_similarity_search_viewable(
filter.map(|filter| bincode::deserialize::<SampleStateFilter>(&filter)).transpose()?.as_ref(),
query,
limit,
offset,
connection)?)?
            },
            web_common::database::Table::Samples => {
bincode::serialize(&NestedSample::strict_word_similarity_search_viewable(
filter.map(|filter| bincode::deserialize::<SampleFilter>(&filter)).transpose()?.as_ref(),
author_user_id,
query,
limit,
offset,
connection)?)?
            },
            web_common::database::Table::Spectra => unimplemented!("Method strict_word_similarity_search_viewable not implemented for table spectra."),
            web_common::database::Table::SpectraCollections => {
bincode::serialize(&NestedSpectraCollection::strict_word_similarity_search_viewable(
filter.map(|filter| bincode::deserialize::<SpectraCollectionFilter>(&filter)).transpose()?.as_ref(),
author_user_id,
query,
limit,
offset,
connection)?)?
            },
            web_common::database::Table::TeamStates => {
bincode::serialize(&NestedTeamState::strict_word_similarity_search_viewable(
filter.map(|filter| bincode::deserialize::<TeamStateFilter>(&filter)).transpose()?.as_ref(),
query,
limit,
offset,
connection)?)?
            },
            web_common::database::Table::Teams => {
bincode::serialize(&NestedTeam::strict_word_similarity_search_viewable(
filter.map(|filter| bincode::deserialize::<TeamFilter>(&filter)).transpose()?.as_ref(),
query,
limit,
offset,
connection)?)?
            },
            web_common::database::Table::TeamsTeamsRoleInvitations => {
bincode::serialize(&NestedTeamsTeamsRoleInvitation::strict_word_similarity_search_viewable(
filter.map(|filter| bincode::deserialize::<TeamsTeamsRoleInvitationFilter>(&filter)).transpose()?.as_ref(),
author_user_id,
query,
limit,
offset,
connection)?)?
            },
            web_common::database::Table::TeamsUsersRoleInvitations => {
bincode::serialize(&NestedTeamsUsersRoleInvitation::strict_word_similarity_search_viewable(
filter.map(|filter| bincode::deserialize::<TeamsUsersRoleInvitationFilter>(&filter)).transpose()?.as_ref(),
author_user_id,
query,
limit,
offset,
connection)?)?
            },
            web_common::database::Table::TeamsUsersRoleRequests => {
bincode::serialize(&NestedTeamsUsersRoleRequest::strict_word_similarity_search_viewable(
filter.map(|filter| bincode::deserialize::<TeamsUsersRoleRequestFilter>(&filter)).transpose()?.as_ref(),
author_user_id,
query,
limit,
offset,
connection)?)?
            },
            web_common::database::Table::TeamsUsersRoles => {
bincode::serialize(&NestedTeamsUsersRole::strict_word_similarity_search_viewable(
filter.map(|filter| bincode::deserialize::<TeamsUsersRoleFilter>(&filter)).transpose()?.as_ref(),
query,
limit,
offset,
connection)?)?
            },
            web_common::database::Table::Units => {
bincode::serialize(&Unit::strict_word_similarity_search_viewable(
query,
limit,
offset,
connection)?)?
            },
            web_common::database::Table::UserEmails => unimplemented!("Method strict_word_similarity_search_viewable not implemented for table user_emails."),
            web_common::database::Table::Users => {
bincode::serialize(&User::strict_word_similarity_search_viewable(
query,
limit,
offset,
connection)?)?
            },
            web_common::database::Table::UsersUsersRoleInvitations => {
bincode::serialize(&NestedUsersUsersRoleInvitation::strict_word_similarity_search_viewable(
filter.map(|filter| bincode::deserialize::<UsersUsersRoleInvitationFilter>(&filter)).transpose()?.as_ref(),
author_user_id,
query,
limit,
offset,
connection)?)?
            },
            web_common::database::Table::UsersUsersRoleRequests => {
bincode::serialize(&NestedUsersUsersRoleRequest::strict_word_similarity_search_viewable(
filter.map(|filter| bincode::deserialize::<UsersUsersRoleRequestFilter>(&filter)).transpose()?.as_ref(),
author_user_id,
query,
limit,
offset,
connection)?)?
            },
            web_common::database::Table::UsersUsersRoles => {
bincode::serialize(&NestedUsersUsersRole::strict_word_similarity_search_viewable(
filter.map(|filter| bincode::deserialize::<UsersUsersRoleFilter>(&filter)).transpose()?.as_ref(),
query,
limit,
offset,
connection)?)?
            },
        })
    }

    /// Check whether the user can update the struct associated to the provided ids.
    ///
    /// * `primary_key` - The primary key(s) of the struct to check.
    /// * `author_user_id` - The ID of the user to check.
    /// * `connection` - The connection to the database.
    ///
     fn can_update_by_id(
        &self,
primary_key: PrimaryKey,
author_user_id: i32,
connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>,
) -> Result<bool, web_common::api::ApiError> {
        Ok(match self {
            web_common::database::Table::BioOttRanks => unimplemented!("Method can_update_by_id not implemented for table bio_ott_ranks."),
            web_common::database::Table::BioOttTaxonItems => unimplemented!("Method can_update_by_id not implemented for table bio_ott_taxon_items."),
            web_common::database::Table::Colors => unimplemented!("Method can_update_by_id not implemented for table colors."),
            web_common::database::Table::Countries => unimplemented!("Method can_update_by_id not implemented for table countries."),
            web_common::database::Table::DerivedSamples => {
NestedDerivedSample::can_update_by_id(
primary_key.into(),
author_user_id,
connection)?            },
            web_common::database::Table::DocumentFormats => unimplemented!("Method can_update_by_id not implemented for table document_formats."),
            web_common::database::Table::FontAwesomeIcons => unimplemented!("Method can_update_by_id not implemented for table font_awesome_icons."),
            web_common::database::Table::LoginProviders => unimplemented!("Method can_update_by_id not implemented for table login_providers."),
            web_common::database::Table::Materials => unimplemented!("Method can_update_by_id not implemented for table materials."),
            web_common::database::Table::NameplateCategories => unimplemented!("Method can_update_by_id not implemented for table nameplate_categories."),
            web_common::database::Table::Nameplates => {
NestedNameplate::can_update_by_id(
primary_key.into(),
author_user_id,
connection)?            },
            web_common::database::Table::Notifications => unimplemented!("Method can_update_by_id not implemented for table notifications."),
            web_common::database::Table::ObservationSubjects => unimplemented!("Method can_update_by_id not implemented for table observation_subjects."),
            web_common::database::Table::Observations => {
NestedObservation::can_update_by_id(
primary_key.into(),
author_user_id,
connection)?            },
            web_common::database::Table::OrganismBioOttTaxonItems => {
NestedOrganismBioOttTaxonItem::can_update_by_id(
primary_key.into(),
author_user_id,
connection)?            },
            web_common::database::Table::Organisms => {
NestedOrganism::can_update_by_id(
primary_key.into(),
author_user_id,
connection)?            },
            web_common::database::Table::Organizations => unimplemented!("Method can_update_by_id not implemented for table organizations."),
            web_common::database::Table::PermanenceCategories => unimplemented!("Method can_update_by_id not implemented for table permanence_categories."),
            web_common::database::Table::ProjectStates => unimplemented!("Method can_update_by_id not implemented for table project_states."),
            web_common::database::Table::Projects => {
NestedProject::can_update_by_id(
primary_key.into(),
author_user_id,
connection)?            },
            web_common::database::Table::ProjectsTeamsRoleInvitations => {
NestedProjectsTeamsRoleInvitation::can_update_by_id(
primary_key.into(),
author_user_id,
connection)?            },
            web_common::database::Table::ProjectsTeamsRoleRequests => {
NestedProjectsTeamsRoleRequest::can_update_by_id(
primary_key.into(),
author_user_id,
connection)?            },
            web_common::database::Table::ProjectsTeamsRoles => {
NestedProjectsTeamsRole::can_update_by_id(
primary_key.into(),
author_user_id,
connection)?            },
            web_common::database::Table::ProjectsUsersRoleInvitations => {
NestedProjectsUsersRoleInvitation::can_update_by_id(
primary_key.into(),
author_user_id,
connection)?            },
            web_common::database::Table::ProjectsUsersRoleRequests => {
NestedProjectsUsersRoleRequest::can_update_by_id(
primary_key.into(),
author_user_id,
connection)?            },
            web_common::database::Table::ProjectsUsersRoles => {
NestedProjectsUsersRole::can_update_by_id(
primary_key.into(),
author_user_id,
connection)?            },
            web_common::database::Table::Roles => unimplemented!("Method can_update_by_id not implemented for table roles."),
            web_common::database::Table::SampleBioOttTaxonItems => {
NestedSampleBioOttTaxonItem::can_update_by_id(
primary_key.into(),
author_user_id,
connection)?            },
            web_common::database::Table::SampleContainerCategories => unimplemented!("Method can_update_by_id not implemented for table sample_container_categories."),
            web_common::database::Table::SampleContainers => {
NestedSampleContainer::can_update_by_id(
primary_key.into(),
author_user_id,
connection)?            },
            web_common::database::Table::SampleStates => unimplemented!("Method can_update_by_id not implemented for table sample_states."),
            web_common::database::Table::Samples => {
NestedSample::can_update_by_id(
primary_key.into(),
author_user_id,
connection)?            },
            web_common::database::Table::Spectra => {
NestedSpectra::can_update_by_id(
primary_key.into(),
author_user_id,
connection)?            },
            web_common::database::Table::SpectraCollections => {
NestedSpectraCollection::can_update_by_id(
primary_key.into(),
author_user_id,
connection)?            },
            web_common::database::Table::TeamStates => unimplemented!("Method can_update_by_id not implemented for table team_states."),
            web_common::database::Table::Teams => {
NestedTeam::can_update_by_id(
primary_key.into(),
author_user_id,
connection)?            },
            web_common::database::Table::TeamsTeamsRoleInvitations => {
NestedTeamsTeamsRoleInvitation::can_update_by_id(
primary_key.into(),
author_user_id,
connection)?            },
            web_common::database::Table::TeamsUsersRoleInvitations => {
NestedTeamsUsersRoleInvitation::can_update_by_id(
primary_key.into(),
author_user_id,
connection)?            },
            web_common::database::Table::TeamsUsersRoleRequests => {
NestedTeamsUsersRoleRequest::can_update_by_id(
primary_key.into(),
author_user_id,
connection)?            },
            web_common::database::Table::TeamsUsersRoles => {
NestedTeamsUsersRole::can_update_by_id(
primary_key.into(),
author_user_id,
connection)?            },
            web_common::database::Table::Units => unimplemented!("Method can_update_by_id not implemented for table units."),
            web_common::database::Table::UserEmails => {
NestedUserEmail::can_update_by_id(
primary_key.into(),
author_user_id,
connection)?            },
            web_common::database::Table::Users => {
User::can_update_by_id(
primary_key.into(),
author_user_id,
connection)?            },
            web_common::database::Table::UsersUsersRoleInvitations => {
NestedUsersUsersRoleInvitation::can_update_by_id(
primary_key.into(),
author_user_id,
connection)?            },
            web_common::database::Table::UsersUsersRoleRequests => {
NestedUsersUsersRoleRequest::can_update_by_id(
primary_key.into(),
author_user_id,
connection)?            },
            web_common::database::Table::UsersUsersRoles => {
NestedUsersUsersRole::can_update_by_id(
primary_key.into(),
author_user_id,
connection)?            },
        })
    }

    /// Get all of the updatable structs from the database.
    ///
    /// * `filter` - The optional filter to apply to the query.
    /// * `author_user_id` - The ID of the user who is performing the search.
    /// * `limit` - The maximum number of results to return.
    /// * `offset` - The number of results to skip.
    /// * `connection` - The connection to the database.
    ///
     fn all_updatable(
        &self,
filter: Option<Vec<u8>>,
author_user_id: i32,
limit: Option<i64>,
offset: Option<i64>,
connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>,
) -> Result<Vec<u8>, web_common::api::ApiError> {
        Ok(match self {
            web_common::database::Table::BioOttRanks => unimplemented!("Method all_updatable not implemented for table bio_ott_ranks."),
            web_common::database::Table::BioOttTaxonItems => unimplemented!("Method all_updatable not implemented for table bio_ott_taxon_items."),
            web_common::database::Table::Colors => unimplemented!("Method all_updatable not implemented for table colors."),
            web_common::database::Table::Countries => unimplemented!("Method all_updatable not implemented for table countries."),
            web_common::database::Table::DerivedSamples => {
bincode::serialize(&NestedDerivedSample::all_updatable(
filter.map(|filter| bincode::deserialize::<DerivedSampleFilter>(&filter)).transpose()?.as_ref(),
author_user_id,
limit,
offset,
connection)?)?
            },
            web_common::database::Table::DocumentFormats => unimplemented!("Method all_updatable not implemented for table document_formats."),
            web_common::database::Table::FontAwesomeIcons => unimplemented!("Method all_updatable not implemented for table font_awesome_icons."),
            web_common::database::Table::LoginProviders => unimplemented!("Method all_updatable not implemented for table login_providers."),
            web_common::database::Table::Materials => unimplemented!("Method all_updatable not implemented for table materials."),
            web_common::database::Table::NameplateCategories => unimplemented!("Method all_updatable not implemented for table nameplate_categories."),
            web_common::database::Table::Nameplates => {
bincode::serialize(&NestedNameplate::all_updatable(
filter.map(|filter| bincode::deserialize::<NameplateFilter>(&filter)).transpose()?.as_ref(),
author_user_id,
limit,
offset,
connection)?)?
            },
            web_common::database::Table::Notifications => unimplemented!("Method all_updatable not implemented for table notifications."),
            web_common::database::Table::ObservationSubjects => unimplemented!("Method all_updatable not implemented for table observation_subjects."),
            web_common::database::Table::Observations => {
bincode::serialize(&NestedObservation::all_updatable(
filter.map(|filter| bincode::deserialize::<ObservationFilter>(&filter)).transpose()?.as_ref(),
author_user_id,
limit,
offset,
connection)?)?
            },
            web_common::database::Table::OrganismBioOttTaxonItems => {
bincode::serialize(&NestedOrganismBioOttTaxonItem::all_updatable(
filter.map(|filter| bincode::deserialize::<OrganismBioOttTaxonItemFilter>(&filter)).transpose()?.as_ref(),
author_user_id,
limit,
offset,
connection)?)?
            },
            web_common::database::Table::Organisms => {
bincode::serialize(&NestedOrganism::all_updatable(
filter.map(|filter| bincode::deserialize::<OrganismFilter>(&filter)).transpose()?.as_ref(),
author_user_id,
limit,
offset,
connection)?)?
            },
            web_common::database::Table::Organizations => unimplemented!("Method all_updatable not implemented for table organizations."),
            web_common::database::Table::PermanenceCategories => unimplemented!("Method all_updatable not implemented for table permanence_categories."),
            web_common::database::Table::ProjectStates => unimplemented!("Method all_updatable not implemented for table project_states."),
            web_common::database::Table::Projects => {
bincode::serialize(&NestedProject::all_updatable(
filter.map(|filter| bincode::deserialize::<ProjectFilter>(&filter)).transpose()?.as_ref(),
author_user_id,
limit,
offset,
connection)?)?
            },
            web_common::database::Table::ProjectsTeamsRoleInvitations => {
bincode::serialize(&NestedProjectsTeamsRoleInvitation::all_updatable(
filter.map(|filter| bincode::deserialize::<ProjectsTeamsRoleInvitationFilter>(&filter)).transpose()?.as_ref(),
author_user_id,
limit,
offset,
connection)?)?
            },
            web_common::database::Table::ProjectsTeamsRoleRequests => {
bincode::serialize(&NestedProjectsTeamsRoleRequest::all_updatable(
filter.map(|filter| bincode::deserialize::<ProjectsTeamsRoleRequestFilter>(&filter)).transpose()?.as_ref(),
author_user_id,
limit,
offset,
connection)?)?
            },
            web_common::database::Table::ProjectsTeamsRoles => {
bincode::serialize(&NestedProjectsTeamsRole::all_updatable(
filter.map(|filter| bincode::deserialize::<ProjectsTeamsRoleFilter>(&filter)).transpose()?.as_ref(),
author_user_id,
limit,
offset,
connection)?)?
            },
            web_common::database::Table::ProjectsUsersRoleInvitations => {
bincode::serialize(&NestedProjectsUsersRoleInvitation::all_updatable(
filter.map(|filter| bincode::deserialize::<ProjectsUsersRoleInvitationFilter>(&filter)).transpose()?.as_ref(),
author_user_id,
limit,
offset,
connection)?)?
            },
            web_common::database::Table::ProjectsUsersRoleRequests => {
bincode::serialize(&NestedProjectsUsersRoleRequest::all_updatable(
filter.map(|filter| bincode::deserialize::<ProjectsUsersRoleRequestFilter>(&filter)).transpose()?.as_ref(),
author_user_id,
limit,
offset,
connection)?)?
            },
            web_common::database::Table::ProjectsUsersRoles => {
bincode::serialize(&NestedProjectsUsersRole::all_updatable(
filter.map(|filter| bincode::deserialize::<ProjectsUsersRoleFilter>(&filter)).transpose()?.as_ref(),
author_user_id,
limit,
offset,
connection)?)?
            },
            web_common::database::Table::Roles => unimplemented!("Method all_updatable not implemented for table roles."),
            web_common::database::Table::SampleBioOttTaxonItems => {
bincode::serialize(&NestedSampleBioOttTaxonItem::all_updatable(
filter.map(|filter| bincode::deserialize::<SampleBioOttTaxonItemFilter>(&filter)).transpose()?.as_ref(),
author_user_id,
limit,
offset,
connection)?)?
            },
            web_common::database::Table::SampleContainerCategories => unimplemented!("Method all_updatable not implemented for table sample_container_categories."),
            web_common::database::Table::SampleContainers => {
bincode::serialize(&NestedSampleContainer::all_updatable(
filter.map(|filter| bincode::deserialize::<SampleContainerFilter>(&filter)).transpose()?.as_ref(),
author_user_id,
limit,
offset,
connection)?)?
            },
            web_common::database::Table::SampleStates => unimplemented!("Method all_updatable not implemented for table sample_states."),
            web_common::database::Table::Samples => {
bincode::serialize(&NestedSample::all_updatable(
filter.map(|filter| bincode::deserialize::<SampleFilter>(&filter)).transpose()?.as_ref(),
author_user_id,
limit,
offset,
connection)?)?
            },
            web_common::database::Table::Spectra => {
bincode::serialize(&NestedSpectra::all_updatable(
filter.map(|filter| bincode::deserialize::<SpectraFilter>(&filter)).transpose()?.as_ref(),
author_user_id,
limit,
offset,
connection)?)?
            },
            web_common::database::Table::SpectraCollections => {
bincode::serialize(&NestedSpectraCollection::all_updatable(
filter.map(|filter| bincode::deserialize::<SpectraCollectionFilter>(&filter)).transpose()?.as_ref(),
author_user_id,
limit,
offset,
connection)?)?
            },
            web_common::database::Table::TeamStates => unimplemented!("Method all_updatable not implemented for table team_states."),
            web_common::database::Table::Teams => {
bincode::serialize(&NestedTeam::all_updatable(
filter.map(|filter| bincode::deserialize::<TeamFilter>(&filter)).transpose()?.as_ref(),
author_user_id,
limit,
offset,
connection)?)?
            },
            web_common::database::Table::TeamsTeamsRoleInvitations => {
bincode::serialize(&NestedTeamsTeamsRoleInvitation::all_updatable(
filter.map(|filter| bincode::deserialize::<TeamsTeamsRoleInvitationFilter>(&filter)).transpose()?.as_ref(),
author_user_id,
limit,
offset,
connection)?)?
            },
            web_common::database::Table::TeamsUsersRoleInvitations => {
bincode::serialize(&NestedTeamsUsersRoleInvitation::all_updatable(
filter.map(|filter| bincode::deserialize::<TeamsUsersRoleInvitationFilter>(&filter)).transpose()?.as_ref(),
author_user_id,
limit,
offset,
connection)?)?
            },
            web_common::database::Table::TeamsUsersRoleRequests => {
bincode::serialize(&NestedTeamsUsersRoleRequest::all_updatable(
filter.map(|filter| bincode::deserialize::<TeamsUsersRoleRequestFilter>(&filter)).transpose()?.as_ref(),
author_user_id,
limit,
offset,
connection)?)?
            },
            web_common::database::Table::TeamsUsersRoles => {
bincode::serialize(&NestedTeamsUsersRole::all_updatable(
filter.map(|filter| bincode::deserialize::<TeamsUsersRoleFilter>(&filter)).transpose()?.as_ref(),
author_user_id,
limit,
offset,
connection)?)?
            },
            web_common::database::Table::Units => unimplemented!("Method all_updatable not implemented for table units."),
            web_common::database::Table::UserEmails => {
bincode::serialize(&NestedUserEmail::all_updatable(
filter.map(|filter| bincode::deserialize::<UserEmailFilter>(&filter)).transpose()?.as_ref(),
author_user_id,
limit,
offset,
connection)?)?
            },
            web_common::database::Table::Users => {
bincode::serialize(&User::all_updatable(
author_user_id,
limit,
offset,
connection)?)?
            },
            web_common::database::Table::UsersUsersRoleInvitations => {
bincode::serialize(&NestedUsersUsersRoleInvitation::all_updatable(
filter.map(|filter| bincode::deserialize::<UsersUsersRoleInvitationFilter>(&filter)).transpose()?.as_ref(),
author_user_id,
limit,
offset,
connection)?)?
            },
            web_common::database::Table::UsersUsersRoleRequests => {
bincode::serialize(&NestedUsersUsersRoleRequest::all_updatable(
filter.map(|filter| bincode::deserialize::<UsersUsersRoleRequestFilter>(&filter)).transpose()?.as_ref(),
author_user_id,
limit,
offset,
connection)?)?
            },
            web_common::database::Table::UsersUsersRoles => {
bincode::serialize(&NestedUsersUsersRole::all_updatable(
filter.map(|filter| bincode::deserialize::<UsersUsersRoleFilter>(&filter)).transpose()?.as_ref(),
author_user_id,
limit,
offset,
connection)?)?
            },
        })
    }

    /// Get all of the sorted updatable structs from the database.
    ///
    /// * `filter` - The optional filter to apply to the query.
    /// * `author_user_id` - The ID of the user who is performing the search.
    /// * `limit` - The maximum number of results to return.
    /// * `offset` - The number of results to skip.
    /// * `connection` - The connection to the database.
    ///
     fn all_updatable_sorted(
        &self,
filter: Option<Vec<u8>>,
author_user_id: i32,
limit: Option<i64>,
offset: Option<i64>,
connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>,
) -> Result<Vec<u8>, web_common::api::ApiError> {
        Ok(match self {
            web_common::database::Table::BioOttRanks => unimplemented!("Method all_updatable_sorted not implemented for table bio_ott_ranks."),
            web_common::database::Table::BioOttTaxonItems => unimplemented!("Method all_updatable_sorted not implemented for table bio_ott_taxon_items."),
            web_common::database::Table::Colors => unimplemented!("Method all_updatable_sorted not implemented for table colors."),
            web_common::database::Table::Countries => unimplemented!("Method all_updatable_sorted not implemented for table countries."),
            web_common::database::Table::DerivedSamples => {
bincode::serialize(&NestedDerivedSample::all_updatable_sorted(
filter.map(|filter| bincode::deserialize::<DerivedSampleFilter>(&filter)).transpose()?.as_ref(),
author_user_id,
limit,
offset,
connection)?)?
            },
            web_common::database::Table::DocumentFormats => unimplemented!("Method all_updatable_sorted not implemented for table document_formats."),
            web_common::database::Table::FontAwesomeIcons => unimplemented!("Method all_updatable_sorted not implemented for table font_awesome_icons."),
            web_common::database::Table::LoginProviders => unimplemented!("Method all_updatable_sorted not implemented for table login_providers."),
            web_common::database::Table::Materials => unimplemented!("Method all_updatable_sorted not implemented for table materials."),
            web_common::database::Table::NameplateCategories => unimplemented!("Method all_updatable_sorted not implemented for table nameplate_categories."),
            web_common::database::Table::Nameplates => {
bincode::serialize(&NestedNameplate::all_updatable_sorted(
filter.map(|filter| bincode::deserialize::<NameplateFilter>(&filter)).transpose()?.as_ref(),
author_user_id,
limit,
offset,
connection)?)?
            },
            web_common::database::Table::Notifications => unimplemented!("Method all_updatable_sorted not implemented for table notifications."),
            web_common::database::Table::ObservationSubjects => unimplemented!("Method all_updatable_sorted not implemented for table observation_subjects."),
            web_common::database::Table::Observations => {
bincode::serialize(&NestedObservation::all_updatable_sorted(
filter.map(|filter| bincode::deserialize::<ObservationFilter>(&filter)).transpose()?.as_ref(),
author_user_id,
limit,
offset,
connection)?)?
            },
            web_common::database::Table::OrganismBioOttTaxonItems => {
bincode::serialize(&NestedOrganismBioOttTaxonItem::all_updatable_sorted(
filter.map(|filter| bincode::deserialize::<OrganismBioOttTaxonItemFilter>(&filter)).transpose()?.as_ref(),
author_user_id,
limit,
offset,
connection)?)?
            },
            web_common::database::Table::Organisms => {
bincode::serialize(&NestedOrganism::all_updatable_sorted(
filter.map(|filter| bincode::deserialize::<OrganismFilter>(&filter)).transpose()?.as_ref(),
author_user_id,
limit,
offset,
connection)?)?
            },
            web_common::database::Table::Organizations => unimplemented!("Method all_updatable_sorted not implemented for table organizations."),
            web_common::database::Table::PermanenceCategories => unimplemented!("Method all_updatable_sorted not implemented for table permanence_categories."),
            web_common::database::Table::ProjectStates => unimplemented!("Method all_updatable_sorted not implemented for table project_states."),
            web_common::database::Table::Projects => {
bincode::serialize(&NestedProject::all_updatable_sorted(
filter.map(|filter| bincode::deserialize::<ProjectFilter>(&filter)).transpose()?.as_ref(),
author_user_id,
limit,
offset,
connection)?)?
            },
            web_common::database::Table::ProjectsTeamsRoleInvitations => {
bincode::serialize(&NestedProjectsTeamsRoleInvitation::all_updatable_sorted(
filter.map(|filter| bincode::deserialize::<ProjectsTeamsRoleInvitationFilter>(&filter)).transpose()?.as_ref(),
author_user_id,
limit,
offset,
connection)?)?
            },
            web_common::database::Table::ProjectsTeamsRoleRequests => {
bincode::serialize(&NestedProjectsTeamsRoleRequest::all_updatable_sorted(
filter.map(|filter| bincode::deserialize::<ProjectsTeamsRoleRequestFilter>(&filter)).transpose()?.as_ref(),
author_user_id,
limit,
offset,
connection)?)?
            },
            web_common::database::Table::ProjectsTeamsRoles => {
bincode::serialize(&NestedProjectsTeamsRole::all_updatable_sorted(
filter.map(|filter| bincode::deserialize::<ProjectsTeamsRoleFilter>(&filter)).transpose()?.as_ref(),
author_user_id,
limit,
offset,
connection)?)?
            },
            web_common::database::Table::ProjectsUsersRoleInvitations => {
bincode::serialize(&NestedProjectsUsersRoleInvitation::all_updatable_sorted(
filter.map(|filter| bincode::deserialize::<ProjectsUsersRoleInvitationFilter>(&filter)).transpose()?.as_ref(),
author_user_id,
limit,
offset,
connection)?)?
            },
            web_common::database::Table::ProjectsUsersRoleRequests => {
bincode::serialize(&NestedProjectsUsersRoleRequest::all_updatable_sorted(
filter.map(|filter| bincode::deserialize::<ProjectsUsersRoleRequestFilter>(&filter)).transpose()?.as_ref(),
author_user_id,
limit,
offset,
connection)?)?
            },
            web_common::database::Table::ProjectsUsersRoles => {
bincode::serialize(&NestedProjectsUsersRole::all_updatable_sorted(
filter.map(|filter| bincode::deserialize::<ProjectsUsersRoleFilter>(&filter)).transpose()?.as_ref(),
author_user_id,
limit,
offset,
connection)?)?
            },
            web_common::database::Table::Roles => unimplemented!("Method all_updatable_sorted not implemented for table roles."),
            web_common::database::Table::SampleBioOttTaxonItems => {
bincode::serialize(&NestedSampleBioOttTaxonItem::all_updatable_sorted(
filter.map(|filter| bincode::deserialize::<SampleBioOttTaxonItemFilter>(&filter)).transpose()?.as_ref(),
author_user_id,
limit,
offset,
connection)?)?
            },
            web_common::database::Table::SampleContainerCategories => unimplemented!("Method all_updatable_sorted not implemented for table sample_container_categories."),
            web_common::database::Table::SampleContainers => {
bincode::serialize(&NestedSampleContainer::all_updatable_sorted(
filter.map(|filter| bincode::deserialize::<SampleContainerFilter>(&filter)).transpose()?.as_ref(),
author_user_id,
limit,
offset,
connection)?)?
            },
            web_common::database::Table::SampleStates => unimplemented!("Method all_updatable_sorted not implemented for table sample_states."),
            web_common::database::Table::Samples => {
bincode::serialize(&NestedSample::all_updatable_sorted(
filter.map(|filter| bincode::deserialize::<SampleFilter>(&filter)).transpose()?.as_ref(),
author_user_id,
limit,
offset,
connection)?)?
            },
            web_common::database::Table::Spectra => {
bincode::serialize(&NestedSpectra::all_updatable_sorted(
filter.map(|filter| bincode::deserialize::<SpectraFilter>(&filter)).transpose()?.as_ref(),
author_user_id,
limit,
offset,
connection)?)?
            },
            web_common::database::Table::SpectraCollections => {
bincode::serialize(&NestedSpectraCollection::all_updatable_sorted(
filter.map(|filter| bincode::deserialize::<SpectraCollectionFilter>(&filter)).transpose()?.as_ref(),
author_user_id,
limit,
offset,
connection)?)?
            },
            web_common::database::Table::TeamStates => unimplemented!("Method all_updatable_sorted not implemented for table team_states."),
            web_common::database::Table::Teams => {
bincode::serialize(&NestedTeam::all_updatable_sorted(
filter.map(|filter| bincode::deserialize::<TeamFilter>(&filter)).transpose()?.as_ref(),
author_user_id,
limit,
offset,
connection)?)?
            },
            web_common::database::Table::TeamsTeamsRoleInvitations => {
bincode::serialize(&NestedTeamsTeamsRoleInvitation::all_updatable_sorted(
filter.map(|filter| bincode::deserialize::<TeamsTeamsRoleInvitationFilter>(&filter)).transpose()?.as_ref(),
author_user_id,
limit,
offset,
connection)?)?
            },
            web_common::database::Table::TeamsUsersRoleInvitations => {
bincode::serialize(&NestedTeamsUsersRoleInvitation::all_updatable_sorted(
filter.map(|filter| bincode::deserialize::<TeamsUsersRoleInvitationFilter>(&filter)).transpose()?.as_ref(),
author_user_id,
limit,
offset,
connection)?)?
            },
            web_common::database::Table::TeamsUsersRoleRequests => {
bincode::serialize(&NestedTeamsUsersRoleRequest::all_updatable_sorted(
filter.map(|filter| bincode::deserialize::<TeamsUsersRoleRequestFilter>(&filter)).transpose()?.as_ref(),
author_user_id,
limit,
offset,
connection)?)?
            },
            web_common::database::Table::TeamsUsersRoles => {
bincode::serialize(&NestedTeamsUsersRole::all_updatable_sorted(
filter.map(|filter| bincode::deserialize::<TeamsUsersRoleFilter>(&filter)).transpose()?.as_ref(),
author_user_id,
limit,
offset,
connection)?)?
            },
            web_common::database::Table::Units => unimplemented!("Method all_updatable_sorted not implemented for table units."),
            web_common::database::Table::UserEmails => {
bincode::serialize(&NestedUserEmail::all_updatable_sorted(
filter.map(|filter| bincode::deserialize::<UserEmailFilter>(&filter)).transpose()?.as_ref(),
author_user_id,
limit,
offset,
connection)?)?
            },
            web_common::database::Table::Users => {
bincode::serialize(&User::all_updatable_sorted(
author_user_id,
limit,
offset,
connection)?)?
            },
            web_common::database::Table::UsersUsersRoleInvitations => {
bincode::serialize(&NestedUsersUsersRoleInvitation::all_updatable_sorted(
filter.map(|filter| bincode::deserialize::<UsersUsersRoleInvitationFilter>(&filter)).transpose()?.as_ref(),
author_user_id,
limit,
offset,
connection)?)?
            },
            web_common::database::Table::UsersUsersRoleRequests => {
bincode::serialize(&NestedUsersUsersRoleRequest::all_updatable_sorted(
filter.map(|filter| bincode::deserialize::<UsersUsersRoleRequestFilter>(&filter)).transpose()?.as_ref(),
author_user_id,
limit,
offset,
connection)?)?
            },
            web_common::database::Table::UsersUsersRoles => {
bincode::serialize(&NestedUsersUsersRole::all_updatable_sorted(
filter.map(|filter| bincode::deserialize::<UsersUsersRoleFilter>(&filter)).transpose()?.as_ref(),
author_user_id,
limit,
offset,
connection)?)?
            },
        })
    }

    /// Search for the updatable structs by a given string by Postgres's `similarity`.
    ///
    /// * `filter` - The optional filter to apply to the query.
    /// * `author_user_id` - The ID of the user who is performing the search.
    /// * `query` - The string to search for.
    /// * `limit` - The maximum number of results to return.
    /// * `offset` - The number of results to skip.
    /// * `connection` - The connection to the database.
    ///
     fn similarity_search_updatable(
        &self,
filter: Option<Vec<u8>>,
author_user_id: i32,
query: &str,
limit: Option<i64>,
offset: Option<i64>,
connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>,
) -> Result<Vec<u8>, web_common::api::ApiError> {
        Ok(match self {
            web_common::database::Table::BioOttRanks => unimplemented!("Method similarity_search_updatable not implemented for table bio_ott_ranks."),
            web_common::database::Table::BioOttTaxonItems => unimplemented!("Method similarity_search_updatable not implemented for table bio_ott_taxon_items."),
            web_common::database::Table::Colors => unimplemented!("Method similarity_search_updatable not implemented for table colors."),
            web_common::database::Table::Countries => unimplemented!("Method similarity_search_updatable not implemented for table countries."),
            web_common::database::Table::DerivedSamples => {
bincode::serialize(&NestedDerivedSample::similarity_search_updatable(
filter.map(|filter| bincode::deserialize::<DerivedSampleFilter>(&filter)).transpose()?.as_ref(),
author_user_id,
query,
limit,
offset,
connection)?)?
            },
            web_common::database::Table::DocumentFormats => unimplemented!("Method similarity_search_updatable not implemented for table document_formats."),
            web_common::database::Table::FontAwesomeIcons => unimplemented!("Method similarity_search_updatable not implemented for table font_awesome_icons."),
            web_common::database::Table::LoginProviders => unimplemented!("Method similarity_search_updatable not implemented for table login_providers."),
            web_common::database::Table::Materials => unimplemented!("Method similarity_search_updatable not implemented for table materials."),
            web_common::database::Table::NameplateCategories => unimplemented!("Method similarity_search_updatable not implemented for table nameplate_categories."),
            web_common::database::Table::Nameplates => {
bincode::serialize(&NestedNameplate::similarity_search_updatable(
filter.map(|filter| bincode::deserialize::<NameplateFilter>(&filter)).transpose()?.as_ref(),
author_user_id,
query,
limit,
offset,
connection)?)?
            },
            web_common::database::Table::Notifications => unimplemented!("Method similarity_search_updatable not implemented for table notifications."),
            web_common::database::Table::ObservationSubjects => unimplemented!("Method similarity_search_updatable not implemented for table observation_subjects."),
            web_common::database::Table::Observations => {
bincode::serialize(&NestedObservation::similarity_search_updatable(
filter.map(|filter| bincode::deserialize::<ObservationFilter>(&filter)).transpose()?.as_ref(),
author_user_id,
query,
limit,
offset,
connection)?)?
            },
            web_common::database::Table::OrganismBioOttTaxonItems => {
bincode::serialize(&NestedOrganismBioOttTaxonItem::similarity_search_updatable(
filter.map(|filter| bincode::deserialize::<OrganismBioOttTaxonItemFilter>(&filter)).transpose()?.as_ref(),
author_user_id,
query,
limit,
offset,
connection)?)?
            },
            web_common::database::Table::Organisms => {
bincode::serialize(&NestedOrganism::similarity_search_updatable(
filter.map(|filter| bincode::deserialize::<OrganismFilter>(&filter)).transpose()?.as_ref(),
author_user_id,
query,
limit,
offset,
connection)?)?
            },
            web_common::database::Table::Organizations => unimplemented!("Method similarity_search_updatable not implemented for table organizations."),
            web_common::database::Table::PermanenceCategories => unimplemented!("Method similarity_search_updatable not implemented for table permanence_categories."),
            web_common::database::Table::ProjectStates => unimplemented!("Method similarity_search_updatable not implemented for table project_states."),
            web_common::database::Table::Projects => {
bincode::serialize(&NestedProject::similarity_search_updatable(
filter.map(|filter| bincode::deserialize::<ProjectFilter>(&filter)).transpose()?.as_ref(),
author_user_id,
query,
limit,
offset,
connection)?)?
            },
            web_common::database::Table::ProjectsTeamsRoleInvitations => {
bincode::serialize(&NestedProjectsTeamsRoleInvitation::similarity_search_updatable(
filter.map(|filter| bincode::deserialize::<ProjectsTeamsRoleInvitationFilter>(&filter)).transpose()?.as_ref(),
author_user_id,
query,
limit,
offset,
connection)?)?
            },
            web_common::database::Table::ProjectsTeamsRoleRequests => {
bincode::serialize(&NestedProjectsTeamsRoleRequest::similarity_search_updatable(
filter.map(|filter| bincode::deserialize::<ProjectsTeamsRoleRequestFilter>(&filter)).transpose()?.as_ref(),
author_user_id,
query,
limit,
offset,
connection)?)?
            },
            web_common::database::Table::ProjectsTeamsRoles => {
bincode::serialize(&NestedProjectsTeamsRole::similarity_search_updatable(
filter.map(|filter| bincode::deserialize::<ProjectsTeamsRoleFilter>(&filter)).transpose()?.as_ref(),
author_user_id,
query,
limit,
offset,
connection)?)?
            },
            web_common::database::Table::ProjectsUsersRoleInvitations => {
bincode::serialize(&NestedProjectsUsersRoleInvitation::similarity_search_updatable(
filter.map(|filter| bincode::deserialize::<ProjectsUsersRoleInvitationFilter>(&filter)).transpose()?.as_ref(),
author_user_id,
query,
limit,
offset,
connection)?)?
            },
            web_common::database::Table::ProjectsUsersRoleRequests => {
bincode::serialize(&NestedProjectsUsersRoleRequest::similarity_search_updatable(
filter.map(|filter| bincode::deserialize::<ProjectsUsersRoleRequestFilter>(&filter)).transpose()?.as_ref(),
author_user_id,
query,
limit,
offset,
connection)?)?
            },
            web_common::database::Table::ProjectsUsersRoles => {
bincode::serialize(&NestedProjectsUsersRole::similarity_search_updatable(
filter.map(|filter| bincode::deserialize::<ProjectsUsersRoleFilter>(&filter)).transpose()?.as_ref(),
author_user_id,
query,
limit,
offset,
connection)?)?
            },
            web_common::database::Table::Roles => unimplemented!("Method similarity_search_updatable not implemented for table roles."),
            web_common::database::Table::SampleBioOttTaxonItems => {
bincode::serialize(&NestedSampleBioOttTaxonItem::similarity_search_updatable(
filter.map(|filter| bincode::deserialize::<SampleBioOttTaxonItemFilter>(&filter)).transpose()?.as_ref(),
author_user_id,
query,
limit,
offset,
connection)?)?
            },
            web_common::database::Table::SampleContainerCategories => unimplemented!("Method similarity_search_updatable not implemented for table sample_container_categories."),
            web_common::database::Table::SampleContainers => {
bincode::serialize(&NestedSampleContainer::similarity_search_updatable(
filter.map(|filter| bincode::deserialize::<SampleContainerFilter>(&filter)).transpose()?.as_ref(),
author_user_id,
query,
limit,
offset,
connection)?)?
            },
            web_common::database::Table::SampleStates => unimplemented!("Method similarity_search_updatable not implemented for table sample_states."),
            web_common::database::Table::Samples => {
bincode::serialize(&NestedSample::similarity_search_updatable(
filter.map(|filter| bincode::deserialize::<SampleFilter>(&filter)).transpose()?.as_ref(),
author_user_id,
query,
limit,
offset,
connection)?)?
            },
            web_common::database::Table::Spectra => unimplemented!("Method similarity_search_updatable not implemented for table spectra."),
            web_common::database::Table::SpectraCollections => {
bincode::serialize(&NestedSpectraCollection::similarity_search_updatable(
filter.map(|filter| bincode::deserialize::<SpectraCollectionFilter>(&filter)).transpose()?.as_ref(),
author_user_id,
query,
limit,
offset,
connection)?)?
            },
            web_common::database::Table::TeamStates => unimplemented!("Method similarity_search_updatable not implemented for table team_states."),
            web_common::database::Table::Teams => {
bincode::serialize(&NestedTeam::similarity_search_updatable(
filter.map(|filter| bincode::deserialize::<TeamFilter>(&filter)).transpose()?.as_ref(),
author_user_id,
query,
limit,
offset,
connection)?)?
            },
            web_common::database::Table::TeamsTeamsRoleInvitations => {
bincode::serialize(&NestedTeamsTeamsRoleInvitation::similarity_search_updatable(
filter.map(|filter| bincode::deserialize::<TeamsTeamsRoleInvitationFilter>(&filter)).transpose()?.as_ref(),
author_user_id,
query,
limit,
offset,
connection)?)?
            },
            web_common::database::Table::TeamsUsersRoleInvitations => {
bincode::serialize(&NestedTeamsUsersRoleInvitation::similarity_search_updatable(
filter.map(|filter| bincode::deserialize::<TeamsUsersRoleInvitationFilter>(&filter)).transpose()?.as_ref(),
author_user_id,
query,
limit,
offset,
connection)?)?
            },
            web_common::database::Table::TeamsUsersRoleRequests => {
bincode::serialize(&NestedTeamsUsersRoleRequest::similarity_search_updatable(
filter.map(|filter| bincode::deserialize::<TeamsUsersRoleRequestFilter>(&filter)).transpose()?.as_ref(),
author_user_id,
query,
limit,
offset,
connection)?)?
            },
            web_common::database::Table::TeamsUsersRoles => {
bincode::serialize(&NestedTeamsUsersRole::similarity_search_updatable(
filter.map(|filter| bincode::deserialize::<TeamsUsersRoleFilter>(&filter)).transpose()?.as_ref(),
author_user_id,
query,
limit,
offset,
connection)?)?
            },
            web_common::database::Table::Units => unimplemented!("Method similarity_search_updatable not implemented for table units."),
            web_common::database::Table::UserEmails => unimplemented!("Method similarity_search_updatable not implemented for table user_emails."),
            web_common::database::Table::Users => {
bincode::serialize(&User::similarity_search_updatable(
author_user_id,
query,
limit,
offset,
connection)?)?
            },
            web_common::database::Table::UsersUsersRoleInvitations => {
bincode::serialize(&NestedUsersUsersRoleInvitation::similarity_search_updatable(
filter.map(|filter| bincode::deserialize::<UsersUsersRoleInvitationFilter>(&filter)).transpose()?.as_ref(),
author_user_id,
query,
limit,
offset,
connection)?)?
            },
            web_common::database::Table::UsersUsersRoleRequests => {
bincode::serialize(&NestedUsersUsersRoleRequest::similarity_search_updatable(
filter.map(|filter| bincode::deserialize::<UsersUsersRoleRequestFilter>(&filter)).transpose()?.as_ref(),
author_user_id,
query,
limit,
offset,
connection)?)?
            },
            web_common::database::Table::UsersUsersRoles => {
bincode::serialize(&NestedUsersUsersRole::similarity_search_updatable(
filter.map(|filter| bincode::deserialize::<UsersUsersRoleFilter>(&filter)).transpose()?.as_ref(),
author_user_id,
query,
limit,
offset,
connection)?)?
            },
        })
    }

    /// Search for the updatable structs by a given string by Postgres's `word_similarity`.
    ///
    /// * `filter` - The optional filter to apply to the query.
    /// * `author_user_id` - The ID of the user who is performing the search.
    /// * `query` - The string to search for.
    /// * `limit` - The maximum number of results to return.
    /// * `offset` - The number of results to skip.
    /// * `connection` - The connection to the database.
    ///
     fn word_similarity_search_updatable(
        &self,
filter: Option<Vec<u8>>,
author_user_id: i32,
query: &str,
limit: Option<i64>,
offset: Option<i64>,
connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>,
) -> Result<Vec<u8>, web_common::api::ApiError> {
        Ok(match self {
            web_common::database::Table::BioOttRanks => unimplemented!("Method word_similarity_search_updatable not implemented for table bio_ott_ranks."),
            web_common::database::Table::BioOttTaxonItems => unimplemented!("Method word_similarity_search_updatable not implemented for table bio_ott_taxon_items."),
            web_common::database::Table::Colors => unimplemented!("Method word_similarity_search_updatable not implemented for table colors."),
            web_common::database::Table::Countries => unimplemented!("Method word_similarity_search_updatable not implemented for table countries."),
            web_common::database::Table::DerivedSamples => {
bincode::serialize(&NestedDerivedSample::word_similarity_search_updatable(
filter.map(|filter| bincode::deserialize::<DerivedSampleFilter>(&filter)).transpose()?.as_ref(),
author_user_id,
query,
limit,
offset,
connection)?)?
            },
            web_common::database::Table::DocumentFormats => unimplemented!("Method word_similarity_search_updatable not implemented for table document_formats."),
            web_common::database::Table::FontAwesomeIcons => unimplemented!("Method word_similarity_search_updatable not implemented for table font_awesome_icons."),
            web_common::database::Table::LoginProviders => unimplemented!("Method word_similarity_search_updatable not implemented for table login_providers."),
            web_common::database::Table::Materials => unimplemented!("Method word_similarity_search_updatable not implemented for table materials."),
            web_common::database::Table::NameplateCategories => unimplemented!("Method word_similarity_search_updatable not implemented for table nameplate_categories."),
            web_common::database::Table::Nameplates => {
bincode::serialize(&NestedNameplate::word_similarity_search_updatable(
filter.map(|filter| bincode::deserialize::<NameplateFilter>(&filter)).transpose()?.as_ref(),
author_user_id,
query,
limit,
offset,
connection)?)?
            },
            web_common::database::Table::Notifications => unimplemented!("Method word_similarity_search_updatable not implemented for table notifications."),
            web_common::database::Table::ObservationSubjects => unimplemented!("Method word_similarity_search_updatable not implemented for table observation_subjects."),
            web_common::database::Table::Observations => {
bincode::serialize(&NestedObservation::word_similarity_search_updatable(
filter.map(|filter| bincode::deserialize::<ObservationFilter>(&filter)).transpose()?.as_ref(),
author_user_id,
query,
limit,
offset,
connection)?)?
            },
            web_common::database::Table::OrganismBioOttTaxonItems => {
bincode::serialize(&NestedOrganismBioOttTaxonItem::word_similarity_search_updatable(
filter.map(|filter| bincode::deserialize::<OrganismBioOttTaxonItemFilter>(&filter)).transpose()?.as_ref(),
author_user_id,
query,
limit,
offset,
connection)?)?
            },
            web_common::database::Table::Organisms => {
bincode::serialize(&NestedOrganism::word_similarity_search_updatable(
filter.map(|filter| bincode::deserialize::<OrganismFilter>(&filter)).transpose()?.as_ref(),
author_user_id,
query,
limit,
offset,
connection)?)?
            },
            web_common::database::Table::Organizations => unimplemented!("Method word_similarity_search_updatable not implemented for table organizations."),
            web_common::database::Table::PermanenceCategories => unimplemented!("Method word_similarity_search_updatable not implemented for table permanence_categories."),
            web_common::database::Table::ProjectStates => unimplemented!("Method word_similarity_search_updatable not implemented for table project_states."),
            web_common::database::Table::Projects => {
bincode::serialize(&NestedProject::word_similarity_search_updatable(
filter.map(|filter| bincode::deserialize::<ProjectFilter>(&filter)).transpose()?.as_ref(),
author_user_id,
query,
limit,
offset,
connection)?)?
            },
            web_common::database::Table::ProjectsTeamsRoleInvitations => {
bincode::serialize(&NestedProjectsTeamsRoleInvitation::word_similarity_search_updatable(
filter.map(|filter| bincode::deserialize::<ProjectsTeamsRoleInvitationFilter>(&filter)).transpose()?.as_ref(),
author_user_id,
query,
limit,
offset,
connection)?)?
            },
            web_common::database::Table::ProjectsTeamsRoleRequests => {
bincode::serialize(&NestedProjectsTeamsRoleRequest::word_similarity_search_updatable(
filter.map(|filter| bincode::deserialize::<ProjectsTeamsRoleRequestFilter>(&filter)).transpose()?.as_ref(),
author_user_id,
query,
limit,
offset,
connection)?)?
            },
            web_common::database::Table::ProjectsTeamsRoles => {
bincode::serialize(&NestedProjectsTeamsRole::word_similarity_search_updatable(
filter.map(|filter| bincode::deserialize::<ProjectsTeamsRoleFilter>(&filter)).transpose()?.as_ref(),
author_user_id,
query,
limit,
offset,
connection)?)?
            },
            web_common::database::Table::ProjectsUsersRoleInvitations => {
bincode::serialize(&NestedProjectsUsersRoleInvitation::word_similarity_search_updatable(
filter.map(|filter| bincode::deserialize::<ProjectsUsersRoleInvitationFilter>(&filter)).transpose()?.as_ref(),
author_user_id,
query,
limit,
offset,
connection)?)?
            },
            web_common::database::Table::ProjectsUsersRoleRequests => {
bincode::serialize(&NestedProjectsUsersRoleRequest::word_similarity_search_updatable(
filter.map(|filter| bincode::deserialize::<ProjectsUsersRoleRequestFilter>(&filter)).transpose()?.as_ref(),
author_user_id,
query,
limit,
offset,
connection)?)?
            },
            web_common::database::Table::ProjectsUsersRoles => {
bincode::serialize(&NestedProjectsUsersRole::word_similarity_search_updatable(
filter.map(|filter| bincode::deserialize::<ProjectsUsersRoleFilter>(&filter)).transpose()?.as_ref(),
author_user_id,
query,
limit,
offset,
connection)?)?
            },
            web_common::database::Table::Roles => unimplemented!("Method word_similarity_search_updatable not implemented for table roles."),
            web_common::database::Table::SampleBioOttTaxonItems => {
bincode::serialize(&NestedSampleBioOttTaxonItem::word_similarity_search_updatable(
filter.map(|filter| bincode::deserialize::<SampleBioOttTaxonItemFilter>(&filter)).transpose()?.as_ref(),
author_user_id,
query,
limit,
offset,
connection)?)?
            },
            web_common::database::Table::SampleContainerCategories => unimplemented!("Method word_similarity_search_updatable not implemented for table sample_container_categories."),
            web_common::database::Table::SampleContainers => {
bincode::serialize(&NestedSampleContainer::word_similarity_search_updatable(
filter.map(|filter| bincode::deserialize::<SampleContainerFilter>(&filter)).transpose()?.as_ref(),
author_user_id,
query,
limit,
offset,
connection)?)?
            },
            web_common::database::Table::SampleStates => unimplemented!("Method word_similarity_search_updatable not implemented for table sample_states."),
            web_common::database::Table::Samples => {
bincode::serialize(&NestedSample::word_similarity_search_updatable(
filter.map(|filter| bincode::deserialize::<SampleFilter>(&filter)).transpose()?.as_ref(),
author_user_id,
query,
limit,
offset,
connection)?)?
            },
            web_common::database::Table::Spectra => unimplemented!("Method word_similarity_search_updatable not implemented for table spectra."),
            web_common::database::Table::SpectraCollections => {
bincode::serialize(&NestedSpectraCollection::word_similarity_search_updatable(
filter.map(|filter| bincode::deserialize::<SpectraCollectionFilter>(&filter)).transpose()?.as_ref(),
author_user_id,
query,
limit,
offset,
connection)?)?
            },
            web_common::database::Table::TeamStates => unimplemented!("Method word_similarity_search_updatable not implemented for table team_states."),
            web_common::database::Table::Teams => {
bincode::serialize(&NestedTeam::word_similarity_search_updatable(
filter.map(|filter| bincode::deserialize::<TeamFilter>(&filter)).transpose()?.as_ref(),
author_user_id,
query,
limit,
offset,
connection)?)?
            },
            web_common::database::Table::TeamsTeamsRoleInvitations => {
bincode::serialize(&NestedTeamsTeamsRoleInvitation::word_similarity_search_updatable(
filter.map(|filter| bincode::deserialize::<TeamsTeamsRoleInvitationFilter>(&filter)).transpose()?.as_ref(),
author_user_id,
query,
limit,
offset,
connection)?)?
            },
            web_common::database::Table::TeamsUsersRoleInvitations => {
bincode::serialize(&NestedTeamsUsersRoleInvitation::word_similarity_search_updatable(
filter.map(|filter| bincode::deserialize::<TeamsUsersRoleInvitationFilter>(&filter)).transpose()?.as_ref(),
author_user_id,
query,
limit,
offset,
connection)?)?
            },
            web_common::database::Table::TeamsUsersRoleRequests => {
bincode::serialize(&NestedTeamsUsersRoleRequest::word_similarity_search_updatable(
filter.map(|filter| bincode::deserialize::<TeamsUsersRoleRequestFilter>(&filter)).transpose()?.as_ref(),
author_user_id,
query,
limit,
offset,
connection)?)?
            },
            web_common::database::Table::TeamsUsersRoles => {
bincode::serialize(&NestedTeamsUsersRole::word_similarity_search_updatable(
filter.map(|filter| bincode::deserialize::<TeamsUsersRoleFilter>(&filter)).transpose()?.as_ref(),
author_user_id,
query,
limit,
offset,
connection)?)?
            },
            web_common::database::Table::Units => unimplemented!("Method word_similarity_search_updatable not implemented for table units."),
            web_common::database::Table::UserEmails => unimplemented!("Method word_similarity_search_updatable not implemented for table user_emails."),
            web_common::database::Table::Users => {
bincode::serialize(&User::word_similarity_search_updatable(
author_user_id,
query,
limit,
offset,
connection)?)?
            },
            web_common::database::Table::UsersUsersRoleInvitations => {
bincode::serialize(&NestedUsersUsersRoleInvitation::word_similarity_search_updatable(
filter.map(|filter| bincode::deserialize::<UsersUsersRoleInvitationFilter>(&filter)).transpose()?.as_ref(),
author_user_id,
query,
limit,
offset,
connection)?)?
            },
            web_common::database::Table::UsersUsersRoleRequests => {
bincode::serialize(&NestedUsersUsersRoleRequest::word_similarity_search_updatable(
filter.map(|filter| bincode::deserialize::<UsersUsersRoleRequestFilter>(&filter)).transpose()?.as_ref(),
author_user_id,
query,
limit,
offset,
connection)?)?
            },
            web_common::database::Table::UsersUsersRoles => {
bincode::serialize(&NestedUsersUsersRole::word_similarity_search_updatable(
filter.map(|filter| bincode::deserialize::<UsersUsersRoleFilter>(&filter)).transpose()?.as_ref(),
author_user_id,
query,
limit,
offset,
connection)?)?
            },
        })
    }

    /// Search for the updatable structs by a given string by Postgres's `strict_word_similarity`.
    ///
    /// * `filter` - The optional filter to apply to the query.
    /// * `author_user_id` - The ID of the user who is performing the search.
    /// * `query` - The string to search for.
    /// * `limit` - The maximum number of results to return.
    /// * `offset` - The number of results to skip.
    /// * `connection` - The connection to the database.
    ///
     fn strict_word_similarity_search_updatable(
        &self,
filter: Option<Vec<u8>>,
author_user_id: i32,
query: &str,
limit: Option<i64>,
offset: Option<i64>,
connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>,
) -> Result<Vec<u8>, web_common::api::ApiError> {
        Ok(match self {
            web_common::database::Table::BioOttRanks => unimplemented!("Method strict_word_similarity_search_updatable not implemented for table bio_ott_ranks."),
            web_common::database::Table::BioOttTaxonItems => unimplemented!("Method strict_word_similarity_search_updatable not implemented for table bio_ott_taxon_items."),
            web_common::database::Table::Colors => unimplemented!("Method strict_word_similarity_search_updatable not implemented for table colors."),
            web_common::database::Table::Countries => unimplemented!("Method strict_word_similarity_search_updatable not implemented for table countries."),
            web_common::database::Table::DerivedSamples => {
bincode::serialize(&NestedDerivedSample::strict_word_similarity_search_updatable(
filter.map(|filter| bincode::deserialize::<DerivedSampleFilter>(&filter)).transpose()?.as_ref(),
author_user_id,
query,
limit,
offset,
connection)?)?
            },
            web_common::database::Table::DocumentFormats => unimplemented!("Method strict_word_similarity_search_updatable not implemented for table document_formats."),
            web_common::database::Table::FontAwesomeIcons => unimplemented!("Method strict_word_similarity_search_updatable not implemented for table font_awesome_icons."),
            web_common::database::Table::LoginProviders => unimplemented!("Method strict_word_similarity_search_updatable not implemented for table login_providers."),
            web_common::database::Table::Materials => unimplemented!("Method strict_word_similarity_search_updatable not implemented for table materials."),
            web_common::database::Table::NameplateCategories => unimplemented!("Method strict_word_similarity_search_updatable not implemented for table nameplate_categories."),
            web_common::database::Table::Nameplates => {
bincode::serialize(&NestedNameplate::strict_word_similarity_search_updatable(
filter.map(|filter| bincode::deserialize::<NameplateFilter>(&filter)).transpose()?.as_ref(),
author_user_id,
query,
limit,
offset,
connection)?)?
            },
            web_common::database::Table::Notifications => unimplemented!("Method strict_word_similarity_search_updatable not implemented for table notifications."),
            web_common::database::Table::ObservationSubjects => unimplemented!("Method strict_word_similarity_search_updatable not implemented for table observation_subjects."),
            web_common::database::Table::Observations => {
bincode::serialize(&NestedObservation::strict_word_similarity_search_updatable(
filter.map(|filter| bincode::deserialize::<ObservationFilter>(&filter)).transpose()?.as_ref(),
author_user_id,
query,
limit,
offset,
connection)?)?
            },
            web_common::database::Table::OrganismBioOttTaxonItems => {
bincode::serialize(&NestedOrganismBioOttTaxonItem::strict_word_similarity_search_updatable(
filter.map(|filter| bincode::deserialize::<OrganismBioOttTaxonItemFilter>(&filter)).transpose()?.as_ref(),
author_user_id,
query,
limit,
offset,
connection)?)?
            },
            web_common::database::Table::Organisms => {
bincode::serialize(&NestedOrganism::strict_word_similarity_search_updatable(
filter.map(|filter| bincode::deserialize::<OrganismFilter>(&filter)).transpose()?.as_ref(),
author_user_id,
query,
limit,
offset,
connection)?)?
            },
            web_common::database::Table::Organizations => unimplemented!("Method strict_word_similarity_search_updatable not implemented for table organizations."),
            web_common::database::Table::PermanenceCategories => unimplemented!("Method strict_word_similarity_search_updatable not implemented for table permanence_categories."),
            web_common::database::Table::ProjectStates => unimplemented!("Method strict_word_similarity_search_updatable not implemented for table project_states."),
            web_common::database::Table::Projects => {
bincode::serialize(&NestedProject::strict_word_similarity_search_updatable(
filter.map(|filter| bincode::deserialize::<ProjectFilter>(&filter)).transpose()?.as_ref(),
author_user_id,
query,
limit,
offset,
connection)?)?
            },
            web_common::database::Table::ProjectsTeamsRoleInvitations => {
bincode::serialize(&NestedProjectsTeamsRoleInvitation::strict_word_similarity_search_updatable(
filter.map(|filter| bincode::deserialize::<ProjectsTeamsRoleInvitationFilter>(&filter)).transpose()?.as_ref(),
author_user_id,
query,
limit,
offset,
connection)?)?
            },
            web_common::database::Table::ProjectsTeamsRoleRequests => {
bincode::serialize(&NestedProjectsTeamsRoleRequest::strict_word_similarity_search_updatable(
filter.map(|filter| bincode::deserialize::<ProjectsTeamsRoleRequestFilter>(&filter)).transpose()?.as_ref(),
author_user_id,
query,
limit,
offset,
connection)?)?
            },
            web_common::database::Table::ProjectsTeamsRoles => {
bincode::serialize(&NestedProjectsTeamsRole::strict_word_similarity_search_updatable(
filter.map(|filter| bincode::deserialize::<ProjectsTeamsRoleFilter>(&filter)).transpose()?.as_ref(),
author_user_id,
query,
limit,
offset,
connection)?)?
            },
            web_common::database::Table::ProjectsUsersRoleInvitations => {
bincode::serialize(&NestedProjectsUsersRoleInvitation::strict_word_similarity_search_updatable(
filter.map(|filter| bincode::deserialize::<ProjectsUsersRoleInvitationFilter>(&filter)).transpose()?.as_ref(),
author_user_id,
query,
limit,
offset,
connection)?)?
            },
            web_common::database::Table::ProjectsUsersRoleRequests => {
bincode::serialize(&NestedProjectsUsersRoleRequest::strict_word_similarity_search_updatable(
filter.map(|filter| bincode::deserialize::<ProjectsUsersRoleRequestFilter>(&filter)).transpose()?.as_ref(),
author_user_id,
query,
limit,
offset,
connection)?)?
            },
            web_common::database::Table::ProjectsUsersRoles => {
bincode::serialize(&NestedProjectsUsersRole::strict_word_similarity_search_updatable(
filter.map(|filter| bincode::deserialize::<ProjectsUsersRoleFilter>(&filter)).transpose()?.as_ref(),
author_user_id,
query,
limit,
offset,
connection)?)?
            },
            web_common::database::Table::Roles => unimplemented!("Method strict_word_similarity_search_updatable not implemented for table roles."),
            web_common::database::Table::SampleBioOttTaxonItems => {
bincode::serialize(&NestedSampleBioOttTaxonItem::strict_word_similarity_search_updatable(
filter.map(|filter| bincode::deserialize::<SampleBioOttTaxonItemFilter>(&filter)).transpose()?.as_ref(),
author_user_id,
query,
limit,
offset,
connection)?)?
            },
            web_common::database::Table::SampleContainerCategories => unimplemented!("Method strict_word_similarity_search_updatable not implemented for table sample_container_categories."),
            web_common::database::Table::SampleContainers => {
bincode::serialize(&NestedSampleContainer::strict_word_similarity_search_updatable(
filter.map(|filter| bincode::deserialize::<SampleContainerFilter>(&filter)).transpose()?.as_ref(),
author_user_id,
query,
limit,
offset,
connection)?)?
            },
            web_common::database::Table::SampleStates => unimplemented!("Method strict_word_similarity_search_updatable not implemented for table sample_states."),
            web_common::database::Table::Samples => {
bincode::serialize(&NestedSample::strict_word_similarity_search_updatable(
filter.map(|filter| bincode::deserialize::<SampleFilter>(&filter)).transpose()?.as_ref(),
author_user_id,
query,
limit,
offset,
connection)?)?
            },
            web_common::database::Table::Spectra => unimplemented!("Method strict_word_similarity_search_updatable not implemented for table spectra."),
            web_common::database::Table::SpectraCollections => {
bincode::serialize(&NestedSpectraCollection::strict_word_similarity_search_updatable(
filter.map(|filter| bincode::deserialize::<SpectraCollectionFilter>(&filter)).transpose()?.as_ref(),
author_user_id,
query,
limit,
offset,
connection)?)?
            },
            web_common::database::Table::TeamStates => unimplemented!("Method strict_word_similarity_search_updatable not implemented for table team_states."),
            web_common::database::Table::Teams => {
bincode::serialize(&NestedTeam::strict_word_similarity_search_updatable(
filter.map(|filter| bincode::deserialize::<TeamFilter>(&filter)).transpose()?.as_ref(),
author_user_id,
query,
limit,
offset,
connection)?)?
            },
            web_common::database::Table::TeamsTeamsRoleInvitations => {
bincode::serialize(&NestedTeamsTeamsRoleInvitation::strict_word_similarity_search_updatable(
filter.map(|filter| bincode::deserialize::<TeamsTeamsRoleInvitationFilter>(&filter)).transpose()?.as_ref(),
author_user_id,
query,
limit,
offset,
connection)?)?
            },
            web_common::database::Table::TeamsUsersRoleInvitations => {
bincode::serialize(&NestedTeamsUsersRoleInvitation::strict_word_similarity_search_updatable(
filter.map(|filter| bincode::deserialize::<TeamsUsersRoleInvitationFilter>(&filter)).transpose()?.as_ref(),
author_user_id,
query,
limit,
offset,
connection)?)?
            },
            web_common::database::Table::TeamsUsersRoleRequests => {
bincode::serialize(&NestedTeamsUsersRoleRequest::strict_word_similarity_search_updatable(
filter.map(|filter| bincode::deserialize::<TeamsUsersRoleRequestFilter>(&filter)).transpose()?.as_ref(),
author_user_id,
query,
limit,
offset,
connection)?)?
            },
            web_common::database::Table::TeamsUsersRoles => {
bincode::serialize(&NestedTeamsUsersRole::strict_word_similarity_search_updatable(
filter.map(|filter| bincode::deserialize::<TeamsUsersRoleFilter>(&filter)).transpose()?.as_ref(),
author_user_id,
query,
limit,
offset,
connection)?)?
            },
            web_common::database::Table::Units => unimplemented!("Method strict_word_similarity_search_updatable not implemented for table units."),
            web_common::database::Table::UserEmails => unimplemented!("Method strict_word_similarity_search_updatable not implemented for table user_emails."),
            web_common::database::Table::Users => {
bincode::serialize(&User::strict_word_similarity_search_updatable(
author_user_id,
query,
limit,
offset,
connection)?)?
            },
            web_common::database::Table::UsersUsersRoleInvitations => {
bincode::serialize(&NestedUsersUsersRoleInvitation::strict_word_similarity_search_updatable(
filter.map(|filter| bincode::deserialize::<UsersUsersRoleInvitationFilter>(&filter)).transpose()?.as_ref(),
author_user_id,
query,
limit,
offset,
connection)?)?
            },
            web_common::database::Table::UsersUsersRoleRequests => {
bincode::serialize(&NestedUsersUsersRoleRequest::strict_word_similarity_search_updatable(
filter.map(|filter| bincode::deserialize::<UsersUsersRoleRequestFilter>(&filter)).transpose()?.as_ref(),
author_user_id,
query,
limit,
offset,
connection)?)?
            },
            web_common::database::Table::UsersUsersRoles => {
bincode::serialize(&NestedUsersUsersRole::strict_word_similarity_search_updatable(
filter.map(|filter| bincode::deserialize::<UsersUsersRoleFilter>(&filter)).transpose()?.as_ref(),
author_user_id,
query,
limit,
offset,
connection)?)?
            },
        })
    }

    /// Check whether the user can admin the struct associated to the provided ids.
    ///
    /// * `primary_key` - The primary key(s) of the struct to check.
    /// * `author_user_id` - The ID of the user to check.
    /// * `connection` - The connection to the database.
    ///
     fn can_admin_by_id(
        &self,
primary_key: PrimaryKey,
author_user_id: i32,
connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>,
) -> Result<bool, web_common::api::ApiError> {
        Ok(match self {
            web_common::database::Table::BioOttRanks => unimplemented!("Method can_admin_by_id not implemented for table bio_ott_ranks."),
            web_common::database::Table::BioOttTaxonItems => unimplemented!("Method can_admin_by_id not implemented for table bio_ott_taxon_items."),
            web_common::database::Table::Colors => unimplemented!("Method can_admin_by_id not implemented for table colors."),
            web_common::database::Table::Countries => unimplemented!("Method can_admin_by_id not implemented for table countries."),
            web_common::database::Table::DerivedSamples => {
NestedDerivedSample::can_admin_by_id(
primary_key.into(),
author_user_id,
connection)?            },
            web_common::database::Table::DocumentFormats => unimplemented!("Method can_admin_by_id not implemented for table document_formats."),
            web_common::database::Table::FontAwesomeIcons => unimplemented!("Method can_admin_by_id not implemented for table font_awesome_icons."),
            web_common::database::Table::LoginProviders => unimplemented!("Method can_admin_by_id not implemented for table login_providers."),
            web_common::database::Table::Materials => unimplemented!("Method can_admin_by_id not implemented for table materials."),
            web_common::database::Table::NameplateCategories => unimplemented!("Method can_admin_by_id not implemented for table nameplate_categories."),
            web_common::database::Table::Nameplates => {
NestedNameplate::can_admin_by_id(
primary_key.into(),
author_user_id,
connection)?            },
            web_common::database::Table::Notifications => unimplemented!("Method can_admin_by_id not implemented for table notifications."),
            web_common::database::Table::ObservationSubjects => unimplemented!("Method can_admin_by_id not implemented for table observation_subjects."),
            web_common::database::Table::Observations => {
NestedObservation::can_admin_by_id(
primary_key.into(),
author_user_id,
connection)?            },
            web_common::database::Table::OrganismBioOttTaxonItems => {
NestedOrganismBioOttTaxonItem::can_admin_by_id(
primary_key.into(),
author_user_id,
connection)?            },
            web_common::database::Table::Organisms => {
NestedOrganism::can_admin_by_id(
primary_key.into(),
author_user_id,
connection)?            },
            web_common::database::Table::Organizations => unimplemented!("Method can_admin_by_id not implemented for table organizations."),
            web_common::database::Table::PermanenceCategories => unimplemented!("Method can_admin_by_id not implemented for table permanence_categories."),
            web_common::database::Table::ProjectStates => unimplemented!("Method can_admin_by_id not implemented for table project_states."),
            web_common::database::Table::Projects => {
NestedProject::can_admin_by_id(
primary_key.into(),
author_user_id,
connection)?            },
            web_common::database::Table::ProjectsTeamsRoleInvitations => {
NestedProjectsTeamsRoleInvitation::can_admin_by_id(
primary_key.into(),
author_user_id,
connection)?            },
            web_common::database::Table::ProjectsTeamsRoleRequests => {
NestedProjectsTeamsRoleRequest::can_admin_by_id(
primary_key.into(),
author_user_id,
connection)?            },
            web_common::database::Table::ProjectsTeamsRoles => {
NestedProjectsTeamsRole::can_admin_by_id(
primary_key.into(),
author_user_id,
connection)?            },
            web_common::database::Table::ProjectsUsersRoleInvitations => {
NestedProjectsUsersRoleInvitation::can_admin_by_id(
primary_key.into(),
author_user_id,
connection)?            },
            web_common::database::Table::ProjectsUsersRoleRequests => {
NestedProjectsUsersRoleRequest::can_admin_by_id(
primary_key.into(),
author_user_id,
connection)?            },
            web_common::database::Table::ProjectsUsersRoles => {
NestedProjectsUsersRole::can_admin_by_id(
primary_key.into(),
author_user_id,
connection)?            },
            web_common::database::Table::Roles => unimplemented!("Method can_admin_by_id not implemented for table roles."),
            web_common::database::Table::SampleBioOttTaxonItems => {
NestedSampleBioOttTaxonItem::can_admin_by_id(
primary_key.into(),
author_user_id,
connection)?            },
            web_common::database::Table::SampleContainerCategories => unimplemented!("Method can_admin_by_id not implemented for table sample_container_categories."),
            web_common::database::Table::SampleContainers => {
NestedSampleContainer::can_admin_by_id(
primary_key.into(),
author_user_id,
connection)?            },
            web_common::database::Table::SampleStates => unimplemented!("Method can_admin_by_id not implemented for table sample_states."),
            web_common::database::Table::Samples => {
NestedSample::can_admin_by_id(
primary_key.into(),
author_user_id,
connection)?            },
            web_common::database::Table::Spectra => {
NestedSpectra::can_admin_by_id(
primary_key.into(),
author_user_id,
connection)?            },
            web_common::database::Table::SpectraCollections => {
NestedSpectraCollection::can_admin_by_id(
primary_key.into(),
author_user_id,
connection)?            },
            web_common::database::Table::TeamStates => unimplemented!("Method can_admin_by_id not implemented for table team_states."),
            web_common::database::Table::Teams => {
NestedTeam::can_admin_by_id(
primary_key.into(),
author_user_id,
connection)?            },
            web_common::database::Table::TeamsTeamsRoleInvitations => {
NestedTeamsTeamsRoleInvitation::can_admin_by_id(
primary_key.into(),
author_user_id,
connection)?            },
            web_common::database::Table::TeamsUsersRoleInvitations => {
NestedTeamsUsersRoleInvitation::can_admin_by_id(
primary_key.into(),
author_user_id,
connection)?            },
            web_common::database::Table::TeamsUsersRoleRequests => {
NestedTeamsUsersRoleRequest::can_admin_by_id(
primary_key.into(),
author_user_id,
connection)?            },
            web_common::database::Table::TeamsUsersRoles => {
NestedTeamsUsersRole::can_admin_by_id(
primary_key.into(),
author_user_id,
connection)?            },
            web_common::database::Table::Units => unimplemented!("Method can_admin_by_id not implemented for table units."),
            web_common::database::Table::UserEmails => {
NestedUserEmail::can_admin_by_id(
primary_key.into(),
author_user_id,
connection)?            },
            web_common::database::Table::Users => {
User::can_admin_by_id(
primary_key.into(),
author_user_id,
connection)?            },
            web_common::database::Table::UsersUsersRoleInvitations => {
NestedUsersUsersRoleInvitation::can_admin_by_id(
primary_key.into(),
author_user_id,
connection)?            },
            web_common::database::Table::UsersUsersRoleRequests => {
NestedUsersUsersRoleRequest::can_admin_by_id(
primary_key.into(),
author_user_id,
connection)?            },
            web_common::database::Table::UsersUsersRoles => {
NestedUsersUsersRole::can_admin_by_id(
primary_key.into(),
author_user_id,
connection)?            },
        })
    }

    /// Get all of the administrable structs from the database.
    ///
    /// * `filter` - The optional filter to apply to the query.
    /// * `author_user_id` - The ID of the user who is performing the search.
    /// * `limit` - The maximum number of results to return.
    /// * `offset` - The number of results to skip.
    /// * `connection` - The connection to the database.
    ///
     fn all_administrable(
        &self,
filter: Option<Vec<u8>>,
author_user_id: i32,
limit: Option<i64>,
offset: Option<i64>,
connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>,
) -> Result<Vec<u8>, web_common::api::ApiError> {
        Ok(match self {
            web_common::database::Table::BioOttRanks => unimplemented!("Method all_administrable not implemented for table bio_ott_ranks."),
            web_common::database::Table::BioOttTaxonItems => unimplemented!("Method all_administrable not implemented for table bio_ott_taxon_items."),
            web_common::database::Table::Colors => unimplemented!("Method all_administrable not implemented for table colors."),
            web_common::database::Table::Countries => unimplemented!("Method all_administrable not implemented for table countries."),
            web_common::database::Table::DerivedSamples => {
bincode::serialize(&NestedDerivedSample::all_administrable(
filter.map(|filter| bincode::deserialize::<DerivedSampleFilter>(&filter)).transpose()?.as_ref(),
author_user_id,
limit,
offset,
connection)?)?
            },
            web_common::database::Table::DocumentFormats => unimplemented!("Method all_administrable not implemented for table document_formats."),
            web_common::database::Table::FontAwesomeIcons => unimplemented!("Method all_administrable not implemented for table font_awesome_icons."),
            web_common::database::Table::LoginProviders => unimplemented!("Method all_administrable not implemented for table login_providers."),
            web_common::database::Table::Materials => unimplemented!("Method all_administrable not implemented for table materials."),
            web_common::database::Table::NameplateCategories => unimplemented!("Method all_administrable not implemented for table nameplate_categories."),
            web_common::database::Table::Nameplates => {
bincode::serialize(&NestedNameplate::all_administrable(
filter.map(|filter| bincode::deserialize::<NameplateFilter>(&filter)).transpose()?.as_ref(),
author_user_id,
limit,
offset,
connection)?)?
            },
            web_common::database::Table::Notifications => unimplemented!("Method all_administrable not implemented for table notifications."),
            web_common::database::Table::ObservationSubjects => unimplemented!("Method all_administrable not implemented for table observation_subjects."),
            web_common::database::Table::Observations => {
bincode::serialize(&NestedObservation::all_administrable(
filter.map(|filter| bincode::deserialize::<ObservationFilter>(&filter)).transpose()?.as_ref(),
author_user_id,
limit,
offset,
connection)?)?
            },
            web_common::database::Table::OrganismBioOttTaxonItems => {
bincode::serialize(&NestedOrganismBioOttTaxonItem::all_administrable(
filter.map(|filter| bincode::deserialize::<OrganismBioOttTaxonItemFilter>(&filter)).transpose()?.as_ref(),
author_user_id,
limit,
offset,
connection)?)?
            },
            web_common::database::Table::Organisms => {
bincode::serialize(&NestedOrganism::all_administrable(
filter.map(|filter| bincode::deserialize::<OrganismFilter>(&filter)).transpose()?.as_ref(),
author_user_id,
limit,
offset,
connection)?)?
            },
            web_common::database::Table::Organizations => unimplemented!("Method all_administrable not implemented for table organizations."),
            web_common::database::Table::PermanenceCategories => unimplemented!("Method all_administrable not implemented for table permanence_categories."),
            web_common::database::Table::ProjectStates => unimplemented!("Method all_administrable not implemented for table project_states."),
            web_common::database::Table::Projects => {
bincode::serialize(&NestedProject::all_administrable(
filter.map(|filter| bincode::deserialize::<ProjectFilter>(&filter)).transpose()?.as_ref(),
author_user_id,
limit,
offset,
connection)?)?
            },
            web_common::database::Table::ProjectsTeamsRoleInvitations => {
bincode::serialize(&NestedProjectsTeamsRoleInvitation::all_administrable(
filter.map(|filter| bincode::deserialize::<ProjectsTeamsRoleInvitationFilter>(&filter)).transpose()?.as_ref(),
author_user_id,
limit,
offset,
connection)?)?
            },
            web_common::database::Table::ProjectsTeamsRoleRequests => {
bincode::serialize(&NestedProjectsTeamsRoleRequest::all_administrable(
filter.map(|filter| bincode::deserialize::<ProjectsTeamsRoleRequestFilter>(&filter)).transpose()?.as_ref(),
author_user_id,
limit,
offset,
connection)?)?
            },
            web_common::database::Table::ProjectsTeamsRoles => {
bincode::serialize(&NestedProjectsTeamsRole::all_administrable(
filter.map(|filter| bincode::deserialize::<ProjectsTeamsRoleFilter>(&filter)).transpose()?.as_ref(),
author_user_id,
limit,
offset,
connection)?)?
            },
            web_common::database::Table::ProjectsUsersRoleInvitations => {
bincode::serialize(&NestedProjectsUsersRoleInvitation::all_administrable(
filter.map(|filter| bincode::deserialize::<ProjectsUsersRoleInvitationFilter>(&filter)).transpose()?.as_ref(),
author_user_id,
limit,
offset,
connection)?)?
            },
            web_common::database::Table::ProjectsUsersRoleRequests => {
bincode::serialize(&NestedProjectsUsersRoleRequest::all_administrable(
filter.map(|filter| bincode::deserialize::<ProjectsUsersRoleRequestFilter>(&filter)).transpose()?.as_ref(),
author_user_id,
limit,
offset,
connection)?)?
            },
            web_common::database::Table::ProjectsUsersRoles => {
bincode::serialize(&NestedProjectsUsersRole::all_administrable(
filter.map(|filter| bincode::deserialize::<ProjectsUsersRoleFilter>(&filter)).transpose()?.as_ref(),
author_user_id,
limit,
offset,
connection)?)?
            },
            web_common::database::Table::Roles => unimplemented!("Method all_administrable not implemented for table roles."),
            web_common::database::Table::SampleBioOttTaxonItems => {
bincode::serialize(&NestedSampleBioOttTaxonItem::all_administrable(
filter.map(|filter| bincode::deserialize::<SampleBioOttTaxonItemFilter>(&filter)).transpose()?.as_ref(),
author_user_id,
limit,
offset,
connection)?)?
            },
            web_common::database::Table::SampleContainerCategories => unimplemented!("Method all_administrable not implemented for table sample_container_categories."),
            web_common::database::Table::SampleContainers => {
bincode::serialize(&NestedSampleContainer::all_administrable(
filter.map(|filter| bincode::deserialize::<SampleContainerFilter>(&filter)).transpose()?.as_ref(),
author_user_id,
limit,
offset,
connection)?)?
            },
            web_common::database::Table::SampleStates => unimplemented!("Method all_administrable not implemented for table sample_states."),
            web_common::database::Table::Samples => {
bincode::serialize(&NestedSample::all_administrable(
filter.map(|filter| bincode::deserialize::<SampleFilter>(&filter)).transpose()?.as_ref(),
author_user_id,
limit,
offset,
connection)?)?
            },
            web_common::database::Table::Spectra => {
bincode::serialize(&NestedSpectra::all_administrable(
filter.map(|filter| bincode::deserialize::<SpectraFilter>(&filter)).transpose()?.as_ref(),
author_user_id,
limit,
offset,
connection)?)?
            },
            web_common::database::Table::SpectraCollections => {
bincode::serialize(&NestedSpectraCollection::all_administrable(
filter.map(|filter| bincode::deserialize::<SpectraCollectionFilter>(&filter)).transpose()?.as_ref(),
author_user_id,
limit,
offset,
connection)?)?
            },
            web_common::database::Table::TeamStates => unimplemented!("Method all_administrable not implemented for table team_states."),
            web_common::database::Table::Teams => {
bincode::serialize(&NestedTeam::all_administrable(
filter.map(|filter| bincode::deserialize::<TeamFilter>(&filter)).transpose()?.as_ref(),
author_user_id,
limit,
offset,
connection)?)?
            },
            web_common::database::Table::TeamsTeamsRoleInvitations => {
bincode::serialize(&NestedTeamsTeamsRoleInvitation::all_administrable(
filter.map(|filter| bincode::deserialize::<TeamsTeamsRoleInvitationFilter>(&filter)).transpose()?.as_ref(),
author_user_id,
limit,
offset,
connection)?)?
            },
            web_common::database::Table::TeamsUsersRoleInvitations => {
bincode::serialize(&NestedTeamsUsersRoleInvitation::all_administrable(
filter.map(|filter| bincode::deserialize::<TeamsUsersRoleInvitationFilter>(&filter)).transpose()?.as_ref(),
author_user_id,
limit,
offset,
connection)?)?
            },
            web_common::database::Table::TeamsUsersRoleRequests => {
bincode::serialize(&NestedTeamsUsersRoleRequest::all_administrable(
filter.map(|filter| bincode::deserialize::<TeamsUsersRoleRequestFilter>(&filter)).transpose()?.as_ref(),
author_user_id,
limit,
offset,
connection)?)?
            },
            web_common::database::Table::TeamsUsersRoles => {
bincode::serialize(&NestedTeamsUsersRole::all_administrable(
filter.map(|filter| bincode::deserialize::<TeamsUsersRoleFilter>(&filter)).transpose()?.as_ref(),
author_user_id,
limit,
offset,
connection)?)?
            },
            web_common::database::Table::Units => unimplemented!("Method all_administrable not implemented for table units."),
            web_common::database::Table::UserEmails => {
bincode::serialize(&NestedUserEmail::all_administrable(
filter.map(|filter| bincode::deserialize::<UserEmailFilter>(&filter)).transpose()?.as_ref(),
author_user_id,
limit,
offset,
connection)?)?
            },
            web_common::database::Table::Users => {
bincode::serialize(&User::all_administrable(
author_user_id,
limit,
offset,
connection)?)?
            },
            web_common::database::Table::UsersUsersRoleInvitations => {
bincode::serialize(&NestedUsersUsersRoleInvitation::all_administrable(
filter.map(|filter| bincode::deserialize::<UsersUsersRoleInvitationFilter>(&filter)).transpose()?.as_ref(),
author_user_id,
limit,
offset,
connection)?)?
            },
            web_common::database::Table::UsersUsersRoleRequests => {
bincode::serialize(&NestedUsersUsersRoleRequest::all_administrable(
filter.map(|filter| bincode::deserialize::<UsersUsersRoleRequestFilter>(&filter)).transpose()?.as_ref(),
author_user_id,
limit,
offset,
connection)?)?
            },
            web_common::database::Table::UsersUsersRoles => {
bincode::serialize(&NestedUsersUsersRole::all_administrable(
filter.map(|filter| bincode::deserialize::<UsersUsersRoleFilter>(&filter)).transpose()?.as_ref(),
author_user_id,
limit,
offset,
connection)?)?
            },
        })
    }

    /// Get all of the sorted administrable structs from the database.
    ///
    /// * `filter` - The optional filter to apply to the query.
    /// * `author_user_id` - The ID of the user who is performing the search.
    /// * `limit` - The maximum number of results to return.
    /// * `offset` - The number of results to skip.
    /// * `connection` - The connection to the database.
    ///
     fn all_administrable_sorted(
        &self,
filter: Option<Vec<u8>>,
author_user_id: i32,
limit: Option<i64>,
offset: Option<i64>,
connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>,
) -> Result<Vec<u8>, web_common::api::ApiError> {
        Ok(match self {
            web_common::database::Table::BioOttRanks => unimplemented!("Method all_administrable_sorted not implemented for table bio_ott_ranks."),
            web_common::database::Table::BioOttTaxonItems => unimplemented!("Method all_administrable_sorted not implemented for table bio_ott_taxon_items."),
            web_common::database::Table::Colors => unimplemented!("Method all_administrable_sorted not implemented for table colors."),
            web_common::database::Table::Countries => unimplemented!("Method all_administrable_sorted not implemented for table countries."),
            web_common::database::Table::DerivedSamples => {
bincode::serialize(&NestedDerivedSample::all_administrable_sorted(
filter.map(|filter| bincode::deserialize::<DerivedSampleFilter>(&filter)).transpose()?.as_ref(),
author_user_id,
limit,
offset,
connection)?)?
            },
            web_common::database::Table::DocumentFormats => unimplemented!("Method all_administrable_sorted not implemented for table document_formats."),
            web_common::database::Table::FontAwesomeIcons => unimplemented!("Method all_administrable_sorted not implemented for table font_awesome_icons."),
            web_common::database::Table::LoginProviders => unimplemented!("Method all_administrable_sorted not implemented for table login_providers."),
            web_common::database::Table::Materials => unimplemented!("Method all_administrable_sorted not implemented for table materials."),
            web_common::database::Table::NameplateCategories => unimplemented!("Method all_administrable_sorted not implemented for table nameplate_categories."),
            web_common::database::Table::Nameplates => {
bincode::serialize(&NestedNameplate::all_administrable_sorted(
filter.map(|filter| bincode::deserialize::<NameplateFilter>(&filter)).transpose()?.as_ref(),
author_user_id,
limit,
offset,
connection)?)?
            },
            web_common::database::Table::Notifications => unimplemented!("Method all_administrable_sorted not implemented for table notifications."),
            web_common::database::Table::ObservationSubjects => unimplemented!("Method all_administrable_sorted not implemented for table observation_subjects."),
            web_common::database::Table::Observations => {
bincode::serialize(&NestedObservation::all_administrable_sorted(
filter.map(|filter| bincode::deserialize::<ObservationFilter>(&filter)).transpose()?.as_ref(),
author_user_id,
limit,
offset,
connection)?)?
            },
            web_common::database::Table::OrganismBioOttTaxonItems => {
bincode::serialize(&NestedOrganismBioOttTaxonItem::all_administrable_sorted(
filter.map(|filter| bincode::deserialize::<OrganismBioOttTaxonItemFilter>(&filter)).transpose()?.as_ref(),
author_user_id,
limit,
offset,
connection)?)?
            },
            web_common::database::Table::Organisms => {
bincode::serialize(&NestedOrganism::all_administrable_sorted(
filter.map(|filter| bincode::deserialize::<OrganismFilter>(&filter)).transpose()?.as_ref(),
author_user_id,
limit,
offset,
connection)?)?
            },
            web_common::database::Table::Organizations => unimplemented!("Method all_administrable_sorted not implemented for table organizations."),
            web_common::database::Table::PermanenceCategories => unimplemented!("Method all_administrable_sorted not implemented for table permanence_categories."),
            web_common::database::Table::ProjectStates => unimplemented!("Method all_administrable_sorted not implemented for table project_states."),
            web_common::database::Table::Projects => {
bincode::serialize(&NestedProject::all_administrable_sorted(
filter.map(|filter| bincode::deserialize::<ProjectFilter>(&filter)).transpose()?.as_ref(),
author_user_id,
limit,
offset,
connection)?)?
            },
            web_common::database::Table::ProjectsTeamsRoleInvitations => {
bincode::serialize(&NestedProjectsTeamsRoleInvitation::all_administrable_sorted(
filter.map(|filter| bincode::deserialize::<ProjectsTeamsRoleInvitationFilter>(&filter)).transpose()?.as_ref(),
author_user_id,
limit,
offset,
connection)?)?
            },
            web_common::database::Table::ProjectsTeamsRoleRequests => {
bincode::serialize(&NestedProjectsTeamsRoleRequest::all_administrable_sorted(
filter.map(|filter| bincode::deserialize::<ProjectsTeamsRoleRequestFilter>(&filter)).transpose()?.as_ref(),
author_user_id,
limit,
offset,
connection)?)?
            },
            web_common::database::Table::ProjectsTeamsRoles => {
bincode::serialize(&NestedProjectsTeamsRole::all_administrable_sorted(
filter.map(|filter| bincode::deserialize::<ProjectsTeamsRoleFilter>(&filter)).transpose()?.as_ref(),
author_user_id,
limit,
offset,
connection)?)?
            },
            web_common::database::Table::ProjectsUsersRoleInvitations => {
bincode::serialize(&NestedProjectsUsersRoleInvitation::all_administrable_sorted(
filter.map(|filter| bincode::deserialize::<ProjectsUsersRoleInvitationFilter>(&filter)).transpose()?.as_ref(),
author_user_id,
limit,
offset,
connection)?)?
            },
            web_common::database::Table::ProjectsUsersRoleRequests => {
bincode::serialize(&NestedProjectsUsersRoleRequest::all_administrable_sorted(
filter.map(|filter| bincode::deserialize::<ProjectsUsersRoleRequestFilter>(&filter)).transpose()?.as_ref(),
author_user_id,
limit,
offset,
connection)?)?
            },
            web_common::database::Table::ProjectsUsersRoles => {
bincode::serialize(&NestedProjectsUsersRole::all_administrable_sorted(
filter.map(|filter| bincode::deserialize::<ProjectsUsersRoleFilter>(&filter)).transpose()?.as_ref(),
author_user_id,
limit,
offset,
connection)?)?
            },
            web_common::database::Table::Roles => unimplemented!("Method all_administrable_sorted not implemented for table roles."),
            web_common::database::Table::SampleBioOttTaxonItems => {
bincode::serialize(&NestedSampleBioOttTaxonItem::all_administrable_sorted(
filter.map(|filter| bincode::deserialize::<SampleBioOttTaxonItemFilter>(&filter)).transpose()?.as_ref(),
author_user_id,
limit,
offset,
connection)?)?
            },
            web_common::database::Table::SampleContainerCategories => unimplemented!("Method all_administrable_sorted not implemented for table sample_container_categories."),
            web_common::database::Table::SampleContainers => {
bincode::serialize(&NestedSampleContainer::all_administrable_sorted(
filter.map(|filter| bincode::deserialize::<SampleContainerFilter>(&filter)).transpose()?.as_ref(),
author_user_id,
limit,
offset,
connection)?)?
            },
            web_common::database::Table::SampleStates => unimplemented!("Method all_administrable_sorted not implemented for table sample_states."),
            web_common::database::Table::Samples => {
bincode::serialize(&NestedSample::all_administrable_sorted(
filter.map(|filter| bincode::deserialize::<SampleFilter>(&filter)).transpose()?.as_ref(),
author_user_id,
limit,
offset,
connection)?)?
            },
            web_common::database::Table::Spectra => {
bincode::serialize(&NestedSpectra::all_administrable_sorted(
filter.map(|filter| bincode::deserialize::<SpectraFilter>(&filter)).transpose()?.as_ref(),
author_user_id,
limit,
offset,
connection)?)?
            },
            web_common::database::Table::SpectraCollections => {
bincode::serialize(&NestedSpectraCollection::all_administrable_sorted(
filter.map(|filter| bincode::deserialize::<SpectraCollectionFilter>(&filter)).transpose()?.as_ref(),
author_user_id,
limit,
offset,
connection)?)?
            },
            web_common::database::Table::TeamStates => unimplemented!("Method all_administrable_sorted not implemented for table team_states."),
            web_common::database::Table::Teams => {
bincode::serialize(&NestedTeam::all_administrable_sorted(
filter.map(|filter| bincode::deserialize::<TeamFilter>(&filter)).transpose()?.as_ref(),
author_user_id,
limit,
offset,
connection)?)?
            },
            web_common::database::Table::TeamsTeamsRoleInvitations => {
bincode::serialize(&NestedTeamsTeamsRoleInvitation::all_administrable_sorted(
filter.map(|filter| bincode::deserialize::<TeamsTeamsRoleInvitationFilter>(&filter)).transpose()?.as_ref(),
author_user_id,
limit,
offset,
connection)?)?
            },
            web_common::database::Table::TeamsUsersRoleInvitations => {
bincode::serialize(&NestedTeamsUsersRoleInvitation::all_administrable_sorted(
filter.map(|filter| bincode::deserialize::<TeamsUsersRoleInvitationFilter>(&filter)).transpose()?.as_ref(),
author_user_id,
limit,
offset,
connection)?)?
            },
            web_common::database::Table::TeamsUsersRoleRequests => {
bincode::serialize(&NestedTeamsUsersRoleRequest::all_administrable_sorted(
filter.map(|filter| bincode::deserialize::<TeamsUsersRoleRequestFilter>(&filter)).transpose()?.as_ref(),
author_user_id,
limit,
offset,
connection)?)?
            },
            web_common::database::Table::TeamsUsersRoles => {
bincode::serialize(&NestedTeamsUsersRole::all_administrable_sorted(
filter.map(|filter| bincode::deserialize::<TeamsUsersRoleFilter>(&filter)).transpose()?.as_ref(),
author_user_id,
limit,
offset,
connection)?)?
            },
            web_common::database::Table::Units => unimplemented!("Method all_administrable_sorted not implemented for table units."),
            web_common::database::Table::UserEmails => {
bincode::serialize(&NestedUserEmail::all_administrable_sorted(
filter.map(|filter| bincode::deserialize::<UserEmailFilter>(&filter)).transpose()?.as_ref(),
author_user_id,
limit,
offset,
connection)?)?
            },
            web_common::database::Table::Users => {
bincode::serialize(&User::all_administrable_sorted(
author_user_id,
limit,
offset,
connection)?)?
            },
            web_common::database::Table::UsersUsersRoleInvitations => {
bincode::serialize(&NestedUsersUsersRoleInvitation::all_administrable_sorted(
filter.map(|filter| bincode::deserialize::<UsersUsersRoleInvitationFilter>(&filter)).transpose()?.as_ref(),
author_user_id,
limit,
offset,
connection)?)?
            },
            web_common::database::Table::UsersUsersRoleRequests => {
bincode::serialize(&NestedUsersUsersRoleRequest::all_administrable_sorted(
filter.map(|filter| bincode::deserialize::<UsersUsersRoleRequestFilter>(&filter)).transpose()?.as_ref(),
author_user_id,
limit,
offset,
connection)?)?
            },
            web_common::database::Table::UsersUsersRoles => {
bincode::serialize(&NestedUsersUsersRole::all_administrable_sorted(
filter.map(|filter| bincode::deserialize::<UsersUsersRoleFilter>(&filter)).transpose()?.as_ref(),
author_user_id,
limit,
offset,
connection)?)?
            },
        })
    }

    /// Search for the administrable structs by a given string by Postgres's `similarity`.
    ///
    /// * `filter` - The optional filter to apply to the query.
    /// * `author_user_id` - The ID of the user who is performing the search.
    /// * `query` - The string to search for.
    /// * `limit` - The maximum number of results to return.
    /// * `offset` - The number of results to skip.
    /// * `connection` - The connection to the database.
    ///
     fn similarity_search_administrable(
        &self,
filter: Option<Vec<u8>>,
author_user_id: i32,
query: &str,
limit: Option<i64>,
offset: Option<i64>,
connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>,
) -> Result<Vec<u8>, web_common::api::ApiError> {
        Ok(match self {
            web_common::database::Table::BioOttRanks => unimplemented!("Method similarity_search_administrable not implemented for table bio_ott_ranks."),
            web_common::database::Table::BioOttTaxonItems => unimplemented!("Method similarity_search_administrable not implemented for table bio_ott_taxon_items."),
            web_common::database::Table::Colors => unimplemented!("Method similarity_search_administrable not implemented for table colors."),
            web_common::database::Table::Countries => unimplemented!("Method similarity_search_administrable not implemented for table countries."),
            web_common::database::Table::DerivedSamples => {
bincode::serialize(&NestedDerivedSample::similarity_search_administrable(
filter.map(|filter| bincode::deserialize::<DerivedSampleFilter>(&filter)).transpose()?.as_ref(),
author_user_id,
query,
limit,
offset,
connection)?)?
            },
            web_common::database::Table::DocumentFormats => unimplemented!("Method similarity_search_administrable not implemented for table document_formats."),
            web_common::database::Table::FontAwesomeIcons => unimplemented!("Method similarity_search_administrable not implemented for table font_awesome_icons."),
            web_common::database::Table::LoginProviders => unimplemented!("Method similarity_search_administrable not implemented for table login_providers."),
            web_common::database::Table::Materials => unimplemented!("Method similarity_search_administrable not implemented for table materials."),
            web_common::database::Table::NameplateCategories => unimplemented!("Method similarity_search_administrable not implemented for table nameplate_categories."),
            web_common::database::Table::Nameplates => {
bincode::serialize(&NestedNameplate::similarity_search_administrable(
filter.map(|filter| bincode::deserialize::<NameplateFilter>(&filter)).transpose()?.as_ref(),
author_user_id,
query,
limit,
offset,
connection)?)?
            },
            web_common::database::Table::Notifications => unimplemented!("Method similarity_search_administrable not implemented for table notifications."),
            web_common::database::Table::ObservationSubjects => unimplemented!("Method similarity_search_administrable not implemented for table observation_subjects."),
            web_common::database::Table::Observations => {
bincode::serialize(&NestedObservation::similarity_search_administrable(
filter.map(|filter| bincode::deserialize::<ObservationFilter>(&filter)).transpose()?.as_ref(),
author_user_id,
query,
limit,
offset,
connection)?)?
            },
            web_common::database::Table::OrganismBioOttTaxonItems => {
bincode::serialize(&NestedOrganismBioOttTaxonItem::similarity_search_administrable(
filter.map(|filter| bincode::deserialize::<OrganismBioOttTaxonItemFilter>(&filter)).transpose()?.as_ref(),
author_user_id,
query,
limit,
offset,
connection)?)?
            },
            web_common::database::Table::Organisms => {
bincode::serialize(&NestedOrganism::similarity_search_administrable(
filter.map(|filter| bincode::deserialize::<OrganismFilter>(&filter)).transpose()?.as_ref(),
author_user_id,
query,
limit,
offset,
connection)?)?
            },
            web_common::database::Table::Organizations => unimplemented!("Method similarity_search_administrable not implemented for table organizations."),
            web_common::database::Table::PermanenceCategories => unimplemented!("Method similarity_search_administrable not implemented for table permanence_categories."),
            web_common::database::Table::ProjectStates => unimplemented!("Method similarity_search_administrable not implemented for table project_states."),
            web_common::database::Table::Projects => {
bincode::serialize(&NestedProject::similarity_search_administrable(
filter.map(|filter| bincode::deserialize::<ProjectFilter>(&filter)).transpose()?.as_ref(),
author_user_id,
query,
limit,
offset,
connection)?)?
            },
            web_common::database::Table::ProjectsTeamsRoleInvitations => {
bincode::serialize(&NestedProjectsTeamsRoleInvitation::similarity_search_administrable(
filter.map(|filter| bincode::deserialize::<ProjectsTeamsRoleInvitationFilter>(&filter)).transpose()?.as_ref(),
author_user_id,
query,
limit,
offset,
connection)?)?
            },
            web_common::database::Table::ProjectsTeamsRoleRequests => {
bincode::serialize(&NestedProjectsTeamsRoleRequest::similarity_search_administrable(
filter.map(|filter| bincode::deserialize::<ProjectsTeamsRoleRequestFilter>(&filter)).transpose()?.as_ref(),
author_user_id,
query,
limit,
offset,
connection)?)?
            },
            web_common::database::Table::ProjectsTeamsRoles => {
bincode::serialize(&NestedProjectsTeamsRole::similarity_search_administrable(
filter.map(|filter| bincode::deserialize::<ProjectsTeamsRoleFilter>(&filter)).transpose()?.as_ref(),
author_user_id,
query,
limit,
offset,
connection)?)?
            },
            web_common::database::Table::ProjectsUsersRoleInvitations => {
bincode::serialize(&NestedProjectsUsersRoleInvitation::similarity_search_administrable(
filter.map(|filter| bincode::deserialize::<ProjectsUsersRoleInvitationFilter>(&filter)).transpose()?.as_ref(),
author_user_id,
query,
limit,
offset,
connection)?)?
            },
            web_common::database::Table::ProjectsUsersRoleRequests => {
bincode::serialize(&NestedProjectsUsersRoleRequest::similarity_search_administrable(
filter.map(|filter| bincode::deserialize::<ProjectsUsersRoleRequestFilter>(&filter)).transpose()?.as_ref(),
author_user_id,
query,
limit,
offset,
connection)?)?
            },
            web_common::database::Table::ProjectsUsersRoles => {
bincode::serialize(&NestedProjectsUsersRole::similarity_search_administrable(
filter.map(|filter| bincode::deserialize::<ProjectsUsersRoleFilter>(&filter)).transpose()?.as_ref(),
author_user_id,
query,
limit,
offset,
connection)?)?
            },
            web_common::database::Table::Roles => unimplemented!("Method similarity_search_administrable not implemented for table roles."),
            web_common::database::Table::SampleBioOttTaxonItems => {
bincode::serialize(&NestedSampleBioOttTaxonItem::similarity_search_administrable(
filter.map(|filter| bincode::deserialize::<SampleBioOttTaxonItemFilter>(&filter)).transpose()?.as_ref(),
author_user_id,
query,
limit,
offset,
connection)?)?
            },
            web_common::database::Table::SampleContainerCategories => unimplemented!("Method similarity_search_administrable not implemented for table sample_container_categories."),
            web_common::database::Table::SampleContainers => {
bincode::serialize(&NestedSampleContainer::similarity_search_administrable(
filter.map(|filter| bincode::deserialize::<SampleContainerFilter>(&filter)).transpose()?.as_ref(),
author_user_id,
query,
limit,
offset,
connection)?)?
            },
            web_common::database::Table::SampleStates => unimplemented!("Method similarity_search_administrable not implemented for table sample_states."),
            web_common::database::Table::Samples => {
bincode::serialize(&NestedSample::similarity_search_administrable(
filter.map(|filter| bincode::deserialize::<SampleFilter>(&filter)).transpose()?.as_ref(),
author_user_id,
query,
limit,
offset,
connection)?)?
            },
            web_common::database::Table::Spectra => unimplemented!("Method similarity_search_administrable not implemented for table spectra."),
            web_common::database::Table::SpectraCollections => {
bincode::serialize(&NestedSpectraCollection::similarity_search_administrable(
filter.map(|filter| bincode::deserialize::<SpectraCollectionFilter>(&filter)).transpose()?.as_ref(),
author_user_id,
query,
limit,
offset,
connection)?)?
            },
            web_common::database::Table::TeamStates => unimplemented!("Method similarity_search_administrable not implemented for table team_states."),
            web_common::database::Table::Teams => {
bincode::serialize(&NestedTeam::similarity_search_administrable(
filter.map(|filter| bincode::deserialize::<TeamFilter>(&filter)).transpose()?.as_ref(),
author_user_id,
query,
limit,
offset,
connection)?)?
            },
            web_common::database::Table::TeamsTeamsRoleInvitations => {
bincode::serialize(&NestedTeamsTeamsRoleInvitation::similarity_search_administrable(
filter.map(|filter| bincode::deserialize::<TeamsTeamsRoleInvitationFilter>(&filter)).transpose()?.as_ref(),
author_user_id,
query,
limit,
offset,
connection)?)?
            },
            web_common::database::Table::TeamsUsersRoleInvitations => {
bincode::serialize(&NestedTeamsUsersRoleInvitation::similarity_search_administrable(
filter.map(|filter| bincode::deserialize::<TeamsUsersRoleInvitationFilter>(&filter)).transpose()?.as_ref(),
author_user_id,
query,
limit,
offset,
connection)?)?
            },
            web_common::database::Table::TeamsUsersRoleRequests => {
bincode::serialize(&NestedTeamsUsersRoleRequest::similarity_search_administrable(
filter.map(|filter| bincode::deserialize::<TeamsUsersRoleRequestFilter>(&filter)).transpose()?.as_ref(),
author_user_id,
query,
limit,
offset,
connection)?)?
            },
            web_common::database::Table::TeamsUsersRoles => {
bincode::serialize(&NestedTeamsUsersRole::similarity_search_administrable(
filter.map(|filter| bincode::deserialize::<TeamsUsersRoleFilter>(&filter)).transpose()?.as_ref(),
author_user_id,
query,
limit,
offset,
connection)?)?
            },
            web_common::database::Table::Units => unimplemented!("Method similarity_search_administrable not implemented for table units."),
            web_common::database::Table::UserEmails => unimplemented!("Method similarity_search_administrable not implemented for table user_emails."),
            web_common::database::Table::Users => {
bincode::serialize(&User::similarity_search_administrable(
author_user_id,
query,
limit,
offset,
connection)?)?
            },
            web_common::database::Table::UsersUsersRoleInvitations => {
bincode::serialize(&NestedUsersUsersRoleInvitation::similarity_search_administrable(
filter.map(|filter| bincode::deserialize::<UsersUsersRoleInvitationFilter>(&filter)).transpose()?.as_ref(),
author_user_id,
query,
limit,
offset,
connection)?)?
            },
            web_common::database::Table::UsersUsersRoleRequests => {
bincode::serialize(&NestedUsersUsersRoleRequest::similarity_search_administrable(
filter.map(|filter| bincode::deserialize::<UsersUsersRoleRequestFilter>(&filter)).transpose()?.as_ref(),
author_user_id,
query,
limit,
offset,
connection)?)?
            },
            web_common::database::Table::UsersUsersRoles => {
bincode::serialize(&NestedUsersUsersRole::similarity_search_administrable(
filter.map(|filter| bincode::deserialize::<UsersUsersRoleFilter>(&filter)).transpose()?.as_ref(),
author_user_id,
query,
limit,
offset,
connection)?)?
            },
        })
    }

    /// Search for the administrable structs by a given string by Postgres's `word_similarity`.
    ///
    /// * `filter` - The optional filter to apply to the query.
    /// * `author_user_id` - The ID of the user who is performing the search.
    /// * `query` - The string to search for.
    /// * `limit` - The maximum number of results to return.
    /// * `offset` - The number of results to skip.
    /// * `connection` - The connection to the database.
    ///
     fn word_similarity_search_administrable(
        &self,
filter: Option<Vec<u8>>,
author_user_id: i32,
query: &str,
limit: Option<i64>,
offset: Option<i64>,
connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>,
) -> Result<Vec<u8>, web_common::api::ApiError> {
        Ok(match self {
            web_common::database::Table::BioOttRanks => unimplemented!("Method word_similarity_search_administrable not implemented for table bio_ott_ranks."),
            web_common::database::Table::BioOttTaxonItems => unimplemented!("Method word_similarity_search_administrable not implemented for table bio_ott_taxon_items."),
            web_common::database::Table::Colors => unimplemented!("Method word_similarity_search_administrable not implemented for table colors."),
            web_common::database::Table::Countries => unimplemented!("Method word_similarity_search_administrable not implemented for table countries."),
            web_common::database::Table::DerivedSamples => {
bincode::serialize(&NestedDerivedSample::word_similarity_search_administrable(
filter.map(|filter| bincode::deserialize::<DerivedSampleFilter>(&filter)).transpose()?.as_ref(),
author_user_id,
query,
limit,
offset,
connection)?)?
            },
            web_common::database::Table::DocumentFormats => unimplemented!("Method word_similarity_search_administrable not implemented for table document_formats."),
            web_common::database::Table::FontAwesomeIcons => unimplemented!("Method word_similarity_search_administrable not implemented for table font_awesome_icons."),
            web_common::database::Table::LoginProviders => unimplemented!("Method word_similarity_search_administrable not implemented for table login_providers."),
            web_common::database::Table::Materials => unimplemented!("Method word_similarity_search_administrable not implemented for table materials."),
            web_common::database::Table::NameplateCategories => unimplemented!("Method word_similarity_search_administrable not implemented for table nameplate_categories."),
            web_common::database::Table::Nameplates => {
bincode::serialize(&NestedNameplate::word_similarity_search_administrable(
filter.map(|filter| bincode::deserialize::<NameplateFilter>(&filter)).transpose()?.as_ref(),
author_user_id,
query,
limit,
offset,
connection)?)?
            },
            web_common::database::Table::Notifications => unimplemented!("Method word_similarity_search_administrable not implemented for table notifications."),
            web_common::database::Table::ObservationSubjects => unimplemented!("Method word_similarity_search_administrable not implemented for table observation_subjects."),
            web_common::database::Table::Observations => {
bincode::serialize(&NestedObservation::word_similarity_search_administrable(
filter.map(|filter| bincode::deserialize::<ObservationFilter>(&filter)).transpose()?.as_ref(),
author_user_id,
query,
limit,
offset,
connection)?)?
            },
            web_common::database::Table::OrganismBioOttTaxonItems => {
bincode::serialize(&NestedOrganismBioOttTaxonItem::word_similarity_search_administrable(
filter.map(|filter| bincode::deserialize::<OrganismBioOttTaxonItemFilter>(&filter)).transpose()?.as_ref(),
author_user_id,
query,
limit,
offset,
connection)?)?
            },
            web_common::database::Table::Organisms => {
bincode::serialize(&NestedOrganism::word_similarity_search_administrable(
filter.map(|filter| bincode::deserialize::<OrganismFilter>(&filter)).transpose()?.as_ref(),
author_user_id,
query,
limit,
offset,
connection)?)?
            },
            web_common::database::Table::Organizations => unimplemented!("Method word_similarity_search_administrable not implemented for table organizations."),
            web_common::database::Table::PermanenceCategories => unimplemented!("Method word_similarity_search_administrable not implemented for table permanence_categories."),
            web_common::database::Table::ProjectStates => unimplemented!("Method word_similarity_search_administrable not implemented for table project_states."),
            web_common::database::Table::Projects => {
bincode::serialize(&NestedProject::word_similarity_search_administrable(
filter.map(|filter| bincode::deserialize::<ProjectFilter>(&filter)).transpose()?.as_ref(),
author_user_id,
query,
limit,
offset,
connection)?)?
            },
            web_common::database::Table::ProjectsTeamsRoleInvitations => {
bincode::serialize(&NestedProjectsTeamsRoleInvitation::word_similarity_search_administrable(
filter.map(|filter| bincode::deserialize::<ProjectsTeamsRoleInvitationFilter>(&filter)).transpose()?.as_ref(),
author_user_id,
query,
limit,
offset,
connection)?)?
            },
            web_common::database::Table::ProjectsTeamsRoleRequests => {
bincode::serialize(&NestedProjectsTeamsRoleRequest::word_similarity_search_administrable(
filter.map(|filter| bincode::deserialize::<ProjectsTeamsRoleRequestFilter>(&filter)).transpose()?.as_ref(),
author_user_id,
query,
limit,
offset,
connection)?)?
            },
            web_common::database::Table::ProjectsTeamsRoles => {
bincode::serialize(&NestedProjectsTeamsRole::word_similarity_search_administrable(
filter.map(|filter| bincode::deserialize::<ProjectsTeamsRoleFilter>(&filter)).transpose()?.as_ref(),
author_user_id,
query,
limit,
offset,
connection)?)?
            },
            web_common::database::Table::ProjectsUsersRoleInvitations => {
bincode::serialize(&NestedProjectsUsersRoleInvitation::word_similarity_search_administrable(
filter.map(|filter| bincode::deserialize::<ProjectsUsersRoleInvitationFilter>(&filter)).transpose()?.as_ref(),
author_user_id,
query,
limit,
offset,
connection)?)?
            },
            web_common::database::Table::ProjectsUsersRoleRequests => {
bincode::serialize(&NestedProjectsUsersRoleRequest::word_similarity_search_administrable(
filter.map(|filter| bincode::deserialize::<ProjectsUsersRoleRequestFilter>(&filter)).transpose()?.as_ref(),
author_user_id,
query,
limit,
offset,
connection)?)?
            },
            web_common::database::Table::ProjectsUsersRoles => {
bincode::serialize(&NestedProjectsUsersRole::word_similarity_search_administrable(
filter.map(|filter| bincode::deserialize::<ProjectsUsersRoleFilter>(&filter)).transpose()?.as_ref(),
author_user_id,
query,
limit,
offset,
connection)?)?
            },
            web_common::database::Table::Roles => unimplemented!("Method word_similarity_search_administrable not implemented for table roles."),
            web_common::database::Table::SampleBioOttTaxonItems => {
bincode::serialize(&NestedSampleBioOttTaxonItem::word_similarity_search_administrable(
filter.map(|filter| bincode::deserialize::<SampleBioOttTaxonItemFilter>(&filter)).transpose()?.as_ref(),
author_user_id,
query,
limit,
offset,
connection)?)?
            },
            web_common::database::Table::SampleContainerCategories => unimplemented!("Method word_similarity_search_administrable not implemented for table sample_container_categories."),
            web_common::database::Table::SampleContainers => {
bincode::serialize(&NestedSampleContainer::word_similarity_search_administrable(
filter.map(|filter| bincode::deserialize::<SampleContainerFilter>(&filter)).transpose()?.as_ref(),
author_user_id,
query,
limit,
offset,
connection)?)?
            },
            web_common::database::Table::SampleStates => unimplemented!("Method word_similarity_search_administrable not implemented for table sample_states."),
            web_common::database::Table::Samples => {
bincode::serialize(&NestedSample::word_similarity_search_administrable(
filter.map(|filter| bincode::deserialize::<SampleFilter>(&filter)).transpose()?.as_ref(),
author_user_id,
query,
limit,
offset,
connection)?)?
            },
            web_common::database::Table::Spectra => unimplemented!("Method word_similarity_search_administrable not implemented for table spectra."),
            web_common::database::Table::SpectraCollections => {
bincode::serialize(&NestedSpectraCollection::word_similarity_search_administrable(
filter.map(|filter| bincode::deserialize::<SpectraCollectionFilter>(&filter)).transpose()?.as_ref(),
author_user_id,
query,
limit,
offset,
connection)?)?
            },
            web_common::database::Table::TeamStates => unimplemented!("Method word_similarity_search_administrable not implemented for table team_states."),
            web_common::database::Table::Teams => {
bincode::serialize(&NestedTeam::word_similarity_search_administrable(
filter.map(|filter| bincode::deserialize::<TeamFilter>(&filter)).transpose()?.as_ref(),
author_user_id,
query,
limit,
offset,
connection)?)?
            },
            web_common::database::Table::TeamsTeamsRoleInvitations => {
bincode::serialize(&NestedTeamsTeamsRoleInvitation::word_similarity_search_administrable(
filter.map(|filter| bincode::deserialize::<TeamsTeamsRoleInvitationFilter>(&filter)).transpose()?.as_ref(),
author_user_id,
query,
limit,
offset,
connection)?)?
            },
            web_common::database::Table::TeamsUsersRoleInvitations => {
bincode::serialize(&NestedTeamsUsersRoleInvitation::word_similarity_search_administrable(
filter.map(|filter| bincode::deserialize::<TeamsUsersRoleInvitationFilter>(&filter)).transpose()?.as_ref(),
author_user_id,
query,
limit,
offset,
connection)?)?
            },
            web_common::database::Table::TeamsUsersRoleRequests => {
bincode::serialize(&NestedTeamsUsersRoleRequest::word_similarity_search_administrable(
filter.map(|filter| bincode::deserialize::<TeamsUsersRoleRequestFilter>(&filter)).transpose()?.as_ref(),
author_user_id,
query,
limit,
offset,
connection)?)?
            },
            web_common::database::Table::TeamsUsersRoles => {
bincode::serialize(&NestedTeamsUsersRole::word_similarity_search_administrable(
filter.map(|filter| bincode::deserialize::<TeamsUsersRoleFilter>(&filter)).transpose()?.as_ref(),
author_user_id,
query,
limit,
offset,
connection)?)?
            },
            web_common::database::Table::Units => unimplemented!("Method word_similarity_search_administrable not implemented for table units."),
            web_common::database::Table::UserEmails => unimplemented!("Method word_similarity_search_administrable not implemented for table user_emails."),
            web_common::database::Table::Users => {
bincode::serialize(&User::word_similarity_search_administrable(
author_user_id,
query,
limit,
offset,
connection)?)?
            },
            web_common::database::Table::UsersUsersRoleInvitations => {
bincode::serialize(&NestedUsersUsersRoleInvitation::word_similarity_search_administrable(
filter.map(|filter| bincode::deserialize::<UsersUsersRoleInvitationFilter>(&filter)).transpose()?.as_ref(),
author_user_id,
query,
limit,
offset,
connection)?)?
            },
            web_common::database::Table::UsersUsersRoleRequests => {
bincode::serialize(&NestedUsersUsersRoleRequest::word_similarity_search_administrable(
filter.map(|filter| bincode::deserialize::<UsersUsersRoleRequestFilter>(&filter)).transpose()?.as_ref(),
author_user_id,
query,
limit,
offset,
connection)?)?
            },
            web_common::database::Table::UsersUsersRoles => {
bincode::serialize(&NestedUsersUsersRole::word_similarity_search_administrable(
filter.map(|filter| bincode::deserialize::<UsersUsersRoleFilter>(&filter)).transpose()?.as_ref(),
author_user_id,
query,
limit,
offset,
connection)?)?
            },
        })
    }

    /// Search for the administrable structs by a given string by Postgres's `strict_word_similarity`.
    ///
    /// * `filter` - The optional filter to apply to the query.
    /// * `author_user_id` - The ID of the user who is performing the search.
    /// * `query` - The string to search for.
    /// * `limit` - The maximum number of results to return.
    /// * `offset` - The number of results to skip.
    /// * `connection` - The connection to the database.
    ///
     fn strict_word_similarity_search_administrable(
        &self,
filter: Option<Vec<u8>>,
author_user_id: i32,
query: &str,
limit: Option<i64>,
offset: Option<i64>,
connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>,
) -> Result<Vec<u8>, web_common::api::ApiError> {
        Ok(match self {
            web_common::database::Table::BioOttRanks => unimplemented!("Method strict_word_similarity_search_administrable not implemented for table bio_ott_ranks."),
            web_common::database::Table::BioOttTaxonItems => unimplemented!("Method strict_word_similarity_search_administrable not implemented for table bio_ott_taxon_items."),
            web_common::database::Table::Colors => unimplemented!("Method strict_word_similarity_search_administrable not implemented for table colors."),
            web_common::database::Table::Countries => unimplemented!("Method strict_word_similarity_search_administrable not implemented for table countries."),
            web_common::database::Table::DerivedSamples => {
bincode::serialize(&NestedDerivedSample::strict_word_similarity_search_administrable(
filter.map(|filter| bincode::deserialize::<DerivedSampleFilter>(&filter)).transpose()?.as_ref(),
author_user_id,
query,
limit,
offset,
connection)?)?
            },
            web_common::database::Table::DocumentFormats => unimplemented!("Method strict_word_similarity_search_administrable not implemented for table document_formats."),
            web_common::database::Table::FontAwesomeIcons => unimplemented!("Method strict_word_similarity_search_administrable not implemented for table font_awesome_icons."),
            web_common::database::Table::LoginProviders => unimplemented!("Method strict_word_similarity_search_administrable not implemented for table login_providers."),
            web_common::database::Table::Materials => unimplemented!("Method strict_word_similarity_search_administrable not implemented for table materials."),
            web_common::database::Table::NameplateCategories => unimplemented!("Method strict_word_similarity_search_administrable not implemented for table nameplate_categories."),
            web_common::database::Table::Nameplates => {
bincode::serialize(&NestedNameplate::strict_word_similarity_search_administrable(
filter.map(|filter| bincode::deserialize::<NameplateFilter>(&filter)).transpose()?.as_ref(),
author_user_id,
query,
limit,
offset,
connection)?)?
            },
            web_common::database::Table::Notifications => unimplemented!("Method strict_word_similarity_search_administrable not implemented for table notifications."),
            web_common::database::Table::ObservationSubjects => unimplemented!("Method strict_word_similarity_search_administrable not implemented for table observation_subjects."),
            web_common::database::Table::Observations => {
bincode::serialize(&NestedObservation::strict_word_similarity_search_administrable(
filter.map(|filter| bincode::deserialize::<ObservationFilter>(&filter)).transpose()?.as_ref(),
author_user_id,
query,
limit,
offset,
connection)?)?
            },
            web_common::database::Table::OrganismBioOttTaxonItems => {
bincode::serialize(&NestedOrganismBioOttTaxonItem::strict_word_similarity_search_administrable(
filter.map(|filter| bincode::deserialize::<OrganismBioOttTaxonItemFilter>(&filter)).transpose()?.as_ref(),
author_user_id,
query,
limit,
offset,
connection)?)?
            },
            web_common::database::Table::Organisms => {
bincode::serialize(&NestedOrganism::strict_word_similarity_search_administrable(
filter.map(|filter| bincode::deserialize::<OrganismFilter>(&filter)).transpose()?.as_ref(),
author_user_id,
query,
limit,
offset,
connection)?)?
            },
            web_common::database::Table::Organizations => unimplemented!("Method strict_word_similarity_search_administrable not implemented for table organizations."),
            web_common::database::Table::PermanenceCategories => unimplemented!("Method strict_word_similarity_search_administrable not implemented for table permanence_categories."),
            web_common::database::Table::ProjectStates => unimplemented!("Method strict_word_similarity_search_administrable not implemented for table project_states."),
            web_common::database::Table::Projects => {
bincode::serialize(&NestedProject::strict_word_similarity_search_administrable(
filter.map(|filter| bincode::deserialize::<ProjectFilter>(&filter)).transpose()?.as_ref(),
author_user_id,
query,
limit,
offset,
connection)?)?
            },
            web_common::database::Table::ProjectsTeamsRoleInvitations => {
bincode::serialize(&NestedProjectsTeamsRoleInvitation::strict_word_similarity_search_administrable(
filter.map(|filter| bincode::deserialize::<ProjectsTeamsRoleInvitationFilter>(&filter)).transpose()?.as_ref(),
author_user_id,
query,
limit,
offset,
connection)?)?
            },
            web_common::database::Table::ProjectsTeamsRoleRequests => {
bincode::serialize(&NestedProjectsTeamsRoleRequest::strict_word_similarity_search_administrable(
filter.map(|filter| bincode::deserialize::<ProjectsTeamsRoleRequestFilter>(&filter)).transpose()?.as_ref(),
author_user_id,
query,
limit,
offset,
connection)?)?
            },
            web_common::database::Table::ProjectsTeamsRoles => {
bincode::serialize(&NestedProjectsTeamsRole::strict_word_similarity_search_administrable(
filter.map(|filter| bincode::deserialize::<ProjectsTeamsRoleFilter>(&filter)).transpose()?.as_ref(),
author_user_id,
query,
limit,
offset,
connection)?)?
            },
            web_common::database::Table::ProjectsUsersRoleInvitations => {
bincode::serialize(&NestedProjectsUsersRoleInvitation::strict_word_similarity_search_administrable(
filter.map(|filter| bincode::deserialize::<ProjectsUsersRoleInvitationFilter>(&filter)).transpose()?.as_ref(),
author_user_id,
query,
limit,
offset,
connection)?)?
            },
            web_common::database::Table::ProjectsUsersRoleRequests => {
bincode::serialize(&NestedProjectsUsersRoleRequest::strict_word_similarity_search_administrable(
filter.map(|filter| bincode::deserialize::<ProjectsUsersRoleRequestFilter>(&filter)).transpose()?.as_ref(),
author_user_id,
query,
limit,
offset,
connection)?)?
            },
            web_common::database::Table::ProjectsUsersRoles => {
bincode::serialize(&NestedProjectsUsersRole::strict_word_similarity_search_administrable(
filter.map(|filter| bincode::deserialize::<ProjectsUsersRoleFilter>(&filter)).transpose()?.as_ref(),
author_user_id,
query,
limit,
offset,
connection)?)?
            },
            web_common::database::Table::Roles => unimplemented!("Method strict_word_similarity_search_administrable not implemented for table roles."),
            web_common::database::Table::SampleBioOttTaxonItems => {
bincode::serialize(&NestedSampleBioOttTaxonItem::strict_word_similarity_search_administrable(
filter.map(|filter| bincode::deserialize::<SampleBioOttTaxonItemFilter>(&filter)).transpose()?.as_ref(),
author_user_id,
query,
limit,
offset,
connection)?)?
            },
            web_common::database::Table::SampleContainerCategories => unimplemented!("Method strict_word_similarity_search_administrable not implemented for table sample_container_categories."),
            web_common::database::Table::SampleContainers => {
bincode::serialize(&NestedSampleContainer::strict_word_similarity_search_administrable(
filter.map(|filter| bincode::deserialize::<SampleContainerFilter>(&filter)).transpose()?.as_ref(),
author_user_id,
query,
limit,
offset,
connection)?)?
            },
            web_common::database::Table::SampleStates => unimplemented!("Method strict_word_similarity_search_administrable not implemented for table sample_states."),
            web_common::database::Table::Samples => {
bincode::serialize(&NestedSample::strict_word_similarity_search_administrable(
filter.map(|filter| bincode::deserialize::<SampleFilter>(&filter)).transpose()?.as_ref(),
author_user_id,
query,
limit,
offset,
connection)?)?
            },
            web_common::database::Table::Spectra => unimplemented!("Method strict_word_similarity_search_administrable not implemented for table spectra."),
            web_common::database::Table::SpectraCollections => {
bincode::serialize(&NestedSpectraCollection::strict_word_similarity_search_administrable(
filter.map(|filter| bincode::deserialize::<SpectraCollectionFilter>(&filter)).transpose()?.as_ref(),
author_user_id,
query,
limit,
offset,
connection)?)?
            },
            web_common::database::Table::TeamStates => unimplemented!("Method strict_word_similarity_search_administrable not implemented for table team_states."),
            web_common::database::Table::Teams => {
bincode::serialize(&NestedTeam::strict_word_similarity_search_administrable(
filter.map(|filter| bincode::deserialize::<TeamFilter>(&filter)).transpose()?.as_ref(),
author_user_id,
query,
limit,
offset,
connection)?)?
            },
            web_common::database::Table::TeamsTeamsRoleInvitations => {
bincode::serialize(&NestedTeamsTeamsRoleInvitation::strict_word_similarity_search_administrable(
filter.map(|filter| bincode::deserialize::<TeamsTeamsRoleInvitationFilter>(&filter)).transpose()?.as_ref(),
author_user_id,
query,
limit,
offset,
connection)?)?
            },
            web_common::database::Table::TeamsUsersRoleInvitations => {
bincode::serialize(&NestedTeamsUsersRoleInvitation::strict_word_similarity_search_administrable(
filter.map(|filter| bincode::deserialize::<TeamsUsersRoleInvitationFilter>(&filter)).transpose()?.as_ref(),
author_user_id,
query,
limit,
offset,
connection)?)?
            },
            web_common::database::Table::TeamsUsersRoleRequests => {
bincode::serialize(&NestedTeamsUsersRoleRequest::strict_word_similarity_search_administrable(
filter.map(|filter| bincode::deserialize::<TeamsUsersRoleRequestFilter>(&filter)).transpose()?.as_ref(),
author_user_id,
query,
limit,
offset,
connection)?)?
            },
            web_common::database::Table::TeamsUsersRoles => {
bincode::serialize(&NestedTeamsUsersRole::strict_word_similarity_search_administrable(
filter.map(|filter| bincode::deserialize::<TeamsUsersRoleFilter>(&filter)).transpose()?.as_ref(),
author_user_id,
query,
limit,
offset,
connection)?)?
            },
            web_common::database::Table::Units => unimplemented!("Method strict_word_similarity_search_administrable not implemented for table units."),
            web_common::database::Table::UserEmails => unimplemented!("Method strict_word_similarity_search_administrable not implemented for table user_emails."),
            web_common::database::Table::Users => {
bincode::serialize(&User::strict_word_similarity_search_administrable(
author_user_id,
query,
limit,
offset,
connection)?)?
            },
            web_common::database::Table::UsersUsersRoleInvitations => {
bincode::serialize(&NestedUsersUsersRoleInvitation::strict_word_similarity_search_administrable(
filter.map(|filter| bincode::deserialize::<UsersUsersRoleInvitationFilter>(&filter)).transpose()?.as_ref(),
author_user_id,
query,
limit,
offset,
connection)?)?
            },
            web_common::database::Table::UsersUsersRoleRequests => {
bincode::serialize(&NestedUsersUsersRoleRequest::strict_word_similarity_search_administrable(
filter.map(|filter| bincode::deserialize::<UsersUsersRoleRequestFilter>(&filter)).transpose()?.as_ref(),
author_user_id,
query,
limit,
offset,
connection)?)?
            },
            web_common::database::Table::UsersUsersRoles => {
bincode::serialize(&NestedUsersUsersRole::strict_word_similarity_search_administrable(
filter.map(|filter| bincode::deserialize::<UsersUsersRoleFilter>(&filter)).transpose()?.as_ref(),
author_user_id,
query,
limit,
offset,
connection)?)?
            },
        })
    }

    /// Delete the struct from the database by its ID.
    ///
    /// * `primary_key` - The primary key(s) of the struct to delete.
    /// * `author_user_id` - The ID of the user who is deleting the struct.
    /// * `connection` - The connection to the database.
    ///
     fn delete_by_id(
        &self,
primary_key: PrimaryKey,
author_user_id: i32,
connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>,
) -> Result<usize, web_common::api::ApiError> {
        Ok(match self {
            web_common::database::Table::BioOttRanks => unimplemented!("Method delete_by_id not implemented for table bio_ott_ranks."),
            web_common::database::Table::BioOttTaxonItems => unimplemented!("Method delete_by_id not implemented for table bio_ott_taxon_items."),
            web_common::database::Table::Colors => unimplemented!("Method delete_by_id not implemented for table colors."),
            web_common::database::Table::Countries => unimplemented!("Method delete_by_id not implemented for table countries."),
            web_common::database::Table::DerivedSamples => {
NestedDerivedSample::delete_by_id(
primary_key.into(),
author_user_id,
connection)?            },
            web_common::database::Table::DocumentFormats => unimplemented!("Method delete_by_id not implemented for table document_formats."),
            web_common::database::Table::FontAwesomeIcons => unimplemented!("Method delete_by_id not implemented for table font_awesome_icons."),
            web_common::database::Table::LoginProviders => unimplemented!("Method delete_by_id not implemented for table login_providers."),
            web_common::database::Table::Materials => unimplemented!("Method delete_by_id not implemented for table materials."),
            web_common::database::Table::NameplateCategories => unimplemented!("Method delete_by_id not implemented for table nameplate_categories."),
            web_common::database::Table::Nameplates => {
NestedNameplate::delete_by_id(
primary_key.into(),
author_user_id,
connection)?            },
            web_common::database::Table::Notifications => unimplemented!("Method delete_by_id not implemented for table notifications."),
            web_common::database::Table::ObservationSubjects => unimplemented!("Method delete_by_id not implemented for table observation_subjects."),
            web_common::database::Table::Observations => {
NestedObservation::delete_by_id(
primary_key.into(),
author_user_id,
connection)?            },
            web_common::database::Table::OrganismBioOttTaxonItems => {
NestedOrganismBioOttTaxonItem::delete_by_id(
primary_key.into(),
author_user_id,
connection)?            },
            web_common::database::Table::Organisms => {
NestedOrganism::delete_by_id(
primary_key.into(),
author_user_id,
connection)?            },
            web_common::database::Table::Organizations => unimplemented!("Method delete_by_id not implemented for table organizations."),
            web_common::database::Table::PermanenceCategories => unimplemented!("Method delete_by_id not implemented for table permanence_categories."),
            web_common::database::Table::ProjectStates => unimplemented!("Method delete_by_id not implemented for table project_states."),
            web_common::database::Table::Projects => {
NestedProject::delete_by_id(
primary_key.into(),
author_user_id,
connection)?            },
            web_common::database::Table::ProjectsTeamsRoleInvitations => {
NestedProjectsTeamsRoleInvitation::delete_by_id(
primary_key.into(),
author_user_id,
connection)?            },
            web_common::database::Table::ProjectsTeamsRoleRequests => {
NestedProjectsTeamsRoleRequest::delete_by_id(
primary_key.into(),
author_user_id,
connection)?            },
            web_common::database::Table::ProjectsTeamsRoles => {
NestedProjectsTeamsRole::delete_by_id(
primary_key.into(),
author_user_id,
connection)?            },
            web_common::database::Table::ProjectsUsersRoleInvitations => {
NestedProjectsUsersRoleInvitation::delete_by_id(
primary_key.into(),
author_user_id,
connection)?            },
            web_common::database::Table::ProjectsUsersRoleRequests => {
NestedProjectsUsersRoleRequest::delete_by_id(
primary_key.into(),
author_user_id,
connection)?            },
            web_common::database::Table::ProjectsUsersRoles => {
NestedProjectsUsersRole::delete_by_id(
primary_key.into(),
author_user_id,
connection)?            },
            web_common::database::Table::Roles => unimplemented!("Method delete_by_id not implemented for table roles."),
            web_common::database::Table::SampleBioOttTaxonItems => {
NestedSampleBioOttTaxonItem::delete_by_id(
primary_key.into(),
author_user_id,
connection)?            },
            web_common::database::Table::SampleContainerCategories => unimplemented!("Method delete_by_id not implemented for table sample_container_categories."),
            web_common::database::Table::SampleContainers => {
NestedSampleContainer::delete_by_id(
primary_key.into(),
author_user_id,
connection)?            },
            web_common::database::Table::SampleStates => unimplemented!("Method delete_by_id not implemented for table sample_states."),
            web_common::database::Table::Samples => {
NestedSample::delete_by_id(
primary_key.into(),
author_user_id,
connection)?            },
            web_common::database::Table::Spectra => {
NestedSpectra::delete_by_id(
primary_key.into(),
author_user_id,
connection)?            },
            web_common::database::Table::SpectraCollections => {
NestedSpectraCollection::delete_by_id(
primary_key.into(),
author_user_id,
connection)?            },
            web_common::database::Table::TeamStates => unimplemented!("Method delete_by_id not implemented for table team_states."),
            web_common::database::Table::Teams => {
NestedTeam::delete_by_id(
primary_key.into(),
author_user_id,
connection)?            },
            web_common::database::Table::TeamsTeamsRoleInvitations => {
NestedTeamsTeamsRoleInvitation::delete_by_id(
primary_key.into(),
author_user_id,
connection)?            },
            web_common::database::Table::TeamsUsersRoleInvitations => {
NestedTeamsUsersRoleInvitation::delete_by_id(
primary_key.into(),
author_user_id,
connection)?            },
            web_common::database::Table::TeamsUsersRoleRequests => {
NestedTeamsUsersRoleRequest::delete_by_id(
primary_key.into(),
author_user_id,
connection)?            },
            web_common::database::Table::TeamsUsersRoles => {
NestedTeamsUsersRole::delete_by_id(
primary_key.into(),
author_user_id,
connection)?            },
            web_common::database::Table::Units => unimplemented!("Method delete_by_id not implemented for table units."),
            web_common::database::Table::UserEmails => {
NestedUserEmail::delete_by_id(
primary_key.into(),
author_user_id,
connection)?            },
            web_common::database::Table::Users => {
User::delete_by_id(
primary_key.into(),
author_user_id,
connection)?            },
            web_common::database::Table::UsersUsersRoleInvitations => {
NestedUsersUsersRoleInvitation::delete_by_id(
primary_key.into(),
author_user_id,
connection)?            },
            web_common::database::Table::UsersUsersRoleRequests => {
NestedUsersUsersRoleRequest::delete_by_id(
primary_key.into(),
author_user_id,
connection)?            },
            web_common::database::Table::UsersUsersRoles => {
NestedUsersUsersRole::delete_by_id(
primary_key.into(),
author_user_id,
connection)?            },
        })
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
            web_common::database::Table::Materials => unreachable!("Table `materials` is not insertable as it does not have a known column associated to a creator user id."),
            web_common::database::Table::NameplateCategories => unreachable!("Table `nameplate_categories` is not insertable as it does not have a known column associated to a creator user id."),
            web_common::database::Table::Nameplates => {
                let row: web_common::database::NewNameplate = bincode::deserialize::<web_common::database::NewNameplate>(&row)?;
                let inserted_row: crate::models::Nameplate = <web_common::database::NewNameplate as InsertRow>::insert(row, user_id, connection)?;
                let nested_row = crate::nested_models::NestedNameplate::from_flat(inserted_row, Some(user_id), connection)?;
                 bincode::serialize(&nested_row)?
            },
            web_common::database::Table::Notifications => unreachable!("Table `notifications` is not insertable as it does not have a known column associated to a creator user id."),
            web_common::database::Table::ObservationSubjects => unreachable!("Table `observation_subjects` is not insertable as it does not have a known column associated to a creator user id."),
            web_common::database::Table::Observations => {
                let row: web_common::database::NewObservation = bincode::deserialize::<web_common::database::NewObservation>(&row)?;
                let inserted_row: crate::models::Observation = <web_common::database::NewObservation as InsertRow>::insert(row, user_id, connection)?;
                let nested_row = crate::nested_models::NestedObservation::from_flat(inserted_row, Some(user_id), connection)?;
                 bincode::serialize(&nested_row)?
            },
            web_common::database::Table::OrganismBioOttTaxonItems => unreachable!("Table `organism_bio_ott_taxon_items` is not insertable as it does not have a known column associated to a creator user id."),
            web_common::database::Table::Organisms => {
                let row: web_common::database::NewOrganism = bincode::deserialize::<web_common::database::NewOrganism>(&row)?;
                let inserted_row: crate::models::Organism = <web_common::database::NewOrganism as InsertRow>::insert(row, user_id, connection)?;
                let nested_row = crate::nested_models::NestedOrganism::from_flat(inserted_row, Some(user_id), connection)?;
                 bincode::serialize(&nested_row)?
            },
            web_common::database::Table::Organizations => unreachable!("Table `organizations` is not insertable as it does not have a known column associated to a creator user id."),
            web_common::database::Table::PermanenceCategories => unreachable!("Table `permanence_categories` is not insertable as it does not have a known column associated to a creator user id."),
            web_common::database::Table::ProjectStates => unreachable!("Table `project_states` is not insertable as it does not have a known column associated to a creator user id."),
            web_common::database::Table::Projects => {
                let row: web_common::database::NewProject = bincode::deserialize::<web_common::database::NewProject>(&row)?;
                let inserted_row: crate::models::Project = <web_common::database::NewProject as InsertRow>::insert(row, user_id, connection)?;
                let nested_row = crate::nested_models::NestedProject::from_flat(inserted_row, Some(user_id), connection)?;
                 bincode::serialize(&nested_row)?
            },
            web_common::database::Table::ProjectsTeamsRoleInvitations => unreachable!("Table `projects_teams_role_invitations` is not insertable as it does not have a known column associated to a creator user id."),
            web_common::database::Table::ProjectsTeamsRoleRequests => unreachable!("Table `projects_teams_role_requests` is not insertable as it does not have a known column associated to a creator user id."),
            web_common::database::Table::ProjectsTeamsRoles => unreachable!("Table `projects_teams_roles` is not insertable as it does not have a known column associated to a creator user id."),
            web_common::database::Table::ProjectsUsersRoleInvitations => unreachable!("Table `projects_users_role_invitations` is not insertable as it does not have a known column associated to a creator user id."),
            web_common::database::Table::ProjectsUsersRoleRequests => unreachable!("Table `projects_users_role_requests` is not insertable as it does not have a known column associated to a creator user id."),
            web_common::database::Table::ProjectsUsersRoles => unreachable!("Table `projects_users_roles` is not insertable as it does not have a known column associated to a creator user id."),
            web_common::database::Table::Roles => unreachable!("Table `roles` is not insertable as it does not have a known column associated to a creator user id."),
            web_common::database::Table::SampleBioOttTaxonItems => unreachable!("Table `sample_bio_ott_taxon_items` is not insertable as it does not have a known column associated to a creator user id."),
            web_common::database::Table::SampleContainerCategories => unreachable!("Table `sample_container_categories` is not insertable as it does not have a known column associated to a creator user id."),
            web_common::database::Table::SampleContainers => {
                let row: web_common::database::NewSampleContainer = bincode::deserialize::<web_common::database::NewSampleContainer>(&row)?;
                let inserted_row: crate::models::SampleContainer = <web_common::database::NewSampleContainer as InsertRow>::insert(row, user_id, connection)?;
                let nested_row = crate::nested_models::NestedSampleContainer::from_flat(inserted_row, Some(user_id), connection)?;
                 bincode::serialize(&nested_row)?
            },
            web_common::database::Table::SampleStates => unreachable!("Table `sample_states` is not insertable as it does not have a known column associated to a creator user id."),
            web_common::database::Table::Samples => {
                let row: web_common::database::NewSample = bincode::deserialize::<web_common::database::NewSample>(&row)?;
                let inserted_row: crate::models::Sample = <web_common::database::NewSample as InsertRow>::insert(row, user_id, connection)?;
                let nested_row = crate::nested_models::NestedSample::from_flat(inserted_row, Some(user_id), connection)?;
                 bincode::serialize(&nested_row)?
            },
            web_common::database::Table::Spectra => {
                let row: web_common::database::NewSpectra = bincode::deserialize::<web_common::database::NewSpectra>(&row)?;
                let inserted_row: crate::models::Spectra = <web_common::database::NewSpectra as InsertRow>::insert(row, user_id, connection)?;
                let nested_row = crate::nested_models::NestedSpectra::from_flat(inserted_row, Some(user_id), connection)?;
                 bincode::serialize(&nested_row)?
            },
            web_common::database::Table::SpectraCollections => {
                let row: web_common::database::NewSpectraCollection = bincode::deserialize::<web_common::database::NewSpectraCollection>(&row)?;
                let inserted_row: crate::models::SpectraCollection = <web_common::database::NewSpectraCollection as InsertRow>::insert(row, user_id, connection)?;
                let nested_row = crate::nested_models::NestedSpectraCollection::from_flat(inserted_row, Some(user_id), connection)?;
                 bincode::serialize(&nested_row)?
            },
            web_common::database::Table::TeamStates => unreachable!("Table `team_states` is not insertable as it does not have a known column associated to a creator user id."),
            web_common::database::Table::Teams => {
                let row: web_common::database::NewTeam = bincode::deserialize::<web_common::database::NewTeam>(&row)?;
                let inserted_row: crate::models::Team = <web_common::database::NewTeam as InsertRow>::insert(row, user_id, connection)?;
                let nested_row = crate::nested_models::NestedTeam::from_flat(inserted_row, connection)?;
                 bincode::serialize(&nested_row)?
            },
            web_common::database::Table::TeamsTeamsRoleInvitations => unreachable!("Table `teams_teams_role_invitations` is not insertable as it does not have a known column associated to a creator user id."),
            web_common::database::Table::TeamsUsersRoleInvitations => unreachable!("Table `teams_users_role_invitations` is not insertable as it does not have a known column associated to a creator user id."),
            web_common::database::Table::TeamsUsersRoleRequests => unreachable!("Table `teams_users_role_requests` is not insertable as it does not have a known column associated to a creator user id."),
            web_common::database::Table::TeamsUsersRoles => unreachable!("Table `teams_users_roles` is not insertable as it does not have a known column associated to a creator user id."),
            web_common::database::Table::Units => unreachable!("Table `units` is not insertable as it does not have a known column associated to a creator user id."),
            web_common::database::Table::UserEmails => {
                let row: web_common::database::NewUserEmail = bincode::deserialize::<web_common::database::NewUserEmail>(&row)?;
                let inserted_row: crate::models::UserEmail = <web_common::database::NewUserEmail as InsertRow>::insert(row, user_id, connection)?;
                let nested_row = crate::nested_models::NestedUserEmail::from_flat(inserted_row, connection)?;
                 bincode::serialize(&nested_row)?
            },
            web_common::database::Table::Users => {
                let row: web_common::database::NewUser = bincode::deserialize::<web_common::database::NewUser>(&row)?;
                let inserted_row: crate::models::User = <web_common::database::NewUser as InsertRow>::insert(row, user_id, connection)?;
                 bincode::serialize(&inserted_row)?
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
            web_common::database::Table::Materials => unreachable!("Table `materials` is not updatable as it does not have a known column associated to an updater user id."),
            web_common::database::Table::NameplateCategories => unreachable!("Table `nameplate_categories` is not updatable as it does not have a known column associated to an updater user id."),
            web_common::database::Table::Nameplates => {
                let row: web_common::database::UpdateNameplate = bincode::deserialize::<web_common::database::UpdateNameplate>(&row)?;
                let updated_row: crate::models::Nameplate = <web_common::database::UpdateNameplate as UpdateRow>::update(row, user_id, connection)?;
                let nested_row = crate::nested_models::NestedNameplate::from_flat(updated_row, Some(user_id), connection)?;
                 bincode::serialize(&nested_row)?
            },
            web_common::database::Table::Notifications => unreachable!("Table `notifications` is not updatable as it does not have a known column associated to an updater user id."),
            web_common::database::Table::ObservationSubjects => unreachable!("Table `observation_subjects` is not updatable as it does not have a known column associated to an updater user id."),
            web_common::database::Table::Observations => {
                let row: web_common::database::NewObservation = bincode::deserialize::<web_common::database::NewObservation>(&row)?;
                let updated_row: crate::models::Observation = <web_common::database::NewObservation as UpdateRow>::update(row, user_id, connection)?;
                let nested_row = crate::nested_models::NestedObservation::from_flat(updated_row, Some(user_id), connection)?;
                 bincode::serialize(&nested_row)?
            },
            web_common::database::Table::OrganismBioOttTaxonItems => unreachable!("Table `organism_bio_ott_taxon_items` is not updatable as it does not have a known column associated to an updater user id."),
            web_common::database::Table::Organisms => {
                let row: web_common::database::NewOrganism = bincode::deserialize::<web_common::database::NewOrganism>(&row)?;
                let updated_row: crate::models::Organism = <web_common::database::NewOrganism as UpdateRow>::update(row, user_id, connection)?;
                let nested_row = crate::nested_models::NestedOrganism::from_flat(updated_row, Some(user_id), connection)?;
                 bincode::serialize(&nested_row)?
            },
            web_common::database::Table::Organizations => unreachable!("Table `organizations` is not updatable as it does not have a known column associated to an updater user id."),
            web_common::database::Table::PermanenceCategories => unreachable!("Table `permanence_categories` is not updatable as it does not have a known column associated to an updater user id."),
            web_common::database::Table::ProjectStates => unreachable!("Table `project_states` is not updatable as it does not have a known column associated to an updater user id."),
            web_common::database::Table::Projects => {
                let row: web_common::database::UpdateProject = bincode::deserialize::<web_common::database::UpdateProject>(&row)?;
                let updated_row: crate::models::Project = <web_common::database::UpdateProject as UpdateRow>::update(row, user_id, connection)?;
                let nested_row = crate::nested_models::NestedProject::from_flat(updated_row, Some(user_id), connection)?;
                 bincode::serialize(&nested_row)?
            },
            web_common::database::Table::ProjectsTeamsRoleInvitations => unreachable!("Table `projects_teams_role_invitations` is not updatable as it does not have a known column associated to an updater user id."),
            web_common::database::Table::ProjectsTeamsRoleRequests => unreachable!("Table `projects_teams_role_requests` is not updatable as it does not have a known column associated to an updater user id."),
            web_common::database::Table::ProjectsTeamsRoles => unreachable!("Table `projects_teams_roles` is not updatable as it does not have a known column associated to an updater user id."),
            web_common::database::Table::ProjectsUsersRoleInvitations => unreachable!("Table `projects_users_role_invitations` is not updatable as it does not have a known column associated to an updater user id."),
            web_common::database::Table::ProjectsUsersRoleRequests => unreachable!("Table `projects_users_role_requests` is not updatable as it does not have a known column associated to an updater user id."),
            web_common::database::Table::ProjectsUsersRoles => unreachable!("Table `projects_users_roles` is not updatable as it does not have a known column associated to an updater user id."),
            web_common::database::Table::Roles => unreachable!("Table `roles` is not updatable as it does not have a known column associated to an updater user id."),
            web_common::database::Table::SampleBioOttTaxonItems => unreachable!("Table `sample_bio_ott_taxon_items` is not updatable as it does not have a known column associated to an updater user id."),
            web_common::database::Table::SampleContainerCategories => unreachable!("Table `sample_container_categories` is not updatable as it does not have a known column associated to an updater user id."),
            web_common::database::Table::SampleContainers => {
                let row: web_common::database::UpdateSampleContainer = bincode::deserialize::<web_common::database::UpdateSampleContainer>(&row)?;
                let updated_row: crate::models::SampleContainer = <web_common::database::UpdateSampleContainer as UpdateRow>::update(row, user_id, connection)?;
                let nested_row = crate::nested_models::NestedSampleContainer::from_flat(updated_row, Some(user_id), connection)?;
                 bincode::serialize(&nested_row)?
            },
            web_common::database::Table::SampleStates => unreachable!("Table `sample_states` is not updatable as it does not have a known column associated to an updater user id."),
            web_common::database::Table::Samples => {
                let row: web_common::database::NewSample = bincode::deserialize::<web_common::database::NewSample>(&row)?;
                let updated_row: crate::models::Sample = <web_common::database::NewSample as UpdateRow>::update(row, user_id, connection)?;
                let nested_row = crate::nested_models::NestedSample::from_flat(updated_row, Some(user_id), connection)?;
                 bincode::serialize(&nested_row)?
            },
            web_common::database::Table::Spectra => {
                let row: web_common::database::UpdateSpectra = bincode::deserialize::<web_common::database::UpdateSpectra>(&row)?;
                let updated_row: crate::models::Spectra = <web_common::database::UpdateSpectra as UpdateRow>::update(row, user_id, connection)?;
                let nested_row = crate::nested_models::NestedSpectra::from_flat(updated_row, Some(user_id), connection)?;
                 bincode::serialize(&nested_row)?
            },
            web_common::database::Table::SpectraCollections => {
                let row: web_common::database::UpdateSpectraCollection = bincode::deserialize::<web_common::database::UpdateSpectraCollection>(&row)?;
                let updated_row: crate::models::SpectraCollection = <web_common::database::UpdateSpectraCollection as UpdateRow>::update(row, user_id, connection)?;
                let nested_row = crate::nested_models::NestedSpectraCollection::from_flat(updated_row, Some(user_id), connection)?;
                 bincode::serialize(&nested_row)?
            },
            web_common::database::Table::TeamStates => unreachable!("Table `team_states` is not updatable as it does not have a known column associated to an updater user id."),
            web_common::database::Table::Teams => {
                let row: web_common::database::UpdateTeam = bincode::deserialize::<web_common::database::UpdateTeam>(&row)?;
                let updated_row: crate::models::Team = <web_common::database::UpdateTeam as UpdateRow>::update(row, user_id, connection)?;
                let nested_row = crate::nested_models::NestedTeam::from_flat(updated_row, connection)?;
                 bincode::serialize(&nested_row)?
            },
            web_common::database::Table::TeamsTeamsRoleInvitations => unreachable!("Table `teams_teams_role_invitations` is not updatable as it does not have a known column associated to an updater user id."),
            web_common::database::Table::TeamsUsersRoleInvitations => unreachable!("Table `teams_users_role_invitations` is not updatable as it does not have a known column associated to an updater user id."),
            web_common::database::Table::TeamsUsersRoleRequests => unreachable!("Table `teams_users_role_requests` is not updatable as it does not have a known column associated to an updater user id."),
            web_common::database::Table::TeamsUsersRoles => unreachable!("Table `teams_users_roles` is not updatable as it does not have a known column associated to an updater user id."),
            web_common::database::Table::Units => unreachable!("Table `units` is not updatable as it does not have a known column associated to an updater user id."),
            web_common::database::Table::UserEmails => unreachable!("Table `user_emails` is not updatable as it does not have a known column associated to an updater user id."),
            web_common::database::Table::Users => {
                let row: web_common::database::UpdateUser = bincode::deserialize::<web_common::database::UpdateUser>(&row)?;
                let updated_row: crate::models::User = <web_common::database::UpdateUser as UpdateRow>::update(row, user_id, connection)?;
                 bincode::serialize(&updated_row)?
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
    /// * `user_id` - The id of the user retrieving the row.
    /// * `connection` - The database connection.
    ///
    /// # Returns
    /// The bincode-serialized row of the table.
    fn from_flat_str(
         &self,
         row: &str,
         user_id: Option<i32>,
         connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>
) -> Result<Vec<u8>, web_common::api::ApiError>;
}

impl FromFlatStrTable for web_common::database::Table {

    fn from_flat_str(
        &self,
        row: &str,
        user_id: Option<i32>,
        connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>
    ) -> Result<Vec<u8>, web_common::api::ApiError> {
        Ok(match self {
            web_common::database::Table::BioOttRanks => {
                let flat_row: crate::models::BioOttRank = serde_json::from_str::<crate::models::BioOttRank>(row)?;
                 let richest_row = crate::nested_models::NestedBioOttRank::from_flat(flat_row, connection)?;
                 bincode::serialize(&richest_row)?
            },
            web_common::database::Table::BioOttTaxonItems => {
                let flat_row: crate::models::BioOttTaxonItem = serde_json::from_str::<crate::models::BioOttTaxonItem>(row)?;
                 let richest_row = crate::nested_models::NestedBioOttTaxonItem::from_flat(flat_row, connection)?;
                 bincode::serialize(&richest_row)?
            },
            web_common::database::Table::Colors => bincode::serialize(&serde_json::from_str::<crate::models::Color>(row)?)?,
            web_common::database::Table::Countries => bincode::serialize(&serde_json::from_str::<crate::models::Country>(row)?)?,
            web_common::database::Table::DerivedSamples => {
                let flat_row: crate::models::DerivedSample = serde_json::from_str::<crate::models::DerivedSample>(row)?;
                 let richest_row = crate::nested_models::NestedDerivedSample::from_flat(flat_row, user_id, connection)?;
                 bincode::serialize(&richest_row)?
            },
            web_common::database::Table::DocumentFormats => {
                let flat_row: crate::models::DocumentFormat = serde_json::from_str::<crate::models::DocumentFormat>(row)?;
                 let richest_row = crate::nested_models::NestedDocumentFormat::from_flat(flat_row, connection)?;
                 bincode::serialize(&richest_row)?
            },
            web_common::database::Table::FontAwesomeIcons => bincode::serialize(&serde_json::from_str::<crate::models::FontAwesomeIcon>(row)?)?,
            web_common::database::Table::LoginProviders => {
                let flat_row: crate::models::LoginProvider = serde_json::from_str::<crate::models::LoginProvider>(row)?;
                 let richest_row = crate::nested_models::NestedLoginProvider::from_flat(flat_row, connection)?;
                 bincode::serialize(&richest_row)?
            },
            web_common::database::Table::Materials => {
                let flat_row: crate::models::Material = serde_json::from_str::<crate::models::Material>(row)?;
                 let richest_row = crate::nested_models::NestedMaterial::from_flat(flat_row, connection)?;
                 bincode::serialize(&richest_row)?
            },
            web_common::database::Table::NameplateCategories => {
                let flat_row: crate::models::NameplateCategory = serde_json::from_str::<crate::models::NameplateCategory>(row)?;
                 let richest_row = crate::nested_models::NestedNameplateCategory::from_flat(flat_row, connection)?;
                 bincode::serialize(&richest_row)?
            },
            web_common::database::Table::Nameplates => {
                let flat_row: crate::models::Nameplate = serde_json::from_str::<crate::models::Nameplate>(row)?;
                 let richest_row = crate::nested_models::NestedNameplate::from_flat(flat_row, user_id, connection)?;
                 bincode::serialize(&richest_row)?
            },
            web_common::database::Table::Notifications => {
                let flat_row: crate::models::Notification = serde_json::from_str::<crate::models::Notification>(row)?;
                 let richest_row = crate::nested_models::NestedNotification::from_flat(flat_row, connection)?;
                 bincode::serialize(&richest_row)?
            },
            web_common::database::Table::ObservationSubjects => {
                let flat_row: crate::models::ObservationSubject = serde_json::from_str::<crate::models::ObservationSubject>(row)?;
                 let richest_row = crate::nested_models::NestedObservationSubject::from_flat(flat_row, connection)?;
                 bincode::serialize(&richest_row)?
            },
            web_common::database::Table::Observations => {
                let flat_row: crate::models::Observation = serde_json::from_str::<crate::models::Observation>(row)?;
                 let richest_row = crate::nested_models::NestedObservation::from_flat(flat_row, user_id, connection)?;
                 bincode::serialize(&richest_row)?
            },
            web_common::database::Table::OrganismBioOttTaxonItems => {
                let flat_row: crate::models::OrganismBioOttTaxonItem = serde_json::from_str::<crate::models::OrganismBioOttTaxonItem>(row)?;
                 let richest_row = crate::nested_models::NestedOrganismBioOttTaxonItem::from_flat(flat_row, user_id, connection)?;
                 bincode::serialize(&richest_row)?
            },
            web_common::database::Table::Organisms => {
                let flat_row: crate::models::Organism = serde_json::from_str::<crate::models::Organism>(row)?;
                 let richest_row = crate::nested_models::NestedOrganism::from_flat(flat_row, user_id, connection)?;
                 bincode::serialize(&richest_row)?
            },
            web_common::database::Table::Organizations => {
                let flat_row: crate::models::Organization = serde_json::from_str::<crate::models::Organization>(row)?;
                 let richest_row = crate::nested_models::NestedOrganization::from_flat(flat_row, connection)?;
                 bincode::serialize(&richest_row)?
            },
            web_common::database::Table::PermanenceCategories => {
                let flat_row: crate::models::PermanenceCategory = serde_json::from_str::<crate::models::PermanenceCategory>(row)?;
                 let richest_row = crate::nested_models::NestedPermanenceCategory::from_flat(flat_row, connection)?;
                 bincode::serialize(&richest_row)?
            },
            web_common::database::Table::ProjectStates => {
                let flat_row: crate::models::ProjectState = serde_json::from_str::<crate::models::ProjectState>(row)?;
                 let richest_row = crate::nested_models::NestedProjectState::from_flat(flat_row, connection)?;
                 bincode::serialize(&richest_row)?
            },
            web_common::database::Table::Projects => {
                let flat_row: crate::models::Project = serde_json::from_str::<crate::models::Project>(row)?;
                 let richest_row = crate::nested_models::NestedProject::from_flat(flat_row, user_id, connection)?;
                 bincode::serialize(&richest_row)?
            },
            web_common::database::Table::ProjectsTeamsRoleInvitations => {
                let flat_row: crate::models::ProjectsTeamsRoleInvitation = serde_json::from_str::<crate::models::ProjectsTeamsRoleInvitation>(row)?;
                 let richest_row = crate::nested_models::NestedProjectsTeamsRoleInvitation::from_flat(flat_row, user_id, connection)?;
                 bincode::serialize(&richest_row)?
            },
            web_common::database::Table::ProjectsTeamsRoleRequests => {
                let flat_row: crate::models::ProjectsTeamsRoleRequest = serde_json::from_str::<crate::models::ProjectsTeamsRoleRequest>(row)?;
                 let richest_row = crate::nested_models::NestedProjectsTeamsRoleRequest::from_flat(flat_row, user_id, connection)?;
                 bincode::serialize(&richest_row)?
            },
            web_common::database::Table::ProjectsTeamsRoles => {
                let flat_row: crate::models::ProjectsTeamsRole = serde_json::from_str::<crate::models::ProjectsTeamsRole>(row)?;
                 let richest_row = crate::nested_models::NestedProjectsTeamsRole::from_flat(flat_row, user_id, connection)?;
                 bincode::serialize(&richest_row)?
            },
            web_common::database::Table::ProjectsUsersRoleInvitations => {
                let flat_row: crate::models::ProjectsUsersRoleInvitation = serde_json::from_str::<crate::models::ProjectsUsersRoleInvitation>(row)?;
                 let richest_row = crate::nested_models::NestedProjectsUsersRoleInvitation::from_flat(flat_row, user_id, connection)?;
                 bincode::serialize(&richest_row)?
            },
            web_common::database::Table::ProjectsUsersRoleRequests => {
                let flat_row: crate::models::ProjectsUsersRoleRequest = serde_json::from_str::<crate::models::ProjectsUsersRoleRequest>(row)?;
                 let richest_row = crate::nested_models::NestedProjectsUsersRoleRequest::from_flat(flat_row, user_id, connection)?;
                 bincode::serialize(&richest_row)?
            },
            web_common::database::Table::ProjectsUsersRoles => {
                let flat_row: crate::models::ProjectsUsersRole = serde_json::from_str::<crate::models::ProjectsUsersRole>(row)?;
                 let richest_row = crate::nested_models::NestedProjectsUsersRole::from_flat(flat_row, user_id, connection)?;
                 bincode::serialize(&richest_row)?
            },
            web_common::database::Table::Roles => {
                let flat_row: crate::models::Role = serde_json::from_str::<crate::models::Role>(row)?;
                 let richest_row = crate::nested_models::NestedRole::from_flat(flat_row, connection)?;
                 bincode::serialize(&richest_row)?
            },
            web_common::database::Table::SampleBioOttTaxonItems => {
                let flat_row: crate::models::SampleBioOttTaxonItem = serde_json::from_str::<crate::models::SampleBioOttTaxonItem>(row)?;
                 let richest_row = crate::nested_models::NestedSampleBioOttTaxonItem::from_flat(flat_row, user_id, connection)?;
                 bincode::serialize(&richest_row)?
            },
            web_common::database::Table::SampleContainerCategories => {
                let flat_row: crate::models::SampleContainerCategory = serde_json::from_str::<crate::models::SampleContainerCategory>(row)?;
                 let richest_row = crate::nested_models::NestedSampleContainerCategory::from_flat(flat_row, connection)?;
                 bincode::serialize(&richest_row)?
            },
            web_common::database::Table::SampleContainers => {
                let flat_row: crate::models::SampleContainer = serde_json::from_str::<crate::models::SampleContainer>(row)?;
                 let richest_row = crate::nested_models::NestedSampleContainer::from_flat(flat_row, user_id, connection)?;
                 bincode::serialize(&richest_row)?
            },
            web_common::database::Table::SampleStates => {
                let flat_row: crate::models::SampleState = serde_json::from_str::<crate::models::SampleState>(row)?;
                 let richest_row = crate::nested_models::NestedSampleState::from_flat(flat_row, connection)?;
                 bincode::serialize(&richest_row)?
            },
            web_common::database::Table::Samples => {
                let flat_row: crate::models::Sample = serde_json::from_str::<crate::models::Sample>(row)?;
                 let richest_row = crate::nested_models::NestedSample::from_flat(flat_row, user_id, connection)?;
                 bincode::serialize(&richest_row)?
            },
            web_common::database::Table::Spectra => {
                let flat_row: crate::models::Spectra = serde_json::from_str::<crate::models::Spectra>(row)?;
                 let richest_row = crate::nested_models::NestedSpectra::from_flat(flat_row, user_id, connection)?;
                 bincode::serialize(&richest_row)?
            },
            web_common::database::Table::SpectraCollections => {
                let flat_row: crate::models::SpectraCollection = serde_json::from_str::<crate::models::SpectraCollection>(row)?;
                 let richest_row = crate::nested_models::NestedSpectraCollection::from_flat(flat_row, user_id, connection)?;
                 bincode::serialize(&richest_row)?
            },
            web_common::database::Table::TeamStates => {
                let flat_row: crate::models::TeamState = serde_json::from_str::<crate::models::TeamState>(row)?;
                 let richest_row = crate::nested_models::NestedTeamState::from_flat(flat_row, connection)?;
                 bincode::serialize(&richest_row)?
            },
            web_common::database::Table::Teams => {
                let flat_row: crate::models::Team = serde_json::from_str::<crate::models::Team>(row)?;
                 let richest_row = crate::nested_models::NestedTeam::from_flat(flat_row, connection)?;
                 bincode::serialize(&richest_row)?
            },
            web_common::database::Table::TeamsTeamsRoleInvitations => {
                let flat_row: crate::models::TeamsTeamsRoleInvitation = serde_json::from_str::<crate::models::TeamsTeamsRoleInvitation>(row)?;
                 let richest_row = crate::nested_models::NestedTeamsTeamsRoleInvitation::from_flat(flat_row, connection)?;
                 bincode::serialize(&richest_row)?
            },
            web_common::database::Table::TeamsUsersRoleInvitations => {
                let flat_row: crate::models::TeamsUsersRoleInvitation = serde_json::from_str::<crate::models::TeamsUsersRoleInvitation>(row)?;
                 let richest_row = crate::nested_models::NestedTeamsUsersRoleInvitation::from_flat(flat_row, connection)?;
                 bincode::serialize(&richest_row)?
            },
            web_common::database::Table::TeamsUsersRoleRequests => {
                let flat_row: crate::models::TeamsUsersRoleRequest = serde_json::from_str::<crate::models::TeamsUsersRoleRequest>(row)?;
                 let richest_row = crate::nested_models::NestedTeamsUsersRoleRequest::from_flat(flat_row, connection)?;
                 bincode::serialize(&richest_row)?
            },
            web_common::database::Table::TeamsUsersRoles => {
                let flat_row: crate::models::TeamsUsersRole = serde_json::from_str::<crate::models::TeamsUsersRole>(row)?;
                 let richest_row = crate::nested_models::NestedTeamsUsersRole::from_flat(flat_row, connection)?;
                 bincode::serialize(&richest_row)?
            },
            web_common::database::Table::Units => bincode::serialize(&serde_json::from_str::<crate::models::Unit>(row)?)?,
            web_common::database::Table::UserEmails => {
                let flat_row: crate::models::UserEmail = serde_json::from_str::<crate::models::UserEmail>(row)?;
                 let richest_row = crate::nested_models::NestedUserEmail::from_flat(flat_row, connection)?;
                 bincode::serialize(&richest_row)?
            },
            web_common::database::Table::Users => bincode::serialize(&serde_json::from_str::<crate::models::User>(row)?)?,
            web_common::database::Table::UsersUsersRoleInvitations => {
                let flat_row: crate::models::UsersUsersRoleInvitation = serde_json::from_str::<crate::models::UsersUsersRoleInvitation>(row)?;
                 let richest_row = crate::nested_models::NestedUsersUsersRoleInvitation::from_flat(flat_row, connection)?;
                 bincode::serialize(&richest_row)?
            },
            web_common::database::Table::UsersUsersRoleRequests => {
                let flat_row: crate::models::UsersUsersRoleRequest = serde_json::from_str::<crate::models::UsersUsersRoleRequest>(row)?;
                 let richest_row = crate::nested_models::NestedUsersUsersRoleRequest::from_flat(flat_row, connection)?;
                 bincode::serialize(&richest_row)?
            },
            web_common::database::Table::UsersUsersRoles => {
                let flat_row: crate::models::UsersUsersRole = serde_json::from_str::<crate::models::UsersUsersRole>(row)?;
                 let richest_row = crate::nested_models::NestedUsersUsersRole::from_flat(flat_row, connection)?;
                 bincode::serialize(&richest_row)?
            },
        })
    }
}
