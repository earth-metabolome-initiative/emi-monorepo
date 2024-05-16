//! This module contains the table names enumeration.
//!
//! This module is automatically generated. Do not write anything here.

use serde::Deserialize;
use serde::Serialize;
#[derive(Deserialize, Serialize, Clone, Debug, PartialEq, Eq, Copy)]
pub enum Table {
    BioOttRanks,
    BioOttTaxonItems,
    Colors,
    Countries,
    DerivedSamples,
    DocumentFormats,
    FontAwesomeIcons,
    LoginProviders,
    Notifications,
    Organizations,
    ProjectStates,
    Projects,
    ProjectsTeamsRoles,
    ProjectsUsersRoles,
    Roles,
    SampleBioOttTaxonItems,
    SampleStates,
    SampledIndividualBioOttTaxonItems,
    SampledIndividuals,
    SampledIndividualsTeamsRoles,
    SampledIndividualsUsersRoles,
    Samples,
    SamplesTeamsRoles,
    SamplesUsersRoles,
    Spectra,
    SpectraCollections,
    SpectraCollectionsTeamsRoles,
    SpectraCollectionsUsersRoles,
    TeamStates,
    Teams,
    TeamsUsersRoles,
    Units,
    UserEmails,
    Users,
}

impl AsRef<str> for Table {
    fn as_ref(&self) -> &str {
        match self {
            Table::BioOttRanks => "bio_ott_ranks",
            Table::BioOttTaxonItems => "bio_ott_taxon_items",
            Table::Colors => "colors",
            Table::Countries => "countries",
            Table::DerivedSamples => "derived_samples",
            Table::DocumentFormats => "document_formats",
            Table::FontAwesomeIcons => "font_awesome_icons",
            Table::LoginProviders => "login_providers",
            Table::Notifications => "notifications",
            Table::Organizations => "organizations",
            Table::ProjectStates => "project_states",
            Table::Projects => "projects",
            Table::ProjectsTeamsRoles => "projects_teams_roles",
            Table::ProjectsUsersRoles => "projects_users_roles",
            Table::Roles => "roles",
            Table::SampleBioOttTaxonItems => "sample_bio_ott_taxon_items",
            Table::SampleStates => "sample_states",
            Table::SampledIndividualBioOttTaxonItems => "sampled_individual_bio_ott_taxon_items",
            Table::SampledIndividuals => "sampled_individuals",
            Table::SampledIndividualsTeamsRoles => "sampled_individuals_teams_roles",
            Table::SampledIndividualsUsersRoles => "sampled_individuals_users_roles",
            Table::Samples => "samples",
            Table::SamplesTeamsRoles => "samples_teams_roles",
            Table::SamplesUsersRoles => "samples_users_roles",
            Table::Spectra => "spectra",
            Table::SpectraCollections => "spectra_collections",
            Table::SpectraCollectionsTeamsRoles => "spectra_collections_teams_roles",
            Table::SpectraCollectionsUsersRoles => "spectra_collections_users_roles",
            Table::TeamStates => "team_states",
            Table::Teams => "teams",
            Table::TeamsUsersRoles => "teams_users_roles",
            Table::Units => "units",
            Table::UserEmails => "user_emails",
            Table::Users => "users",
        }
    }
}
impl std::fmt::Display for Table {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_ref())
    }
}
impl From<Table> for String {
    fn from(table: Table) -> Self {
        table.to_string()
    }
}
impl std::convert::TryFrom<&str> for Table {
    type Error = String;
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "bio_ott_ranks" => Ok(Table::BioOttRanks),
            "bio_ott_taxon_items" => Ok(Table::BioOttTaxonItems),
            "colors" => Ok(Table::Colors),
            "countries" => Ok(Table::Countries),
            "derived_samples" => Ok(Table::DerivedSamples),
            "document_formats" => Ok(Table::DocumentFormats),
            "font_awesome_icons" => Ok(Table::FontAwesomeIcons),
            "login_providers" => Ok(Table::LoginProviders),
            "notifications" => Ok(Table::Notifications),
            "organizations" => Ok(Table::Organizations),
            "project_states" => Ok(Table::ProjectStates),
            "projects" => Ok(Table::Projects),
            "projects_teams_roles" => Ok(Table::ProjectsTeamsRoles),
            "projects_users_roles" => Ok(Table::ProjectsUsersRoles),
            "roles" => Ok(Table::Roles),
            "sample_bio_ott_taxon_items" => Ok(Table::SampleBioOttTaxonItems),
            "sample_states" => Ok(Table::SampleStates),
            "sampled_individual_bio_ott_taxon_items" => Ok(Table::SampledIndividualBioOttTaxonItems),
            "sampled_individuals" => Ok(Table::SampledIndividuals),
            "sampled_individuals_teams_roles" => Ok(Table::SampledIndividualsTeamsRoles),
            "sampled_individuals_users_roles" => Ok(Table::SampledIndividualsUsersRoles),
            "samples" => Ok(Table::Samples),
            "samples_teams_roles" => Ok(Table::SamplesTeamsRoles),
            "samples_users_roles" => Ok(Table::SamplesUsersRoles),
            "spectra" => Ok(Table::Spectra),
            "spectra_collections" => Ok(Table::SpectraCollections),
            "spectra_collections_teams_roles" => Ok(Table::SpectraCollectionsTeamsRoles),
            "spectra_collections_users_roles" => Ok(Table::SpectraCollectionsUsersRoles),
            "team_states" => Ok(Table::TeamStates),
            "teams" => Ok(Table::Teams),
            "teams_users_roles" => Ok(Table::TeamsUsersRoles),
            "units" => Ok(Table::Units),
            "user_emails" => Ok(Table::UserEmails),
            "users" => Ok(Table::Users),
            table_name => Err(format!("Unknown table name: {table_name}")),
        }
    }
}
impl std::convert::TryFrom<String> for Table {
    type Error = String;
    fn try_from(value: String) -> Result<Self, Self::Error> {
        Self::try_from(value.as_str())
    }
}
#[cfg(feature = "frontend")]
impl Table {
    /// Delete the row from the table.
    ///
    /// # Arguments
    /// * `primary_key` - The primary key of the row.
    /// * `connection` - The database connection.
    ///
    /// # Returns
    /// The number of rows deleted.
    pub async fn delete<C>(
        &self,
        primary_key: crate::database::operations::PrimaryKey,
        connection: &mut gluesql::prelude::Glue<C>,
    ) -> Result<usize, gluesql::prelude::Error> where
        C: gluesql::core::store::GStore + gluesql::core::store::GStoreMut,
    {
        match self {
            Table::BioOttRanks => {
                crate::database::BioOttRank::delete_from_id(primary_key.into(), connection).await
            },
            Table::BioOttTaxonItems => {
                crate::database::BioOttTaxonItem::delete_from_id(primary_key.into(), connection).await
            },
            Table::Colors => {
                crate::database::Color::delete_from_id(primary_key.into(), connection).await
            },
            Table::Countries => {
                crate::database::Country::delete_from_id(primary_key.into(), connection).await
            },
            Table::DerivedSamples => {
                crate::database::DerivedSample::delete_from_id(primary_key.into(), connection).await
            },
            Table::DocumentFormats => {
                crate::database::DocumentFormat::delete_from_id(primary_key.into(), connection).await
            },
            Table::FontAwesomeIcons => {
                crate::database::FontAwesomeIcon::delete_from_id(primary_key.into(), connection).await
            },
            Table::LoginProviders => {
                crate::database::LoginProvider::delete_from_id(primary_key.into(), connection).await
            },
            Table::Notifications => {
                crate::database::Notification::delete_from_id(primary_key.into(), connection).await
            },
            Table::Organizations => {
                crate::database::Organization::delete_from_id(primary_key.into(), connection).await
            },
            Table::ProjectStates => {
                crate::database::ProjectState::delete_from_id(primary_key.into(), connection).await
            },
            Table::Projects => {
                crate::database::Project::delete_from_id(primary_key.into(), connection).await
            },
            Table::ProjectsTeamsRoles => {
                crate::database::ProjectsTeamsRole::delete_from_id(primary_key.into(), connection).await
            },
            Table::ProjectsUsersRoles => {
                crate::database::ProjectsUsersRole::delete_from_id(primary_key.into(), connection).await
            },
            Table::Roles => {
                crate::database::Role::delete_from_id(primary_key.into(), connection).await
            },
            Table::SampleBioOttTaxonItems => {
                crate::database::SampleBioOttTaxonItem::delete_from_id(primary_key.into(), connection).await
            },
            Table::SampleStates => {
                crate::database::SampleState::delete_from_id(primary_key.into(), connection).await
            },
            Table::SampledIndividualBioOttTaxonItems => {
                crate::database::SampledIndividualBioOttTaxonItem::delete_from_id(primary_key.into(), connection).await
            },
            Table::SampledIndividuals => {
                crate::database::SampledIndividual::delete_from_id(primary_key.into(), connection).await
            },
            Table::SampledIndividualsTeamsRoles => {
                crate::database::SampledIndividualsTeamsRole::delete_from_id(primary_key.into(), connection).await
            },
            Table::SampledIndividualsUsersRoles => {
                crate::database::SampledIndividualsUsersRole::delete_from_id(primary_key.into(), connection).await
            },
            Table::Samples => {
                crate::database::Sample::delete_from_id(primary_key.into(), connection).await
            },
            Table::SamplesTeamsRoles => {
                crate::database::SamplesTeamsRole::delete_from_id(primary_key.into(), connection).await
            },
            Table::SamplesUsersRoles => {
                crate::database::SamplesUsersRole::delete_from_id(primary_key.into(), connection).await
            },
            Table::Spectra => {
                crate::database::Spectra::delete_from_id(primary_key.into(), connection).await
            },
            Table::SpectraCollections => {
                crate::database::SpectraCollection::delete_from_id(primary_key.into(), connection).await
            },
            Table::SpectraCollectionsTeamsRoles => {
                crate::database::SpectraCollectionsTeamsRole::delete_from_id(primary_key.into(), connection).await
            },
            Table::SpectraCollectionsUsersRoles => {
                crate::database::SpectraCollectionsUsersRole::delete_from_id(primary_key.into(), connection).await
            },
            Table::TeamStates => {
                crate::database::TeamState::delete_from_id(primary_key.into(), connection).await
            },
            Table::Teams => {
                crate::database::Team::delete_from_id(primary_key.into(), connection).await
            },
            Table::TeamsUsersRoles => {
                crate::database::TeamsUsersRole::delete_from_id(primary_key.into(), connection).await
            },
            Table::Units => {
                crate::database::Unit::delete_from_id(primary_key.into(), connection).await
            },
            Table::UserEmails => {
                crate::database::UserEmail::delete_from_id(primary_key.into(), connection).await
            },
            Table::Users => {
                crate::database::User::delete_from_id(primary_key.into(), connection).await
            },
        }
    }
    /// Get the row from the table by the primary key.
    ///
    /// # Arguments
    /// * `primary_key` - The primary key of the row.
    /// * `connection` - The database connection.
    ///
    /// # Returns
    /// The row of the table.
    pub async fn get<C>(
        &self,
        primary_key: crate::database::operations::PrimaryKey,
        connection: &mut gluesql::prelude::Glue<C>,
    ) -> Result<Option<Vec<u8>>, crate::api::ApiError> where
        C: gluesql::core::store::GStore + gluesql::core::store::GStoreMut,
    {
        Ok(match self {
            Table::BioOttRanks => crate::database::NestedBioOttRank::get(primary_key.into(), connection).await?.map(|row| bincode::serialize(&row)).transpose()?,
            Table::BioOttTaxonItems => crate::database::NestedBioOttTaxonItem::get(primary_key.into(), connection).await?.map(|row| bincode::serialize(&row)).transpose()?,
            Table::Colors => crate::database::Color::get(primary_key.into(), connection).await?.map(|row| bincode::serialize(&row)).transpose()?,
            Table::Countries => crate::database::Country::get(primary_key.into(), connection).await?.map(|row| bincode::serialize(&row)).transpose()?,
            Table::DerivedSamples => crate::database::NestedDerivedSample::get(primary_key.into(), connection).await?.map(|row| bincode::serialize(&row)).transpose()?,
            Table::DocumentFormats => crate::database::DocumentFormat::get(primary_key.into(), connection).await?.map(|row| bincode::serialize(&row)).transpose()?,
            Table::FontAwesomeIcons => crate::database::FontAwesomeIcon::get(primary_key.into(), connection).await?.map(|row| bincode::serialize(&row)).transpose()?,
            Table::LoginProviders => crate::database::NestedLoginProvider::get(primary_key.into(), connection).await?.map(|row| bincode::serialize(&row)).transpose()?,
            Table::Notifications => crate::database::NestedNotification::get(primary_key.into(), connection).await?.map(|row| bincode::serialize(&row)).transpose()?,
            Table::Organizations => crate::database::NestedOrganization::get(primary_key.into(), connection).await?.map(|row| bincode::serialize(&row)).transpose()?,
            Table::ProjectStates => crate::database::NestedProjectState::get(primary_key.into(), connection).await?.map(|row| bincode::serialize(&row)).transpose()?,
            Table::Projects => crate::database::NestedProject::get(primary_key.into(), connection).await?.map(|row| bincode::serialize(&row)).transpose()?,
            Table::ProjectsTeamsRoles => crate::database::NestedProjectsTeamsRole::get(primary_key.into(), connection).await?.map(|row| bincode::serialize(&row)).transpose()?,
            Table::ProjectsUsersRoles => crate::database::NestedProjectsUsersRole::get(primary_key.into(), connection).await?.map(|row| bincode::serialize(&row)).transpose()?,
            Table::Roles => crate::database::NestedRole::get(primary_key.into(), connection).await?.map(|row| bincode::serialize(&row)).transpose()?,
            Table::SampleBioOttTaxonItems => crate::database::NestedSampleBioOttTaxonItem::get(primary_key.into(), connection).await?.map(|row| bincode::serialize(&row)).transpose()?,
            Table::SampleStates => crate::database::NestedSampleState::get(primary_key.into(), connection).await?.map(|row| bincode::serialize(&row)).transpose()?,
            Table::SampledIndividualBioOttTaxonItems => crate::database::NestedSampledIndividualBioOttTaxonItem::get(primary_key.into(), connection).await?.map(|row| bincode::serialize(&row)).transpose()?,
            Table::SampledIndividuals => crate::database::NestedSampledIndividual::get(primary_key.into(), connection).await?.map(|row| bincode::serialize(&row)).transpose()?,
            Table::SampledIndividualsTeamsRoles => crate::database::NestedSampledIndividualsTeamsRole::get(primary_key.into(), connection).await?.map(|row| bincode::serialize(&row)).transpose()?,
            Table::SampledIndividualsUsersRoles => crate::database::NestedSampledIndividualsUsersRole::get(primary_key.into(), connection).await?.map(|row| bincode::serialize(&row)).transpose()?,
            Table::Samples => crate::database::NestedSample::get(primary_key.into(), connection).await?.map(|row| bincode::serialize(&row)).transpose()?,
            Table::SamplesTeamsRoles => crate::database::NestedSamplesTeamsRole::get(primary_key.into(), connection).await?.map(|row| bincode::serialize(&row)).transpose()?,
            Table::SamplesUsersRoles => crate::database::NestedSamplesUsersRole::get(primary_key.into(), connection).await?.map(|row| bincode::serialize(&row)).transpose()?,
            Table::Spectra => crate::database::NestedSpectra::get(primary_key.into(), connection).await?.map(|row| bincode::serialize(&row)).transpose()?,
            Table::SpectraCollections => crate::database::NestedSpectraCollection::get(primary_key.into(), connection).await?.map(|row| bincode::serialize(&row)).transpose()?,
            Table::SpectraCollectionsTeamsRoles => crate::database::NestedSpectraCollectionsTeamsRole::get(primary_key.into(), connection).await?.map(|row| bincode::serialize(&row)).transpose()?,
            Table::SpectraCollectionsUsersRoles => crate::database::NestedSpectraCollectionsUsersRole::get(primary_key.into(), connection).await?.map(|row| bincode::serialize(&row)).transpose()?,
            Table::TeamStates => crate::database::NestedTeamState::get(primary_key.into(), connection).await?.map(|row| bincode::serialize(&row)).transpose()?,
            Table::Teams => crate::database::NestedTeam::get(primary_key.into(), connection).await?.map(|row| bincode::serialize(&row)).transpose()?,
            Table::TeamsUsersRoles => crate::database::NestedTeamsUsersRole::get(primary_key.into(), connection).await?.map(|row| bincode::serialize(&row)).transpose()?,
            Table::Units => crate::database::Unit::get(primary_key.into(), connection).await?.map(|row| bincode::serialize(&row)).transpose()?,
            Table::UserEmails => crate::database::NestedUserEmail::get(primary_key.into(), connection).await?.map(|row| bincode::serialize(&row)).transpose()?,
            Table::Users => crate::database::User::get(primary_key.into(), connection).await?.map(|row| bincode::serialize(&row)).transpose()?,
        })
    }
    /// Get all the rows from the table.
    ///
    /// # Arguments
    /// * `limit` - The maximum number of rows to return.
    /// * `offset` - The number of rows to skip. By default `0`.
    /// * `connection` - The database connection.
    ///
    /// # Returns
    /// A vector of the rows of the table.
    pub async fn all<C>(
        &self,
        limit: Option<i64>,
        offset: Option<i64>,
        connection: &mut gluesql::prelude::Glue<C>,
    ) -> Result<Vec<Vec<u8>>, crate::api::ApiError> where
        C: gluesql::core::store::GStore + gluesql::core::store::GStoreMut,
    {
        match self {
            Table::BioOttRanks => crate::database::NestedBioOttRank::all(limit, offset, connection).await?.into_iter().map(|row| bincode::serialize(&row).map_err(crate::api::ApiError::from)).collect(),
            Table::BioOttTaxonItems => crate::database::NestedBioOttTaxonItem::all(limit, offset, connection).await?.into_iter().map(|row| bincode::serialize(&row).map_err(crate::api::ApiError::from)).collect(),
            Table::Colors => crate::database::Color::all(limit, offset, connection).await?.into_iter().map(|row| bincode::serialize(&row).map_err(crate::api::ApiError::from)).collect(),
            Table::Countries => crate::database::Country::all(limit, offset, connection).await?.into_iter().map(|row| bincode::serialize(&row).map_err(crate::api::ApiError::from)).collect(),
            Table::DerivedSamples => crate::database::NestedDerivedSample::all(limit, offset, connection).await?.into_iter().map(|row| bincode::serialize(&row).map_err(crate::api::ApiError::from)).collect(),
            Table::DocumentFormats => crate::database::DocumentFormat::all(limit, offset, connection).await?.into_iter().map(|row| bincode::serialize(&row).map_err(crate::api::ApiError::from)).collect(),
            Table::FontAwesomeIcons => crate::database::FontAwesomeIcon::all(limit, offset, connection).await?.into_iter().map(|row| bincode::serialize(&row).map_err(crate::api::ApiError::from)).collect(),
            Table::LoginProviders => crate::database::NestedLoginProvider::all(limit, offset, connection).await?.into_iter().map(|row| bincode::serialize(&row).map_err(crate::api::ApiError::from)).collect(),
            Table::Notifications => crate::database::NestedNotification::all(limit, offset, connection).await?.into_iter().map(|row| bincode::serialize(&row).map_err(crate::api::ApiError::from)).collect(),
            Table::Organizations => crate::database::NestedOrganization::all(limit, offset, connection).await?.into_iter().map(|row| bincode::serialize(&row).map_err(crate::api::ApiError::from)).collect(),
            Table::ProjectStates => crate::database::NestedProjectState::all(limit, offset, connection).await?.into_iter().map(|row| bincode::serialize(&row).map_err(crate::api::ApiError::from)).collect(),
            Table::Projects => crate::database::NestedProject::all(limit, offset, connection).await?.into_iter().map(|row| bincode::serialize(&row).map_err(crate::api::ApiError::from)).collect(),
            Table::ProjectsTeamsRoles => crate::database::NestedProjectsTeamsRole::all(limit, offset, connection).await?.into_iter().map(|row| bincode::serialize(&row).map_err(crate::api::ApiError::from)).collect(),
            Table::ProjectsUsersRoles => crate::database::NestedProjectsUsersRole::all(limit, offset, connection).await?.into_iter().map(|row| bincode::serialize(&row).map_err(crate::api::ApiError::from)).collect(),
            Table::Roles => crate::database::NestedRole::all(limit, offset, connection).await?.into_iter().map(|row| bincode::serialize(&row).map_err(crate::api::ApiError::from)).collect(),
            Table::SampleBioOttTaxonItems => crate::database::NestedSampleBioOttTaxonItem::all(limit, offset, connection).await?.into_iter().map(|row| bincode::serialize(&row).map_err(crate::api::ApiError::from)).collect(),
            Table::SampleStates => crate::database::NestedSampleState::all(limit, offset, connection).await?.into_iter().map(|row| bincode::serialize(&row).map_err(crate::api::ApiError::from)).collect(),
            Table::SampledIndividualBioOttTaxonItems => crate::database::NestedSampledIndividualBioOttTaxonItem::all(limit, offset, connection).await?.into_iter().map(|row| bincode::serialize(&row).map_err(crate::api::ApiError::from)).collect(),
            Table::SampledIndividuals => crate::database::NestedSampledIndividual::all(limit, offset, connection).await?.into_iter().map(|row| bincode::serialize(&row).map_err(crate::api::ApiError::from)).collect(),
            Table::SampledIndividualsTeamsRoles => crate::database::NestedSampledIndividualsTeamsRole::all(limit, offset, connection).await?.into_iter().map(|row| bincode::serialize(&row).map_err(crate::api::ApiError::from)).collect(),
            Table::SampledIndividualsUsersRoles => crate::database::NestedSampledIndividualsUsersRole::all(limit, offset, connection).await?.into_iter().map(|row| bincode::serialize(&row).map_err(crate::api::ApiError::from)).collect(),
            Table::Samples => crate::database::NestedSample::all(limit, offset, connection).await?.into_iter().map(|row| bincode::serialize(&row).map_err(crate::api::ApiError::from)).collect(),
            Table::SamplesTeamsRoles => crate::database::NestedSamplesTeamsRole::all(limit, offset, connection).await?.into_iter().map(|row| bincode::serialize(&row).map_err(crate::api::ApiError::from)).collect(),
            Table::SamplesUsersRoles => crate::database::NestedSamplesUsersRole::all(limit, offset, connection).await?.into_iter().map(|row| bincode::serialize(&row).map_err(crate::api::ApiError::from)).collect(),
            Table::Spectra => crate::database::NestedSpectra::all(limit, offset, connection).await?.into_iter().map(|row| bincode::serialize(&row).map_err(crate::api::ApiError::from)).collect(),
            Table::SpectraCollections => crate::database::NestedSpectraCollection::all(limit, offset, connection).await?.into_iter().map(|row| bincode::serialize(&row).map_err(crate::api::ApiError::from)).collect(),
            Table::SpectraCollectionsTeamsRoles => crate::database::NestedSpectraCollectionsTeamsRole::all(limit, offset, connection).await?.into_iter().map(|row| bincode::serialize(&row).map_err(crate::api::ApiError::from)).collect(),
            Table::SpectraCollectionsUsersRoles => crate::database::NestedSpectraCollectionsUsersRole::all(limit, offset, connection).await?.into_iter().map(|row| bincode::serialize(&row).map_err(crate::api::ApiError::from)).collect(),
            Table::TeamStates => crate::database::NestedTeamState::all(limit, offset, connection).await?.into_iter().map(|row| bincode::serialize(&row).map_err(crate::api::ApiError::from)).collect(),
            Table::Teams => crate::database::NestedTeam::all(limit, offset, connection).await?.into_iter().map(|row| bincode::serialize(&row).map_err(crate::api::ApiError::from)).collect(),
            Table::TeamsUsersRoles => crate::database::NestedTeamsUsersRole::all(limit, offset, connection).await?.into_iter().map(|row| bincode::serialize(&row).map_err(crate::api::ApiError::from)).collect(),
            Table::Units => crate::database::Unit::all(limit, offset, connection).await?.into_iter().map(|row| bincode::serialize(&row).map_err(crate::api::ApiError::from)).collect(),
            Table::UserEmails => crate::database::NestedUserEmail::all(limit, offset, connection).await?.into_iter().map(|row| bincode::serialize(&row).map_err(crate::api::ApiError::from)).collect(),
            Table::Users => crate::database::User::all(limit, offset, connection).await?.into_iter().map(|row| bincode::serialize(&row).map_err(crate::api::ApiError::from)).collect(),
        }
    }
    /// Get all the rows from the table ordered by the `updated_at` column.
    ///
    /// # Arguments
    /// * `limit` - The maximum number of rows to return.
    /// * `offset` - The number of rows to skip. By default `0`.
    /// * `connection` - The database connection.
    ///
    /// # Returns
    /// A vector of the rows of the table.
    pub async fn all_by_updated_at<C>(
        &self,
        limit: Option<i64>,
        offset: Option<i64>,
        connection: &mut gluesql::prelude::Glue<C>,
    ) -> Result<Vec<Vec<u8>>, crate::api::ApiError> where
        C: gluesql::core::store::GStore + gluesql::core::store::GStoreMut,
    {
        match self {
            Table::BioOttRanks => unimplemented!("all_by_updated_at not implemented for bio_ott_ranks."),
            Table::BioOttTaxonItems => unimplemented!("all_by_updated_at not implemented for bio_ott_taxon_items."),
            Table::Colors => unimplemented!("all_by_updated_at not implemented for colors."),
            Table::Countries => unimplemented!("all_by_updated_at not implemented for countries."),
            Table::DerivedSamples => crate::database::NestedDerivedSample::all_by_updated_at(limit, offset, connection).await?.into_iter().map(|row| bincode::serialize(&row).map_err(crate::api::ApiError::from)).collect(),
            Table::DocumentFormats => unimplemented!("all_by_updated_at not implemented for document_formats."),
            Table::FontAwesomeIcons => unimplemented!("all_by_updated_at not implemented for font_awesome_icons."),
            Table::LoginProviders => unimplemented!("all_by_updated_at not implemented for login_providers."),
            Table::Notifications => unimplemented!("all_by_updated_at not implemented for notifications."),
            Table::Organizations => unimplemented!("all_by_updated_at not implemented for organizations."),
            Table::ProjectStates => unimplemented!("all_by_updated_at not implemented for project_states."),
            Table::Projects => crate::database::NestedProject::all_by_updated_at(limit, offset, connection).await?.into_iter().map(|row| bincode::serialize(&row).map_err(crate::api::ApiError::from)).collect(),
            Table::ProjectsTeamsRoles => crate::database::NestedProjectsTeamsRole::all_by_updated_at(limit, offset, connection).await?.into_iter().map(|row| bincode::serialize(&row).map_err(crate::api::ApiError::from)).collect(),
            Table::ProjectsUsersRoles => crate::database::NestedProjectsUsersRole::all_by_updated_at(limit, offset, connection).await?.into_iter().map(|row| bincode::serialize(&row).map_err(crate::api::ApiError::from)).collect(),
            Table::Roles => unimplemented!("all_by_updated_at not implemented for roles."),
            Table::SampleBioOttTaxonItems => crate::database::NestedSampleBioOttTaxonItem::all_by_updated_at(limit, offset, connection).await?.into_iter().map(|row| bincode::serialize(&row).map_err(crate::api::ApiError::from)).collect(),
            Table::SampleStates => unimplemented!("all_by_updated_at not implemented for sample_states."),
            Table::SampledIndividualBioOttTaxonItems => crate::database::NestedSampledIndividualBioOttTaxonItem::all_by_updated_at(limit, offset, connection).await?.into_iter().map(|row| bincode::serialize(&row).map_err(crate::api::ApiError::from)).collect(),
            Table::SampledIndividuals => crate::database::NestedSampledIndividual::all_by_updated_at(limit, offset, connection).await?.into_iter().map(|row| bincode::serialize(&row).map_err(crate::api::ApiError::from)).collect(),
            Table::SampledIndividualsTeamsRoles => crate::database::NestedSampledIndividualsTeamsRole::all_by_updated_at(limit, offset, connection).await?.into_iter().map(|row| bincode::serialize(&row).map_err(crate::api::ApiError::from)).collect(),
            Table::SampledIndividualsUsersRoles => crate::database::NestedSampledIndividualsUsersRole::all_by_updated_at(limit, offset, connection).await?.into_iter().map(|row| bincode::serialize(&row).map_err(crate::api::ApiError::from)).collect(),
            Table::Samples => crate::database::NestedSample::all_by_updated_at(limit, offset, connection).await?.into_iter().map(|row| bincode::serialize(&row).map_err(crate::api::ApiError::from)).collect(),
            Table::SamplesTeamsRoles => crate::database::NestedSamplesTeamsRole::all_by_updated_at(limit, offset, connection).await?.into_iter().map(|row| bincode::serialize(&row).map_err(crate::api::ApiError::from)).collect(),
            Table::SamplesUsersRoles => crate::database::NestedSamplesUsersRole::all_by_updated_at(limit, offset, connection).await?.into_iter().map(|row| bincode::serialize(&row).map_err(crate::api::ApiError::from)).collect(),
            Table::Spectra => unimplemented!("all_by_updated_at not implemented for spectra."),
            Table::SpectraCollections => crate::database::NestedSpectraCollection::all_by_updated_at(limit, offset, connection).await?.into_iter().map(|row| bincode::serialize(&row).map_err(crate::api::ApiError::from)).collect(),
            Table::SpectraCollectionsTeamsRoles => crate::database::NestedSpectraCollectionsTeamsRole::all_by_updated_at(limit, offset, connection).await?.into_iter().map(|row| bincode::serialize(&row).map_err(crate::api::ApiError::from)).collect(),
            Table::SpectraCollectionsUsersRoles => crate::database::NestedSpectraCollectionsUsersRole::all_by_updated_at(limit, offset, connection).await?.into_iter().map(|row| bincode::serialize(&row).map_err(crate::api::ApiError::from)).collect(),
            Table::TeamStates => unimplemented!("all_by_updated_at not implemented for team_states."),
            Table::Teams => crate::database::NestedTeam::all_by_updated_at(limit, offset, connection).await?.into_iter().map(|row| bincode::serialize(&row).map_err(crate::api::ApiError::from)).collect(),
            Table::TeamsUsersRoles => crate::database::NestedTeamsUsersRole::all_by_updated_at(limit, offset, connection).await?.into_iter().map(|row| bincode::serialize(&row).map_err(crate::api::ApiError::from)).collect(),
            Table::Units => unimplemented!("all_by_updated_at not implemented for units."),
            Table::UserEmails => unimplemented!("all_by_updated_at not implemented for user_emails."),
            Table::Users => crate::database::User::all_by_updated_at(limit, offset, connection).await?.into_iter().map(|row| bincode::serialize(&row).map_err(crate::api::ApiError::from)).collect(),
        }
    }
    /// Insert a new row into the table.
    ///
    /// # Arguments
    /// * `new_row` - The bincode-serialized row of the table.
    /// * `user_id` - The user ID of the user performing the operation.
    /// * `connection` - The database connection.
    ///
    /// # Returns
    /// The bincode-serialized row of the table.
    pub async fn insert<C>(
        &self,
        new_row: Vec<u8>,
        user_id: i32,
        connection: &mut gluesql::prelude::Glue<C>,
    ) -> Result<Vec<u8>, crate::api::ApiError> where
        C: gluesql::core::store::GStore + gluesql::core::store::GStoreMut,
    {
        Ok(match self {
            Table::BioOttRanks => unimplemented!("Insert not implemented for bio_ott_ranks."),
            Table::BioOttTaxonItems => unimplemented!("Insert not implemented for bio_ott_taxon_items."),
            Table::Colors => unimplemented!("Insert not implemented for colors."),
            Table::Countries => unimplemented!("Insert not implemented for countries."),
            Table::DerivedSamples => todo!("Insert not implemented for derived_samples."),
            Table::DocumentFormats => unimplemented!("Insert not implemented for document_formats."),
            Table::FontAwesomeIcons => unimplemented!("Insert not implemented for font_awesome_icons."),
            Table::LoginProviders => unimplemented!("Insert not implemented for login_providers."),
            Table::Notifications => unimplemented!("Insert not implemented for notifications."),
            Table::Organizations => unimplemented!("Insert not implemented for organizations."),
            Table::ProjectStates => unimplemented!("Insert not implemented for project_states."),
            Table::Projects => unimplemented!("Insert not implemented for projects in frontend as it does not have a UUID primary key."),
            Table::ProjectsTeamsRoles => unimplemented!("Insert not implemented for projects_teams_roles in frontend as it does not have a UUID primary key."),
            Table::ProjectsUsersRoles => unimplemented!("Insert not implemented for projects_users_roles in frontend as it does not have a UUID primary key."),
            Table::Roles => unimplemented!("Insert not implemented for roles."),
            Table::SampleBioOttTaxonItems => unimplemented!("Insert not implemented for sample_bio_ott_taxon_items in frontend as it does not have a UUID primary key."),
            Table::SampleStates => unimplemented!("Insert not implemented for sample_states."),
            Table::SampledIndividualBioOttTaxonItems => unimplemented!("Insert not implemented for sampled_individual_bio_ott_taxon_items in frontend as it does not have a UUID primary key."),
            Table::SampledIndividuals => {
                let new_row: super::NewSampledIndividual = bincode::deserialize::<super::NewSampledIndividual>(&new_row).map_err(crate::api::ApiError::from)?;
                let inserted_row: super::SampledIndividual = new_row.insert(user_id, connection).await?;
                let nested_row = super::NestedSampledIndividual::from_flat(inserted_row, connection).await?;
                 bincode::serialize(&nested_row).map_err(crate::api::ApiError::from)?
            },
            Table::SampledIndividualsTeamsRoles => unimplemented!("Insert not implemented for sampled_individuals_teams_roles in frontend as it does not have a UUID primary key."),
            Table::SampledIndividualsUsersRoles => unimplemented!("Insert not implemented for sampled_individuals_users_roles in frontend as it does not have a UUID primary key."),
            Table::Samples => {
                let new_row: super::NewSample = bincode::deserialize::<super::NewSample>(&new_row).map_err(crate::api::ApiError::from)?;
                let inserted_row: super::Sample = new_row.insert(user_id, connection).await?;
                let nested_row = super::NestedSample::from_flat(inserted_row, connection).await?;
                 bincode::serialize(&nested_row).map_err(crate::api::ApiError::from)?
            },
            Table::SamplesTeamsRoles => unimplemented!("Insert not implemented for samples_teams_roles in frontend as it does not have a UUID primary key."),
            Table::SamplesUsersRoles => unimplemented!("Insert not implemented for samples_users_roles in frontend as it does not have a UUID primary key."),
            Table::Spectra => unimplemented!("Insert not implemented for spectra."),
            Table::SpectraCollections => unimplemented!("Insert not implemented for spectra_collections in frontend as it does not have a UUID primary key."),
            Table::SpectraCollectionsTeamsRoles => unimplemented!("Insert not implemented for spectra_collections_teams_roles in frontend as it does not have a UUID primary key."),
            Table::SpectraCollectionsUsersRoles => unimplemented!("Insert not implemented for spectra_collections_users_roles in frontend as it does not have a UUID primary key."),
            Table::TeamStates => unimplemented!("Insert not implemented for team_states."),
            Table::Teams => unimplemented!("Insert not implemented for teams in frontend as it does not have a UUID primary key."),
            Table::TeamsUsersRoles => unimplemented!("Insert not implemented for teams_users_roles in frontend as it does not have a UUID primary key."),
            Table::Units => unimplemented!("Insert not implemented for units."),
            Table::UserEmails => unimplemented!("Insert not implemented for user_emails in frontend as it does not have a UUID primary key."),
            Table::Users => unimplemented!("Insert not implemented for users in frontend as it does not have a UUID primary key."),
})
    }
    /// Update a row in the table.
    ///
    /// # Arguments
    /// * `update_row` - The bincode-serialized row of the table.
    /// * `user_id` - The user ID of the user performing the operation.
    /// * `connection` - The database connection.
    ///
    /// # Returns
    /// The bincode-serialized row of the table.
    pub async fn update<C>(
        &self,
        update_row: Vec<u8>,
        user_id: i32,
        connection: &mut gluesql::prelude::Glue<C>,
    ) -> Result<Vec<u8>, crate::api::ApiError> where
        C: gluesql::core::store::GStore + gluesql::core::store::GStoreMut,
    {
        Ok(match self {
            Table::BioOttRanks => unimplemented!("Update not implemented for bio_ott_ranks."),
            Table::BioOttTaxonItems => unimplemented!("Update not implemented for bio_ott_taxon_items."),
            Table::Colors => unimplemented!("Update not implemented for colors."),
            Table::Countries => unimplemented!("Update not implemented for countries."),
            Table::DerivedSamples => unimplemented!("Update not implemented for derived_samples."),
            Table::DocumentFormats => unimplemented!("Update not implemented for document_formats."),
            Table::FontAwesomeIcons => unimplemented!("Update not implemented for font_awesome_icons."),
            Table::LoginProviders => unimplemented!("Update not implemented for login_providers."),
            Table::Notifications => unimplemented!("Update not implemented for notifications."),
            Table::Organizations => unimplemented!("Update not implemented for organizations."),
            Table::ProjectStates => unimplemented!("Update not implemented for project_states."),
            Table::Projects => {
                let update_row: super::UpdateProject = bincode::deserialize::<super::UpdateProject>(&update_row).map_err(crate::api::ApiError::from)?;
                let id = update_row.id;
                update_row.update(user_id, connection).await?;
                let updated_row: super::Project = super::Project::get(id, connection).await?.unwrap();
                let nested_row = super::NestedProject::from_flat(updated_row, connection).await?;
                 bincode::serialize(&nested_row).map_err(crate::api::ApiError::from)?
            },
            Table::ProjectsTeamsRoles => unimplemented!("Update not implemented for projects_teams_roles."),
            Table::ProjectsUsersRoles => unimplemented!("Update not implemented for projects_users_roles."),
            Table::Roles => unimplemented!("Update not implemented for roles."),
            Table::SampleBioOttTaxonItems => unimplemented!("Update not implemented for sample_bio_ott_taxon_items."),
            Table::SampleStates => unimplemented!("Update not implemented for sample_states."),
            Table::SampledIndividualBioOttTaxonItems => unimplemented!("Update not implemented for sampled_individual_bio_ott_taxon_items."),
            Table::SampledIndividuals => {
                let update_row: super::NewSampledIndividual = bincode::deserialize::<super::NewSampledIndividual>(&update_row).map_err(crate::api::ApiError::from)?;
                let id = update_row.id;
                update_row.update(user_id, connection).await?;
                let updated_row: super::SampledIndividual = super::SampledIndividual::get(id, connection).await?.unwrap();
                let nested_row = super::NestedSampledIndividual::from_flat(updated_row, connection).await?;
                 bincode::serialize(&nested_row).map_err(crate::api::ApiError::from)?
            },
            Table::SampledIndividualsTeamsRoles => unimplemented!("Update not implemented for sampled_individuals_teams_roles."),
            Table::SampledIndividualsUsersRoles => unimplemented!("Update not implemented for sampled_individuals_users_roles."),
            Table::Samples => {
                let update_row: super::NewSample = bincode::deserialize::<super::NewSample>(&update_row).map_err(crate::api::ApiError::from)?;
                let id = update_row.id;
                update_row.update(user_id, connection).await?;
                let updated_row: super::Sample = super::Sample::get(id, connection).await?.unwrap();
                let nested_row = super::NestedSample::from_flat(updated_row, connection).await?;
                 bincode::serialize(&nested_row).map_err(crate::api::ApiError::from)?
            },
            Table::SamplesTeamsRoles => unimplemented!("Update not implemented for samples_teams_roles."),
            Table::SamplesUsersRoles => unimplemented!("Update not implemented for samples_users_roles."),
            Table::Spectra => unimplemented!("Update not implemented for spectra."),
            Table::SpectraCollections => todo!("Update not implemented for spectra_collections."),
            Table::SpectraCollectionsTeamsRoles => unimplemented!("Update not implemented for spectra_collections_teams_roles."),
            Table::SpectraCollectionsUsersRoles => unimplemented!("Update not implemented for spectra_collections_users_roles."),
            Table::TeamStates => unimplemented!("Update not implemented for team_states."),
            Table::Teams => {
                let update_row: super::UpdateTeam = bincode::deserialize::<super::UpdateTeam>(&update_row).map_err(crate::api::ApiError::from)?;
                let id = update_row.id;
                update_row.update(user_id, connection).await?;
                let updated_row: super::Team = super::Team::get(id, connection).await?.unwrap();
                let nested_row = super::NestedTeam::from_flat(updated_row, connection).await?;
                 bincode::serialize(&nested_row).map_err(crate::api::ApiError::from)?
            },
            Table::TeamsUsersRoles => unimplemented!("Update not implemented for teams_users_roles."),
            Table::Units => unimplemented!("Update not implemented for units."),
            Table::UserEmails => unimplemented!("Update not implemented for user_emails."),
            Table::Users => {
                let update_row: super::UpdateUser = bincode::deserialize::<super::UpdateUser>(&update_row).map_err(crate::api::ApiError::from)?;
                let id = update_row.id;
                update_row.update(connection).await?;
                let updated_row: super::User = super::User::get(id, connection).await?.unwrap();
                bincode::serialize(&updated_row).map_err(crate::api::ApiError::from)?
            },
})
    }
    /// Update or insert a row into the table.
    ///
    /// # Arguments
    /// * `rows` - The bincode-serialized rows of the table.
    /// * `connection` - The database connection.
    ///
    /// # Returns
    /// An empty tuple.
    pub async fn update_or_insert<C>(
        &self,
        rows: Vec<Vec<u8>>,        connection: &mut gluesql::prelude::Glue<C>,
    ) -> Result<(), crate::api::ApiError> where
        C: gluesql::core::store::GStore + gluesql::core::store::GStoreMut,
    {
        match self {
            Table::BioOttRanks => {
                for row in rows {
                    let row: super::NestedBioOttRank = bincode::deserialize::<super::NestedBioOttRank>(&row).map_err(crate::api::ApiError::from)?;
                    row.update_or_insert(connection).await?;
                }
            },
            Table::BioOttTaxonItems => {
                for row in rows {
                    let row: super::NestedBioOttTaxonItem = bincode::deserialize::<super::NestedBioOttTaxonItem>(&row).map_err(crate::api::ApiError::from)?;
                    row.update_or_insert(connection).await?;
                }
            },
            Table::Colors => {
                for row in rows {
                    let row: super::Color = bincode::deserialize::<super::Color>(&row).map_err(crate::api::ApiError::from)?;
                    row.update_or_insert(connection).await?;
                }
            },
            Table::Countries => {
                for row in rows {
                    let row: super::Country = bincode::deserialize::<super::Country>(&row).map_err(crate::api::ApiError::from)?;
                    row.update_or_insert(connection).await?;
                }
            },
            Table::DerivedSamples => {
                for row in rows {
                    let row: super::NestedDerivedSample = bincode::deserialize::<super::NestedDerivedSample>(&row).map_err(crate::api::ApiError::from)?;
                    row.update_or_insert(connection).await?;
                }
            },
            Table::DocumentFormats => {
                for row in rows {
                    let row: super::DocumentFormat = bincode::deserialize::<super::DocumentFormat>(&row).map_err(crate::api::ApiError::from)?;
                    row.update_or_insert(connection).await?;
                }
            },
            Table::FontAwesomeIcons => {
                for row in rows {
                    let row: super::FontAwesomeIcon = bincode::deserialize::<super::FontAwesomeIcon>(&row).map_err(crate::api::ApiError::from)?;
                    row.update_or_insert(connection).await?;
                }
            },
            Table::LoginProviders => {
                for row in rows {
                    let row: super::NestedLoginProvider = bincode::deserialize::<super::NestedLoginProvider>(&row).map_err(crate::api::ApiError::from)?;
                    row.update_or_insert(connection).await?;
                }
            },
            Table::Notifications => {
                for row in rows {
                    let row: super::NestedNotification = bincode::deserialize::<super::NestedNotification>(&row).map_err(crate::api::ApiError::from)?;
                    row.update_or_insert(connection).await?;
                }
            },
            Table::Organizations => {
                for row in rows {
                    let row: super::NestedOrganization = bincode::deserialize::<super::NestedOrganization>(&row).map_err(crate::api::ApiError::from)?;
                    row.update_or_insert(connection).await?;
                }
            },
            Table::ProjectStates => {
                for row in rows {
                    let row: super::NestedProjectState = bincode::deserialize::<super::NestedProjectState>(&row).map_err(crate::api::ApiError::from)?;
                    row.update_or_insert(connection).await?;
                }
            },
            Table::Projects => {
                for row in rows {
                    let row: super::NestedProject = bincode::deserialize::<super::NestedProject>(&row).map_err(crate::api::ApiError::from)?;
                    row.update_or_insert(connection).await?;
                }
            },
            Table::ProjectsTeamsRoles => {
                for row in rows {
                    let row: super::NestedProjectsTeamsRole = bincode::deserialize::<super::NestedProjectsTeamsRole>(&row).map_err(crate::api::ApiError::from)?;
                    row.update_or_insert(connection).await?;
                }
            },
            Table::ProjectsUsersRoles => {
                for row in rows {
                    let row: super::NestedProjectsUsersRole = bincode::deserialize::<super::NestedProjectsUsersRole>(&row).map_err(crate::api::ApiError::from)?;
                    row.update_or_insert(connection).await?;
                }
            },
            Table::Roles => {
                for row in rows {
                    let row: super::NestedRole = bincode::deserialize::<super::NestedRole>(&row).map_err(crate::api::ApiError::from)?;
                    row.update_or_insert(connection).await?;
                }
            },
            Table::SampleBioOttTaxonItems => {
                for row in rows {
                    let row: super::NestedSampleBioOttTaxonItem = bincode::deserialize::<super::NestedSampleBioOttTaxonItem>(&row).map_err(crate::api::ApiError::from)?;
                    row.update_or_insert(connection).await?;
                }
            },
            Table::SampleStates => {
                for row in rows {
                    let row: super::NestedSampleState = bincode::deserialize::<super::NestedSampleState>(&row).map_err(crate::api::ApiError::from)?;
                    row.update_or_insert(connection).await?;
                }
            },
            Table::SampledIndividualBioOttTaxonItems => {
                for row in rows {
                    let row: super::NestedSampledIndividualBioOttTaxonItem = bincode::deserialize::<super::NestedSampledIndividualBioOttTaxonItem>(&row).map_err(crate::api::ApiError::from)?;
                    row.update_or_insert(connection).await?;
                }
            },
            Table::SampledIndividuals => {
                for row in rows {
                    let row: super::NestedSampledIndividual = bincode::deserialize::<super::NestedSampledIndividual>(&row).map_err(crate::api::ApiError::from)?;
                    row.update_or_insert(connection).await?;
                }
            },
            Table::SampledIndividualsTeamsRoles => {
                for row in rows {
                    let row: super::NestedSampledIndividualsTeamsRole = bincode::deserialize::<super::NestedSampledIndividualsTeamsRole>(&row).map_err(crate::api::ApiError::from)?;
                    row.update_or_insert(connection).await?;
                }
            },
            Table::SampledIndividualsUsersRoles => {
                for row in rows {
                    let row: super::NestedSampledIndividualsUsersRole = bincode::deserialize::<super::NestedSampledIndividualsUsersRole>(&row).map_err(crate::api::ApiError::from)?;
                    row.update_or_insert(connection).await?;
                }
            },
            Table::Samples => {
                for row in rows {
                    let row: super::NestedSample = bincode::deserialize::<super::NestedSample>(&row).map_err(crate::api::ApiError::from)?;
                    row.update_or_insert(connection).await?;
                }
            },
            Table::SamplesTeamsRoles => {
                for row in rows {
                    let row: super::NestedSamplesTeamsRole = bincode::deserialize::<super::NestedSamplesTeamsRole>(&row).map_err(crate::api::ApiError::from)?;
                    row.update_or_insert(connection).await?;
                }
            },
            Table::SamplesUsersRoles => {
                for row in rows {
                    let row: super::NestedSamplesUsersRole = bincode::deserialize::<super::NestedSamplesUsersRole>(&row).map_err(crate::api::ApiError::from)?;
                    row.update_or_insert(connection).await?;
                }
            },
            Table::Spectra => {
                for row in rows {
                    let row: super::NestedSpectra = bincode::deserialize::<super::NestedSpectra>(&row).map_err(crate::api::ApiError::from)?;
                    row.update_or_insert(connection).await?;
                }
            },
            Table::SpectraCollections => {
                for row in rows {
                    let row: super::NestedSpectraCollection = bincode::deserialize::<super::NestedSpectraCollection>(&row).map_err(crate::api::ApiError::from)?;
                    row.update_or_insert(connection).await?;
                }
            },
            Table::SpectraCollectionsTeamsRoles => {
                for row in rows {
                    let row: super::NestedSpectraCollectionsTeamsRole = bincode::deserialize::<super::NestedSpectraCollectionsTeamsRole>(&row).map_err(crate::api::ApiError::from)?;
                    row.update_or_insert(connection).await?;
                }
            },
            Table::SpectraCollectionsUsersRoles => {
                for row in rows {
                    let row: super::NestedSpectraCollectionsUsersRole = bincode::deserialize::<super::NestedSpectraCollectionsUsersRole>(&row).map_err(crate::api::ApiError::from)?;
                    row.update_or_insert(connection).await?;
                }
            },
            Table::TeamStates => {
                for row in rows {
                    let row: super::NestedTeamState = bincode::deserialize::<super::NestedTeamState>(&row).map_err(crate::api::ApiError::from)?;
                    row.update_or_insert(connection).await?;
                }
            },
            Table::Teams => {
                for row in rows {
                    let row: super::NestedTeam = bincode::deserialize::<super::NestedTeam>(&row).map_err(crate::api::ApiError::from)?;
                    row.update_or_insert(connection).await?;
                }
            },
            Table::TeamsUsersRoles => {
                for row in rows {
                    let row: super::NestedTeamsUsersRole = bincode::deserialize::<super::NestedTeamsUsersRole>(&row).map_err(crate::api::ApiError::from)?;
                    row.update_or_insert(connection).await?;
                }
            },
            Table::Units => {
                for row in rows {
                    let row: super::Unit = bincode::deserialize::<super::Unit>(&row).map_err(crate::api::ApiError::from)?;
                    row.update_or_insert(connection).await?;
                }
            },
            Table::UserEmails => {
                for row in rows {
                    let row: super::NestedUserEmail = bincode::deserialize::<super::NestedUserEmail>(&row).map_err(crate::api::ApiError::from)?;
                    row.update_or_insert(connection).await?;
                }
            },
            Table::Users => {
                for row in rows {
                    let row: super::User = bincode::deserialize::<super::User>(&row).map_err(crate::api::ApiError::from)?;
                    row.update_or_insert(connection).await?;
                }
            },
        }
    Ok(())}
}
