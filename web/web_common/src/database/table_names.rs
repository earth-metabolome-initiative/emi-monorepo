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
    Materials,
    NameplateCategories,
    Nameplates,
    Notifications,
    ObservationSubjects,
    Observations,
    OrganismBioOttTaxonItems,
    Organisms,
    Organizations,
    PermanenceCategories,
    ProjectStates,
    Projects,
    ProjectsTeamsRoleInvitations,
    ProjectsTeamsRoleRequests,
    ProjectsTeamsRoles,
    ProjectsUsersRoleInvitations,
    ProjectsUsersRoleRequests,
    ProjectsUsersRoles,
    Roles,
    SampleBioOttTaxonItems,
    SampleContainerCategories,
    SampleContainers,
    SampleStates,
    Samples,
    Spectra,
    SpectraCollections,
    TeamStates,
    Teams,
    TeamsTeamsRoleInvitations,
    TeamsUsersRoleInvitations,
    TeamsUsersRoleRequests,
    TeamsUsersRoles,
    Units,
    UserEmails,
    Users,
    UsersUsersRoleInvitations,
    UsersUsersRoleRequests,
    UsersUsersRoles,
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
            Table::Materials => "materials",
            Table::NameplateCategories => "nameplate_categories",
            Table::Nameplates => "nameplates",
            Table::Notifications => "notifications",
            Table::ObservationSubjects => "observation_subjects",
            Table::Observations => "observations",
            Table::OrganismBioOttTaxonItems => "organism_bio_ott_taxon_items",
            Table::Organisms => "organisms",
            Table::Organizations => "organizations",
            Table::PermanenceCategories => "permanence_categories",
            Table::ProjectStates => "project_states",
            Table::Projects => "projects",
            Table::ProjectsTeamsRoleInvitations => "projects_teams_role_invitations",
            Table::ProjectsTeamsRoleRequests => "projects_teams_role_requests",
            Table::ProjectsTeamsRoles => "projects_teams_roles",
            Table::ProjectsUsersRoleInvitations => "projects_users_role_invitations",
            Table::ProjectsUsersRoleRequests => "projects_users_role_requests",
            Table::ProjectsUsersRoles => "projects_users_roles",
            Table::Roles => "roles",
            Table::SampleBioOttTaxonItems => "sample_bio_ott_taxon_items",
            Table::SampleContainerCategories => "sample_container_categories",
            Table::SampleContainers => "sample_containers",
            Table::SampleStates => "sample_states",
            Table::Samples => "samples",
            Table::Spectra => "spectra",
            Table::SpectraCollections => "spectra_collections",
            Table::TeamStates => "team_states",
            Table::Teams => "teams",
            Table::TeamsTeamsRoleInvitations => "teams_teams_role_invitations",
            Table::TeamsUsersRoleInvitations => "teams_users_role_invitations",
            Table::TeamsUsersRoleRequests => "teams_users_role_requests",
            Table::TeamsUsersRoles => "teams_users_roles",
            Table::Units => "units",
            Table::UserEmails => "user_emails",
            Table::Users => "users",
            Table::UsersUsersRoleInvitations => "users_users_role_invitations",
            Table::UsersUsersRoleRequests => "users_users_role_requests",
            Table::UsersUsersRoles => "users_users_roles",
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
            "materials" => Ok(Table::Materials),
            "nameplate_categories" => Ok(Table::NameplateCategories),
            "nameplates" => Ok(Table::Nameplates),
            "notifications" => Ok(Table::Notifications),
            "observation_subjects" => Ok(Table::ObservationSubjects),
            "observations" => Ok(Table::Observations),
            "organism_bio_ott_taxon_items" => Ok(Table::OrganismBioOttTaxonItems),
            "organisms" => Ok(Table::Organisms),
            "organizations" => Ok(Table::Organizations),
            "permanence_categories" => Ok(Table::PermanenceCategories),
            "project_states" => Ok(Table::ProjectStates),
            "projects" => Ok(Table::Projects),
            "projects_teams_role_invitations" => Ok(Table::ProjectsTeamsRoleInvitations),
            "projects_teams_role_requests" => Ok(Table::ProjectsTeamsRoleRequests),
            "projects_teams_roles" => Ok(Table::ProjectsTeamsRoles),
            "projects_users_role_invitations" => Ok(Table::ProjectsUsersRoleInvitations),
            "projects_users_role_requests" => Ok(Table::ProjectsUsersRoleRequests),
            "projects_users_roles" => Ok(Table::ProjectsUsersRoles),
            "roles" => Ok(Table::Roles),
            "sample_bio_ott_taxon_items" => Ok(Table::SampleBioOttTaxonItems),
            "sample_container_categories" => Ok(Table::SampleContainerCategories),
            "sample_containers" => Ok(Table::SampleContainers),
            "sample_states" => Ok(Table::SampleStates),
            "samples" => Ok(Table::Samples),
            "spectra" => Ok(Table::Spectra),
            "spectra_collections" => Ok(Table::SpectraCollections),
            "team_states" => Ok(Table::TeamStates),
            "teams" => Ok(Table::Teams),
            "teams_teams_role_invitations" => Ok(Table::TeamsTeamsRoleInvitations),
            "teams_users_role_invitations" => Ok(Table::TeamsUsersRoleInvitations),
            "teams_users_role_requests" => Ok(Table::TeamsUsersRoleRequests),
            "teams_users_roles" => Ok(Table::TeamsUsersRoles),
            "units" => Ok(Table::Units),
            "user_emails" => Ok(Table::UserEmails),
            "users" => Ok(Table::Users),
            "users_users_role_invitations" => Ok(Table::UsersUsersRoleInvitations),
            "users_users_role_requests" => Ok(Table::UsersUsersRoleRequests),
            "users_users_roles" => Ok(Table::UsersUsersRoles),
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
    ) -> Result<usize, crate::api::ApiError>
    where
        C: gluesql::core::store::GStore + gluesql::core::store::GStoreMut,
    {
        match self {
            Table::BioOttRanks => {
                crate::database::BioOttRank::delete_from_id(primary_key.into(), connection).await
            }
            Table::BioOttTaxonItems => {
                crate::database::BioOttTaxonItem::delete_from_id(primary_key.into(), connection)
                    .await
            }
            Table::Colors => {
                crate::database::Color::delete_from_id(primary_key.into(), connection).await
            }
            Table::Countries => {
                crate::database::Country::delete_from_id(primary_key.into(), connection).await
            }
            Table::DerivedSamples => {
                crate::database::DerivedSample::delete_from_id(primary_key.into(), connection).await
            }
            Table::DocumentFormats => {
                crate::database::DocumentFormat::delete_from_id(primary_key.into(), connection)
                    .await
            }
            Table::FontAwesomeIcons => {
                crate::database::FontAwesomeIcon::delete_from_id(primary_key.into(), connection)
                    .await
            }
            Table::LoginProviders => {
                crate::database::LoginProvider::delete_from_id(primary_key.into(), connection).await
            }
            Table::Materials => {
                crate::database::Material::delete_from_id(primary_key.into(), connection).await
            }
            Table::NameplateCategories => {
                crate::database::NameplateCategory::delete_from_id(primary_key.into(), connection)
                    .await
            }
            Table::Nameplates => {
                crate::database::Nameplate::delete_from_id(primary_key.into(), connection).await
            }
            Table::Notifications => {
                crate::database::Notification::delete_from_id(primary_key.into(), connection).await
            }
            Table::ObservationSubjects => {
                crate::database::ObservationSubject::delete_from_id(primary_key.into(), connection)
                    .await
            }
            Table::Observations => {
                crate::database::Observation::delete_from_id(primary_key.into(), connection).await
            }
            Table::OrganismBioOttTaxonItems => {
                crate::database::OrganismBioOttTaxonItem::delete_from_id(
                    primary_key.into(),
                    connection,
                )
                .await
            }
            Table::Organisms => {
                crate::database::Organism::delete_from_id(primary_key.into(), connection).await
            }
            Table::Organizations => {
                crate::database::Organization::delete_from_id(primary_key.into(), connection).await
            }
            Table::PermanenceCategories => {
                crate::database::PermanenceCategory::delete_from_id(primary_key.into(), connection)
                    .await
            }
            Table::ProjectStates => {
                crate::database::ProjectState::delete_from_id(primary_key.into(), connection).await
            }
            Table::Projects => {
                crate::database::Project::delete_from_id(primary_key.into(), connection).await
            }
            Table::ProjectsTeamsRoleInvitations => {
                crate::database::ProjectsTeamsRoleInvitation::delete_from_id(
                    primary_key.into(),
                    connection,
                )
                .await
            }
            Table::ProjectsTeamsRoleRequests => {
                crate::database::ProjectsTeamsRoleRequest::delete_from_id(
                    primary_key.into(),
                    connection,
                )
                .await
            }
            Table::ProjectsTeamsRoles => {
                crate::database::ProjectsTeamsRole::delete_from_id(primary_key.into(), connection)
                    .await
            }
            Table::ProjectsUsersRoleInvitations => {
                crate::database::ProjectsUsersRoleInvitation::delete_from_id(
                    primary_key.into(),
                    connection,
                )
                .await
            }
            Table::ProjectsUsersRoleRequests => {
                crate::database::ProjectsUsersRoleRequest::delete_from_id(
                    primary_key.into(),
                    connection,
                )
                .await
            }
            Table::ProjectsUsersRoles => {
                crate::database::ProjectsUsersRole::delete_from_id(primary_key.into(), connection)
                    .await
            }
            Table::Roles => {
                crate::database::Role::delete_from_id(primary_key.into(), connection).await
            }
            Table::SampleBioOttTaxonItems => {
                crate::database::SampleBioOttTaxonItem::delete_from_id(
                    primary_key.into(),
                    connection,
                )
                .await
            }
            Table::SampleContainerCategories => {
                crate::database::SampleContainerCategory::delete_from_id(
                    primary_key.into(),
                    connection,
                )
                .await
            }
            Table::SampleContainers => {
                crate::database::SampleContainer::delete_from_id(primary_key.into(), connection)
                    .await
            }
            Table::SampleStates => {
                crate::database::SampleState::delete_from_id(primary_key.into(), connection).await
            }
            Table::Samples => {
                crate::database::Sample::delete_from_id(primary_key.into(), connection).await
            }
            Table::Spectra => {
                crate::database::Spectra::delete_from_id(primary_key.into(), connection).await
            }
            Table::SpectraCollections => {
                crate::database::SpectraCollection::delete_from_id(primary_key.into(), connection)
                    .await
            }
            Table::TeamStates => {
                crate::database::TeamState::delete_from_id(primary_key.into(), connection).await
            }
            Table::Teams => {
                crate::database::Team::delete_from_id(primary_key.into(), connection).await
            }
            Table::TeamsTeamsRoleInvitations => {
                crate::database::TeamsTeamsRoleInvitation::delete_from_id(
                    primary_key.into(),
                    connection,
                )
                .await
            }
            Table::TeamsUsersRoleInvitations => {
                crate::database::TeamsUsersRoleInvitation::delete_from_id(
                    primary_key.into(),
                    connection,
                )
                .await
            }
            Table::TeamsUsersRoleRequests => {
                crate::database::TeamsUsersRoleRequest::delete_from_id(
                    primary_key.into(),
                    connection,
                )
                .await
            }
            Table::TeamsUsersRoles => {
                crate::database::TeamsUsersRole::delete_from_id(primary_key.into(), connection)
                    .await
            }
            Table::Units => {
                crate::database::Unit::delete_from_id(primary_key.into(), connection).await
            }
            Table::UserEmails => {
                crate::database::UserEmail::delete_from_id(primary_key.into(), connection).await
            }
            Table::Users => {
                crate::database::User::delete_from_id(primary_key.into(), connection).await
            }
            Table::UsersUsersRoleInvitations => {
                crate::database::UsersUsersRoleInvitation::delete_from_id(
                    primary_key.into(),
                    connection,
                )
                .await
            }
            Table::UsersUsersRoleRequests => {
                crate::database::UsersUsersRoleRequest::delete_from_id(
                    primary_key.into(),
                    connection,
                )
                .await
            }
            Table::UsersUsersRoles => {
                crate::database::UsersUsersRole::delete_from_id(primary_key.into(), connection)
                    .await
            }
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
    ) -> Result<Option<Vec<u8>>, crate::api::ApiError>
    where
        C: gluesql::core::store::GStore + gluesql::core::store::GStoreMut,
    {
        Ok(match self {
            Table::BioOttRanks => {
                crate::database::NestedBioOttRank::get(primary_key.into(), connection)
                    .await?
                    .map(|row| bincode::serialize(&row))
                    .transpose()?
            }
            Table::BioOttTaxonItems => {
                crate::database::NestedBioOttTaxonItem::get(primary_key.into(), connection)
                    .await?
                    .map(|row| bincode::serialize(&row))
                    .transpose()?
            }
            Table::Colors => crate::database::Color::get(primary_key.into(), connection)
                .await?
                .map(|row| bincode::serialize(&row))
                .transpose()?,
            Table::Countries => crate::database::Country::get(primary_key.into(), connection)
                .await?
                .map(|row| bincode::serialize(&row))
                .transpose()?,
            Table::DerivedSamples => {
                crate::database::NestedDerivedSample::get(primary_key.into(), connection)
                    .await?
                    .map(|row| bincode::serialize(&row))
                    .transpose()?
            }
            Table::DocumentFormats => {
                crate::database::NestedDocumentFormat::get(primary_key.into(), connection)
                    .await?
                    .map(|row| bincode::serialize(&row))
                    .transpose()?
            }
            Table::FontAwesomeIcons => {
                crate::database::FontAwesomeIcon::get(primary_key.into(), connection)
                    .await?
                    .map(|row| bincode::serialize(&row))
                    .transpose()?
            }
            Table::LoginProviders => {
                crate::database::NestedLoginProvider::get(primary_key.into(), connection)
                    .await?
                    .map(|row| bincode::serialize(&row))
                    .transpose()?
            }
            Table::Materials => {
                crate::database::NestedMaterial::get(primary_key.into(), connection)
                    .await?
                    .map(|row| bincode::serialize(&row))
                    .transpose()?
            }
            Table::NameplateCategories => {
                crate::database::NestedNameplateCategory::get(primary_key.into(), connection)
                    .await?
                    .map(|row| bincode::serialize(&row))
                    .transpose()?
            }
            Table::Nameplates => {
                crate::database::NestedNameplate::get(primary_key.into(), connection)
                    .await?
                    .map(|row| bincode::serialize(&row))
                    .transpose()?
            }
            Table::Notifications => {
                crate::database::NestedNotification::get(primary_key.into(), connection)
                    .await?
                    .map(|row| bincode::serialize(&row))
                    .transpose()?
            }
            Table::ObservationSubjects => {
                crate::database::NestedObservationSubject::get(primary_key.into(), connection)
                    .await?
                    .map(|row| bincode::serialize(&row))
                    .transpose()?
            }
            Table::Observations => {
                crate::database::NestedObservation::get(primary_key.into(), connection)
                    .await?
                    .map(|row| bincode::serialize(&row))
                    .transpose()?
            }
            Table::OrganismBioOttTaxonItems => {
                crate::database::NestedOrganismBioOttTaxonItem::get(primary_key.into(), connection)
                    .await?
                    .map(|row| bincode::serialize(&row))
                    .transpose()?
            }
            Table::Organisms => {
                crate::database::NestedOrganism::get(primary_key.into(), connection)
                    .await?
                    .map(|row| bincode::serialize(&row))
                    .transpose()?
            }
            Table::Organizations => {
                crate::database::NestedOrganization::get(primary_key.into(), connection)
                    .await?
                    .map(|row| bincode::serialize(&row))
                    .transpose()?
            }
            Table::PermanenceCategories => {
                crate::database::NestedPermanenceCategory::get(primary_key.into(), connection)
                    .await?
                    .map(|row| bincode::serialize(&row))
                    .transpose()?
            }
            Table::ProjectStates => {
                crate::database::NestedProjectState::get(primary_key.into(), connection)
                    .await?
                    .map(|row| bincode::serialize(&row))
                    .transpose()?
            }
            Table::Projects => crate::database::NestedProject::get(primary_key.into(), connection)
                .await?
                .map(|row| bincode::serialize(&row))
                .transpose()?,
            Table::ProjectsTeamsRoleInvitations => {
                crate::database::NestedProjectsTeamsRoleInvitation::get(
                    primary_key.into(),
                    connection,
                )
                .await?
                .map(|row| bincode::serialize(&row))
                .transpose()?
            }
            Table::ProjectsTeamsRoleRequests => {
                crate::database::NestedProjectsTeamsRoleRequest::get(primary_key.into(), connection)
                    .await?
                    .map(|row| bincode::serialize(&row))
                    .transpose()?
            }
            Table::ProjectsTeamsRoles => {
                crate::database::NestedProjectsTeamsRole::get(primary_key.into(), connection)
                    .await?
                    .map(|row| bincode::serialize(&row))
                    .transpose()?
            }
            Table::ProjectsUsersRoleInvitations => {
                crate::database::NestedProjectsUsersRoleInvitation::get(
                    primary_key.into(),
                    connection,
                )
                .await?
                .map(|row| bincode::serialize(&row))
                .transpose()?
            }
            Table::ProjectsUsersRoleRequests => {
                crate::database::NestedProjectsUsersRoleRequest::get(primary_key.into(), connection)
                    .await?
                    .map(|row| bincode::serialize(&row))
                    .transpose()?
            }
            Table::ProjectsUsersRoles => {
                crate::database::NestedProjectsUsersRole::get(primary_key.into(), connection)
                    .await?
                    .map(|row| bincode::serialize(&row))
                    .transpose()?
            }
            Table::Roles => crate::database::NestedRole::get(primary_key.into(), connection)
                .await?
                .map(|row| bincode::serialize(&row))
                .transpose()?,
            Table::SampleBioOttTaxonItems => {
                crate::database::NestedSampleBioOttTaxonItem::get(primary_key.into(), connection)
                    .await?
                    .map(|row| bincode::serialize(&row))
                    .transpose()?
            }
            Table::SampleContainerCategories => {
                crate::database::NestedSampleContainerCategory::get(primary_key.into(), connection)
                    .await?
                    .map(|row| bincode::serialize(&row))
                    .transpose()?
            }
            Table::SampleContainers => {
                crate::database::NestedSampleContainer::get(primary_key.into(), connection)
                    .await?
                    .map(|row| bincode::serialize(&row))
                    .transpose()?
            }
            Table::SampleStates => {
                crate::database::NestedSampleState::get(primary_key.into(), connection)
                    .await?
                    .map(|row| bincode::serialize(&row))
                    .transpose()?
            }
            Table::Samples => crate::database::NestedSample::get(primary_key.into(), connection)
                .await?
                .map(|row| bincode::serialize(&row))
                .transpose()?,
            Table::Spectra => crate::database::NestedSpectra::get(primary_key.into(), connection)
                .await?
                .map(|row| bincode::serialize(&row))
                .transpose()?,
            Table::SpectraCollections => {
                crate::database::NestedSpectraCollection::get(primary_key.into(), connection)
                    .await?
                    .map(|row| bincode::serialize(&row))
                    .transpose()?
            }
            Table::TeamStates => {
                crate::database::NestedTeamState::get(primary_key.into(), connection)
                    .await?
                    .map(|row| bincode::serialize(&row))
                    .transpose()?
            }
            Table::Teams => crate::database::NestedTeam::get(primary_key.into(), connection)
                .await?
                .map(|row| bincode::serialize(&row))
                .transpose()?,
            Table::TeamsTeamsRoleInvitations => {
                crate::database::NestedTeamsTeamsRoleInvitation::get(primary_key.into(), connection)
                    .await?
                    .map(|row| bincode::serialize(&row))
                    .transpose()?
            }
            Table::TeamsUsersRoleInvitations => {
                crate::database::NestedTeamsUsersRoleInvitation::get(primary_key.into(), connection)
                    .await?
                    .map(|row| bincode::serialize(&row))
                    .transpose()?
            }
            Table::TeamsUsersRoleRequests => {
                crate::database::NestedTeamsUsersRoleRequest::get(primary_key.into(), connection)
                    .await?
                    .map(|row| bincode::serialize(&row))
                    .transpose()?
            }
            Table::TeamsUsersRoles => {
                crate::database::NestedTeamsUsersRole::get(primary_key.into(), connection)
                    .await?
                    .map(|row| bincode::serialize(&row))
                    .transpose()?
            }
            Table::Units => crate::database::NestedUnit::get(primary_key.into(), connection)
                .await?
                .map(|row| bincode::serialize(&row))
                .transpose()?,
            Table::UserEmails => {
                crate::database::NestedUserEmail::get(primary_key.into(), connection)
                    .await?
                    .map(|row| bincode::serialize(&row))
                    .transpose()?
            }
            Table::Users => crate::database::NestedUser::get(primary_key.into(), connection)
                .await?
                .map(|row| bincode::serialize(&row))
                .transpose()?,
            Table::UsersUsersRoleInvitations => {
                crate::database::NestedUsersUsersRoleInvitation::get(primary_key.into(), connection)
                    .await?
                    .map(|row| bincode::serialize(&row))
                    .transpose()?
            }
            Table::UsersUsersRoleRequests => {
                crate::database::NestedUsersUsersRoleRequest::get(primary_key.into(), connection)
                    .await?
                    .map(|row| bincode::serialize(&row))
                    .transpose()?
            }
            Table::UsersUsersRoles => {
                crate::database::NestedUsersUsersRole::get(primary_key.into(), connection)
                    .await?
                    .map(|row| bincode::serialize(&row))
                    .transpose()?
            }
        })
    }
    /// Get all the rows from the table.
    ///
    /// # Arguments
    /// * `filter` - The filter to apply to the rows.
    /// * `limit` - The maximum number of rows to return.
    /// * `offset` - The number of rows to skip. By default `0`.
    /// * `connection` - The database connection.
    ///
    /// # Returns
    /// A vector of the rows of the table.
    pub async fn all<C>(
        &self,
        filter: Option<Vec<u8>>,
        limit: Option<i64>,
        offset: Option<i64>,
        connection: &mut gluesql::prelude::Glue<C>,
    ) -> Result<Vec<Vec<u8>>, crate::api::ApiError>
    where
        C: gluesql::core::store::GStore + gluesql::core::store::GStoreMut,
    {
        match self {
            Table::BioOttRanks => {
                let filter: Option<crate::database::BioOttRankFilter> = filter
                    .map(|filter| bincode::deserialize(&filter).map_err(crate::api::ApiError::from))
                    .transpose()?;
                crate::database::NestedBioOttRank::all(filter.as_ref(), limit, offset, connection)
                    .await?
                    .into_iter()
                    .map(|row| bincode::serialize(&row).map_err(crate::api::ApiError::from))
                    .collect()
            }
            Table::BioOttTaxonItems => {
                let filter: Option<crate::database::BioOttTaxonItemFilter> = filter
                    .map(|filter| bincode::deserialize(&filter).map_err(crate::api::ApiError::from))
                    .transpose()?;
                crate::database::NestedBioOttTaxonItem::all(
                    filter.as_ref(),
                    limit,
                    offset,
                    connection,
                )
                .await?
                .into_iter()
                .map(|row| bincode::serialize(&row).map_err(crate::api::ApiError::from))
                .collect()
            }
            Table::Colors => {
                assert!(filter.is_none(), "Filter not implemented for this table.");
                crate::database::Color::all(limit, offset, connection)
                    .await?
                    .into_iter()
                    .map(|row| bincode::serialize(&row).map_err(crate::api::ApiError::from))
                    .collect()
            }
            Table::Countries => {
                assert!(filter.is_none(), "Filter not implemented for this table.");
                crate::database::Country::all(limit, offset, connection)
                    .await?
                    .into_iter()
                    .map(|row| bincode::serialize(&row).map_err(crate::api::ApiError::from))
                    .collect()
            }
            Table::DerivedSamples => {
                let filter: Option<crate::database::DerivedSampleFilter> = filter
                    .map(|filter| bincode::deserialize(&filter).map_err(crate::api::ApiError::from))
                    .transpose()?;
                crate::database::NestedDerivedSample::all(
                    filter.as_ref(),
                    limit,
                    offset,
                    connection,
                )
                .await?
                .into_iter()
                .map(|row| bincode::serialize(&row).map_err(crate::api::ApiError::from))
                .collect()
            }
            Table::DocumentFormats => {
                let filter: Option<crate::database::DocumentFormatFilter> = filter
                    .map(|filter| bincode::deserialize(&filter).map_err(crate::api::ApiError::from))
                    .transpose()?;
                crate::database::NestedDocumentFormat::all(
                    filter.as_ref(),
                    limit,
                    offset,
                    connection,
                )
                .await?
                .into_iter()
                .map(|row| bincode::serialize(&row).map_err(crate::api::ApiError::from))
                .collect()
            }
            Table::FontAwesomeIcons => {
                assert!(filter.is_none(), "Filter not implemented for this table.");
                crate::database::FontAwesomeIcon::all(limit, offset, connection)
                    .await?
                    .into_iter()
                    .map(|row| bincode::serialize(&row).map_err(crate::api::ApiError::from))
                    .collect()
            }
            Table::LoginProviders => {
                let filter: Option<crate::database::LoginProviderFilter> = filter
                    .map(|filter| bincode::deserialize(&filter).map_err(crate::api::ApiError::from))
                    .transpose()?;
                crate::database::NestedLoginProvider::all(
                    filter.as_ref(),
                    limit,
                    offset,
                    connection,
                )
                .await?
                .into_iter()
                .map(|row| bincode::serialize(&row).map_err(crate::api::ApiError::from))
                .collect()
            }
            Table::Materials => {
                let filter: Option<crate::database::MaterialFilter> = filter
                    .map(|filter| bincode::deserialize(&filter).map_err(crate::api::ApiError::from))
                    .transpose()?;
                crate::database::NestedMaterial::all(filter.as_ref(), limit, offset, connection)
                    .await?
                    .into_iter()
                    .map(|row| bincode::serialize(&row).map_err(crate::api::ApiError::from))
                    .collect()
            }
            Table::NameplateCategories => {
                let filter: Option<crate::database::NameplateCategoryFilter> = filter
                    .map(|filter| bincode::deserialize(&filter).map_err(crate::api::ApiError::from))
                    .transpose()?;
                crate::database::NestedNameplateCategory::all(
                    filter.as_ref(),
                    limit,
                    offset,
                    connection,
                )
                .await?
                .into_iter()
                .map(|row| bincode::serialize(&row).map_err(crate::api::ApiError::from))
                .collect()
            }
            Table::Nameplates => {
                let filter: Option<crate::database::NameplateFilter> = filter
                    .map(|filter| bincode::deserialize(&filter).map_err(crate::api::ApiError::from))
                    .transpose()?;
                crate::database::NestedNameplate::all(filter.as_ref(), limit, offset, connection)
                    .await?
                    .into_iter()
                    .map(|row| bincode::serialize(&row).map_err(crate::api::ApiError::from))
                    .collect()
            }
            Table::Notifications => {
                let filter: Option<crate::database::NotificationFilter> = filter
                    .map(|filter| bincode::deserialize(&filter).map_err(crate::api::ApiError::from))
                    .transpose()?;
                crate::database::NestedNotification::all(filter.as_ref(), limit, offset, connection)
                    .await?
                    .into_iter()
                    .map(|row| bincode::serialize(&row).map_err(crate::api::ApiError::from))
                    .collect()
            }
            Table::ObservationSubjects => {
                let filter: Option<crate::database::ObservationSubjectFilter> = filter
                    .map(|filter| bincode::deserialize(&filter).map_err(crate::api::ApiError::from))
                    .transpose()?;
                crate::database::NestedObservationSubject::all(
                    filter.as_ref(),
                    limit,
                    offset,
                    connection,
                )
                .await?
                .into_iter()
                .map(|row| bincode::serialize(&row).map_err(crate::api::ApiError::from))
                .collect()
            }
            Table::Observations => {
                let filter: Option<crate::database::ObservationFilter> = filter
                    .map(|filter| bincode::deserialize(&filter).map_err(crate::api::ApiError::from))
                    .transpose()?;
                crate::database::NestedObservation::all(filter.as_ref(), limit, offset, connection)
                    .await?
                    .into_iter()
                    .map(|row| bincode::serialize(&row).map_err(crate::api::ApiError::from))
                    .collect()
            }
            Table::OrganismBioOttTaxonItems => {
                let filter: Option<crate::database::OrganismBioOttTaxonItemFilter> = filter
                    .map(|filter| bincode::deserialize(&filter).map_err(crate::api::ApiError::from))
                    .transpose()?;
                crate::database::NestedOrganismBioOttTaxonItem::all(
                    filter.as_ref(),
                    limit,
                    offset,
                    connection,
                )
                .await?
                .into_iter()
                .map(|row| bincode::serialize(&row).map_err(crate::api::ApiError::from))
                .collect()
            }
            Table::Organisms => {
                let filter: Option<crate::database::OrganismFilter> = filter
                    .map(|filter| bincode::deserialize(&filter).map_err(crate::api::ApiError::from))
                    .transpose()?;
                crate::database::NestedOrganism::all(filter.as_ref(), limit, offset, connection)
                    .await?
                    .into_iter()
                    .map(|row| bincode::serialize(&row).map_err(crate::api::ApiError::from))
                    .collect()
            }
            Table::Organizations => {
                let filter: Option<crate::database::OrganizationFilter> = filter
                    .map(|filter| bincode::deserialize(&filter).map_err(crate::api::ApiError::from))
                    .transpose()?;
                crate::database::NestedOrganization::all(filter.as_ref(), limit, offset, connection)
                    .await?
                    .into_iter()
                    .map(|row| bincode::serialize(&row).map_err(crate::api::ApiError::from))
                    .collect()
            }
            Table::PermanenceCategories => {
                let filter: Option<crate::database::PermanenceCategoryFilter> = filter
                    .map(|filter| bincode::deserialize(&filter).map_err(crate::api::ApiError::from))
                    .transpose()?;
                crate::database::NestedPermanenceCategory::all(
                    filter.as_ref(),
                    limit,
                    offset,
                    connection,
                )
                .await?
                .into_iter()
                .map(|row| bincode::serialize(&row).map_err(crate::api::ApiError::from))
                .collect()
            }
            Table::ProjectStates => {
                let filter: Option<crate::database::ProjectStateFilter> = filter
                    .map(|filter| bincode::deserialize(&filter).map_err(crate::api::ApiError::from))
                    .transpose()?;
                crate::database::NestedProjectState::all(filter.as_ref(), limit, offset, connection)
                    .await?
                    .into_iter()
                    .map(|row| bincode::serialize(&row).map_err(crate::api::ApiError::from))
                    .collect()
            }
            Table::Projects => {
                let filter: Option<crate::database::ProjectFilter> = filter
                    .map(|filter| bincode::deserialize(&filter).map_err(crate::api::ApiError::from))
                    .transpose()?;
                crate::database::NestedProject::all(filter.as_ref(), limit, offset, connection)
                    .await?
                    .into_iter()
                    .map(|row| bincode::serialize(&row).map_err(crate::api::ApiError::from))
                    .collect()
            }
            Table::ProjectsTeamsRoleInvitations => {
                let filter: Option<crate::database::ProjectsTeamsRoleInvitationFilter> = filter
                    .map(|filter| bincode::deserialize(&filter).map_err(crate::api::ApiError::from))
                    .transpose()?;
                crate::database::NestedProjectsTeamsRoleInvitation::all(
                    filter.as_ref(),
                    limit,
                    offset,
                    connection,
                )
                .await?
                .into_iter()
                .map(|row| bincode::serialize(&row).map_err(crate::api::ApiError::from))
                .collect()
            }
            Table::ProjectsTeamsRoleRequests => {
                let filter: Option<crate::database::ProjectsTeamsRoleRequestFilter> = filter
                    .map(|filter| bincode::deserialize(&filter).map_err(crate::api::ApiError::from))
                    .transpose()?;
                crate::database::NestedProjectsTeamsRoleRequest::all(
                    filter.as_ref(),
                    limit,
                    offset,
                    connection,
                )
                .await?
                .into_iter()
                .map(|row| bincode::serialize(&row).map_err(crate::api::ApiError::from))
                .collect()
            }
            Table::ProjectsTeamsRoles => {
                let filter: Option<crate::database::ProjectsTeamsRoleFilter> = filter
                    .map(|filter| bincode::deserialize(&filter).map_err(crate::api::ApiError::from))
                    .transpose()?;
                crate::database::NestedProjectsTeamsRole::all(
                    filter.as_ref(),
                    limit,
                    offset,
                    connection,
                )
                .await?
                .into_iter()
                .map(|row| bincode::serialize(&row).map_err(crate::api::ApiError::from))
                .collect()
            }
            Table::ProjectsUsersRoleInvitations => {
                let filter: Option<crate::database::ProjectsUsersRoleInvitationFilter> = filter
                    .map(|filter| bincode::deserialize(&filter).map_err(crate::api::ApiError::from))
                    .transpose()?;
                crate::database::NestedProjectsUsersRoleInvitation::all(
                    filter.as_ref(),
                    limit,
                    offset,
                    connection,
                )
                .await?
                .into_iter()
                .map(|row| bincode::serialize(&row).map_err(crate::api::ApiError::from))
                .collect()
            }
            Table::ProjectsUsersRoleRequests => {
                let filter: Option<crate::database::ProjectsUsersRoleRequestFilter> = filter
                    .map(|filter| bincode::deserialize(&filter).map_err(crate::api::ApiError::from))
                    .transpose()?;
                crate::database::NestedProjectsUsersRoleRequest::all(
                    filter.as_ref(),
                    limit,
                    offset,
                    connection,
                )
                .await?
                .into_iter()
                .map(|row| bincode::serialize(&row).map_err(crate::api::ApiError::from))
                .collect()
            }
            Table::ProjectsUsersRoles => {
                let filter: Option<crate::database::ProjectsUsersRoleFilter> = filter
                    .map(|filter| bincode::deserialize(&filter).map_err(crate::api::ApiError::from))
                    .transpose()?;
                crate::database::NestedProjectsUsersRole::all(
                    filter.as_ref(),
                    limit,
                    offset,
                    connection,
                )
                .await?
                .into_iter()
                .map(|row| bincode::serialize(&row).map_err(crate::api::ApiError::from))
                .collect()
            }
            Table::Roles => {
                let filter: Option<crate::database::RoleFilter> = filter
                    .map(|filter| bincode::deserialize(&filter).map_err(crate::api::ApiError::from))
                    .transpose()?;
                crate::database::NestedRole::all(filter.as_ref(), limit, offset, connection)
                    .await?
                    .into_iter()
                    .map(|row| bincode::serialize(&row).map_err(crate::api::ApiError::from))
                    .collect()
            }
            Table::SampleBioOttTaxonItems => {
                let filter: Option<crate::database::SampleBioOttTaxonItemFilter> = filter
                    .map(|filter| bincode::deserialize(&filter).map_err(crate::api::ApiError::from))
                    .transpose()?;
                crate::database::NestedSampleBioOttTaxonItem::all(
                    filter.as_ref(),
                    limit,
                    offset,
                    connection,
                )
                .await?
                .into_iter()
                .map(|row| bincode::serialize(&row).map_err(crate::api::ApiError::from))
                .collect()
            }
            Table::SampleContainerCategories => {
                let filter: Option<crate::database::SampleContainerCategoryFilter> = filter
                    .map(|filter| bincode::deserialize(&filter).map_err(crate::api::ApiError::from))
                    .transpose()?;
                crate::database::NestedSampleContainerCategory::all(
                    filter.as_ref(),
                    limit,
                    offset,
                    connection,
                )
                .await?
                .into_iter()
                .map(|row| bincode::serialize(&row).map_err(crate::api::ApiError::from))
                .collect()
            }
            Table::SampleContainers => {
                let filter: Option<crate::database::SampleContainerFilter> = filter
                    .map(|filter| bincode::deserialize(&filter).map_err(crate::api::ApiError::from))
                    .transpose()?;
                crate::database::NestedSampleContainer::all(
                    filter.as_ref(),
                    limit,
                    offset,
                    connection,
                )
                .await?
                .into_iter()
                .map(|row| bincode::serialize(&row).map_err(crate::api::ApiError::from))
                .collect()
            }
            Table::SampleStates => {
                let filter: Option<crate::database::SampleStateFilter> = filter
                    .map(|filter| bincode::deserialize(&filter).map_err(crate::api::ApiError::from))
                    .transpose()?;
                crate::database::NestedSampleState::all(filter.as_ref(), limit, offset, connection)
                    .await?
                    .into_iter()
                    .map(|row| bincode::serialize(&row).map_err(crate::api::ApiError::from))
                    .collect()
            }
            Table::Samples => {
                let filter: Option<crate::database::SampleFilter> = filter
                    .map(|filter| bincode::deserialize(&filter).map_err(crate::api::ApiError::from))
                    .transpose()?;
                crate::database::NestedSample::all(filter.as_ref(), limit, offset, connection)
                    .await?
                    .into_iter()
                    .map(|row| bincode::serialize(&row).map_err(crate::api::ApiError::from))
                    .collect()
            }
            Table::Spectra => {
                let filter: Option<crate::database::SpectraFilter> = filter
                    .map(|filter| bincode::deserialize(&filter).map_err(crate::api::ApiError::from))
                    .transpose()?;
                crate::database::NestedSpectra::all(filter.as_ref(), limit, offset, connection)
                    .await?
                    .into_iter()
                    .map(|row| bincode::serialize(&row).map_err(crate::api::ApiError::from))
                    .collect()
            }
            Table::SpectraCollections => {
                let filter: Option<crate::database::SpectraCollectionFilter> = filter
                    .map(|filter| bincode::deserialize(&filter).map_err(crate::api::ApiError::from))
                    .transpose()?;
                crate::database::NestedSpectraCollection::all(
                    filter.as_ref(),
                    limit,
                    offset,
                    connection,
                )
                .await?
                .into_iter()
                .map(|row| bincode::serialize(&row).map_err(crate::api::ApiError::from))
                .collect()
            }
            Table::TeamStates => {
                let filter: Option<crate::database::TeamStateFilter> = filter
                    .map(|filter| bincode::deserialize(&filter).map_err(crate::api::ApiError::from))
                    .transpose()?;
                crate::database::NestedTeamState::all(filter.as_ref(), limit, offset, connection)
                    .await?
                    .into_iter()
                    .map(|row| bincode::serialize(&row).map_err(crate::api::ApiError::from))
                    .collect()
            }
            Table::Teams => {
                let filter: Option<crate::database::TeamFilter> = filter
                    .map(|filter| bincode::deserialize(&filter).map_err(crate::api::ApiError::from))
                    .transpose()?;
                crate::database::NestedTeam::all(filter.as_ref(), limit, offset, connection)
                    .await?
                    .into_iter()
                    .map(|row| bincode::serialize(&row).map_err(crate::api::ApiError::from))
                    .collect()
            }
            Table::TeamsTeamsRoleInvitations => {
                let filter: Option<crate::database::TeamsTeamsRoleInvitationFilter> = filter
                    .map(|filter| bincode::deserialize(&filter).map_err(crate::api::ApiError::from))
                    .transpose()?;
                crate::database::NestedTeamsTeamsRoleInvitation::all(
                    filter.as_ref(),
                    limit,
                    offset,
                    connection,
                )
                .await?
                .into_iter()
                .map(|row| bincode::serialize(&row).map_err(crate::api::ApiError::from))
                .collect()
            }
            Table::TeamsUsersRoleInvitations => {
                let filter: Option<crate::database::TeamsUsersRoleInvitationFilter> = filter
                    .map(|filter| bincode::deserialize(&filter).map_err(crate::api::ApiError::from))
                    .transpose()?;
                crate::database::NestedTeamsUsersRoleInvitation::all(
                    filter.as_ref(),
                    limit,
                    offset,
                    connection,
                )
                .await?
                .into_iter()
                .map(|row| bincode::serialize(&row).map_err(crate::api::ApiError::from))
                .collect()
            }
            Table::TeamsUsersRoleRequests => {
                let filter: Option<crate::database::TeamsUsersRoleRequestFilter> = filter
                    .map(|filter| bincode::deserialize(&filter).map_err(crate::api::ApiError::from))
                    .transpose()?;
                crate::database::NestedTeamsUsersRoleRequest::all(
                    filter.as_ref(),
                    limit,
                    offset,
                    connection,
                )
                .await?
                .into_iter()
                .map(|row| bincode::serialize(&row).map_err(crate::api::ApiError::from))
                .collect()
            }
            Table::TeamsUsersRoles => {
                let filter: Option<crate::database::TeamsUsersRoleFilter> = filter
                    .map(|filter| bincode::deserialize(&filter).map_err(crate::api::ApiError::from))
                    .transpose()?;
                crate::database::NestedTeamsUsersRole::all(
                    filter.as_ref(),
                    limit,
                    offset,
                    connection,
                )
                .await?
                .into_iter()
                .map(|row| bincode::serialize(&row).map_err(crate::api::ApiError::from))
                .collect()
            }
            Table::Units => {
                let filter: Option<crate::database::UnitFilter> = filter
                    .map(|filter| bincode::deserialize(&filter).map_err(crate::api::ApiError::from))
                    .transpose()?;
                crate::database::NestedUnit::all(filter.as_ref(), limit, offset, connection)
                    .await?
                    .into_iter()
                    .map(|row| bincode::serialize(&row).map_err(crate::api::ApiError::from))
                    .collect()
            }
            Table::UserEmails => {
                let filter: Option<crate::database::UserEmailFilter> = filter
                    .map(|filter| bincode::deserialize(&filter).map_err(crate::api::ApiError::from))
                    .transpose()?;
                crate::database::NestedUserEmail::all(filter.as_ref(), limit, offset, connection)
                    .await?
                    .into_iter()
                    .map(|row| bincode::serialize(&row).map_err(crate::api::ApiError::from))
                    .collect()
            }
            Table::Users => {
                let filter: Option<crate::database::UserFilter> = filter
                    .map(|filter| bincode::deserialize(&filter).map_err(crate::api::ApiError::from))
                    .transpose()?;
                crate::database::NestedUser::all(filter.as_ref(), limit, offset, connection)
                    .await?
                    .into_iter()
                    .map(|row| bincode::serialize(&row).map_err(crate::api::ApiError::from))
                    .collect()
            }
            Table::UsersUsersRoleInvitations => {
                let filter: Option<crate::database::UsersUsersRoleInvitationFilter> = filter
                    .map(|filter| bincode::deserialize(&filter).map_err(crate::api::ApiError::from))
                    .transpose()?;
                crate::database::NestedUsersUsersRoleInvitation::all(
                    filter.as_ref(),
                    limit,
                    offset,
                    connection,
                )
                .await?
                .into_iter()
                .map(|row| bincode::serialize(&row).map_err(crate::api::ApiError::from))
                .collect()
            }
            Table::UsersUsersRoleRequests => {
                let filter: Option<crate::database::UsersUsersRoleRequestFilter> = filter
                    .map(|filter| bincode::deserialize(&filter).map_err(crate::api::ApiError::from))
                    .transpose()?;
                crate::database::NestedUsersUsersRoleRequest::all(
                    filter.as_ref(),
                    limit,
                    offset,
                    connection,
                )
                .await?
                .into_iter()
                .map(|row| bincode::serialize(&row).map_err(crate::api::ApiError::from))
                .collect()
            }
            Table::UsersUsersRoles => {
                let filter: Option<crate::database::UsersUsersRoleFilter> = filter
                    .map(|filter| bincode::deserialize(&filter).map_err(crate::api::ApiError::from))
                    .transpose()?;
                crate::database::NestedUsersUsersRole::all(
                    filter.as_ref(),
                    limit,
                    offset,
                    connection,
                )
                .await?
                .into_iter()
                .map(|row| bincode::serialize(&row).map_err(crate::api::ApiError::from))
                .collect()
            }
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
    ) -> Result<Vec<u8>, crate::api::ApiError>
    where
        C: gluesql::core::store::GStore + gluesql::core::store::GStoreMut,
    {
        Ok(match self {
            Table::BioOttRanks => unimplemented!("Insert not implemented for bio_ott_ranks."),
            Table::BioOttTaxonItems => unimplemented!("Insert not implemented for bio_ott_taxon_items."),
            Table::Colors => unimplemented!("Insert not implemented for colors."),
            Table::Countries => unimplemented!("Insert not implemented for countries."),
            Table::DerivedSamples => unimplemented!("Insert not implemented for derived_samples in frontend as it does not have a UUID primary key."),
            Table::DocumentFormats => unimplemented!("Insert not implemented for document_formats."),
            Table::FontAwesomeIcons => unimplemented!("Insert not implemented for font_awesome_icons."),
            Table::LoginProviders => unimplemented!("Insert not implemented for login_providers."),
            Table::Materials => unimplemented!("Insert not implemented for materials."),
            Table::NameplateCategories => unimplemented!("Insert not implemented for nameplate_categories."),
            Table::Nameplates => unimplemented!("Insert not implemented for nameplates in frontend as it does not have a UUID primary key."),
            Table::Notifications => unimplemented!("Insert not implemented for notifications."),
            Table::ObservationSubjects => unimplemented!("Insert not implemented for observation_subjects."),
            Table::Observations => {
                let new_row: super::NewObservation = bincode::deserialize::<super::NewObservation>(&new_row).map_err(crate::api::ApiError::from)?;
                let inserted_row: super::Observation = new_row.insert(user_id, connection).await?;
                let nested_row = super::NestedObservation::from_flat(inserted_row, connection).await?;
                 bincode::serialize(&nested_row).map_err(crate::api::ApiError::from)?
            },
            Table::OrganismBioOttTaxonItems => unimplemented!("Insert not implemented for organism_bio_ott_taxon_items in frontend as it does not have a UUID primary key."),
            Table::Organisms => {
                let new_row: super::NewOrganism = bincode::deserialize::<super::NewOrganism>(&new_row).map_err(crate::api::ApiError::from)?;
                let inserted_row: super::Organism = new_row.insert(user_id, connection).await?;
                let nested_row = super::NestedOrganism::from_flat(inserted_row, connection).await?;
                 bincode::serialize(&nested_row).map_err(crate::api::ApiError::from)?
            },
            Table::Organizations => unimplemented!("Insert not implemented for organizations."),
            Table::PermanenceCategories => unimplemented!("Insert not implemented for permanence_categories."),
            Table::ProjectStates => unimplemented!("Insert not implemented for project_states."),
            Table::Projects => unimplemented!("Insert not implemented for projects in frontend as it does not have a UUID primary key."),
            Table::ProjectsTeamsRoleInvitations => unimplemented!("Insert not implemented for projects_teams_role_invitations in frontend as it does not have a UUID primary key."),
            Table::ProjectsTeamsRoleRequests => unimplemented!("Insert not implemented for projects_teams_role_requests in frontend as it does not have a UUID primary key."),
            Table::ProjectsTeamsRoles => unimplemented!("Insert not implemented for projects_teams_roles in frontend as it does not have a UUID primary key."),
            Table::ProjectsUsersRoleInvitations => unimplemented!("Insert not implemented for projects_users_role_invitations in frontend as it does not have a UUID primary key."),
            Table::ProjectsUsersRoleRequests => unimplemented!("Insert not implemented for projects_users_role_requests in frontend as it does not have a UUID primary key."),
            Table::ProjectsUsersRoles => unimplemented!("Insert not implemented for projects_users_roles in frontend as it does not have a UUID primary key."),
            Table::Roles => unimplemented!("Insert not implemented for roles."),
            Table::SampleBioOttTaxonItems => unimplemented!("Insert not implemented for sample_bio_ott_taxon_items in frontend as it does not have a UUID primary key."),
            Table::SampleContainerCategories => unimplemented!("Insert not implemented for sample_container_categories."),
            Table::SampleContainers => unimplemented!("Insert not implemented for sample_containers in frontend as it does not have a UUID primary key."),
            Table::SampleStates => unimplemented!("Insert not implemented for sample_states."),
            Table::Samples => {
                let new_row: super::NewSample = bincode::deserialize::<super::NewSample>(&new_row).map_err(crate::api::ApiError::from)?;
                let inserted_row: super::Sample = new_row.insert(user_id, connection).await?;
                let nested_row = super::NestedSample::from_flat(inserted_row, connection).await?;
                 bincode::serialize(&nested_row).map_err(crate::api::ApiError::from)?
            },
            Table::Spectra => unimplemented!("Insert not implemented for spectra."),
            Table::SpectraCollections => unimplemented!("Insert not implemented for spectra_collections in frontend as it does not have a UUID primary key."),
            Table::TeamStates => unimplemented!("Insert not implemented for team_states."),
            Table::Teams => unimplemented!("Insert not implemented for teams in frontend as it does not have a UUID primary key."),
            Table::TeamsTeamsRoleInvitations => unimplemented!("Insert not implemented for teams_teams_role_invitations in frontend as it does not have a UUID primary key."),
            Table::TeamsUsersRoleInvitations => unimplemented!("Insert not implemented for teams_users_role_invitations in frontend as it does not have a UUID primary key."),
            Table::TeamsUsersRoleRequests => unimplemented!("Insert not implemented for teams_users_role_requests in frontend as it does not have a UUID primary key."),
            Table::TeamsUsersRoles => unimplemented!("Insert not implemented for teams_users_roles in frontend as it does not have a UUID primary key."),
            Table::Units => unimplemented!("Insert not implemented for units."),
            Table::UserEmails => unimplemented!("Insert not implemented for user_emails in frontend as it does not have a UUID primary key."),
            Table::Users => unimplemented!("Insert not implemented for users in frontend as it does not have a UUID primary key."),
            Table::UsersUsersRoleInvitations => unimplemented!("Insert not implemented for users_users_role_invitations in frontend as it does not have a UUID primary key."),
            Table::UsersUsersRoleRequests => unimplemented!("Insert not implemented for users_users_role_requests in frontend as it does not have a UUID primary key."),
            Table::UsersUsersRoles => unimplemented!("Insert not implemented for users_users_roles in frontend as it does not have a UUID primary key."),
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
    ) -> Result<Vec<u8>, crate::api::ApiError>
    where
        C: gluesql::core::store::GStore + gluesql::core::store::GStoreMut,
    {
        Ok(match self {
            Table::BioOttRanks => unimplemented!("Update not implemented for bio_ott_ranks."),
            Table::BioOttTaxonItems => {
                unimplemented!("Update not implemented for bio_ott_taxon_items.")
            }
            Table::Colors => unimplemented!("Update not implemented for colors."),
            Table::Countries => unimplemented!("Update not implemented for countries."),
            Table::DerivedSamples => unimplemented!("Update not implemented for derived_samples."),
            Table::DocumentFormats => {
                unimplemented!("Update not implemented for document_formats.")
            }
            Table::FontAwesomeIcons => {
                unimplemented!("Update not implemented for font_awesome_icons.")
            }
            Table::LoginProviders => unimplemented!("Update not implemented for login_providers."),
            Table::Materials => unimplemented!("Update not implemented for materials."),
            Table::NameplateCategories => {
                unimplemented!("Update not implemented for nameplate_categories.")
            }
            Table::Nameplates => {
                let update_row: super::UpdateNameplate =
                    bincode::deserialize::<super::UpdateNameplate>(&update_row)
                        .map_err(crate::api::ApiError::from)?;
                let id = update_row.id;
                update_row.update(user_id, connection).await?;
                let updated_row: super::Nameplate =
                    super::Nameplate::get(id, connection).await?.unwrap();
                let nested_row = super::NestedNameplate::from_flat(updated_row, connection).await?;
                bincode::serialize(&nested_row).map_err(crate::api::ApiError::from)?
            }
            Table::Notifications => unimplemented!("Update not implemented for notifications."),
            Table::ObservationSubjects => {
                unimplemented!("Update not implemented for observation_subjects.")
            }
            Table::Observations => {
                let update_row: super::NewObservation =
                    bincode::deserialize::<super::NewObservation>(&update_row)
                        .map_err(crate::api::ApiError::from)?;
                let id = update_row.id;
                update_row.update(user_id, connection).await?;
                let updated_row: super::Observation =
                    super::Observation::get(id, connection).await?.unwrap();
                let nested_row =
                    super::NestedObservation::from_flat(updated_row, connection).await?;
                bincode::serialize(&nested_row).map_err(crate::api::ApiError::from)?
            }
            Table::OrganismBioOttTaxonItems => {
                unimplemented!("Update not implemented for organism_bio_ott_taxon_items.")
            }
            Table::Organisms => {
                let update_row: super::NewOrganism =
                    bincode::deserialize::<super::NewOrganism>(&update_row)
                        .map_err(crate::api::ApiError::from)?;
                let id = update_row.id;
                update_row.update(user_id, connection).await?;
                let updated_row: super::Organism =
                    super::Organism::get(id, connection).await?.unwrap();
                let nested_row = super::NestedOrganism::from_flat(updated_row, connection).await?;
                bincode::serialize(&nested_row).map_err(crate::api::ApiError::from)?
            }
            Table::Organizations => unimplemented!("Update not implemented for organizations."),
            Table::PermanenceCategories => {
                unimplemented!("Update not implemented for permanence_categories.")
            }
            Table::ProjectStates => unimplemented!("Update not implemented for project_states."),
            Table::Projects => {
                let update_row: super::UpdateProject =
                    bincode::deserialize::<super::UpdateProject>(&update_row)
                        .map_err(crate::api::ApiError::from)?;
                let id = update_row.id;
                update_row.update(user_id, connection).await?;
                let updated_row: super::Project =
                    super::Project::get(id, connection).await?.unwrap();
                let nested_row = super::NestedProject::from_flat(updated_row, connection).await?;
                bincode::serialize(&nested_row).map_err(crate::api::ApiError::from)?
            }
            Table::ProjectsTeamsRoleInvitations => {
                unimplemented!("Update not implemented for projects_teams_role_invitations.")
            }
            Table::ProjectsTeamsRoleRequests => {
                unimplemented!("Update not implemented for projects_teams_role_requests.")
            }
            Table::ProjectsTeamsRoles => {
                unimplemented!("Update not implemented for projects_teams_roles.")
            }
            Table::ProjectsUsersRoleInvitations => {
                unimplemented!("Update not implemented for projects_users_role_invitations.")
            }
            Table::ProjectsUsersRoleRequests => {
                unimplemented!("Update not implemented for projects_users_role_requests.")
            }
            Table::ProjectsUsersRoles => {
                unimplemented!("Update not implemented for projects_users_roles.")
            }
            Table::Roles => unimplemented!("Update not implemented for roles."),
            Table::SampleBioOttTaxonItems => {
                unimplemented!("Update not implemented for sample_bio_ott_taxon_items.")
            }
            Table::SampleContainerCategories => {
                unimplemented!("Update not implemented for sample_container_categories.")
            }
            Table::SampleContainers => {
                let update_row: super::UpdateSampleContainer =
                    bincode::deserialize::<super::UpdateSampleContainer>(&update_row)
                        .map_err(crate::api::ApiError::from)?;
                let id = update_row.id;
                update_row.update(user_id, connection).await?;
                let updated_row: super::SampleContainer =
                    super::SampleContainer::get(id, connection).await?.unwrap();
                let nested_row =
                    super::NestedSampleContainer::from_flat(updated_row, connection).await?;
                bincode::serialize(&nested_row).map_err(crate::api::ApiError::from)?
            }
            Table::SampleStates => unimplemented!("Update not implemented for sample_states."),
            Table::Samples => {
                let update_row: super::NewSample =
                    bincode::deserialize::<super::NewSample>(&update_row)
                        .map_err(crate::api::ApiError::from)?;
                let id = update_row.id;
                update_row.update(user_id, connection).await?;
                let updated_row: super::Sample = super::Sample::get(id, connection).await?.unwrap();
                let nested_row = super::NestedSample::from_flat(updated_row, connection).await?;
                bincode::serialize(&nested_row).map_err(crate::api::ApiError::from)?
            }
            Table::Spectra => unimplemented!("Update not implemented for spectra."),
            Table::SpectraCollections => {
                let update_row: super::UpdateSpectraCollection =
                    bincode::deserialize::<super::UpdateSpectraCollection>(&update_row)
                        .map_err(crate::api::ApiError::from)?;
                let id = update_row.id;
                update_row.update(user_id, connection).await?;
                let updated_row: super::SpectraCollection =
                    super::SpectraCollection::get(id, connection)
                        .await?
                        .unwrap();
                let nested_row =
                    super::NestedSpectraCollection::from_flat(updated_row, connection).await?;
                bincode::serialize(&nested_row).map_err(crate::api::ApiError::from)?
            }
            Table::TeamStates => unimplemented!("Update not implemented for team_states."),
            Table::Teams => {
                let update_row: super::UpdateTeam =
                    bincode::deserialize::<super::UpdateTeam>(&update_row)
                        .map_err(crate::api::ApiError::from)?;
                let id = update_row.id;
                update_row.update(user_id, connection).await?;
                let updated_row: super::Team = super::Team::get(id, connection).await?.unwrap();
                let nested_row = super::NestedTeam::from_flat(updated_row, connection).await?;
                bincode::serialize(&nested_row).map_err(crate::api::ApiError::from)?
            }
            Table::TeamsTeamsRoleInvitations => {
                unimplemented!("Update not implemented for teams_teams_role_invitations.")
            }
            Table::TeamsUsersRoleInvitations => {
                unimplemented!("Update not implemented for teams_users_role_invitations.")
            }
            Table::TeamsUsersRoleRequests => {
                unimplemented!("Update not implemented for teams_users_role_requests.")
            }
            Table::TeamsUsersRoles => {
                unimplemented!("Update not implemented for teams_users_roles.")
            }
            Table::Units => unimplemented!("Update not implemented for units."),
            Table::UserEmails => unimplemented!("Update not implemented for user_emails."),
            Table::Users => {
                let update_row: super::UpdateUser =
                    bincode::deserialize::<super::UpdateUser>(&update_row)
                        .map_err(crate::api::ApiError::from)?;
                let id = update_row.id;
                update_row.update(connection).await?;
                let updated_row: super::User = super::User::get(id, connection).await?.unwrap();
                let nested_row = super::NestedUser::from_flat(updated_row, connection).await?;
                bincode::serialize(&nested_row).map_err(crate::api::ApiError::from)?
            }
            Table::UsersUsersRoleInvitations => {
                unimplemented!("Update not implemented for users_users_role_invitations.")
            }
            Table::UsersUsersRoleRequests => {
                unimplemented!("Update not implemented for users_users_role_requests.")
            }
            Table::UsersUsersRoles => {
                unimplemented!("Update not implemented for users_users_roles.")
            }
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
        rows: Vec<Vec<u8>>,
        connection: &mut gluesql::prelude::Glue<C>,
    ) -> Result<(), crate::api::ApiError>
    where
        C: gluesql::core::store::GStore + gluesql::core::store::GStoreMut,
    {
        match self {
            Table::BioOttRanks => {
                for row in rows {
                    let row: super::NestedBioOttRank =
                        bincode::deserialize::<super::NestedBioOttRank>(&row)
                            .map_err(crate::api::ApiError::from)?;
                    row.update_or_insert(connection).await?;
                }
            }
            Table::BioOttTaxonItems => {
                for row in rows {
                    let row: super::NestedBioOttTaxonItem =
                        bincode::deserialize::<super::NestedBioOttTaxonItem>(&row)
                            .map_err(crate::api::ApiError::from)?;
                    row.update_or_insert(connection).await?;
                }
            }
            Table::Colors => {
                for row in rows {
                    let row: super::Color = bincode::deserialize::<super::Color>(&row)
                        .map_err(crate::api::ApiError::from)?;
                    row.update_or_insert(connection).await?;
                }
            }
            Table::Countries => {
                for row in rows {
                    let row: super::Country = bincode::deserialize::<super::Country>(&row)
                        .map_err(crate::api::ApiError::from)?;
                    row.update_or_insert(connection).await?;
                }
            }
            Table::DerivedSamples => {
                for row in rows {
                    let row: super::NestedDerivedSample =
                        bincode::deserialize::<super::NestedDerivedSample>(&row)
                            .map_err(crate::api::ApiError::from)?;
                    row.update_or_insert(connection).await?;
                }
            }
            Table::DocumentFormats => {
                for row in rows {
                    let row: super::NestedDocumentFormat =
                        bincode::deserialize::<super::NestedDocumentFormat>(&row)
                            .map_err(crate::api::ApiError::from)?;
                    row.update_or_insert(connection).await?;
                }
            }
            Table::FontAwesomeIcons => {
                for row in rows {
                    let row: super::FontAwesomeIcon =
                        bincode::deserialize::<super::FontAwesomeIcon>(&row)
                            .map_err(crate::api::ApiError::from)?;
                    row.update_or_insert(connection).await?;
                }
            }
            Table::LoginProviders => {
                for row in rows {
                    let row: super::NestedLoginProvider =
                        bincode::deserialize::<super::NestedLoginProvider>(&row)
                            .map_err(crate::api::ApiError::from)?;
                    row.update_or_insert(connection).await?;
                }
            }
            Table::Materials => {
                for row in rows {
                    let row: super::NestedMaterial =
                        bincode::deserialize::<super::NestedMaterial>(&row)
                            .map_err(crate::api::ApiError::from)?;
                    row.update_or_insert(connection).await?;
                }
            }
            Table::NameplateCategories => {
                for row in rows {
                    let row: super::NestedNameplateCategory =
                        bincode::deserialize::<super::NestedNameplateCategory>(&row)
                            .map_err(crate::api::ApiError::from)?;
                    row.update_or_insert(connection).await?;
                }
            }
            Table::Nameplates => {
                for row in rows {
                    let row: super::NestedNameplate =
                        bincode::deserialize::<super::NestedNameplate>(&row)
                            .map_err(crate::api::ApiError::from)?;
                    row.update_or_insert(connection).await?;
                }
            }
            Table::Notifications => {
                for row in rows {
                    let row: super::NestedNotification =
                        bincode::deserialize::<super::NestedNotification>(&row)
                            .map_err(crate::api::ApiError::from)?;
                    row.update_or_insert(connection).await?;
                }
            }
            Table::ObservationSubjects => {
                for row in rows {
                    let row: super::NestedObservationSubject =
                        bincode::deserialize::<super::NestedObservationSubject>(&row)
                            .map_err(crate::api::ApiError::from)?;
                    row.update_or_insert(connection).await?;
                }
            }
            Table::Observations => {
                for row in rows {
                    let row: super::NestedObservation =
                        bincode::deserialize::<super::NestedObservation>(&row)
                            .map_err(crate::api::ApiError::from)?;
                    row.update_or_insert(connection).await?;
                }
            }
            Table::OrganismBioOttTaxonItems => {
                for row in rows {
                    let row: super::NestedOrganismBioOttTaxonItem =
                        bincode::deserialize::<super::NestedOrganismBioOttTaxonItem>(&row)
                            .map_err(crate::api::ApiError::from)?;
                    row.update_or_insert(connection).await?;
                }
            }
            Table::Organisms => {
                for row in rows {
                    let row: super::NestedOrganism =
                        bincode::deserialize::<super::NestedOrganism>(&row)
                            .map_err(crate::api::ApiError::from)?;
                    row.update_or_insert(connection).await?;
                }
            }
            Table::Organizations => {
                for row in rows {
                    let row: super::NestedOrganization =
                        bincode::deserialize::<super::NestedOrganization>(&row)
                            .map_err(crate::api::ApiError::from)?;
                    row.update_or_insert(connection).await?;
                }
            }
            Table::PermanenceCategories => {
                for row in rows {
                    let row: super::NestedPermanenceCategory =
                        bincode::deserialize::<super::NestedPermanenceCategory>(&row)
                            .map_err(crate::api::ApiError::from)?;
                    row.update_or_insert(connection).await?;
                }
            }
            Table::ProjectStates => {
                for row in rows {
                    let row: super::NestedProjectState =
                        bincode::deserialize::<super::NestedProjectState>(&row)
                            .map_err(crate::api::ApiError::from)?;
                    row.update_or_insert(connection).await?;
                }
            }
            Table::Projects => {
                for row in rows {
                    let row: super::NestedProject =
                        bincode::deserialize::<super::NestedProject>(&row)
                            .map_err(crate::api::ApiError::from)?;
                    row.update_or_insert(connection).await?;
                }
            }
            Table::ProjectsTeamsRoleInvitations => {
                for row in rows {
                    let row: super::NestedProjectsTeamsRoleInvitation =
                        bincode::deserialize::<super::NestedProjectsTeamsRoleInvitation>(&row)
                            .map_err(crate::api::ApiError::from)?;
                    row.update_or_insert(connection).await?;
                }
            }
            Table::ProjectsTeamsRoleRequests => {
                for row in rows {
                    let row: super::NestedProjectsTeamsRoleRequest =
                        bincode::deserialize::<super::NestedProjectsTeamsRoleRequest>(&row)
                            .map_err(crate::api::ApiError::from)?;
                    row.update_or_insert(connection).await?;
                }
            }
            Table::ProjectsTeamsRoles => {
                for row in rows {
                    let row: super::NestedProjectsTeamsRole =
                        bincode::deserialize::<super::NestedProjectsTeamsRole>(&row)
                            .map_err(crate::api::ApiError::from)?;
                    row.update_or_insert(connection).await?;
                }
            }
            Table::ProjectsUsersRoleInvitations => {
                for row in rows {
                    let row: super::NestedProjectsUsersRoleInvitation =
                        bincode::deserialize::<super::NestedProjectsUsersRoleInvitation>(&row)
                            .map_err(crate::api::ApiError::from)?;
                    row.update_or_insert(connection).await?;
                }
            }
            Table::ProjectsUsersRoleRequests => {
                for row in rows {
                    let row: super::NestedProjectsUsersRoleRequest =
                        bincode::deserialize::<super::NestedProjectsUsersRoleRequest>(&row)
                            .map_err(crate::api::ApiError::from)?;
                    row.update_or_insert(connection).await?;
                }
            }
            Table::ProjectsUsersRoles => {
                for row in rows {
                    let row: super::NestedProjectsUsersRole =
                        bincode::deserialize::<super::NestedProjectsUsersRole>(&row)
                            .map_err(crate::api::ApiError::from)?;
                    row.update_or_insert(connection).await?;
                }
            }
            Table::Roles => {
                for row in rows {
                    let row: super::NestedRole = bincode::deserialize::<super::NestedRole>(&row)
                        .map_err(crate::api::ApiError::from)?;
                    row.update_or_insert(connection).await?;
                }
            }
            Table::SampleBioOttTaxonItems => {
                for row in rows {
                    let row: super::NestedSampleBioOttTaxonItem =
                        bincode::deserialize::<super::NestedSampleBioOttTaxonItem>(&row)
                            .map_err(crate::api::ApiError::from)?;
                    row.update_or_insert(connection).await?;
                }
            }
            Table::SampleContainerCategories => {
                for row in rows {
                    let row: super::NestedSampleContainerCategory =
                        bincode::deserialize::<super::NestedSampleContainerCategory>(&row)
                            .map_err(crate::api::ApiError::from)?;
                    row.update_or_insert(connection).await?;
                }
            }
            Table::SampleContainers => {
                for row in rows {
                    let row: super::NestedSampleContainer =
                        bincode::deserialize::<super::NestedSampleContainer>(&row)
                            .map_err(crate::api::ApiError::from)?;
                    row.update_or_insert(connection).await?;
                }
            }
            Table::SampleStates => {
                for row in rows {
                    let row: super::NestedSampleState =
                        bincode::deserialize::<super::NestedSampleState>(&row)
                            .map_err(crate::api::ApiError::from)?;
                    row.update_or_insert(connection).await?;
                }
            }
            Table::Samples => {
                for row in rows {
                    let row: super::NestedSample =
                        bincode::deserialize::<super::NestedSample>(&row)
                            .map_err(crate::api::ApiError::from)?;
                    row.update_or_insert(connection).await?;
                }
            }
            Table::Spectra => {
                for row in rows {
                    let row: super::NestedSpectra =
                        bincode::deserialize::<super::NestedSpectra>(&row)
                            .map_err(crate::api::ApiError::from)?;
                    row.update_or_insert(connection).await?;
                }
            }
            Table::SpectraCollections => {
                for row in rows {
                    let row: super::NestedSpectraCollection =
                        bincode::deserialize::<super::NestedSpectraCollection>(&row)
                            .map_err(crate::api::ApiError::from)?;
                    row.update_or_insert(connection).await?;
                }
            }
            Table::TeamStates => {
                for row in rows {
                    let row: super::NestedTeamState =
                        bincode::deserialize::<super::NestedTeamState>(&row)
                            .map_err(crate::api::ApiError::from)?;
                    row.update_or_insert(connection).await?;
                }
            }
            Table::Teams => {
                for row in rows {
                    let row: super::NestedTeam = bincode::deserialize::<super::NestedTeam>(&row)
                        .map_err(crate::api::ApiError::from)?;
                    row.update_or_insert(connection).await?;
                }
            }
            Table::TeamsTeamsRoleInvitations => {
                for row in rows {
                    let row: super::NestedTeamsTeamsRoleInvitation =
                        bincode::deserialize::<super::NestedTeamsTeamsRoleInvitation>(&row)
                            .map_err(crate::api::ApiError::from)?;
                    row.update_or_insert(connection).await?;
                }
            }
            Table::TeamsUsersRoleInvitations => {
                for row in rows {
                    let row: super::NestedTeamsUsersRoleInvitation =
                        bincode::deserialize::<super::NestedTeamsUsersRoleInvitation>(&row)
                            .map_err(crate::api::ApiError::from)?;
                    row.update_or_insert(connection).await?;
                }
            }
            Table::TeamsUsersRoleRequests => {
                for row in rows {
                    let row: super::NestedTeamsUsersRoleRequest =
                        bincode::deserialize::<super::NestedTeamsUsersRoleRequest>(&row)
                            .map_err(crate::api::ApiError::from)?;
                    row.update_or_insert(connection).await?;
                }
            }
            Table::TeamsUsersRoles => {
                for row in rows {
                    let row: super::NestedTeamsUsersRole =
                        bincode::deserialize::<super::NestedTeamsUsersRole>(&row)
                            .map_err(crate::api::ApiError::from)?;
                    row.update_or_insert(connection).await?;
                }
            }
            Table::Units => {
                for row in rows {
                    let row: super::NestedUnit = bincode::deserialize::<super::NestedUnit>(&row)
                        .map_err(crate::api::ApiError::from)?;
                    row.update_or_insert(connection).await?;
                }
            }
            Table::UserEmails => {
                for row in rows {
                    let row: super::NestedUserEmail =
                        bincode::deserialize::<super::NestedUserEmail>(&row)
                            .map_err(crate::api::ApiError::from)?;
                    row.update_or_insert(connection).await?;
                }
            }
            Table::Users => {
                for row in rows {
                    let row: super::NestedUser = bincode::deserialize::<super::NestedUser>(&row)
                        .map_err(crate::api::ApiError::from)?;
                    row.update_or_insert(connection).await?;
                }
            }
            Table::UsersUsersRoleInvitations => {
                for row in rows {
                    let row: super::NestedUsersUsersRoleInvitation =
                        bincode::deserialize::<super::NestedUsersUsersRoleInvitation>(&row)
                            .map_err(crate::api::ApiError::from)?;
                    row.update_or_insert(connection).await?;
                }
            }
            Table::UsersUsersRoleRequests => {
                for row in rows {
                    let row: super::NestedUsersUsersRoleRequest =
                        bincode::deserialize::<super::NestedUsersUsersRoleRequest>(&row)
                            .map_err(crate::api::ApiError::from)?;
                    row.update_or_insert(connection).await?;
                }
            }
            Table::UsersUsersRoles => {
                for row in rows {
                    let row: super::NestedUsersUsersRole =
                        bincode::deserialize::<super::NestedUsersUsersRole>(&row)
                            .map_err(crate::api::ApiError::from)?;
                    row.update_or_insert(connection).await?;
                }
            }
        }
        Ok(())
    }
}
