//! This module contains the table names enumeration.
//!
//! This module is automatically generated. Do not write anything here.

use crate::database::*;

#[derive(
    serde::Deserialize, serde::Serialize, Clone, Debug, PartialEq, Eq, Copy, PartialOrd, Ord,
)]
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
impl crate::database::Table {
    /// Get the BioOttRank from the database by its ID.
    ///
    /// * `primary_key` - The primary key(s) of the struct to check.
    /// * `connection` - The connection to the database.
    pub async fn get<C: gluesql::core::store::GStore + gluesql::core::store::GStoreMut>(
        &self,
        primary_key: PrimaryKey,
        connection: &mut gluesql::prelude::Glue<C>,
    ) -> Result<Option<Vec<u8>>, crate::api::ApiError> {
        Ok(match self {
            crate::database::Table::BioOttRanks => {
                let result = NestedBioOttRank::get(primary_key.into(), connection).await?;
                result
                    .map(|result| bincode::serialize(&result))
                    .transpose()?
            }
            crate::database::Table::BioOttTaxonItems => {
                let result = NestedBioOttTaxonItem::get(primary_key.into(), connection).await?;
                result
                    .map(|result| bincode::serialize(&result))
                    .transpose()?
            }
            crate::database::Table::Colors => {
                let result = Color::get(primary_key.into(), connection).await?;
                result
                    .map(|result| bincode::serialize(&result))
                    .transpose()?
            }
            crate::database::Table::Countries => {
                let result = Country::get(primary_key.into(), connection).await?;
                result
                    .map(|result| bincode::serialize(&result))
                    .transpose()?
            }
            crate::database::Table::DerivedSamples => {
                let result = NestedDerivedSample::get(primary_key.into(), connection).await?;
                result
                    .map(|result| bincode::serialize(&result))
                    .transpose()?
            }
            crate::database::Table::DocumentFormats => {
                let result = NestedDocumentFormat::get(primary_key.into(), connection).await?;
                result
                    .map(|result| bincode::serialize(&result))
                    .transpose()?
            }
            crate::database::Table::FontAwesomeIcons => {
                let result = FontAwesomeIcon::get(primary_key.into(), connection).await?;
                result
                    .map(|result| bincode::serialize(&result))
                    .transpose()?
            }
            crate::database::Table::LoginProviders => {
                let result = NestedLoginProvider::get(primary_key.into(), connection).await?;
                result
                    .map(|result| bincode::serialize(&result))
                    .transpose()?
            }
            crate::database::Table::Materials => {
                let result = NestedMaterial::get(primary_key.into(), connection).await?;
                result
                    .map(|result| bincode::serialize(&result))
                    .transpose()?
            }
            crate::database::Table::NameplateCategories => {
                let result = NestedNameplateCategory::get(primary_key.into(), connection).await?;
                result
                    .map(|result| bincode::serialize(&result))
                    .transpose()?
            }
            crate::database::Table::Nameplates => {
                let result = NestedNameplate::get(primary_key.into(), connection).await?;
                result
                    .map(|result| bincode::serialize(&result))
                    .transpose()?
            }
            crate::database::Table::Notifications => {
                let result = NestedNotification::get(primary_key.into(), connection).await?;
                result
                    .map(|result| bincode::serialize(&result))
                    .transpose()?
            }
            crate::database::Table::ObservationSubjects => {
                let result = NestedObservationSubject::get(primary_key.into(), connection).await?;
                result
                    .map(|result| bincode::serialize(&result))
                    .transpose()?
            }
            crate::database::Table::Observations => {
                let result = NestedObservation::get(primary_key.into(), connection).await?;
                result
                    .map(|result| bincode::serialize(&result))
                    .transpose()?
            }
            crate::database::Table::OrganismBioOttTaxonItems => {
                let result =
                    NestedOrganismBioOttTaxonItem::get(primary_key.into(), connection).await?;
                result
                    .map(|result| bincode::serialize(&result))
                    .transpose()?
            }
            crate::database::Table::Organisms => {
                let result = NestedOrganism::get(primary_key.into(), connection).await?;
                result
                    .map(|result| bincode::serialize(&result))
                    .transpose()?
            }
            crate::database::Table::Organizations => {
                let result = NestedOrganization::get(primary_key.into(), connection).await?;
                result
                    .map(|result| bincode::serialize(&result))
                    .transpose()?
            }
            crate::database::Table::PermanenceCategories => {
                let result = NestedPermanenceCategory::get(primary_key.into(), connection).await?;
                result
                    .map(|result| bincode::serialize(&result))
                    .transpose()?
            }
            crate::database::Table::ProjectStates => {
                let result = NestedProjectState::get(primary_key.into(), connection).await?;
                result
                    .map(|result| bincode::serialize(&result))
                    .transpose()?
            }
            crate::database::Table::Projects => {
                let result = NestedProject::get(primary_key.into(), connection).await?;
                result
                    .map(|result| bincode::serialize(&result))
                    .transpose()?
            }
            crate::database::Table::ProjectsTeamsRoleInvitations => {
                let result =
                    NestedProjectsTeamsRoleInvitation::get(primary_key.into(), connection).await?;
                result
                    .map(|result| bincode::serialize(&result))
                    .transpose()?
            }
            crate::database::Table::ProjectsTeamsRoleRequests => {
                let result =
                    NestedProjectsTeamsRoleRequest::get(primary_key.into(), connection).await?;
                result
                    .map(|result| bincode::serialize(&result))
                    .transpose()?
            }
            crate::database::Table::ProjectsTeamsRoles => {
                let result = NestedProjectsTeamsRole::get(primary_key.into(), connection).await?;
                result
                    .map(|result| bincode::serialize(&result))
                    .transpose()?
            }
            crate::database::Table::ProjectsUsersRoleInvitations => {
                let result =
                    NestedProjectsUsersRoleInvitation::get(primary_key.into(), connection).await?;
                result
                    .map(|result| bincode::serialize(&result))
                    .transpose()?
            }
            crate::database::Table::ProjectsUsersRoleRequests => {
                let result =
                    NestedProjectsUsersRoleRequest::get(primary_key.into(), connection).await?;
                result
                    .map(|result| bincode::serialize(&result))
                    .transpose()?
            }
            crate::database::Table::ProjectsUsersRoles => {
                let result = NestedProjectsUsersRole::get(primary_key.into(), connection).await?;
                result
                    .map(|result| bincode::serialize(&result))
                    .transpose()?
            }
            crate::database::Table::Roles => {
                let result = NestedRole::get(primary_key.into(), connection).await?;
                result
                    .map(|result| bincode::serialize(&result))
                    .transpose()?
            }
            crate::database::Table::SampleBioOttTaxonItems => {
                let result =
                    NestedSampleBioOttTaxonItem::get(primary_key.into(), connection).await?;
                result
                    .map(|result| bincode::serialize(&result))
                    .transpose()?
            }
            crate::database::Table::SampleContainerCategories => {
                let result =
                    NestedSampleContainerCategory::get(primary_key.into(), connection).await?;
                result
                    .map(|result| bincode::serialize(&result))
                    .transpose()?
            }
            crate::database::Table::SampleContainers => {
                let result = NestedSampleContainer::get(primary_key.into(), connection).await?;
                result
                    .map(|result| bincode::serialize(&result))
                    .transpose()?
            }
            crate::database::Table::SampleStates => {
                let result = NestedSampleState::get(primary_key.into(), connection).await?;
                result
                    .map(|result| bincode::serialize(&result))
                    .transpose()?
            }
            crate::database::Table::Samples => {
                let result = NestedSample::get(primary_key.into(), connection).await?;
                result
                    .map(|result| bincode::serialize(&result))
                    .transpose()?
            }
            crate::database::Table::Spectra => {
                let result = NestedSpectra::get(primary_key.into(), connection).await?;
                result
                    .map(|result| bincode::serialize(&result))
                    .transpose()?
            }
            crate::database::Table::SpectraCollections => {
                let result = NestedSpectraCollection::get(primary_key.into(), connection).await?;
                result
                    .map(|result| bincode::serialize(&result))
                    .transpose()?
            }
            crate::database::Table::TeamStates => {
                let result = NestedTeamState::get(primary_key.into(), connection).await?;
                result
                    .map(|result| bincode::serialize(&result))
                    .transpose()?
            }
            crate::database::Table::Teams => {
                let result = NestedTeam::get(primary_key.into(), connection).await?;
                result
                    .map(|result| bincode::serialize(&result))
                    .transpose()?
            }
            crate::database::Table::TeamsTeamsRoleInvitations => {
                let result =
                    NestedTeamsTeamsRoleInvitation::get(primary_key.into(), connection).await?;
                result
                    .map(|result| bincode::serialize(&result))
                    .transpose()?
            }
            crate::database::Table::TeamsUsersRoleInvitations => {
                let result =
                    NestedTeamsUsersRoleInvitation::get(primary_key.into(), connection).await?;
                result
                    .map(|result| bincode::serialize(&result))
                    .transpose()?
            }
            crate::database::Table::TeamsUsersRoleRequests => {
                let result =
                    NestedTeamsUsersRoleRequest::get(primary_key.into(), connection).await?;
                result
                    .map(|result| bincode::serialize(&result))
                    .transpose()?
            }
            crate::database::Table::TeamsUsersRoles => {
                let result = NestedTeamsUsersRole::get(primary_key.into(), connection).await?;
                result
                    .map(|result| bincode::serialize(&result))
                    .transpose()?
            }
            crate::database::Table::Units => {
                let result = NestedUnit::get(primary_key.into(), connection).await?;
                result
                    .map(|result| bincode::serialize(&result))
                    .transpose()?
            }
            crate::database::Table::UserEmails => {
                let result = NestedUserEmail::get(primary_key.into(), connection).await?;
                result
                    .map(|result| bincode::serialize(&result))
                    .transpose()?
            }
            crate::database::Table::Users => {
                let result = NestedUser::get(primary_key.into(), connection).await?;
                result
                    .map(|result| bincode::serialize(&result))
                    .transpose()?
            }
            crate::database::Table::UsersUsersRoleInvitations => {
                let result =
                    NestedUsersUsersRoleInvitation::get(primary_key.into(), connection).await?;
                result
                    .map(|result| bincode::serialize(&result))
                    .transpose()?
            }
            crate::database::Table::UsersUsersRoleRequests => {
                let result =
                    NestedUsersUsersRoleRequest::get(primary_key.into(), connection).await?;
                result
                    .map(|result| bincode::serialize(&result))
                    .transpose()?
            }
            crate::database::Table::UsersUsersRoles => {
                let result = NestedUsersUsersRole::get(primary_key.into(), connection).await?;
                result
                    .map(|result| bincode::serialize(&result))
                    .transpose()?
            }
        })
    }

