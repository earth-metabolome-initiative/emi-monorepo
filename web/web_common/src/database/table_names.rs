//! This module contains the table names enumeration.
//!
//! This module is automatically generated. Do not write anything here.

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
        primary_key: crate::database::PrimaryKey,
        connection: &mut gluesql::prelude::Glue<C>,
    ) -> Result<Option<Vec<u8>>, crate::api::ApiError> {
        Ok(match self {
            crate::database::Table::BioOttRanks => {
                let result = crate::database::nested_variants::NestedBioOttRank::get(
                    primary_key.into(),
                    connection,
                )
                .await?;
                result
                    .map(|result| bincode::serialize(&result))
                    .transpose()?
            }
            crate::database::Table::BioOttTaxonItems => {
                let result = crate::database::nested_variants::NestedBioOttTaxonItem::get(
                    primary_key.into(),
                    connection,
                )
                .await?;
                result
                    .map(|result| bincode::serialize(&result))
                    .transpose()?
            }
            crate::database::Table::Colors => {
                let result =
                    crate::database::flat_variants::Color::get(primary_key.into(), connection)
                        .await?;
                result
                    .map(|result| bincode::serialize(&result))
                    .transpose()?
            }
            crate::database::Table::Countries => {
                let result =
                    crate::database::flat_variants::Country::get(primary_key.into(), connection)
                        .await?;
                result
                    .map(|result| bincode::serialize(&result))
                    .transpose()?
            }
            crate::database::Table::DerivedSamples => {
                let result = crate::database::nested_variants::NestedDerivedSample::get(
                    primary_key.into(),
                    connection,
                )
                .await?;
                result
                    .map(|result| bincode::serialize(&result))
                    .transpose()?
            }
            crate::database::Table::DocumentFormats => {
                let result = crate::database::nested_variants::NestedDocumentFormat::get(
                    primary_key.into(),
                    connection,
                )
                .await?;
                result
                    .map(|result| bincode::serialize(&result))
                    .transpose()?
            }
            crate::database::Table::FontAwesomeIcons => {
                let result = crate::database::flat_variants::FontAwesomeIcon::get(
                    primary_key.into(),
                    connection,
                )
                .await?;
                result
                    .map(|result| bincode::serialize(&result))
                    .transpose()?
            }
            crate::database::Table::LoginProviders => {
                let result = crate::database::nested_variants::NestedLoginProvider::get(
                    primary_key.into(),
                    connection,
                )
                .await?;
                result
                    .map(|result| bincode::serialize(&result))
                    .transpose()?
            }
            crate::database::Table::Materials => {
                let result = crate::database::nested_variants::NestedMaterial::get(
                    primary_key.into(),
                    connection,
                )
                .await?;
                result
                    .map(|result| bincode::serialize(&result))
                    .transpose()?
            }
            crate::database::Table::NameplateCategories => {
                let result = crate::database::nested_variants::NestedNameplateCategory::get(
                    primary_key.into(),
                    connection,
                )
                .await?;
                result
                    .map(|result| bincode::serialize(&result))
                    .transpose()?
            }
            crate::database::Table::Nameplates => {
                let result = crate::database::nested_variants::NestedNameplate::get(
                    primary_key.into(),
                    connection,
                )
                .await?;
                result
                    .map(|result| bincode::serialize(&result))
                    .transpose()?
            }
            crate::database::Table::Notifications => {
                let result = crate::database::nested_variants::NestedNotification::get(
                    primary_key.into(),
                    connection,
                )
                .await?;
                result
                    .map(|result| bincode::serialize(&result))
                    .transpose()?
            }
            crate::database::Table::ObservationSubjects => {
                let result = crate::database::nested_variants::NestedObservationSubject::get(
                    primary_key.into(),
                    connection,
                )
                .await?;
                result
                    .map(|result| bincode::serialize(&result))
                    .transpose()?
            }
            crate::database::Table::Observations => {
                let result = crate::database::nested_variants::NestedObservation::get(
                    primary_key.into(),
                    connection,
                )
                .await?;
                result
                    .map(|result| bincode::serialize(&result))
                    .transpose()?
            }
            crate::database::Table::OrganismBioOttTaxonItems => {
                let result = crate::database::nested_variants::NestedOrganismBioOttTaxonItem::get(
                    primary_key.into(),
                    connection,
                )
                .await?;
                result
                    .map(|result| bincode::serialize(&result))
                    .transpose()?
            }
            crate::database::Table::Organisms => {
                let result = crate::database::nested_variants::NestedOrganism::get(
                    primary_key.into(),
                    connection,
                )
                .await?;
                result
                    .map(|result| bincode::serialize(&result))
                    .transpose()?
            }
            crate::database::Table::Organizations => {
                let result = crate::database::nested_variants::NestedOrganization::get(
                    primary_key.into(),
                    connection,
                )
                .await?;
                result
                    .map(|result| bincode::serialize(&result))
                    .transpose()?
            }
            crate::database::Table::PermanenceCategories => {
                let result = crate::database::nested_variants::NestedPermanenceCategory::get(
                    primary_key.into(),
                    connection,
                )
                .await?;
                result
                    .map(|result| bincode::serialize(&result))
                    .transpose()?
            }
            crate::database::Table::ProjectStates => {
                let result = crate::database::nested_variants::NestedProjectState::get(
                    primary_key.into(),
                    connection,
                )
                .await?;
                result
                    .map(|result| bincode::serialize(&result))
                    .transpose()?
            }
            crate::database::Table::Projects => {
                let result = crate::database::nested_variants::NestedProject::get(
                    primary_key.into(),
                    connection,
                )
                .await?;
                result
                    .map(|result| bincode::serialize(&result))
                    .transpose()?
            }
            crate::database::Table::ProjectsTeamsRoleInvitations => {
                let result =
                    crate::database::nested_variants::NestedProjectsTeamsRoleInvitation::get(
                        primary_key.into(),
                        connection,
                    )
                    .await?;
                result
                    .map(|result| bincode::serialize(&result))
                    .transpose()?
            }
            crate::database::Table::ProjectsTeamsRoleRequests => {
                let result = crate::database::nested_variants::NestedProjectsTeamsRoleRequest::get(
                    primary_key.into(),
                    connection,
                )
                .await?;
                result
                    .map(|result| bincode::serialize(&result))
                    .transpose()?
            }
            crate::database::Table::ProjectsTeamsRoles => {
                let result = crate::database::nested_variants::NestedProjectsTeamsRole::get(
                    primary_key.into(),
                    connection,
                )
                .await?;
                result
                    .map(|result| bincode::serialize(&result))
                    .transpose()?
            }
            crate::database::Table::ProjectsUsersRoleInvitations => {
                let result =
                    crate::database::nested_variants::NestedProjectsUsersRoleInvitation::get(
                        primary_key.into(),
                        connection,
                    )
                    .await?;
                result
                    .map(|result| bincode::serialize(&result))
                    .transpose()?
            }
            crate::database::Table::ProjectsUsersRoleRequests => {
                let result = crate::database::nested_variants::NestedProjectsUsersRoleRequest::get(
                    primary_key.into(),
                    connection,
                )
                .await?;
                result
                    .map(|result| bincode::serialize(&result))
                    .transpose()?
            }
            crate::database::Table::ProjectsUsersRoles => {
                let result = crate::database::nested_variants::NestedProjectsUsersRole::get(
                    primary_key.into(),
                    connection,
                )
                .await?;
                result
                    .map(|result| bincode::serialize(&result))
                    .transpose()?
            }
            crate::database::Table::Roles => {
                let result = crate::database::nested_variants::NestedRole::get(
                    primary_key.into(),
                    connection,
                )
                .await?;
                result
                    .map(|result| bincode::serialize(&result))
                    .transpose()?
            }
            crate::database::Table::SampleBioOttTaxonItems => {
                let result = crate::database::nested_variants::NestedSampleBioOttTaxonItem::get(
                    primary_key.into(),
                    connection,
                )
                .await?;
                result
                    .map(|result| bincode::serialize(&result))
                    .transpose()?
            }
            crate::database::Table::SampleContainerCategories => {
                let result = crate::database::nested_variants::NestedSampleContainerCategory::get(
                    primary_key.into(),
                    connection,
                )
                .await?;
                result
                    .map(|result| bincode::serialize(&result))
                    .transpose()?
            }
            crate::database::Table::SampleContainers => {
                let result = crate::database::nested_variants::NestedSampleContainer::get(
                    primary_key.into(),
                    connection,
                )
                .await?;
                result
                    .map(|result| bincode::serialize(&result))
                    .transpose()?
            }
            crate::database::Table::SampleStates => {
                let result = crate::database::nested_variants::NestedSampleState::get(
                    primary_key.into(),
                    connection,
                )
                .await?;
                result
                    .map(|result| bincode::serialize(&result))
                    .transpose()?
            }
            crate::database::Table::Samples => {
                let result = crate::database::nested_variants::NestedSample::get(
                    primary_key.into(),
                    connection,
                )
                .await?;
                result
                    .map(|result| bincode::serialize(&result))
                    .transpose()?
            }
            crate::database::Table::Spectra => {
                let result = crate::database::nested_variants::NestedSpectra::get(
                    primary_key.into(),
                    connection,
                )
                .await?;
                result
                    .map(|result| bincode::serialize(&result))
                    .transpose()?
            }
            crate::database::Table::SpectraCollections => {
                let result = crate::database::nested_variants::NestedSpectraCollection::get(
                    primary_key.into(),
                    connection,
                )
                .await?;
                result
                    .map(|result| bincode::serialize(&result))
                    .transpose()?
            }
            crate::database::Table::TeamStates => {
                let result = crate::database::nested_variants::NestedTeamState::get(
                    primary_key.into(),
                    connection,
                )
                .await?;
                result
                    .map(|result| bincode::serialize(&result))
                    .transpose()?
            }
            crate::database::Table::Teams => {
                let result = crate::database::nested_variants::NestedTeam::get(
                    primary_key.into(),
                    connection,
                )
                .await?;
                result
                    .map(|result| bincode::serialize(&result))
                    .transpose()?
            }
            crate::database::Table::TeamsTeamsRoleInvitations => {
                let result = crate::database::nested_variants::NestedTeamsTeamsRoleInvitation::get(
                    primary_key.into(),
                    connection,
                )
                .await?;
                result
                    .map(|result| bincode::serialize(&result))
                    .transpose()?
            }
            crate::database::Table::TeamsUsersRoleInvitations => {
                let result = crate::database::nested_variants::NestedTeamsUsersRoleInvitation::get(
                    primary_key.into(),
                    connection,
                )
                .await?;
                result
                    .map(|result| bincode::serialize(&result))
                    .transpose()?
            }
            crate::database::Table::TeamsUsersRoleRequests => {
                let result = crate::database::nested_variants::NestedTeamsUsersRoleRequest::get(
                    primary_key.into(),
                    connection,
                )
                .await?;
                result
                    .map(|result| bincode::serialize(&result))
                    .transpose()?
            }
            crate::database::Table::TeamsUsersRoles => {
                let result = crate::database::nested_variants::NestedTeamsUsersRole::get(
                    primary_key.into(),
                    connection,
                )
                .await?;
                result
                    .map(|result| bincode::serialize(&result))
                    .transpose()?
            }
            crate::database::Table::Units => {
                let result = crate::database::nested_variants::NestedUnit::get(
                    primary_key.into(),
                    connection,
                )
                .await?;
                result
                    .map(|result| bincode::serialize(&result))
                    .transpose()?
            }
            crate::database::Table::UserEmails => {
                let result = crate::database::nested_variants::NestedUserEmail::get(
                    primary_key.into(),
                    connection,
                )
                .await?;
                result
                    .map(|result| bincode::serialize(&result))
                    .transpose()?
            }
            crate::database::Table::Users => {
                let result = crate::database::nested_variants::NestedUser::get(
                    primary_key.into(),
                    connection,
                )
                .await?;
                result
                    .map(|result| bincode::serialize(&result))
                    .transpose()?
            }
            crate::database::Table::UsersUsersRoleInvitations => {
                let result = crate::database::nested_variants::NestedUsersUsersRoleInvitation::get(
                    primary_key.into(),
                    connection,
                )
                .await?;
                result
                    .map(|result| bincode::serialize(&result))
                    .transpose()?
            }
            crate::database::Table::UsersUsersRoleRequests => {
                let result = crate::database::nested_variants::NestedUsersUsersRoleRequest::get(
                    primary_key.into(),
                    connection,
                )
                .await?;
                result
                    .map(|result| bincode::serialize(&result))
                    .transpose()?
            }
            crate::database::Table::UsersUsersRoles => {
                let result = crate::database::nested_variants::NestedUsersUsersRole::get(
                    primary_key.into(),
                    connection,
                )
                .await?;
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
                crate::database::nested_variants::NestedBioOttRank::delete(
                    bincode::deserialize::<crate::database::nested_variants::NestedBioOttRank>(
                        &row,
                    )?,
                    connection,
                )
                .await?
            }
            crate::database::Table::BioOttTaxonItems => {
                crate::database::nested_variants::NestedBioOttTaxonItem::delete(
                    bincode::deserialize::<crate::database::nested_variants::NestedBioOttTaxonItem>(
                        &row,
                    )?,
                    connection,
                )
                .await?
            }
            crate::database::Table::Colors => {
                crate::database::flat_variants::Color::delete(
                    bincode::deserialize::<crate::database::flat_variants::Color>(&row)?,
                    connection,
                )
                .await?
            }
            crate::database::Table::Countries => {
                crate::database::flat_variants::Country::delete(
                    bincode::deserialize::<crate::database::flat_variants::Country>(&row)?,
                    connection,
                )
                .await?
            }
            crate::database::Table::DerivedSamples => {
                crate::database::nested_variants::NestedDerivedSample::delete(
                    bincode::deserialize::<crate::database::nested_variants::NestedDerivedSample>(
                        &row,
                    )?,
                    connection,
                )
                .await?
            }
            crate::database::Table::DocumentFormats => {
                crate::database::nested_variants::NestedDocumentFormat::delete(
                    bincode::deserialize::<crate::database::nested_variants::NestedDocumentFormat>(
                        &row,
                    )?,
                    connection,
                )
                .await?
            }
            crate::database::Table::FontAwesomeIcons => {
                crate::database::flat_variants::FontAwesomeIcon::delete(
                    bincode::deserialize::<crate::database::flat_variants::FontAwesomeIcon>(&row)?,
                    connection,
                )
                .await?
            }
            crate::database::Table::LoginProviders => {
                crate::database::nested_variants::NestedLoginProvider::delete(
                    bincode::deserialize::<crate::database::nested_variants::NestedLoginProvider>(
                        &row,
                    )?,
                    connection,
                )
                .await?
            }
            crate::database::Table::Materials => {
                crate::database::nested_variants::NestedMaterial::delete(
                    bincode::deserialize::<crate::database::nested_variants::NestedMaterial>(&row)?,
                    connection,
                )
                .await?
            }
            crate::database::Table::NameplateCategories => {
                crate::database::nested_variants::NestedNameplateCategory::delete(
                    bincode::deserialize::<
                        crate::database::nested_variants::NestedNameplateCategory,
                    >(&row)?,
                    connection,
                )
                .await?
            }
            crate::database::Table::Nameplates => {
                crate::database::nested_variants::NestedNameplate::delete(
                    bincode::deserialize::<crate::database::nested_variants::NestedNameplate>(
                        &row,
                    )?,
                    connection,
                )
                .await?
            }
            crate::database::Table::Notifications => {
                crate::database::nested_variants::NestedNotification::delete(
                    bincode::deserialize::<crate::database::nested_variants::NestedNotification>(
                        &row,
                    )?,
                    connection,
                )
                .await?
            }
            crate::database::Table::ObservationSubjects => {
                crate::database::nested_variants::NestedObservationSubject::delete(
                    bincode::deserialize::<
                        crate::database::nested_variants::NestedObservationSubject,
                    >(&row)?,
                    connection,
                )
                .await?
            }
            crate::database::Table::Observations => {
                crate::database::nested_variants::NestedObservation::delete(
                    bincode::deserialize::<crate::database::nested_variants::NestedObservation>(
                        &row,
                    )?,
                    connection,
                )
                .await?
            }
            crate::database::Table::OrganismBioOttTaxonItems => {
                crate::database::nested_variants::NestedOrganismBioOttTaxonItem::delete(
                    bincode::deserialize::<
                        crate::database::nested_variants::NestedOrganismBioOttTaxonItem,
                    >(&row)?,
                    connection,
                )
                .await?
            }
            crate::database::Table::Organisms => {
                crate::database::nested_variants::NestedOrganism::delete(
                    bincode::deserialize::<crate::database::nested_variants::NestedOrganism>(&row)?,
                    connection,
                )
                .await?
            }
            crate::database::Table::Organizations => {
                crate::database::nested_variants::NestedOrganization::delete(
                    bincode::deserialize::<crate::database::nested_variants::NestedOrganization>(
                        &row,
                    )?,
                    connection,
                )
                .await?
            }
            crate::database::Table::PermanenceCategories => {
                crate::database::nested_variants::NestedPermanenceCategory::delete(
                    bincode::deserialize::<
                        crate::database::nested_variants::NestedPermanenceCategory,
                    >(&row)?,
                    connection,
                )
                .await?
            }
            crate::database::Table::ProjectStates => {
                crate::database::nested_variants::NestedProjectState::delete(
                    bincode::deserialize::<crate::database::nested_variants::NestedProjectState>(
                        &row,
                    )?,
                    connection,
                )
                .await?
            }
            crate::database::Table::Projects => {
                crate::database::nested_variants::NestedProject::delete(
                    bincode::deserialize::<crate::database::nested_variants::NestedProject>(&row)?,
                    connection,
                )
                .await?
            }
            crate::database::Table::ProjectsTeamsRoleInvitations => {
                crate::database::nested_variants::NestedProjectsTeamsRoleInvitation::delete(
                    bincode::deserialize::<
                        crate::database::nested_variants::NestedProjectsTeamsRoleInvitation,
                    >(&row)?,
                    connection,
                )
                .await?
            }
            crate::database::Table::ProjectsTeamsRoleRequests => {
                crate::database::nested_variants::NestedProjectsTeamsRoleRequest::delete(
                    bincode::deserialize::<
                        crate::database::nested_variants::NestedProjectsTeamsRoleRequest,
                    >(&row)?,
                    connection,
                )
                .await?
            }
            crate::database::Table::ProjectsTeamsRoles => {
                crate::database::nested_variants::NestedProjectsTeamsRole::delete(
                    bincode::deserialize::<
                        crate::database::nested_variants::NestedProjectsTeamsRole,
                    >(&row)?,
                    connection,
                )
                .await?
            }
            crate::database::Table::ProjectsUsersRoleInvitations => {
                crate::database::nested_variants::NestedProjectsUsersRoleInvitation::delete(
                    bincode::deserialize::<
                        crate::database::nested_variants::NestedProjectsUsersRoleInvitation,
                    >(&row)?,
                    connection,
                )
                .await?
            }
            crate::database::Table::ProjectsUsersRoleRequests => {
                crate::database::nested_variants::NestedProjectsUsersRoleRequest::delete(
                    bincode::deserialize::<
                        crate::database::nested_variants::NestedProjectsUsersRoleRequest,
                    >(&row)?,
                    connection,
                )
                .await?
            }
            crate::database::Table::ProjectsUsersRoles => {
                crate::database::nested_variants::NestedProjectsUsersRole::delete(
                    bincode::deserialize::<
                        crate::database::nested_variants::NestedProjectsUsersRole,
                    >(&row)?,
                    connection,
                )
                .await?
            }
            crate::database::Table::Roles => {
                crate::database::nested_variants::NestedRole::delete(
                    bincode::deserialize::<crate::database::nested_variants::NestedRole>(&row)?,
                    connection,
                )
                .await?
            }
            crate::database::Table::SampleBioOttTaxonItems => {
                crate::database::nested_variants::NestedSampleBioOttTaxonItem::delete(
                    bincode::deserialize::<
                        crate::database::nested_variants::NestedSampleBioOttTaxonItem,
                    >(&row)?,
                    connection,
                )
                .await?
            }
            crate::database::Table::SampleContainerCategories => {
                crate::database::nested_variants::NestedSampleContainerCategory::delete(
                    bincode::deserialize::<
                        crate::database::nested_variants::NestedSampleContainerCategory,
                    >(&row)?,
                    connection,
                )
                .await?
            }
            crate::database::Table::SampleContainers => {
                crate::database::nested_variants::NestedSampleContainer::delete(
                    bincode::deserialize::<crate::database::nested_variants::NestedSampleContainer>(
                        &row,
                    )?,
                    connection,
                )
                .await?
            }
            crate::database::Table::SampleStates => {
                crate::database::nested_variants::NestedSampleState::delete(
                    bincode::deserialize::<crate::database::nested_variants::NestedSampleState>(
                        &row,
                    )?,
                    connection,
                )
                .await?
            }
            crate::database::Table::Samples => {
                crate::database::nested_variants::NestedSample::delete(
                    bincode::deserialize::<crate::database::nested_variants::NestedSample>(&row)?,
                    connection,
                )
                .await?
            }
            crate::database::Table::Spectra => {
                crate::database::nested_variants::NestedSpectra::delete(
                    bincode::deserialize::<crate::database::nested_variants::NestedSpectra>(&row)?,
                    connection,
                )
                .await?
            }
            crate::database::Table::SpectraCollections => {
                crate::database::nested_variants::NestedSpectraCollection::delete(
                    bincode::deserialize::<
                        crate::database::nested_variants::NestedSpectraCollection,
                    >(&row)?,
                    connection,
                )
                .await?
            }
            crate::database::Table::TeamStates => {
                crate::database::nested_variants::NestedTeamState::delete(
                    bincode::deserialize::<crate::database::nested_variants::NestedTeamState>(
                        &row,
                    )?,
                    connection,
                )
                .await?
            }
            crate::database::Table::Teams => {
                crate::database::nested_variants::NestedTeam::delete(
                    bincode::deserialize::<crate::database::nested_variants::NestedTeam>(&row)?,
                    connection,
                )
                .await?
            }
            crate::database::Table::TeamsTeamsRoleInvitations => {
                crate::database::nested_variants::NestedTeamsTeamsRoleInvitation::delete(
                    bincode::deserialize::<
                        crate::database::nested_variants::NestedTeamsTeamsRoleInvitation,
                    >(&row)?,
                    connection,
                )
                .await?
            }
            crate::database::Table::TeamsUsersRoleInvitations => {
                crate::database::nested_variants::NestedTeamsUsersRoleInvitation::delete(
                    bincode::deserialize::<
                        crate::database::nested_variants::NestedTeamsUsersRoleInvitation,
                    >(&row)?,
                    connection,
                )
                .await?
            }
            crate::database::Table::TeamsUsersRoleRequests => {
                crate::database::nested_variants::NestedTeamsUsersRoleRequest::delete(
                    bincode::deserialize::<
                        crate::database::nested_variants::NestedTeamsUsersRoleRequest,
                    >(&row)?,
                    connection,
                )
                .await?
            }
            crate::database::Table::TeamsUsersRoles => {
                crate::database::nested_variants::NestedTeamsUsersRole::delete(
                    bincode::deserialize::<crate::database::nested_variants::NestedTeamsUsersRole>(
                        &row,
                    )?,
                    connection,
                )
                .await?
            }
            crate::database::Table::Units => {
                crate::database::nested_variants::NestedUnit::delete(
                    bincode::deserialize::<crate::database::nested_variants::NestedUnit>(&row)?,
                    connection,
                )
                .await?
            }
            crate::database::Table::UserEmails => {
                crate::database::nested_variants::NestedUserEmail::delete(
                    bincode::deserialize::<crate::database::nested_variants::NestedUserEmail>(
                        &row,
                    )?,
                    connection,
                )
                .await?
            }
            crate::database::Table::Users => {
                crate::database::nested_variants::NestedUser::delete(
                    bincode::deserialize::<crate::database::nested_variants::NestedUser>(&row)?,
                    connection,
                )
                .await?
            }
            crate::database::Table::UsersUsersRoleInvitations => {
                crate::database::nested_variants::NestedUsersUsersRoleInvitation::delete(
                    bincode::deserialize::<
                        crate::database::nested_variants::NestedUsersUsersRoleInvitation,
                    >(&row)?,
                    connection,
                )
                .await?
            }
            crate::database::Table::UsersUsersRoleRequests => {
                crate::database::nested_variants::NestedUsersUsersRoleRequest::delete(
                    bincode::deserialize::<
                        crate::database::nested_variants::NestedUsersUsersRoleRequest,
                    >(&row)?,
                    connection,
                )
                .await?
            }
            crate::database::Table::UsersUsersRoles => {
                crate::database::nested_variants::NestedUsersUsersRole::delete(
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
        primary_key: crate::database::PrimaryKey,
        connection: &mut gluesql::prelude::Glue<C>,
    ) -> Result<usize, crate::api::ApiError> {
        Ok(match self {
            crate::database::Table::BioOttRanks => {
                crate::database::nested_variants::NestedBioOttRank::delete_from_id(
                    primary_key.into(),
                    connection,
                )
                .await?
            }
            crate::database::Table::BioOttTaxonItems => {
                crate::database::nested_variants::NestedBioOttTaxonItem::delete_from_id(
                    primary_key.into(),
                    connection,
                )
                .await?
            }
            crate::database::Table::Colors => {
                crate::database::flat_variants::Color::delete_from_id(
                    primary_key.into(),
                    connection,
                )
                .await?
            }
            crate::database::Table::Countries => {
                crate::database::flat_variants::Country::delete_from_id(
                    primary_key.into(),
                    connection,
                )
                .await?
            }
            crate::database::Table::DerivedSamples => {
                crate::database::nested_variants::NestedDerivedSample::delete_from_id(
                    primary_key.into(),
                    connection,
                )
                .await?
            }
            crate::database::Table::DocumentFormats => {
                crate::database::nested_variants::NestedDocumentFormat::delete_from_id(
                    primary_key.into(),
                    connection,
                )
                .await?
            }
            crate::database::Table::FontAwesomeIcons => {
                crate::database::flat_variants::FontAwesomeIcon::delete_from_id(
                    primary_key.into(),
                    connection,
                )
                .await?
            }
            crate::database::Table::LoginProviders => {
                crate::database::nested_variants::NestedLoginProvider::delete_from_id(
                    primary_key.into(),
                    connection,
                )
                .await?
            }
            crate::database::Table::Materials => {
                crate::database::nested_variants::NestedMaterial::delete_from_id(
                    primary_key.into(),
                    connection,
                )
                .await?
            }
            crate::database::Table::NameplateCategories => {
                crate::database::nested_variants::NestedNameplateCategory::delete_from_id(
                    primary_key.into(),
                    connection,
                )
                .await?
            }
            crate::database::Table::Nameplates => {
                crate::database::nested_variants::NestedNameplate::delete_from_id(
                    primary_key.into(),
                    connection,
                )
                .await?
            }
            crate::database::Table::Notifications => {
                crate::database::nested_variants::NestedNotification::delete_from_id(
                    primary_key.into(),
                    connection,
                )
                .await?
            }
            crate::database::Table::ObservationSubjects => {
                crate::database::nested_variants::NestedObservationSubject::delete_from_id(
                    primary_key.into(),
                    connection,
                )
                .await?
            }
            crate::database::Table::Observations => {
                crate::database::nested_variants::NestedObservation::delete_from_id(
                    primary_key.into(),
                    connection,
                )
                .await?
            }
            crate::database::Table::OrganismBioOttTaxonItems => {
                crate::database::nested_variants::NestedOrganismBioOttTaxonItem::delete_from_id(
                    primary_key.into(),
                    connection,
                )
                .await?
            }
            crate::database::Table::Organisms => {
                crate::database::nested_variants::NestedOrganism::delete_from_id(
                    primary_key.into(),
                    connection,
                )
                .await?
            }
            crate::database::Table::Organizations => {
                crate::database::nested_variants::NestedOrganization::delete_from_id(
                    primary_key.into(),
                    connection,
                )
                .await?
            }
            crate::database::Table::PermanenceCategories => {
                crate::database::nested_variants::NestedPermanenceCategory::delete_from_id(
                    primary_key.into(),
                    connection,
                )
                .await?
            }
            crate::database::Table::ProjectStates => {
                crate::database::nested_variants::NestedProjectState::delete_from_id(
                    primary_key.into(),
                    connection,
                )
                .await?
            }
            crate::database::Table::Projects => {
                crate::database::nested_variants::NestedProject::delete_from_id(
                    primary_key.into(),
                    connection,
                )
                .await?
            }
            crate::database::Table::ProjectsTeamsRoleInvitations => {
                crate::database::nested_variants::NestedProjectsTeamsRoleInvitation::delete_from_id(
                    primary_key.into(),
                    connection,
                )
                .await?
            }
            crate::database::Table::ProjectsTeamsRoleRequests => {
                crate::database::nested_variants::NestedProjectsTeamsRoleRequest::delete_from_id(
                    primary_key.into(),
                    connection,
                )
                .await?
            }
            crate::database::Table::ProjectsTeamsRoles => {
                crate::database::nested_variants::NestedProjectsTeamsRole::delete_from_id(
                    primary_key.into(),
                    connection,
                )
                .await?
            }
            crate::database::Table::ProjectsUsersRoleInvitations => {
                crate::database::nested_variants::NestedProjectsUsersRoleInvitation::delete_from_id(
                    primary_key.into(),
                    connection,
                )
                .await?
            }
            crate::database::Table::ProjectsUsersRoleRequests => {
                crate::database::nested_variants::NestedProjectsUsersRoleRequest::delete_from_id(
                    primary_key.into(),
                    connection,
                )
                .await?
            }
            crate::database::Table::ProjectsUsersRoles => {
                crate::database::nested_variants::NestedProjectsUsersRole::delete_from_id(
                    primary_key.into(),
                    connection,
                )
                .await?
            }
            crate::database::Table::Roles => {
                crate::database::nested_variants::NestedRole::delete_from_id(
                    primary_key.into(),
                    connection,
                )
                .await?
            }
            crate::database::Table::SampleBioOttTaxonItems => {
                crate::database::nested_variants::NestedSampleBioOttTaxonItem::delete_from_id(
                    primary_key.into(),
                    connection,
                )
                .await?
            }
            crate::database::Table::SampleContainerCategories => {
                crate::database::nested_variants::NestedSampleContainerCategory::delete_from_id(
                    primary_key.into(),
                    connection,
                )
                .await?
            }
            crate::database::Table::SampleContainers => {
                crate::database::nested_variants::NestedSampleContainer::delete_from_id(
                    primary_key.into(),
                    connection,
                )
                .await?
            }
            crate::database::Table::SampleStates => {
                crate::database::nested_variants::NestedSampleState::delete_from_id(
                    primary_key.into(),
                    connection,
                )
                .await?
            }
            crate::database::Table::Samples => {
                crate::database::nested_variants::NestedSample::delete_from_id(
                    primary_key.into(),
                    connection,
                )
                .await?
            }
            crate::database::Table::Spectra => {
                crate::database::nested_variants::NestedSpectra::delete_from_id(
                    primary_key.into(),
                    connection,
                )
                .await?
            }
            crate::database::Table::SpectraCollections => {
                crate::database::nested_variants::NestedSpectraCollection::delete_from_id(
                    primary_key.into(),
                    connection,
                )
                .await?
            }
            crate::database::Table::TeamStates => {
                crate::database::nested_variants::NestedTeamState::delete_from_id(
                    primary_key.into(),
                    connection,
                )
                .await?
            }
            crate::database::Table::Teams => {
                crate::database::nested_variants::NestedTeam::delete_from_id(
                    primary_key.into(),
                    connection,
                )
                .await?
            }
            crate::database::Table::TeamsTeamsRoleInvitations => {
                crate::database::nested_variants::NestedTeamsTeamsRoleInvitation::delete_from_id(
                    primary_key.into(),
                    connection,
                )
                .await?
            }
            crate::database::Table::TeamsUsersRoleInvitations => {
                crate::database::nested_variants::NestedTeamsUsersRoleInvitation::delete_from_id(
                    primary_key.into(),
                    connection,
                )
                .await?
            }
            crate::database::Table::TeamsUsersRoleRequests => {
                crate::database::nested_variants::NestedTeamsUsersRoleRequest::delete_from_id(
                    primary_key.into(),
                    connection,
                )
                .await?
            }
            crate::database::Table::TeamsUsersRoles => {
                crate::database::nested_variants::NestedTeamsUsersRole::delete_from_id(
                    primary_key.into(),
                    connection,
                )
                .await?
            }
            crate::database::Table::Units => {
                crate::database::nested_variants::NestedUnit::delete_from_id(
                    primary_key.into(),
                    connection,
                )
                .await?
            }
            crate::database::Table::UserEmails => {
                crate::database::nested_variants::NestedUserEmail::delete_from_id(
                    primary_key.into(),
                    connection,
                )
                .await?
            }
            crate::database::Table::Users => {
                crate::database::nested_variants::NestedUser::delete_from_id(
                    primary_key.into(),
                    connection,
                )
                .await?
            }
            crate::database::Table::UsersUsersRoleInvitations => {
                crate::database::nested_variants::NestedUsersUsersRoleInvitation::delete_from_id(
                    primary_key.into(),
                    connection,
                )
                .await?
            }
            crate::database::Table::UsersUsersRoleRequests => {
                crate::database::nested_variants::NestedUsersUsersRoleRequest::delete_from_id(
                    primary_key.into(),
                    connection,
                )
                .await?
            }
            crate::database::Table::UsersUsersRoles => {
                crate::database::nested_variants::NestedUsersUsersRole::delete_from_id(
                    primary_key.into(),
                    connection,
                )
                .await?
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
                    crate::database::nested_variants::NestedBioOttRank::all(
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
                let result = crate::database::nested_variants::NestedBioOttTaxonItem::all(
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
                let result =
                    crate::database::flat_variants::Color::all(limit, offset, connection).await?;
                bincode::serialize(&result)?
            }
            crate::database::Table::Countries => {
                let result =
                    crate::database::flat_variants::Country::all(limit, offset, connection).await?;
                bincode::serialize(&result)?
            }
            crate::database::Table::DerivedSamples => {
                let result = crate::database::nested_variants::NestedDerivedSample::all(
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
                let result = crate::database::nested_variants::NestedDocumentFormat::all(
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
                let result =
                    crate::database::flat_variants::FontAwesomeIcon::all(limit, offset, connection)
                        .await?;
                bincode::serialize(&result)?
            }
            crate::database::Table::LoginProviders => {
                let result = crate::database::nested_variants::NestedLoginProvider::all(
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
                    crate::database::nested_variants::NestedMaterial::all(
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
                let result = crate::database::nested_variants::NestedNameplateCategory::all(
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
                    crate::database::nested_variants::NestedNameplate::all(
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
                let result = crate::database::nested_variants::NestedNotification::all(
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
                let result = crate::database::nested_variants::NestedObservationSubject::all(
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
                let result = crate::database::nested_variants::NestedObservation::all(
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
                let result = crate::database::nested_variants::NestedOrganismBioOttTaxonItem::all(
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
                    crate::database::nested_variants::NestedOrganism::all(
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
                let result = crate::database::nested_variants::NestedOrganization::all(
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
                let result = crate::database::nested_variants::NestedPermanenceCategory::all(
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
                let result = crate::database::nested_variants::NestedProjectState::all(
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
                let result = crate::database::nested_variants::NestedProject::all(
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
                let result = crate::database::nested_variants::NestedProjectsTeamsRoleInvitation::all(
filter.map(|filter| bincode::deserialize::<crate::database::filter_variants::ProjectsTeamsRoleInvitationFilter>(&filter)).transpose()?.as_ref(),
limit,
offset,
connection).await?;
                bincode::serialize(&result)?
            }
            crate::database::Table::ProjectsTeamsRoleRequests => {
                let result = crate::database::nested_variants::NestedProjectsTeamsRoleRequest::all(
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
                let result = crate::database::nested_variants::NestedProjectsTeamsRole::all(
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
                let result = crate::database::nested_variants::NestedProjectsUsersRoleInvitation::all(
filter.map(|filter| bincode::deserialize::<crate::database::filter_variants::ProjectsUsersRoleInvitationFilter>(&filter)).transpose()?.as_ref(),
limit,
offset,
connection).await?;
                bincode::serialize(&result)?
            }
            crate::database::Table::ProjectsUsersRoleRequests => {
                let result = crate::database::nested_variants::NestedProjectsUsersRoleRequest::all(
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
                let result = crate::database::nested_variants::NestedProjectsUsersRole::all(
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
                let result = crate::database::nested_variants::NestedRole::all(
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
                let result = crate::database::nested_variants::NestedSampleBioOttTaxonItem::all(
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
                let result = crate::database::nested_variants::NestedSampleContainerCategory::all(
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
                let result = crate::database::nested_variants::NestedSampleContainer::all(
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
                let result = crate::database::nested_variants::NestedSampleState::all(
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
                let result = crate::database::nested_variants::NestedSample::all(
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
                let result = crate::database::nested_variants::NestedSpectra::all(
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
                let result = crate::database::nested_variants::NestedSpectraCollection::all(
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
                    crate::database::nested_variants::NestedTeamState::all(
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
                let result = crate::database::nested_variants::NestedTeam::all(
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
                let result = crate::database::nested_variants::NestedTeamsTeamsRoleInvitation::all(
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
                let result = crate::database::nested_variants::NestedTeamsUsersRoleInvitation::all(
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
                let result = crate::database::nested_variants::NestedTeamsUsersRoleRequest::all(
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
                let result = crate::database::nested_variants::NestedTeamsUsersRole::all(
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
                let result = crate::database::nested_variants::NestedUnit::all(
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
                    crate::database::nested_variants::NestedUserEmail::all(
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
                let result = crate::database::nested_variants::NestedUser::all(
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
                let result = crate::database::nested_variants::NestedUsersUsersRoleInvitation::all(
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
                let result = crate::database::nested_variants::NestedUsersUsersRoleRequest::all(
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
                let result = crate::database::nested_variants::NestedUsersUsersRole::all(
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
                let new_row: crate::database::new_variants::NewObservation = bincode::deserialize::<crate::database::new_variants::NewObservation>(&new_row).map_err(crate::api::ApiError::from)?;
                let inserted_row: crate::database::flat_variants::Observation = new_row.insert(user_id, connection).await?;
                let nested_row = crate::database::nested_variants::NestedObservation::from_flat(inserted_row, connection).await?;
                 bincode::serialize(&nested_row).map_err(crate::api::ApiError::from)?
            },
            Table::OrganismBioOttTaxonItems => unimplemented!("Insert not implemented for organism_bio_ott_taxon_items in frontend as it does not have a UUID primary key."),
            Table::Organisms => {
                let new_row: crate::database::new_variants::NewOrganism = bincode::deserialize::<crate::database::new_variants::NewOrganism>(&new_row).map_err(crate::api::ApiError::from)?;
                let inserted_row: crate::database::flat_variants::Organism = new_row.insert(user_id, connection).await?;
                let nested_row = crate::database::nested_variants::NestedOrganism::from_flat(inserted_row, connection).await?;
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
                let new_row: crate::database::new_variants::NewSample = bincode::deserialize::<crate::database::new_variants::NewSample>(&new_row).map_err(crate::api::ApiError::from)?;
                let inserted_row: crate::database::flat_variants::Sample = new_row.insert(user_id, connection).await?;
                let nested_row = crate::database::nested_variants::NestedSample::from_flat(inserted_row, connection).await?;
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
                let update_row: crate::database::update_variants::UpdateNameplate =
                    bincode::deserialize::<crate::database::update_variants::UpdateNameplate>(
                        &update_row,
                    )
                    .map_err(crate::api::ApiError::from)?;
                let id = update_row.id;
                update_row.update(user_id, connection).await?;
                let updated_row: crate::database::flat_variants::Nameplate =
                    crate::database::flat_variants::Nameplate::get(id, connection)
                        .await?
                        .unwrap();
                let nested_row = crate::database::nested_variants::NestedNameplate::from_flat(
                    updated_row,
                    connection,
                )
                .await?;
                bincode::serialize(&nested_row).map_err(crate::api::ApiError::from)?
            }
            Table::Notifications => unimplemented!("Update not implemented for notifications."),
            Table::ObservationSubjects => {
                unimplemented!("Update not implemented for observation_subjects.")
            }
            Table::Observations => {
                let update_row: crate::database::new_variants::NewObservation =
                    bincode::deserialize::<crate::database::new_variants::NewObservation>(
                        &update_row,
                    )
                    .map_err(crate::api::ApiError::from)?;
                let id = update_row.id;
                update_row.update(user_id, connection).await?;
                let updated_row: crate::database::flat_variants::Observation =
                    crate::database::flat_variants::Observation::get(id, connection)
                        .await?
                        .unwrap();
                let nested_row = crate::database::nested_variants::NestedObservation::from_flat(
                    updated_row,
                    connection,
                )
                .await?;
                bincode::serialize(&nested_row).map_err(crate::api::ApiError::from)?
            }
            Table::OrganismBioOttTaxonItems => {
                unimplemented!("Update not implemented for organism_bio_ott_taxon_items.")
            }
            Table::Organisms => {
                let update_row: crate::database::new_variants::NewOrganism =
                    bincode::deserialize::<crate::database::new_variants::NewOrganism>(&update_row)
                        .map_err(crate::api::ApiError::from)?;
                let id = update_row.id;
                update_row.update(user_id, connection).await?;
                let updated_row: crate::database::flat_variants::Organism =
                    crate::database::flat_variants::Organism::get(id, connection)
                        .await?
                        .unwrap();
                let nested_row = crate::database::nested_variants::NestedOrganism::from_flat(
                    updated_row,
                    connection,
                )
                .await?;
                bincode::serialize(&nested_row).map_err(crate::api::ApiError::from)?
            }
            Table::Organizations => unimplemented!("Update not implemented for organizations."),
            Table::PermanenceCategories => {
                unimplemented!("Update not implemented for permanence_categories.")
            }
            Table::ProjectStates => unimplemented!("Update not implemented for project_states."),
            Table::Projects => {
                let update_row: crate::database::update_variants::UpdateProject =
                    bincode::deserialize::<crate::database::update_variants::UpdateProject>(
                        &update_row,
                    )
                    .map_err(crate::api::ApiError::from)?;
                let id = update_row.id;
                update_row.update(user_id, connection).await?;
                let updated_row: crate::database::flat_variants::Project =
                    crate::database::flat_variants::Project::get(id, connection)
                        .await?
                        .unwrap();
                let nested_row = crate::database::nested_variants::NestedProject::from_flat(
                    updated_row,
                    connection,
                )
                .await?;
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
                let update_row: crate::database::update_variants::UpdateSampleContainer = bincode::deserialize::<crate::database::update_variants::UpdateSampleContainer>(&update_row).map_err(crate::api::ApiError::from)?;
                let id = update_row.id;
                update_row.update(user_id, connection).await?;
                let updated_row: crate::database::flat_variants::SampleContainer =
                    crate::database::flat_variants::SampleContainer::get(id, connection)
                        .await?
                        .unwrap();
                let nested_row =
                    crate::database::nested_variants::NestedSampleContainer::from_flat(
                        updated_row,
                        connection,
                    )
                    .await?;
                bincode::serialize(&nested_row).map_err(crate::api::ApiError::from)?
            }
            Table::SampleStates => unimplemented!("Update not implemented for sample_states."),
            Table::Samples => {
                let update_row: crate::database::new_variants::NewSample =
                    bincode::deserialize::<crate::database::new_variants::NewSample>(&update_row)
                        .map_err(crate::api::ApiError::from)?;
                let id = update_row.id;
                update_row.update(user_id, connection).await?;
                let updated_row: crate::database::flat_variants::Sample =
                    crate::database::flat_variants::Sample::get(id, connection)
                        .await?
                        .unwrap();
                let nested_row = crate::database::nested_variants::NestedSample::from_flat(
                    updated_row,
                    connection,
                )
                .await?;
                bincode::serialize(&nested_row).map_err(crate::api::ApiError::from)?
            }
            Table::Spectra => unimplemented!("Update not implemented for spectra."),
            Table::SpectraCollections => {
                let update_row: crate::database::update_variants::UpdateSpectraCollection = bincode::deserialize::<crate::database::update_variants::UpdateSpectraCollection>(&update_row).map_err(crate::api::ApiError::from)?;
                let id = update_row.id;
                update_row.update(user_id, connection).await?;
                let updated_row: crate::database::flat_variants::SpectraCollection =
                    crate::database::flat_variants::SpectraCollection::get(id, connection)
                        .await?
                        .unwrap();
                let nested_row =
                    crate::database::nested_variants::NestedSpectraCollection::from_flat(
                        updated_row,
                        connection,
                    )
                    .await?;
                bincode::serialize(&nested_row).map_err(crate::api::ApiError::from)?
            }
            Table::TeamStates => unimplemented!("Update not implemented for team_states."),
            Table::Teams => {
                let update_row: crate::database::update_variants::UpdateTeam =
                    bincode::deserialize::<crate::database::update_variants::UpdateTeam>(
                        &update_row,
                    )
                    .map_err(crate::api::ApiError::from)?;
                let id = update_row.id;
                update_row.update(user_id, connection).await?;
                let updated_row: crate::database::flat_variants::Team =
                    crate::database::flat_variants::Team::get(id, connection)
                        .await?
                        .unwrap();
                let nested_row = crate::database::nested_variants::NestedTeam::from_flat(
                    updated_row,
                    connection,
                )
                .await?;
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
                let update_row: crate::database::update_variants::UpdateUser =
                    bincode::deserialize::<crate::database::update_variants::UpdateUser>(
                        &update_row,
                    )
                    .map_err(crate::api::ApiError::from)?;
                let id = update_row.id;
                update_row.update(connection).await?;
                let updated_row: crate::database::flat_variants::User =
                    crate::database::flat_variants::User::get(id, connection)
                        .await?
                        .unwrap();
                let nested_row = crate::database::nested_variants::NestedUser::from_flat(
                    updated_row,
                    connection,
                )
                .await?;
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
                    let row: crate::database::nested_variants::NestedBioOttRank =
                        bincode::deserialize::<crate::database::nested_variants::NestedBioOttRank>(
                            &row,
                        )
                        .map_err(crate::api::ApiError::from)?;
                    row.update_or_insert(connection).await?;
                }
            }
            Table::BioOttTaxonItems => {
                for row in rows {
                    let row: crate::database::nested_variants::NestedBioOttTaxonItem =
                        bincode::deserialize::<
                            crate::database::nested_variants::NestedBioOttTaxonItem,
                        >(&row)
                        .map_err(crate::api::ApiError::from)?;
                    row.update_or_insert(connection).await?;
                }
            }
            Table::Colors => {
                for row in rows {
                    let row: crate::database::flat_variants::Color =
                        bincode::deserialize::<crate::database::flat_variants::Color>(&row)
                            .map_err(crate::api::ApiError::from)?;
                    row.update_or_insert(connection).await?;
                }
            }
            Table::Countries => {
                for row in rows {
                    let row: crate::database::flat_variants::Country =
                        bincode::deserialize::<crate::database::flat_variants::Country>(&row)
                            .map_err(crate::api::ApiError::from)?;
                    row.update_or_insert(connection).await?;
                }
            }
            Table::DerivedSamples => {
                for row in rows {
                    let row: crate::database::nested_variants::NestedDerivedSample = bincode::deserialize::<crate::database::nested_variants::NestedDerivedSample>(&row).map_err(crate::api::ApiError::from)?;
                    row.update_or_insert(connection).await?;
                }
            }
            Table::DocumentFormats => {
                for row in rows {
                    let row: crate::database::nested_variants::NestedDocumentFormat =
                        bincode::deserialize::<
                            crate::database::nested_variants::NestedDocumentFormat,
                        >(&row)
                        .map_err(crate::api::ApiError::from)?;
                    row.update_or_insert(connection).await?;
                }
            }
            Table::FontAwesomeIcons => {
                for row in rows {
                    let row: crate::database::flat_variants::FontAwesomeIcon =
                        bincode::deserialize::<crate::database::flat_variants::FontAwesomeIcon>(
                            &row,
                        )
                        .map_err(crate::api::ApiError::from)?;
                    row.update_or_insert(connection).await?;
                }
            }
            Table::LoginProviders => {
                for row in rows {
                    let row: crate::database::nested_variants::NestedLoginProvider = bincode::deserialize::<crate::database::nested_variants::NestedLoginProvider>(&row).map_err(crate::api::ApiError::from)?;
                    row.update_or_insert(connection).await?;
                }
            }
            Table::Materials => {
                for row in rows {
                    let row: crate::database::nested_variants::NestedMaterial =
                        bincode::deserialize::<crate::database::nested_variants::NestedMaterial>(
                            &row,
                        )
                        .map_err(crate::api::ApiError::from)?;
                    row.update_or_insert(connection).await?;
                }
            }
            Table::NameplateCategories => {
                for row in rows {
                    let row: crate::database::nested_variants::NestedNameplateCategory =
                        bincode::deserialize::<
                            crate::database::nested_variants::NestedNameplateCategory,
                        >(&row)
                        .map_err(crate::api::ApiError::from)?;
                    row.update_or_insert(connection).await?;
                }
            }
            Table::Nameplates => {
                for row in rows {
                    let row: crate::database::nested_variants::NestedNameplate =
                        bincode::deserialize::<crate::database::nested_variants::NestedNameplate>(
                            &row,
                        )
                        .map_err(crate::api::ApiError::from)?;
                    row.update_or_insert(connection).await?;
                }
            }
            Table::Notifications => {
                for row in rows {
                    let row: crate::database::nested_variants::NestedNotification = bincode::deserialize::<crate::database::nested_variants::NestedNotification>(&row).map_err(crate::api::ApiError::from)?;
                    row.update_or_insert(connection).await?;
                }
            }
            Table::ObservationSubjects => {
                for row in rows {
                    let row: crate::database::nested_variants::NestedObservationSubject =
                        bincode::deserialize::<
                            crate::database::nested_variants::NestedObservationSubject,
                        >(&row)
                        .map_err(crate::api::ApiError::from)?;
                    row.update_or_insert(connection).await?;
                }
            }
            Table::Observations => {
                for row in rows {
                    let row: crate::database::nested_variants::NestedObservation = bincode::deserialize::<crate::database::nested_variants::NestedObservation>(&row).map_err(crate::api::ApiError::from)?;
                    row.update_or_insert(connection).await?;
                }
            }
            Table::OrganismBioOttTaxonItems => {
                for row in rows {
                    let row: crate::database::nested_variants::NestedOrganismBioOttTaxonItem =
                        bincode::deserialize::<
                            crate::database::nested_variants::NestedOrganismBioOttTaxonItem,
                        >(&row)
                        .map_err(crate::api::ApiError::from)?;
                    row.update_or_insert(connection).await?;
                }
            }
            Table::Organisms => {
                for row in rows {
                    let row: crate::database::nested_variants::NestedOrganism =
                        bincode::deserialize::<crate::database::nested_variants::NestedOrganism>(
                            &row,
                        )
                        .map_err(crate::api::ApiError::from)?;
                    row.update_or_insert(connection).await?;
                }
            }
            Table::Organizations => {
                for row in rows {
                    let row: crate::database::nested_variants::NestedOrganization = bincode::deserialize::<crate::database::nested_variants::NestedOrganization>(&row).map_err(crate::api::ApiError::from)?;
                    row.update_or_insert(connection).await?;
                }
            }
            Table::PermanenceCategories => {
                for row in rows {
                    let row: crate::database::nested_variants::NestedPermanenceCategory =
                        bincode::deserialize::<
                            crate::database::nested_variants::NestedPermanenceCategory,
                        >(&row)
                        .map_err(crate::api::ApiError::from)?;
                    row.update_or_insert(connection).await?;
                }
            }
            Table::ProjectStates => {
                for row in rows {
                    let row: crate::database::nested_variants::NestedProjectState = bincode::deserialize::<crate::database::nested_variants::NestedProjectState>(&row).map_err(crate::api::ApiError::from)?;
                    row.update_or_insert(connection).await?;
                }
            }
            Table::Projects => {
                for row in rows {
                    let row: crate::database::nested_variants::NestedProject =
                        bincode::deserialize::<crate::database::nested_variants::NestedProject>(
                            &row,
                        )
                        .map_err(crate::api::ApiError::from)?;
                    row.update_or_insert(connection).await?;
                }
            }
            Table::ProjectsTeamsRoleInvitations => {
                for row in rows {
                    let row: crate::database::nested_variants::NestedProjectsTeamsRoleInvitation =
                        bincode::deserialize::<
                            crate::database::nested_variants::NestedProjectsTeamsRoleInvitation,
                        >(&row)
                        .map_err(crate::api::ApiError::from)?;
                    row.update_or_insert(connection).await?;
                }
            }
            Table::ProjectsTeamsRoleRequests => {
                for row in rows {
                    let row: crate::database::nested_variants::NestedProjectsTeamsRoleRequest =
                        bincode::deserialize::<
                            crate::database::nested_variants::NestedProjectsTeamsRoleRequest,
                        >(&row)
                        .map_err(crate::api::ApiError::from)?;
                    row.update_or_insert(connection).await?;
                }
            }
            Table::ProjectsTeamsRoles => {
                for row in rows {
                    let row: crate::database::nested_variants::NestedProjectsTeamsRole =
                        bincode::deserialize::<
                            crate::database::nested_variants::NestedProjectsTeamsRole,
                        >(&row)
                        .map_err(crate::api::ApiError::from)?;
                    row.update_or_insert(connection).await?;
                }
            }
            Table::ProjectsUsersRoleInvitations => {
                for row in rows {
                    let row: crate::database::nested_variants::NestedProjectsUsersRoleInvitation =
                        bincode::deserialize::<
                            crate::database::nested_variants::NestedProjectsUsersRoleInvitation,
                        >(&row)
                        .map_err(crate::api::ApiError::from)?;
                    row.update_or_insert(connection).await?;
                }
            }
            Table::ProjectsUsersRoleRequests => {
                for row in rows {
                    let row: crate::database::nested_variants::NestedProjectsUsersRoleRequest =
                        bincode::deserialize::<
                            crate::database::nested_variants::NestedProjectsUsersRoleRequest,
                        >(&row)
                        .map_err(crate::api::ApiError::from)?;
                    row.update_or_insert(connection).await?;
                }
            }
            Table::ProjectsUsersRoles => {
                for row in rows {
                    let row: crate::database::nested_variants::NestedProjectsUsersRole =
                        bincode::deserialize::<
                            crate::database::nested_variants::NestedProjectsUsersRole,
                        >(&row)
                        .map_err(crate::api::ApiError::from)?;
                    row.update_or_insert(connection).await?;
                }
            }
            Table::Roles => {
                for row in rows {
                    let row: crate::database::nested_variants::NestedRole =
                        bincode::deserialize::<crate::database::nested_variants::NestedRole>(&row)
                            .map_err(crate::api::ApiError::from)?;
                    row.update_or_insert(connection).await?;
                }
            }
            Table::SampleBioOttTaxonItems => {
                for row in rows {
                    let row: crate::database::nested_variants::NestedSampleBioOttTaxonItem =
                        bincode::deserialize::<
                            crate::database::nested_variants::NestedSampleBioOttTaxonItem,
                        >(&row)
                        .map_err(crate::api::ApiError::from)?;
                    row.update_or_insert(connection).await?;
                }
            }
            Table::SampleContainerCategories => {
                for row in rows {
                    let row: crate::database::nested_variants::NestedSampleContainerCategory =
                        bincode::deserialize::<
                            crate::database::nested_variants::NestedSampleContainerCategory,
                        >(&row)
                        .map_err(crate::api::ApiError::from)?;
                    row.update_or_insert(connection).await?;
                }
            }
            Table::SampleContainers => {
                for row in rows {
                    let row: crate::database::nested_variants::NestedSampleContainer =
                        bincode::deserialize::<
                            crate::database::nested_variants::NestedSampleContainer,
                        >(&row)
                        .map_err(crate::api::ApiError::from)?;
                    row.update_or_insert(connection).await?;
                }
            }
            Table::SampleStates => {
                for row in rows {
                    let row: crate::database::nested_variants::NestedSampleState = bincode::deserialize::<crate::database::nested_variants::NestedSampleState>(&row).map_err(crate::api::ApiError::from)?;
                    row.update_or_insert(connection).await?;
                }
            }
            Table::Samples => {
                for row in rows {
                    let row: crate::database::nested_variants::NestedSample =
                        bincode::deserialize::<crate::database::nested_variants::NestedSample>(
                            &row,
                        )
                        .map_err(crate::api::ApiError::from)?;
                    row.update_or_insert(connection).await?;
                }
            }
            Table::Spectra => {
                for row in rows {
                    let row: crate::database::nested_variants::NestedSpectra =
                        bincode::deserialize::<crate::database::nested_variants::NestedSpectra>(
                            &row,
                        )
                        .map_err(crate::api::ApiError::from)?;
                    row.update_or_insert(connection).await?;
                }
            }
            Table::SpectraCollections => {
                for row in rows {
                    let row: crate::database::nested_variants::NestedSpectraCollection =
                        bincode::deserialize::<
                            crate::database::nested_variants::NestedSpectraCollection,
                        >(&row)
                        .map_err(crate::api::ApiError::from)?;
                    row.update_or_insert(connection).await?;
                }
            }
            Table::TeamStates => {
                for row in rows {
                    let row: crate::database::nested_variants::NestedTeamState =
                        bincode::deserialize::<crate::database::nested_variants::NestedTeamState>(
                            &row,
                        )
                        .map_err(crate::api::ApiError::from)?;
                    row.update_or_insert(connection).await?;
                }
            }
            Table::Teams => {
                for row in rows {
                    let row: crate::database::nested_variants::NestedTeam =
                        bincode::deserialize::<crate::database::nested_variants::NestedTeam>(&row)
                            .map_err(crate::api::ApiError::from)?;
                    row.update_or_insert(connection).await?;
                }
            }
            Table::TeamsTeamsRoleInvitations => {
                for row in rows {
                    let row: crate::database::nested_variants::NestedTeamsTeamsRoleInvitation =
                        bincode::deserialize::<
                            crate::database::nested_variants::NestedTeamsTeamsRoleInvitation,
                        >(&row)
                        .map_err(crate::api::ApiError::from)?;
                    row.update_or_insert(connection).await?;
                }
            }
            Table::TeamsUsersRoleInvitations => {
                for row in rows {
                    let row: crate::database::nested_variants::NestedTeamsUsersRoleInvitation =
                        bincode::deserialize::<
                            crate::database::nested_variants::NestedTeamsUsersRoleInvitation,
                        >(&row)
                        .map_err(crate::api::ApiError::from)?;
                    row.update_or_insert(connection).await?;
                }
            }
            Table::TeamsUsersRoleRequests => {
                for row in rows {
                    let row: crate::database::nested_variants::NestedTeamsUsersRoleRequest =
                        bincode::deserialize::<
                            crate::database::nested_variants::NestedTeamsUsersRoleRequest,
                        >(&row)
                        .map_err(crate::api::ApiError::from)?;
                    row.update_or_insert(connection).await?;
                }
            }
            Table::TeamsUsersRoles => {
                for row in rows {
                    let row: crate::database::nested_variants::NestedTeamsUsersRole =
                        bincode::deserialize::<
                            crate::database::nested_variants::NestedTeamsUsersRole,
                        >(&row)
                        .map_err(crate::api::ApiError::from)?;
                    row.update_or_insert(connection).await?;
                }
            }
            Table::Units => {
                for row in rows {
                    let row: crate::database::nested_variants::NestedUnit =
                        bincode::deserialize::<crate::database::nested_variants::NestedUnit>(&row)
                            .map_err(crate::api::ApiError::from)?;
                    row.update_or_insert(connection).await?;
                }
            }
            Table::UserEmails => {
                for row in rows {
                    let row: crate::database::nested_variants::NestedUserEmail =
                        bincode::deserialize::<crate::database::nested_variants::NestedUserEmail>(
                            &row,
                        )
                        .map_err(crate::api::ApiError::from)?;
                    row.update_or_insert(connection).await?;
                }
            }
            Table::Users => {
                for row in rows {
                    let row: crate::database::nested_variants::NestedUser =
                        bincode::deserialize::<crate::database::nested_variants::NestedUser>(&row)
                            .map_err(crate::api::ApiError::from)?;
                    row.update_or_insert(connection).await?;
                }
            }
            Table::UsersUsersRoleInvitations => {
                for row in rows {
                    let row: crate::database::nested_variants::NestedUsersUsersRoleInvitation =
                        bincode::deserialize::<
                            crate::database::nested_variants::NestedUsersUsersRoleInvitation,
                        >(&row)
                        .map_err(crate::api::ApiError::from)?;
                    row.update_or_insert(connection).await?;
                }
            }
            Table::UsersUsersRoleRequests => {
                for row in rows {
                    let row: crate::database::nested_variants::NestedUsersUsersRoleRequest =
                        bincode::deserialize::<
                            crate::database::nested_variants::NestedUsersUsersRoleRequest,
                        >(&row)
                        .map_err(crate::api::ApiError::from)?;
                    row.update_or_insert(connection).await?;
                }
            }
            Table::UsersUsersRoles => {
                for row in rows {
                    let row: crate::database::nested_variants::NestedUsersUsersRole =
                        bincode::deserialize::<
                            crate::database::nested_variants::NestedUsersUsersRole,
                        >(&row)
                        .map_err(crate::api::ApiError::from)?;
                    row.update_or_insert(connection).await?;
                }
            }
        }
        Ok(())
    }
}