    /// Delete the BioOttRank from the database.
    ///
    /// * `row` - Row to be processed
    /// * `connection` - The connection to the database.
    pub async fn delete<C: gluesql::core::store::GStore + gluesql::core::store::GStoreMut>(
        &self,
        row: Vec<u8>,
        connection: &mut gluesql::prelude::Glue<C>,
    ) -> Result<usize, crate::api::ApiError> {
        Ok(match self {
            crate::database::Table::BioOttRanks => {
                NestedBioOttRank::delete(
                    bincode::deserialize::<crate::database::nested_variants::NestedBioOttRank>(
                        &row,
                    )?,
                    connection,
                )
                .await?
            }
            crate::database::Table::BioOttTaxonItems => {
                NestedBioOttTaxonItem::delete(
                    bincode::deserialize::<crate::database::nested_variants::NestedBioOttTaxonItem>(
                        &row,
                    )?,
                    connection,
                )
                .await?
            }
            crate::database::Table::Colors => {
                Color::delete(
                    bincode::deserialize::<crate::database::flat_variants::Color>(&row)?,
                    connection,
                )
                .await?
            }
            crate::database::Table::Countries => {
                Country::delete(
                    bincode::deserialize::<crate::database::flat_variants::Country>(&row)?,
                    connection,
                )
                .await?
            }
            crate::database::Table::DerivedSamples => {
                NestedDerivedSample::delete(
                    bincode::deserialize::<crate::database::nested_variants::NestedDerivedSample>(
                        &row,
                    )?,
                    connection,
                )
                .await?
            }
            crate::database::Table::DocumentFormats => {
                NestedDocumentFormat::delete(
                    bincode::deserialize::<crate::database::nested_variants::NestedDocumentFormat>(
                        &row,
                    )?,
                    connection,
                )
                .await?
            }
            crate::database::Table::FontAwesomeIcons => {
                FontAwesomeIcon::delete(
                    bincode::deserialize::<crate::database::flat_variants::FontAwesomeIcon>(&row)?,
                    connection,
                )
                .await?
            }
            crate::database::Table::LoginProviders => {
                NestedLoginProvider::delete(
                    bincode::deserialize::<crate::database::nested_variants::NestedLoginProvider>(
                        &row,
                    )?,
                    connection,
                )
                .await?
            }
            crate::database::Table::Materials => {
                NestedMaterial::delete(
                    bincode::deserialize::<crate::database::nested_variants::NestedMaterial>(&row)?,
                    connection,
                )
                .await?
            }
            crate::database::Table::NameplateCategories => {
                NestedNameplateCategory::delete(
                    bincode::deserialize::<
                        crate::database::nested_variants::NestedNameplateCategory,
                    >(&row)?,
                    connection,
                )
                .await?
            }
            crate::database::Table::Nameplates => {
                NestedNameplate::delete(
                    bincode::deserialize::<crate::database::nested_variants::NestedNameplate>(
                        &row,
                    )?,
                    connection,
                )
                .await?
            }
            crate::database::Table::Notifications => {
                NestedNotification::delete(
                    bincode::deserialize::<crate::database::nested_variants::NestedNotification>(
                        &row,
                    )?,
                    connection,
                )
                .await?
            }
            crate::database::Table::ObservationSubjects => {
                NestedObservationSubject::delete(
                    bincode::deserialize::<
                        crate::database::nested_variants::NestedObservationSubject,
                    >(&row)?,
                    connection,
                )
                .await?
            }
            crate::database::Table::Observations => {
                NestedObservation::delete(
                    bincode::deserialize::<crate::database::nested_variants::NestedObservation>(
                        &row,
                    )?,
                    connection,
                )
                .await?
            }
            crate::database::Table::OrganismBioOttTaxonItems => {
                NestedOrganismBioOttTaxonItem::delete(
                    bincode::deserialize::<
                        crate::database::nested_variants::NestedOrganismBioOttTaxonItem,
                    >(&row)?,
                    connection,
                )
                .await?
            }
            crate::database::Table::Organisms => {
                NestedOrganism::delete(
                    bincode::deserialize::<crate::database::nested_variants::NestedOrganism>(&row)?,
                    connection,
                )
                .await?
            }
            crate::database::Table::Organizations => {
                NestedOrganization::delete(
                    bincode::deserialize::<crate::database::nested_variants::NestedOrganization>(
                        &row,
                    )?,
                    connection,
                )
                .await?
            }
            crate::database::Table::PermanenceCategories => {
                NestedPermanenceCategory::delete(
                    bincode::deserialize::<
                        crate::database::nested_variants::NestedPermanenceCategory,
                    >(&row)?,
                    connection,
                )
                .await?
            }
            crate::database::Table::ProjectStates => {
                NestedProjectState::delete(
                    bincode::deserialize::<crate::database::nested_variants::NestedProjectState>(
                        &row,
                    )?,
                    connection,
                )
                .await?
            }
            crate::database::Table::Projects => {
                NestedProject::delete(
                    bincode::deserialize::<crate::database::nested_variants::NestedProject>(&row)?,
                    connection,
                )
                .await?
            }
            crate::database::Table::ProjectsTeamsRoleInvitations => {
                NestedProjectsTeamsRoleInvitation::delete(
                    bincode::deserialize::<
                        crate::database::nested_variants::NestedProjectsTeamsRoleInvitation,
                    >(&row)?,
                    connection,
                )
                .await?
            }
            crate::database::Table::ProjectsTeamsRoleRequests => {
                NestedProjectsTeamsRoleRequest::delete(
                    bincode::deserialize::<
                        crate::database::nested_variants::NestedProjectsTeamsRoleRequest,
                    >(&row)?,
                    connection,
                )
                .await?
            }
            crate::database::Table::ProjectsTeamsRoles => {
                NestedProjectsTeamsRole::delete(
                    bincode::deserialize::<
                        crate::database::nested_variants::NestedProjectsTeamsRole,
                    >(&row)?,
                    connection,
                )
                .await?
            }
            crate::database::Table::ProjectsUsersRoleInvitations => {
                NestedProjectsUsersRoleInvitation::delete(
                    bincode::deserialize::<
                        crate::database::nested_variants::NestedProjectsUsersRoleInvitation,
                    >(&row)?,
                    connection,
                )
                .await?
            }
            crate::database::Table::ProjectsUsersRoleRequests => {
                NestedProjectsUsersRoleRequest::delete(
                    bincode::deserialize::<
                        crate::database::nested_variants::NestedProjectsUsersRoleRequest,
                    >(&row)?,
                    connection,
                )
                .await?
            }
            crate::database::Table::ProjectsUsersRoles => {
                NestedProjectsUsersRole::delete(
                    bincode::deserialize::<
                        crate::database::nested_variants::NestedProjectsUsersRole,
                    >(&row)?,
                    connection,
                )
                .await?
            }
            crate::database::Table::Roles => {
                NestedRole::delete(
                    bincode::deserialize::<crate::database::nested_variants::NestedRole>(&row)?,
                    connection,
                )
                .await?
            }
            crate::database::Table::SampleBioOttTaxonItems => {
                NestedSampleBioOttTaxonItem::delete(
                    bincode::deserialize::<
                        crate::database::nested_variants::NestedSampleBioOttTaxonItem,
                    >(&row)?,
                    connection,
                )
                .await?
            }
            crate::database::Table::SampleContainerCategories => {
                NestedSampleContainerCategory::delete(
                    bincode::deserialize::<
                        crate::database::nested_variants::NestedSampleContainerCategory,
                    >(&row)?,
                    connection,
                )
                .await?
            }
            crate::database::Table::SampleContainers => {
                NestedSampleContainer::delete(
                    bincode::deserialize::<crate::database::nested_variants::NestedSampleContainer>(
                        &row,
                    )?,
                    connection,
                )
                .await?
            }
            crate::database::Table::SampleStates => {
                NestedSampleState::delete(
                    bincode::deserialize::<crate::database::nested_variants::NestedSampleState>(
                        &row,
                    )?,
                    connection,
                )
                .await?
            }
            crate::database::Table::Samples => {
                NestedSample::delete(
                    bincode::deserialize::<crate::database::nested_variants::NestedSample>(&row)?,
                    connection,
                )
                .await?
            }
            crate::database::Table::Spectra => {
                NestedSpectra::delete(
                    bincode::deserialize::<crate::database::nested_variants::NestedSpectra>(&row)?,
                    connection,
                )
                .await?
            }
            crate::database::Table::SpectraCollections => {
                NestedSpectraCollection::delete(
                    bincode::deserialize::<
                        crate::database::nested_variants::NestedSpectraCollection,
                    >(&row)?,
                    connection,
                )
                .await?
            }
            crate::database::Table::TeamStates => {
                NestedTeamState::delete(
                    bincode::deserialize::<crate::database::nested_variants::NestedTeamState>(
                        &row,
                    )?,
                    connection,
                )
                .await?
            }
            crate::database::Table::Teams => {
                NestedTeam::delete(
                    bincode::deserialize::<crate::database::nested_variants::NestedTeam>(&row)?,
                    connection,
                )
                .await?
            }
            crate::database::Table::TeamsTeamsRoleInvitations => {
                NestedTeamsTeamsRoleInvitation::delete(
                    bincode::deserialize::<
                        crate::database::nested_variants::NestedTeamsTeamsRoleInvitation,
                    >(&row)?,
                    connection,
                )
                .await?
            }
            crate::database::Table::TeamsUsersRoleInvitations => {
                NestedTeamsUsersRoleInvitation::delete(
                    bincode::deserialize::<
                        crate::database::nested_variants::NestedTeamsUsersRoleInvitation,
                    >(&row)?,
                    connection,
                )
                .await?
            }
            crate::database::Table::TeamsUsersRoleRequests => {
                NestedTeamsUsersRoleRequest::delete(
                    bincode::deserialize::<
                        crate::database::nested_variants::NestedTeamsUsersRoleRequest,
                    >(&row)?,
                    connection,
                )
                .await?
            }
            crate::database::Table::TeamsUsersRoles => {
                NestedTeamsUsersRole::delete(
                    bincode::deserialize::<crate::database::nested_variants::NestedTeamsUsersRole>(
                        &row,
                    )?,
                    connection,
                )
                .await?
            }
            crate::database::Table::Units => {
                NestedUnit::delete(
                    bincode::deserialize::<crate::database::nested_variants::NestedUnit>(&row)?,
                    connection,
                )
                .await?
            }
            crate::database::Table::UserEmails => {
                NestedUserEmail::delete(
                    bincode::deserialize::<crate::database::nested_variants::NestedUserEmail>(
                        &row,
                    )?,
                    connection,
                )
                .await?
            }
            crate::database::Table::Users => {
                NestedUser::delete(
                    bincode::deserialize::<crate::database::nested_variants::NestedUser>(&row)?,
                    connection,
                )
                .await?
            }
            crate::database::Table::UsersUsersRoleInvitations => {
                NestedUsersUsersRoleInvitation::delete(
                    bincode::deserialize::<
                        crate::database::nested_variants::NestedUsersUsersRoleInvitation,
                    >(&row)?,
                    connection,
                )
                .await?
            }
            crate::database::Table::UsersUsersRoleRequests => {
                NestedUsersUsersRoleRequest::delete(
                    bincode::deserialize::<
                        crate::database::nested_variants::NestedUsersUsersRoleRequest,
                    >(&row)?,
                    connection,
                )
                .await?
            }
            crate::database::Table::UsersUsersRoles => {
                NestedUsersUsersRole::delete(
                    bincode::deserialize::<crate::database::nested_variants::NestedUsersUsersRole>(
                        &row,
                    )?,
                    connection,
                )
                .await?
            }
        })
    }

    /// Delete the BioOttRank from the database by its ID.
    ///
    /// * `primary_key` - The primary key(s) of the struct to delete.
    /// * `connection` - The connection to the database.
    pub async fn delete_from_id<
        C: gluesql::core::store::GStore + gluesql::core::store::GStoreMut,
    >(
        &self,
        primary_key: PrimaryKey,
        connection: &mut gluesql::prelude::Glue<C>,
    ) -> Result<usize, crate::api::ApiError> {
        Ok(match self {
            crate::database::Table::BioOttRanks => {
                NestedBioOttRank::delete_from_id(primary_key.into(), connection).await?
            }
            crate::database::Table::BioOttTaxonItems => {
                NestedBioOttTaxonItem::delete_from_id(primary_key.into(), connection).await?
            }
            crate::database::Table::Colors => {
                Color::delete_from_id(primary_key.into(), connection).await?
            }
            crate::database::Table::Countries => {
                Country::delete_from_id(primary_key.into(), connection).await?
            }
            crate::database::Table::DerivedSamples => {
                NestedDerivedSample::delete_from_id(primary_key.into(), connection).await?
            }
            crate::database::Table::DocumentFormats => {
                NestedDocumentFormat::delete_from_id(primary_key.into(), connection).await?
            }
            crate::database::Table::FontAwesomeIcons => {
                FontAwesomeIcon::delete_from_id(primary_key.into(), connection).await?
            }
            crate::database::Table::LoginProviders => {
                NestedLoginProvider::delete_from_id(primary_key.into(), connection).await?
            }
            crate::database::Table::Materials => {
                NestedMaterial::delete_from_id(primary_key.into(), connection).await?
            }
            crate::database::Table::NameplateCategories => {
                NestedNameplateCategory::delete_from_id(primary_key.into(), connection).await?
            }
            crate::database::Table::Nameplates => {
                NestedNameplate::delete_from_id(primary_key.into(), connection).await?
            }
            crate::database::Table::Notifications => {
                NestedNotification::delete_from_id(primary_key.into(), connection).await?
            }
            crate::database::Table::ObservationSubjects => {
                NestedObservationSubject::delete_from_id(primary_key.into(), connection).await?
            }
            crate::database::Table::Observations => {
                NestedObservation::delete_from_id(primary_key.into(), connection).await?
            }
            crate::database::Table::OrganismBioOttTaxonItems => {
                NestedOrganismBioOttTaxonItem::delete_from_id(primary_key.into(), connection)
                    .await?
            }
            crate::database::Table::Organisms => {
                NestedOrganism::delete_from_id(primary_key.into(), connection).await?
            }
            crate::database::Table::Organizations => {
                NestedOrganization::delete_from_id(primary_key.into(), connection).await?
            }
            crate::database::Table::PermanenceCategories => {
                NestedPermanenceCategory::delete_from_id(primary_key.into(), connection).await?
            }
            crate::database::Table::ProjectStates => {
                NestedProjectState::delete_from_id(primary_key.into(), connection).await?
            }
            crate::database::Table::Projects => {
                NestedProject::delete_from_id(primary_key.into(), connection).await?
            }
            crate::database::Table::ProjectsTeamsRoleInvitations => {
                NestedProjectsTeamsRoleInvitation::delete_from_id(primary_key.into(), connection)
                    .await?
            }
            crate::database::Table::ProjectsTeamsRoleRequests => {
                NestedProjectsTeamsRoleRequest::delete_from_id(primary_key.into(), connection)
                    .await?
            }
            crate::database::Table::ProjectsTeamsRoles => {
                NestedProjectsTeamsRole::delete_from_id(primary_key.into(), connection).await?
            }
            crate::database::Table::ProjectsUsersRoleInvitations => {
                NestedProjectsUsersRoleInvitation::delete_from_id(primary_key.into(), connection)
                    .await?
            }
            crate::database::Table::ProjectsUsersRoleRequests => {
                NestedProjectsUsersRoleRequest::delete_from_id(primary_key.into(), connection)
                    .await?
            }
            crate::database::Table::ProjectsUsersRoles => {
                NestedProjectsUsersRole::delete_from_id(primary_key.into(), connection).await?
            }
            crate::database::Table::Roles => {
                NestedRole::delete_from_id(primary_key.into(), connection).await?
            }
            crate::database::Table::SampleBioOttTaxonItems => {
                NestedSampleBioOttTaxonItem::delete_from_id(primary_key.into(), connection).await?
            }
            crate::database::Table::SampleContainerCategories => {
                NestedSampleContainerCategory::delete_from_id(primary_key.into(), connection)
                    .await?
            }
            crate::database::Table::SampleContainers => {
                NestedSampleContainer::delete_from_id(primary_key.into(), connection).await?
            }
            crate::database::Table::SampleStates => {
                NestedSampleState::delete_from_id(primary_key.into(), connection).await?
            }
            crate::database::Table::Samples => {
                NestedSample::delete_from_id(primary_key.into(), connection).await?
            }
            crate::database::Table::Spectra => {
                NestedSpectra::delete_from_id(primary_key.into(), connection).await?
            }
            crate::database::Table::SpectraCollections => {
                NestedSpectraCollection::delete_from_id(primary_key.into(), connection).await?
            }
            crate::database::Table::TeamStates => {
                NestedTeamState::delete_from_id(primary_key.into(), connection).await?
            }
            crate::database::Table::Teams => {
                NestedTeam::delete_from_id(primary_key.into(), connection).await?
            }
            crate::database::Table::TeamsTeamsRoleInvitations => {
                NestedTeamsTeamsRoleInvitation::delete_from_id(primary_key.into(), connection)
                    .await?
            }
            crate::database::Table::TeamsUsersRoleInvitations => {
                NestedTeamsUsersRoleInvitation::delete_from_id(primary_key.into(), connection)
                    .await?
            }
            crate::database::Table::TeamsUsersRoleRequests => {
                NestedTeamsUsersRoleRequest::delete_from_id(primary_key.into(), connection).await?
            }
            crate::database::Table::TeamsUsersRoles => {
                NestedTeamsUsersRole::delete_from_id(primary_key.into(), connection).await?
            }
            crate::database::Table::Units => {
                NestedUnit::delete_from_id(primary_key.into(), connection).await?
            }
            crate::database::Table::UserEmails => {
                NestedUserEmail::delete_from_id(primary_key.into(), connection).await?
            }
            crate::database::Table::Users => {
                NestedUser::delete_from_id(primary_key.into(), connection).await?
            }
            crate::database::Table::UsersUsersRoleInvitations => {
                NestedUsersUsersRoleInvitation::delete_from_id(primary_key.into(), connection)
                    .await?
            }
            crate::database::Table::UsersUsersRoleRequests => {
                NestedUsersUsersRoleRequest::delete_from_id(primary_key.into(), connection).await?
            }
            crate::database::Table::UsersUsersRoles => {
                NestedUsersUsersRole::delete_from_id(primary_key.into(), connection).await?
            }
        })
    }

    /// Get all BioOttRank from the database.
    ///
    /// * `filter` - The filter to apply to the results.
    /// * `limit` - The maximum number of results, by default `10`.
    /// * `offset` - The offset of the results, by default `0`.
    /// * `connection` - The connection to the database.
    pub async fn all<C: gluesql::core::store::GStore + gluesql::core::store::GStoreMut>(
        &self,
        filter: Option<Vec<u8>>,
        limit: Option<i64>,
        offset: Option<i64>,
        connection: &mut gluesql::prelude::Glue<C>,
    ) -> Result<Vec<u8>, crate::api::ApiError> {
        Ok(match self {
            crate::database::Table::BioOttRanks => {
                let result =
                    NestedBioOttRank::all(
                        filter
                            .map(|filter| {
                                bincode::deserialize::<
                                    crate::database::filter_variants::BioOttRankFilter,
                                >(&filter)
                            })
                            .transpose()?
                            .as_ref(),
                        limit,
                        offset,
                        connection,
                    )
                    .await?;
                bincode::serialize(&result)?
            }
            crate::database::Table::BioOttTaxonItems => {
                let result = NestedBioOttTaxonItem::all(
                    filter
                        .map(|filter| {
                            bincode::deserialize::<
                                crate::database::filter_variants::BioOttTaxonItemFilter,
                            >(&filter)
                        })
                        .transpose()?
                        .as_ref(),
                    limit,
                    offset,
                    connection,
                )
                .await?;
                bincode::serialize(&result)?
            }
            crate::database::Table::Colors => {
                let result = Color::all(limit, offset, connection).await?;
                bincode::serialize(&result)?
            }
            crate::database::Table::Countries => {
                let result = Country::all(limit, offset, connection).await?;
                bincode::serialize(&result)?
            }
            crate::database::Table::DerivedSamples => {
                let result = NestedDerivedSample::all(
                    filter
                        .map(|filter| {
                            bincode::deserialize::<
                                crate::database::filter_variants::DerivedSampleFilter,
                            >(&filter)
                        })
                        .transpose()?
                        .as_ref(),
                    limit,
                    offset,
                    connection,
                )
                .await?;
                bincode::serialize(&result)?
            }
            crate::database::Table::DocumentFormats => {
                let result = NestedDocumentFormat::all(
                    filter
                        .map(|filter| {
                            bincode::deserialize::<
                                crate::database::filter_variants::DocumentFormatFilter,
                            >(&filter)
                        })
                        .transpose()?
                        .as_ref(),
                    limit,
                    offset,
                    connection,
                )
                .await?;
                bincode::serialize(&result)?
            }
            crate::database::Table::FontAwesomeIcons => {
                let result = FontAwesomeIcon::all(limit, offset, connection).await?;
                bincode::serialize(&result)?
            }
            crate::database::Table::LoginProviders => {
                let result = NestedLoginProvider::all(
                    filter
                        .map(|filter| {
                            bincode::deserialize::<
                                crate::database::filter_variants::LoginProviderFilter,
                            >(&filter)
                        })
                        .transpose()?
                        .as_ref(),
                    limit,
                    offset,
                    connection,
                )
                .await?;
                bincode::serialize(&result)?
            }
            crate::database::Table::Materials => {
                let result =
                    NestedMaterial::all(
                        filter
                            .map(|filter| {
                                bincode::deserialize::<
                                    crate::database::filter_variants::MaterialFilter,
                                >(&filter)
                            })
                            .transpose()?
                            .as_ref(),
                        limit,
                        offset,
                        connection,
                    )
                    .await?;
                bincode::serialize(&result)?
            }
            crate::database::Table::NameplateCategories => {
                let result = NestedNameplateCategory::all(
                    filter
                        .map(|filter| {
                            bincode::deserialize::<
                                crate::database::filter_variants::NameplateCategoryFilter,
                            >(&filter)
                        })
                        .transpose()?
                        .as_ref(),
                    limit,
                    offset,
                    connection,
                )
                .await?;
                bincode::serialize(&result)?
            }
            crate::database::Table::Nameplates => {
                let result =
                    NestedNameplate::all(
                        filter
                            .map(|filter| {
                                bincode::deserialize::<
                                    crate::database::filter_variants::NameplateFilter,
                                >(&filter)
                            })
                            .transpose()?
                            .as_ref(),
                        limit,
                        offset,
                        connection,
                    )
                    .await?;
                bincode::serialize(&result)?
            }
            crate::database::Table::Notifications => {
                let result = NestedNotification::all(
                    filter
                        .map(|filter| {
                            bincode::deserialize::<
                                crate::database::filter_variants::NotificationFilter,
                            >(&filter)
                        })
                        .transpose()?
                        .as_ref(),
                    limit,
                    offset,
                    connection,
                )
                .await?;
                bincode::serialize(&result)?
            }
            crate::database::Table::ObservationSubjects => {
                let result = NestedObservationSubject::all(
                    filter
                        .map(|filter| {
                            bincode::deserialize::<
                                crate::database::filter_variants::ObservationSubjectFilter,
                            >(&filter)
                        })
                        .transpose()?
                        .as_ref(),
                    limit,
                    offset,
                    connection,
                )
                .await?;
                bincode::serialize(&result)?
            }
            crate::database::Table::Observations => {
                let result = NestedObservation::all(
                    filter
                        .map(|filter| {
                            bincode::deserialize::<
                                crate::database::filter_variants::ObservationFilter,
                            >(&filter)
                        })
                        .transpose()?
                        .as_ref(),
                    limit,
                    offset,
                    connection,
                )
                .await?;
                bincode::serialize(&result)?
            }
            crate::database::Table::OrganismBioOttTaxonItems => {
                let result = NestedOrganismBioOttTaxonItem::all(
                    filter
                        .map(|filter| {
                            bincode::deserialize::<
                                crate::database::filter_variants::OrganismBioOttTaxonItemFilter,
                            >(&filter)
                        })
                        .transpose()?
                        .as_ref(),
                    limit,
                    offset,
                    connection,
                )
                .await?;
                bincode::serialize(&result)?
            }
            crate::database::Table::Organisms => {
                let result =
                    NestedOrganism::all(
                        filter
                            .map(|filter| {
                                bincode::deserialize::<
                                    crate::database::filter_variants::OrganismFilter,
                                >(&filter)
                            })
                            .transpose()?
                            .as_ref(),
                        limit,
                        offset,
                        connection,
                    )
                    .await?;
                bincode::serialize(&result)?
            }
            crate::database::Table::Organizations => {
                let result = NestedOrganization::all(
                    filter
                        .map(|filter| {
                            bincode::deserialize::<
                                crate::database::filter_variants::OrganizationFilter,
                            >(&filter)
                        })
                        .transpose()?
                        .as_ref(),
                    limit,
                    offset,
                    connection,
                )
                .await?;
                bincode::serialize(&result)?
            }
            crate::database::Table::PermanenceCategories => {
                let result = NestedPermanenceCategory::all(
                    filter
                        .map(|filter| {
                            bincode::deserialize::<
                                crate::database::filter_variants::PermanenceCategoryFilter,
                            >(&filter)
                        })
                        .transpose()?
                        .as_ref(),
                    limit,
                    offset,
                    connection,
                )
                .await?;
                bincode::serialize(&result)?
            }
            crate::database::Table::ProjectStates => {
                let result = NestedProjectState::all(
                    filter
                        .map(|filter| {
                            bincode::deserialize::<
                                crate::database::filter_variants::ProjectStateFilter,
                            >(&filter)
                        })
                        .transpose()?
                        .as_ref(),
                    limit,
                    offset,
                    connection,
                )
                .await?;
                bincode::serialize(&result)?
            }
            crate::database::Table::Projects => {
                let result = NestedProject::all(
                    filter
                        .map(|filter| {
                            bincode::deserialize::<crate::database::filter_variants::ProjectFilter>(
                                &filter,
                            )
                        })
                        .transpose()?
                        .as_ref(),
                    limit,
                    offset,
                    connection,
                )
                .await?;
                bincode::serialize(&result)?
            }
            crate::database::Table::ProjectsTeamsRoleInvitations => {
                let result = NestedProjectsTeamsRoleInvitation::all(
                    filter
                        .map(|filter| {
                            bincode::deserialize::<
                                crate::database::filter_variants::ProjectsTeamsRoleInvitationFilter,
                            >(&filter)
                        })
                        .transpose()?
                        .as_ref(),
                    limit,
                    offset,
                    connection,
                )
                .await?;
                bincode::serialize(&result)?
            }
            crate::database::Table::ProjectsTeamsRoleRequests => {
                let result = NestedProjectsTeamsRoleRequest::all(
                    filter
                        .map(|filter| {
                            bincode::deserialize::<
                                crate::database::filter_variants::ProjectsTeamsRoleRequestFilter,
                            >(&filter)
                        })
                        .transpose()?
                        .as_ref(),
                    limit,
                    offset,
                    connection,
                )
                .await?;
                bincode::serialize(&result)?
            }
            crate::database::Table::ProjectsTeamsRoles => {
                let result = NestedProjectsTeamsRole::all(
                    filter
                        .map(|filter| {
                            bincode::deserialize::<
                                crate::database::filter_variants::ProjectsTeamsRoleFilter,
                            >(&filter)
                        })
                        .transpose()?
                        .as_ref(),
                    limit,
                    offset,
                    connection,
                )
                .await?;
                bincode::serialize(&result)?
            }
            crate::database::Table::ProjectsUsersRoleInvitations => {
                let result = NestedProjectsUsersRoleInvitation::all(
                    filter
                        .map(|filter| {
                            bincode::deserialize::<
                                crate::database::filter_variants::ProjectsUsersRoleInvitationFilter,
                            >(&filter)
                        })
                        .transpose()?
                        .as_ref(),
                    limit,
                    offset,
                    connection,
                )
                .await?;
                bincode::serialize(&result)?
            }
            crate::database::Table::ProjectsUsersRoleRequests => {
                let result = NestedProjectsUsersRoleRequest::all(
                    filter
                        .map(|filter| {
                            bincode::deserialize::<
                                crate::database::filter_variants::ProjectsUsersRoleRequestFilter,
                            >(&filter)
                        })
                        .transpose()?
                        .as_ref(),
                    limit,
                    offset,
                    connection,
                )
                .await?;
                bincode::serialize(&result)?
            }
            crate::database::Table::ProjectsUsersRoles => {
                let result = NestedProjectsUsersRole::all(
                    filter
                        .map(|filter| {
                            bincode::deserialize::<
                                crate::database::filter_variants::ProjectsUsersRoleFilter,
                            >(&filter)
                        })
                        .transpose()?
                        .as_ref(),
                    limit,
                    offset,
                    connection,
                )
                .await?;
                bincode::serialize(&result)?
            }
            crate::database::Table::Roles => {
                let result = NestedRole::all(
                    filter
                        .map(|filter| {
                            bincode::deserialize::<crate::database::filter_variants::RoleFilter>(
                                &filter,
                            )
                        })
                        .transpose()?
                        .as_ref(),
                    limit,
                    offset,
                    connection,
                )
                .await?;
                bincode::serialize(&result)?
            }
            crate::database::Table::SampleBioOttTaxonItems => {
                let result = NestedSampleBioOttTaxonItem::all(
                    filter
                        .map(|filter| {
                            bincode::deserialize::<
                                crate::database::filter_variants::SampleBioOttTaxonItemFilter,
                            >(&filter)
                        })
                        .transpose()?
                        .as_ref(),
                    limit,
                    offset,
                    connection,
                )
                .await?;
                bincode::serialize(&result)?
            }
            crate::database::Table::SampleContainerCategories => {
                let result = NestedSampleContainerCategory::all(
                    filter
                        .map(|filter| {
                            bincode::deserialize::<
                                crate::database::filter_variants::SampleContainerCategoryFilter,
                            >(&filter)
                        })
                        .transpose()?
                        .as_ref(),
                    limit,
                    offset,
                    connection,
                )
                .await?;
                bincode::serialize(&result)?
            }
            crate::database::Table::SampleContainers => {
                let result = NestedSampleContainer::all(
                    filter
                        .map(|filter| {
                            bincode::deserialize::<
                                crate::database::filter_variants::SampleContainerFilter,
                            >(&filter)
                        })
                        .transpose()?
                        .as_ref(),
                    limit,
                    offset,
                    connection,
                )
                .await?;
                bincode::serialize(&result)?
            }
            crate::database::Table::SampleStates => {
                let result = NestedSampleState::all(
                    filter
                        .map(|filter| {
                            bincode::deserialize::<
                                crate::database::filter_variants::SampleStateFilter,
                            >(&filter)
                        })
                        .transpose()?
                        .as_ref(),
                    limit,
                    offset,
                    connection,
                )
                .await?;
                bincode::serialize(&result)?
            }
            crate::database::Table::Samples => {
                let result = NestedSample::all(
                    filter
                        .map(|filter| {
                            bincode::deserialize::<crate::database::filter_variants::SampleFilter>(
                                &filter,
                            )
                        })
                        .transpose()?
                        .as_ref(),
                    limit,
                    offset,
                    connection,
                )
                .await?;
                bincode::serialize(&result)?
            }
            crate::database::Table::Spectra => {
                let result = NestedSpectra::all(
                    filter
                        .map(|filter| {
                            bincode::deserialize::<crate::database::filter_variants::SpectraFilter>(
                                &filter,
                            )
                        })
                        .transpose()?
                        .as_ref(),
                    limit,
                    offset,
                    connection,
                )
                .await?;
                bincode::serialize(&result)?
            }
            crate::database::Table::SpectraCollections => {
                let result = NestedSpectraCollection::all(
                    filter
                        .map(|filter| {
                            bincode::deserialize::<
                                crate::database::filter_variants::SpectraCollectionFilter,
                            >(&filter)
                        })
                        .transpose()?
                        .as_ref(),
                    limit,
                    offset,
                    connection,
                )
                .await?;
                bincode::serialize(&result)?
            }
            crate::database::Table::TeamStates => {
                let result =
                    NestedTeamState::all(
                        filter
                            .map(|filter| {
                                bincode::deserialize::<
                                    crate::database::filter_variants::TeamStateFilter,
                                >(&filter)
                            })
                            .transpose()?
                            .as_ref(),
                        limit,
                        offset,
                        connection,
                    )
                    .await?;
                bincode::serialize(&result)?
            }
            crate::database::Table::Teams => {
                let result = NestedTeam::all(
                    filter
                        .map(|filter| {
                            bincode::deserialize::<crate::database::filter_variants::TeamFilter>(
                                &filter,
                            )
                        })
                        .transpose()?
                        .as_ref(),
                    limit,
                    offset,
                    connection,
                )
                .await?;
                bincode::serialize(&result)?
            }
            crate::database::Table::TeamsTeamsRoleInvitations => {
                let result = NestedTeamsTeamsRoleInvitation::all(
                    filter
                        .map(|filter| {
                            bincode::deserialize::<
                                crate::database::filter_variants::TeamsTeamsRoleInvitationFilter,
                            >(&filter)
                        })
                        .transpose()?
                        .as_ref(),
                    limit,
                    offset,
                    connection,
                )
                .await?;
                bincode::serialize(&result)?
            }
            crate::database::Table::TeamsUsersRoleInvitations => {
                let result = NestedTeamsUsersRoleInvitation::all(
                    filter
                        .map(|filter| {
                            bincode::deserialize::<
                                crate::database::filter_variants::TeamsUsersRoleInvitationFilter,
                            >(&filter)
                        })
                        .transpose()?
                        .as_ref(),
                    limit,
                    offset,
                    connection,
                )
                .await?;
                bincode::serialize(&result)?
            }
            crate::database::Table::TeamsUsersRoleRequests => {
                let result = NestedTeamsUsersRoleRequest::all(
                    filter
                        .map(|filter| {
                            bincode::deserialize::<
                                crate::database::filter_variants::TeamsUsersRoleRequestFilter,
                            >(&filter)
                        })
                        .transpose()?
                        .as_ref(),
                    limit,
                    offset,
                    connection,
                )
                .await?;
                bincode::serialize(&result)?
            }
            crate::database::Table::TeamsUsersRoles => {
                let result = NestedTeamsUsersRole::all(
                    filter
                        .map(|filter| {
                            bincode::deserialize::<
                                crate::database::filter_variants::TeamsUsersRoleFilter,
                            >(&filter)
                        })
                        .transpose()?
                        .as_ref(),
                    limit,
                    offset,
                    connection,
                )
                .await?;
                bincode::serialize(&result)?
            }
            crate::database::Table::Units => {
                let result = NestedUnit::all(
                    filter
                        .map(|filter| {
                            bincode::deserialize::<crate::database::filter_variants::UnitFilter>(
                                &filter,
                            )
                        })
                        .transpose()?
                        .as_ref(),
                    limit,
                    offset,
                    connection,
                )
                .await?;
                bincode::serialize(&result)?
            }
            crate::database::Table::UserEmails => {
                let result =
                    NestedUserEmail::all(
                        filter
                            .map(|filter| {
                                bincode::deserialize::<
                                    crate::database::filter_variants::UserEmailFilter,
                                >(&filter)
                            })
                            .transpose()?
                            .as_ref(),
                        limit,
                        offset,
                        connection,
                    )
                    .await?;
                bincode::serialize(&result)?
            }
            crate::database::Table::Users => {
                let result = NestedUser::all(
                    filter
                        .map(|filter| {
                            bincode::deserialize::<crate::database::filter_variants::UserFilter>(
                                &filter,
                            )
                        })
                        .transpose()?
                        .as_ref(),
                    limit,
                    offset,
                    connection,
                )
                .await?;
                bincode::serialize(&result)?
            }
            crate::database::Table::UsersUsersRoleInvitations => {
                let result = NestedUsersUsersRoleInvitation::all(
                    filter
                        .map(|filter| {
                            bincode::deserialize::<
                                crate::database::filter_variants::UsersUsersRoleInvitationFilter,
                            >(&filter)
                        })
                        .transpose()?
                        .as_ref(),
                    limit,
                    offset,
                    connection,
                )
                .await?;
                bincode::serialize(&result)?
            }
            crate::database::Table::UsersUsersRoleRequests => {
                let result = NestedUsersUsersRoleRequest::all(
                    filter
                        .map(|filter| {
                            bincode::deserialize::<
                                crate::database::filter_variants::UsersUsersRoleRequestFilter,
                            >(&filter)
                        })
                        .transpose()?
                        .as_ref(),
                    limit,
                    offset,
                    connection,
                )
                .await?;
                bincode::serialize(&result)?
            }
            crate::database::Table::UsersUsersRoles => {
                let result = NestedUsersUsersRole::all(
                    filter
                        .map(|filter| {
                            bincode::deserialize::<
                                crate::database::filter_variants::UsersUsersRoleFilter,
                            >(&filter)
                        })
                        .transpose()?
                        .as_ref(),
                    limit,
                    offset,
                    connection,
                )
                .await?;
                bincode::serialize(&result)?
            }
        })
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
