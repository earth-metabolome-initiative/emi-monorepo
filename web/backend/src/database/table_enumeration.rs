//! This module contains the table names enumeration.
//!
//! This module is automatically generated. Do not write anything here.

use crate::database::*;
use diesel::r2d2::ConnectionManager;
use diesel::r2d2::PooledConnection;
use web_common::database::PrimaryKey;

/// Trait providing the backend implementations for the Table enumeration
pub trait BackendTable {}

impl BackendTable for web_common::database::Table {}

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
        connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>,
    ) -> Result<Vec<u8>, web_common::api::ApiError>;
}

impl InsertableTable for web_common::database::Table {
    fn insert(
        &self,
        row: Vec<u8>,
        user_id: i32,
        connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>,
    ) -> Result<Vec<u8>, web_common::api::ApiError> {
        Ok(match self {
            web_common::database::Table::BioOttRanks => unreachable!("Table `bio_ott_ranks` is not insertable as it does not have a known column associated to a creator user id."),
            web_common::database::Table::BioOttTaxonItems => unreachable!("Table `bio_ott_taxon_items` is not insertable as it does not have a known column associated to a creator user id."),
            web_common::database::Table::Colors => unreachable!("Table `colors` is not insertable as it does not have a known column associated to a creator user id."),
            web_common::database::Table::Countries => unreachable!("Table `countries` is not insertable as it does not have a known column associated to a creator user id."),
            web_common::database::Table::DerivedSamples => {
                let row: web_common::database::new_variants::NewDerivedSample = bincode::deserialize::<web_common::database::new_variants::NewDerivedSample>(&row)?;
                let inserted_row: crate::database::flat_variants::DerivedSample = <web_common::database::new_variants::NewDerivedSample as InsertRow>::insert(row, user_id, connection)?;
                let nested_row = crate::database::nested_variants::NestedDerivedSample::from_flat(inserted_row, Some(user_id), connection)?;
                 bincode::serialize(&nested_row)?
            },
            web_common::database::Table::DocumentFormats => unreachable!("Table `document_formats` is not insertable as it does not have a known column associated to a creator user id."),
            web_common::database::Table::FontAwesomeIcons => unreachable!("Table `font_awesome_icons` is not insertable as it does not have a known column associated to a creator user id."),
            web_common::database::Table::LoginProviders => unreachable!("Table `login_providers` is not insertable as it does not have a known column associated to a creator user id."),
            web_common::database::Table::Materials => unreachable!("Table `materials` is not insertable as it does not have a known column associated to a creator user id."),
            web_common::database::Table::NameplateCategories => unreachable!("Table `nameplate_categories` is not insertable as it does not have a known column associated to a creator user id."),
            web_common::database::Table::Nameplates => {
                let row: web_common::database::new_variants::NewNameplate = bincode::deserialize::<web_common::database::new_variants::NewNameplate>(&row)?;
                let inserted_row: crate::database::flat_variants::Nameplate = <web_common::database::new_variants::NewNameplate as InsertRow>::insert(row, user_id, connection)?;
                let nested_row = crate::database::nested_variants::NestedNameplate::from_flat(inserted_row, Some(user_id), connection)?;
                 bincode::serialize(&nested_row)?
            },
            web_common::database::Table::Notifications => unreachable!("Table `notifications` is not insertable as it does not have a known column associated to a creator user id."),
            web_common::database::Table::ObservationSubjects => unreachable!("Table `observation_subjects` is not insertable as it does not have a known column associated to a creator user id."),
            web_common::database::Table::Observations => {
                let row: web_common::database::new_variants::NewObservation = bincode::deserialize::<web_common::database::new_variants::NewObservation>(&row)?;
                let inserted_row: crate::database::flat_variants::Observation = <web_common::database::new_variants::NewObservation as InsertRow>::insert(row, user_id, connection)?;
                let nested_row = crate::database::nested_variants::NestedObservation::from_flat(inserted_row, Some(user_id), connection)?;
                 bincode::serialize(&nested_row)?
            },
            web_common::database::Table::OrganismBioOttTaxonItems => {
                let row: web_common::database::new_variants::NewOrganismBioOttTaxonItem = bincode::deserialize::<web_common::database::new_variants::NewOrganismBioOttTaxonItem>(&row)?;
                let inserted_row: crate::database::flat_variants::OrganismBioOttTaxonItem = <web_common::database::new_variants::NewOrganismBioOttTaxonItem as InsertRow>::insert(row, user_id, connection)?;
                let nested_row = crate::database::nested_variants::NestedOrganismBioOttTaxonItem::from_flat(inserted_row, Some(user_id), connection)?;
                 bincode::serialize(&nested_row)?
            },
            web_common::database::Table::Organisms => {
                let row: web_common::database::new_variants::NewOrganism = bincode::deserialize::<web_common::database::new_variants::NewOrganism>(&row)?;
                let inserted_row: crate::database::flat_variants::Organism = <web_common::database::new_variants::NewOrganism as InsertRow>::insert(row, user_id, connection)?;
                let nested_row = crate::database::nested_variants::NestedOrganism::from_flat(inserted_row, Some(user_id), connection)?;
                 bincode::serialize(&nested_row)?
            },
            web_common::database::Table::Organizations => unreachable!("Table `organizations` is not insertable as it does not have a known column associated to a creator user id."),
            web_common::database::Table::PermanenceCategories => unreachable!("Table `permanence_categories` is not insertable as it does not have a known column associated to a creator user id."),
            web_common::database::Table::ProjectStates => unreachable!("Table `project_states` is not insertable as it does not have a known column associated to a creator user id."),
            web_common::database::Table::Projects => {
                let row: web_common::database::new_variants::NewProject = bincode::deserialize::<web_common::database::new_variants::NewProject>(&row)?;
                let inserted_row: crate::database::flat_variants::Project = <web_common::database::new_variants::NewProject as InsertRow>::insert(row, user_id, connection)?;
                let nested_row = crate::database::nested_variants::NestedProject::from_flat(inserted_row, Some(user_id), connection)?;
                 bincode::serialize(&nested_row)?
            },
            web_common::database::Table::ProjectsTeamsRoleInvitations => {
                let row: web_common::database::new_variants::NewProjectsTeamsRoleInvitation = bincode::deserialize::<web_common::database::new_variants::NewProjectsTeamsRoleInvitation>(&row)?;
                let inserted_row: crate::database::flat_variants::ProjectsTeamsRoleInvitation = <web_common::database::new_variants::NewProjectsTeamsRoleInvitation as InsertRow>::insert(row, user_id, connection)?;
                let nested_row = crate::database::nested_variants::NestedProjectsTeamsRoleInvitation::from_flat(inserted_row, Some(user_id), connection)?;
                 bincode::serialize(&nested_row)?
            },
            web_common::database::Table::ProjectsTeamsRoleRequests => {
                let row: web_common::database::new_variants::NewProjectsTeamsRoleRequest = bincode::deserialize::<web_common::database::new_variants::NewProjectsTeamsRoleRequest>(&row)?;
                let inserted_row: crate::database::flat_variants::ProjectsTeamsRoleRequest = <web_common::database::new_variants::NewProjectsTeamsRoleRequest as InsertRow>::insert(row, user_id, connection)?;
                let nested_row = crate::database::nested_variants::NestedProjectsTeamsRoleRequest::from_flat(inserted_row, Some(user_id), connection)?;
                 bincode::serialize(&nested_row)?
            },
            web_common::database::Table::ProjectsTeamsRoles => {
                let row: web_common::database::new_variants::NewProjectsTeamsRole = bincode::deserialize::<web_common::database::new_variants::NewProjectsTeamsRole>(&row)?;
                let inserted_row: crate::database::flat_variants::ProjectsTeamsRole = <web_common::database::new_variants::NewProjectsTeamsRole as InsertRow>::insert(row, user_id, connection)?;
                let nested_row = crate::database::nested_variants::NestedProjectsTeamsRole::from_flat(inserted_row, Some(user_id), connection)?;
                 bincode::serialize(&nested_row)?
            },
            web_common::database::Table::ProjectsUsersRoleInvitations => {
                let row: web_common::database::new_variants::NewProjectsUsersRoleInvitation = bincode::deserialize::<web_common::database::new_variants::NewProjectsUsersRoleInvitation>(&row)?;
                let inserted_row: crate::database::flat_variants::ProjectsUsersRoleInvitation = <web_common::database::new_variants::NewProjectsUsersRoleInvitation as InsertRow>::insert(row, user_id, connection)?;
                let nested_row = crate::database::nested_variants::NestedProjectsUsersRoleInvitation::from_flat(inserted_row, Some(user_id), connection)?;
                 bincode::serialize(&nested_row)?
            },
            web_common::database::Table::ProjectsUsersRoleRequests => {
                let row: web_common::database::new_variants::NewProjectsUsersRoleRequest = bincode::deserialize::<web_common::database::new_variants::NewProjectsUsersRoleRequest>(&row)?;
                let inserted_row: crate::database::flat_variants::ProjectsUsersRoleRequest = <web_common::database::new_variants::NewProjectsUsersRoleRequest as InsertRow>::insert(row, user_id, connection)?;
                let nested_row = crate::database::nested_variants::NestedProjectsUsersRoleRequest::from_flat(inserted_row, Some(user_id), connection)?;
                 bincode::serialize(&nested_row)?
            },
            web_common::database::Table::ProjectsUsersRoles => {
                let row: web_common::database::new_variants::NewProjectsUsersRole = bincode::deserialize::<web_common::database::new_variants::NewProjectsUsersRole>(&row)?;
                let inserted_row: crate::database::flat_variants::ProjectsUsersRole = <web_common::database::new_variants::NewProjectsUsersRole as InsertRow>::insert(row, user_id, connection)?;
                let nested_row = crate::database::nested_variants::NestedProjectsUsersRole::from_flat(inserted_row, Some(user_id), connection)?;
                 bincode::serialize(&nested_row)?
            },
            web_common::database::Table::Roles => unreachable!("Table `roles` is not insertable as it does not have a known column associated to a creator user id."),
            web_common::database::Table::SampleBioOttTaxonItems => {
                let row: web_common::database::new_variants::NewSampleBioOttTaxonItem = bincode::deserialize::<web_common::database::new_variants::NewSampleBioOttTaxonItem>(&row)?;
                let inserted_row: crate::database::flat_variants::SampleBioOttTaxonItem = <web_common::database::new_variants::NewSampleBioOttTaxonItem as InsertRow>::insert(row, user_id, connection)?;
                let nested_row = crate::database::nested_variants::NestedSampleBioOttTaxonItem::from_flat(inserted_row, Some(user_id), connection)?;
                 bincode::serialize(&nested_row)?
            },
            web_common::database::Table::SampleContainerCategories => unreachable!("Table `sample_container_categories` is not insertable as it does not have a known column associated to a creator user id."),
            web_common::database::Table::SampleContainers => {
                let row: web_common::database::new_variants::NewSampleContainer = bincode::deserialize::<web_common::database::new_variants::NewSampleContainer>(&row)?;
                let inserted_row: crate::database::flat_variants::SampleContainer = <web_common::database::new_variants::NewSampleContainer as InsertRow>::insert(row, user_id, connection)?;
                let nested_row = crate::database::nested_variants::NestedSampleContainer::from_flat(inserted_row, Some(user_id), connection)?;
                 bincode::serialize(&nested_row)?
            },
            web_common::database::Table::SampleStates => unreachable!("Table `sample_states` is not insertable as it does not have a known column associated to a creator user id."),
            web_common::database::Table::Samples => {
                let row: web_common::database::new_variants::NewSample = bincode::deserialize::<web_common::database::new_variants::NewSample>(&row)?;
                let inserted_row: crate::database::flat_variants::Sample = <web_common::database::new_variants::NewSample as InsertRow>::insert(row, user_id, connection)?;
                let nested_row = crate::database::nested_variants::NestedSample::from_flat(inserted_row, Some(user_id), connection)?;
                 bincode::serialize(&nested_row)?
            },
            web_common::database::Table::Spectra => unreachable!("Table `spectra` is not insertable as it does not have a known column associated to a creator user id."),
            web_common::database::Table::SpectraCollections => {
                let row: web_common::database::new_variants::NewSpectraCollection = bincode::deserialize::<web_common::database::new_variants::NewSpectraCollection>(&row)?;
                let inserted_row: crate::database::flat_variants::SpectraCollection = <web_common::database::new_variants::NewSpectraCollection as InsertRow>::insert(row, user_id, connection)?;
                let nested_row = crate::database::nested_variants::NestedSpectraCollection::from_flat(inserted_row, Some(user_id), connection)?;
                 bincode::serialize(&nested_row)?
            },
            web_common::database::Table::TeamStates => unreachable!("Table `team_states` is not insertable as it does not have a known column associated to a creator user id."),
            web_common::database::Table::Teams => {
                let row: web_common::database::new_variants::NewTeam = bincode::deserialize::<web_common::database::new_variants::NewTeam>(&row)?;
                let inserted_row: crate::database::flat_variants::Team = <web_common::database::new_variants::NewTeam as InsertRow>::insert(row, user_id, connection)?;
                let nested_row = crate::database::nested_variants::NestedTeam::from_flat(inserted_row, connection)?;
                 bincode::serialize(&nested_row)?
            },
            web_common::database::Table::TeamsTeamsRoleInvitations => {
                let row: web_common::database::new_variants::NewTeamsTeamsRoleInvitation = bincode::deserialize::<web_common::database::new_variants::NewTeamsTeamsRoleInvitation>(&row)?;
                let inserted_row: crate::database::flat_variants::TeamsTeamsRoleInvitation = <web_common::database::new_variants::NewTeamsTeamsRoleInvitation as InsertRow>::insert(row, user_id, connection)?;
                let nested_row = crate::database::nested_variants::NestedTeamsTeamsRoleInvitation::from_flat(inserted_row, connection)?;
                 bincode::serialize(&nested_row)?
            },
            web_common::database::Table::TeamsUsersRoleInvitations => {
                let row: web_common::database::new_variants::NewTeamsUsersRoleInvitation = bincode::deserialize::<web_common::database::new_variants::NewTeamsUsersRoleInvitation>(&row)?;
                let inserted_row: crate::database::flat_variants::TeamsUsersRoleInvitation = <web_common::database::new_variants::NewTeamsUsersRoleInvitation as InsertRow>::insert(row, user_id, connection)?;
                let nested_row = crate::database::nested_variants::NestedTeamsUsersRoleInvitation::from_flat(inserted_row, connection)?;
                 bincode::serialize(&nested_row)?
            },
            web_common::database::Table::TeamsUsersRoleRequests => {
                let row: web_common::database::new_variants::NewTeamsUsersRoleRequest = bincode::deserialize::<web_common::database::new_variants::NewTeamsUsersRoleRequest>(&row)?;
                let inserted_row: crate::database::flat_variants::TeamsUsersRoleRequest = <web_common::database::new_variants::NewTeamsUsersRoleRequest as InsertRow>::insert(row, user_id, connection)?;
                let nested_row = crate::database::nested_variants::NestedTeamsUsersRoleRequest::from_flat(inserted_row, connection)?;
                 bincode::serialize(&nested_row)?
            },
            web_common::database::Table::TeamsUsersRoles => {
                let row: web_common::database::new_variants::NewTeamsUsersRole = bincode::deserialize::<web_common::database::new_variants::NewTeamsUsersRole>(&row)?;
                let inserted_row: crate::database::flat_variants::TeamsUsersRole = <web_common::database::new_variants::NewTeamsUsersRole as InsertRow>::insert(row, user_id, connection)?;
                let nested_row = crate::database::nested_variants::NestedTeamsUsersRole::from_flat(inserted_row, connection)?;
                 bincode::serialize(&nested_row)?
            },
            web_common::database::Table::Units => unreachable!("Table `units` is not insertable as it does not have a known column associated to a creator user id."),
            web_common::database::Table::UserEmails => {
                let row: web_common::database::new_variants::NewUserEmail = bincode::deserialize::<web_common::database::new_variants::NewUserEmail>(&row)?;
                let inserted_row: crate::database::flat_variants::UserEmail = <web_common::database::new_variants::NewUserEmail as InsertRow>::insert(row, user_id, connection)?;
                let nested_row = crate::database::nested_variants::NestedUserEmail::from_flat(inserted_row, connection)?;
                 bincode::serialize(&nested_row)?
            },
            web_common::database::Table::Users => {
                let row: web_common::database::new_variants::NewUser = bincode::deserialize::<web_common::database::new_variants::NewUser>(&row)?;
                let inserted_row: crate::database::flat_variants::User = <web_common::database::new_variants::NewUser as InsertRow>::insert(row, user_id, connection)?;
                let nested_row = crate::database::nested_variants::NestedUser::from_flat(inserted_row, connection)?;
                 bincode::serialize(&nested_row)?
            },
            web_common::database::Table::UsersUsersRoleInvitations => {
                let row: web_common::database::new_variants::NewUsersUsersRoleInvitation = bincode::deserialize::<web_common::database::new_variants::NewUsersUsersRoleInvitation>(&row)?;
                let inserted_row: crate::database::flat_variants::UsersUsersRoleInvitation = <web_common::database::new_variants::NewUsersUsersRoleInvitation as InsertRow>::insert(row, user_id, connection)?;
                let nested_row = crate::database::nested_variants::NestedUsersUsersRoleInvitation::from_flat(inserted_row, connection)?;
                 bincode::serialize(&nested_row)?
            },
            web_common::database::Table::UsersUsersRoleRequests => {
                let row: web_common::database::new_variants::NewUsersUsersRoleRequest = bincode::deserialize::<web_common::database::new_variants::NewUsersUsersRoleRequest>(&row)?;
                let inserted_row: crate::database::flat_variants::UsersUsersRoleRequest = <web_common::database::new_variants::NewUsersUsersRoleRequest as InsertRow>::insert(row, user_id, connection)?;
                let nested_row = crate::database::nested_variants::NestedUsersUsersRoleRequest::from_flat(inserted_row, connection)?;
                 bincode::serialize(&nested_row)?
            },
            web_common::database::Table::UsersUsersRoles => {
                let row: web_common::database::new_variants::NewUsersUsersRole = bincode::deserialize::<web_common::database::new_variants::NewUsersUsersRole>(&row)?;
                let inserted_row: crate::database::flat_variants::UsersUsersRole = <web_common::database::new_variants::NewUsersUsersRole as InsertRow>::insert(row, user_id, connection)?;
                let nested_row = crate::database::nested_variants::NestedUsersUsersRole::from_flat(inserted_row, connection)?;
                 bincode::serialize(&nested_row)?
            },
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
        connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>,
    ) -> Result<Vec<u8>, web_common::api::ApiError>;
}

impl UpdatableTable for web_common::database::Table {
    fn update(
        &self,
        row: Vec<u8>,
        user_id: i32,
        connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>,
    ) -> Result<Vec<u8>, web_common::api::ApiError> {
        Ok(match self {
            web_common::database::Table::BioOttRanks => unreachable!("Table `bio_ott_ranks` is not updatable as it does not have a known column associated to an updater user id."),
            web_common::database::Table::BioOttTaxonItems => unreachable!("Table `bio_ott_taxon_items` is not updatable as it does not have a known column associated to an updater user id."),
            web_common::database::Table::Colors => unreachable!("Table `colors` is not updatable as it does not have a known column associated to an updater user id."),
            web_common::database::Table::Countries => unreachable!("Table `countries` is not updatable as it does not have a known column associated to an updater user id."),
            web_common::database::Table::DerivedSamples => {
                let row: web_common::database::UpdateDerivedSample = bincode::deserialize::<web_common::database::UpdateDerivedSample>(&row)?;
                let updated_row: crate::database::flat_variants::DerivedSample = <web_common::database::UpdateDerivedSample as UpdateRow>::update(row, user_id, connection)?;
                let nested_row = crate::database::nested_variants::NestedDerivedSample::from_flat(updated_row, Some(user_id), connection)?;
                 bincode::serialize(&nested_row)?
            },
            web_common::database::Table::DocumentFormats => unreachable!("Table `document_formats` is not updatable as it does not have a known column associated to an updater user id."),
            web_common::database::Table::FontAwesomeIcons => unreachable!("Table `font_awesome_icons` is not updatable as it does not have a known column associated to an updater user id."),
            web_common::database::Table::LoginProviders => unreachable!("Table `login_providers` is not updatable as it does not have a known column associated to an updater user id."),
            web_common::database::Table::Materials => unreachable!("Table `materials` is not updatable as it does not have a known column associated to an updater user id."),
            web_common::database::Table::NameplateCategories => unreachable!("Table `nameplate_categories` is not updatable as it does not have a known column associated to an updater user id."),
            web_common::database::Table::Nameplates => {
                let row: web_common::database::UpdateNameplate = bincode::deserialize::<web_common::database::UpdateNameplate>(&row)?;
                let updated_row: crate::database::flat_variants::Nameplate = <web_common::database::UpdateNameplate as UpdateRow>::update(row, user_id, connection)?;
                let nested_row = crate::database::nested_variants::NestedNameplate::from_flat(updated_row, Some(user_id), connection)?;
                 bincode::serialize(&nested_row)?
            },
            web_common::database::Table::Notifications => unreachable!("Table `notifications` is not updatable as it does not have a known column associated to an updater user id."),
            web_common::database::Table::ObservationSubjects => unreachable!("Table `observation_subjects` is not updatable as it does not have a known column associated to an updater user id."),
            web_common::database::Table::Observations => {
                let row: web_common::database::NewObservation = bincode::deserialize::<web_common::database::NewObservation>(&row)?;
                let updated_row: crate::database::flat_variants::Observation = <web_common::database::NewObservation as UpdateRow>::update(row, user_id, connection)?;
                let nested_row = crate::database::nested_variants::NestedObservation::from_flat(updated_row, Some(user_id), connection)?;
                 bincode::serialize(&nested_row)?
            },
            web_common::database::Table::OrganismBioOttTaxonItems => unreachable!("Table `organism_bio_ott_taxon_items` is not updatable as it does not have a known column associated to an updater user id."),
            web_common::database::Table::Organisms => {
                let row: web_common::database::NewOrganism = bincode::deserialize::<web_common::database::NewOrganism>(&row)?;
                let updated_row: crate::database::flat_variants::Organism = <web_common::database::NewOrganism as UpdateRow>::update(row, user_id, connection)?;
                let nested_row = crate::database::nested_variants::NestedOrganism::from_flat(updated_row, Some(user_id), connection)?;
                 bincode::serialize(&nested_row)?
            },
            web_common::database::Table::Organizations => unreachable!("Table `organizations` is not updatable as it does not have a known column associated to an updater user id."),
            web_common::database::Table::PermanenceCategories => unreachable!("Table `permanence_categories` is not updatable as it does not have a known column associated to an updater user id."),
            web_common::database::Table::ProjectStates => unreachable!("Table `project_states` is not updatable as it does not have a known column associated to an updater user id."),
            web_common::database::Table::Projects => {
                let row: web_common::database::UpdateProject = bincode::deserialize::<web_common::database::UpdateProject>(&row)?;
                let updated_row: crate::database::flat_variants::Project = <web_common::database::UpdateProject as UpdateRow>::update(row, user_id, connection)?;
                let nested_row = crate::database::nested_variants::NestedProject::from_flat(updated_row, Some(user_id), connection)?;
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
                let updated_row: crate::database::flat_variants::SampleContainer = <web_common::database::UpdateSampleContainer as UpdateRow>::update(row, user_id, connection)?;
                let nested_row = crate::database::nested_variants::NestedSampleContainer::from_flat(updated_row, Some(user_id), connection)?;
                 bincode::serialize(&nested_row)?
            },
            web_common::database::Table::SampleStates => unreachable!("Table `sample_states` is not updatable as it does not have a known column associated to an updater user id."),
            web_common::database::Table::Samples => {
                let row: web_common::database::NewSample = bincode::deserialize::<web_common::database::NewSample>(&row)?;
                let updated_row: crate::database::flat_variants::Sample = <web_common::database::NewSample as UpdateRow>::update(row, user_id, connection)?;
                let nested_row = crate::database::nested_variants::NestedSample::from_flat(updated_row, Some(user_id), connection)?;
                 bincode::serialize(&nested_row)?
            },
            web_common::database::Table::Spectra => unreachable!("Table `spectra` is not updatable as it does not have a known column associated to an updater user id."),
            web_common::database::Table::SpectraCollections => {
                let row: web_common::database::UpdateSpectraCollection = bincode::deserialize::<web_common::database::UpdateSpectraCollection>(&row)?;
                let updated_row: crate::database::flat_variants::SpectraCollection = <web_common::database::UpdateSpectraCollection as UpdateRow>::update(row, user_id, connection)?;
                let nested_row = crate::database::nested_variants::NestedSpectraCollection::from_flat(updated_row, Some(user_id), connection)?;
                 bincode::serialize(&nested_row)?
            },
            web_common::database::Table::TeamStates => unreachable!("Table `team_states` is not updatable as it does not have a known column associated to an updater user id."),
            web_common::database::Table::Teams => {
                let row: web_common::database::UpdateTeam = bincode::deserialize::<web_common::database::UpdateTeam>(&row)?;
                let updated_row: crate::database::flat_variants::Team = <web_common::database::UpdateTeam as UpdateRow>::update(row, user_id, connection)?;
                let nested_row = crate::database::nested_variants::NestedTeam::from_flat(updated_row, connection)?;
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
                let updated_row: crate::database::flat_variants::User = <web_common::database::UpdateUser as UpdateRow>::update(row, user_id, connection)?;
                let nested_row = crate::database::nested_variants::NestedUser::from_flat(updated_row, connection)?;
                 bincode::serialize(&nested_row)?
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
        connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>,
    ) -> Result<Vec<u8>, web_common::api::ApiError>;
}

impl FromFlatStrTable for web_common::database::Table {
    fn from_flat_str(
        &self,
        row: &str,
        user_id: Option<i32>,
        connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>,
    ) -> Result<Vec<u8>, web_common::api::ApiError> {
        Ok(match self {
            web_common::database::Table::BioOttRanks => {
                let flat_row: crate::database::flat_variants::BioOttRank =
                    serde_json::from_str::<crate::database::flat_variants::BioOttRank>(row)?;
                let richest_row = crate::database::nested_variants::NestedBioOttRank::from_flat(
                    flat_row, connection,
                )?;
                bincode::serialize(&richest_row)?
            }
            web_common::database::Table::BioOttTaxonItems => {
                let flat_row: crate::database::flat_variants::BioOttTaxonItem =
                    serde_json::from_str::<crate::database::flat_variants::BioOttTaxonItem>(row)?;
                let richest_row =
                    crate::database::nested_variants::NestedBioOttTaxonItem::from_flat(
                        flat_row, connection,
                    )?;
                bincode::serialize(&richest_row)?
            }
            web_common::database::Table::Colors => bincode::serialize(&serde_json::from_str::<
                crate::database::flat_variants::Color,
            >(row)?)?,
            web_common::database::Table::Countries => bincode::serialize(&serde_json::from_str::<
                crate::database::flat_variants::Country,
            >(row)?)?,
            web_common::database::Table::DerivedSamples => {
                let flat_row: crate::database::flat_variants::DerivedSample =
                    serde_json::from_str::<crate::database::flat_variants::DerivedSample>(row)?;
                let richest_row = crate::database::nested_variants::NestedDerivedSample::from_flat(
                    flat_row, user_id, connection,
                )?;
                bincode::serialize(&richest_row)?
            }
            web_common::database::Table::DocumentFormats => {
                let flat_row: crate::database::flat_variants::DocumentFormat =
                    serde_json::from_str::<crate::database::flat_variants::DocumentFormat>(row)?;
                let richest_row =
                    crate::database::nested_variants::NestedDocumentFormat::from_flat(
                        flat_row, connection,
                    )?;
                bincode::serialize(&richest_row)?
            }
            web_common::database::Table::FontAwesomeIcons => {
                bincode::serialize(&serde_json::from_str::<
                    crate::database::flat_variants::FontAwesomeIcon,
                >(row)?)?
            }
            web_common::database::Table::LoginProviders => {
                let flat_row: crate::database::flat_variants::LoginProvider =
                    serde_json::from_str::<crate::database::flat_variants::LoginProvider>(row)?;
                let richest_row = crate::database::nested_variants::NestedLoginProvider::from_flat(
                    flat_row, connection,
                )?;
                bincode::serialize(&richest_row)?
            }
            web_common::database::Table::Materials => {
                let flat_row: crate::database::flat_variants::Material =
                    serde_json::from_str::<crate::database::flat_variants::Material>(row)?;
                let richest_row = crate::database::nested_variants::NestedMaterial::from_flat(
                    flat_row, connection,
                )?;
                bincode::serialize(&richest_row)?
            }
            web_common::database::Table::NameplateCategories => {
                let flat_row: crate::database::flat_variants::NameplateCategory =
                    serde_json::from_str::<crate::database::flat_variants::NameplateCategory>(row)?;
                let richest_row =
                    crate::database::nested_variants::NestedNameplateCategory::from_flat(
                        flat_row, connection,
                    )?;
                bincode::serialize(&richest_row)?
            }
            web_common::database::Table::Nameplates => {
                let flat_row: crate::database::flat_variants::Nameplate =
                    serde_json::from_str::<crate::database::flat_variants::Nameplate>(row)?;
                let richest_row = crate::database::nested_variants::NestedNameplate::from_flat(
                    flat_row, user_id, connection,
                )?;
                bincode::serialize(&richest_row)?
            }
            web_common::database::Table::Notifications => {
                let flat_row: crate::database::flat_variants::Notification =
                    serde_json::from_str::<crate::database::flat_variants::Notification>(row)?;
                let richest_row = crate::database::nested_variants::NestedNotification::from_flat(
                    flat_row, connection,
                )?;
                bincode::serialize(&richest_row)?
            }
            web_common::database::Table::ObservationSubjects => {
                let flat_row: crate::database::flat_variants::ObservationSubject =
                    serde_json::from_str::<crate::database::flat_variants::ObservationSubject>(
                        row,
                    )?;
                let richest_row =
                    crate::database::nested_variants::NestedObservationSubject::from_flat(
                        flat_row, connection,
                    )?;
                bincode::serialize(&richest_row)?
            }
            web_common::database::Table::Observations => {
                let flat_row: crate::database::flat_variants::Observation =
                    serde_json::from_str::<crate::database::flat_variants::Observation>(row)?;
                let richest_row = crate::database::nested_variants::NestedObservation::from_flat(
                    flat_row, user_id, connection,
                )?;
                bincode::serialize(&richest_row)?
            }
            web_common::database::Table::OrganismBioOttTaxonItems => {
                let flat_row: crate::database::flat_variants::OrganismBioOttTaxonItem =
                    serde_json::from_str::<crate::database::flat_variants::OrganismBioOttTaxonItem>(
                        row,
                    )?;
                let richest_row =
                    crate::database::nested_variants::NestedOrganismBioOttTaxonItem::from_flat(
                        flat_row, user_id, connection,
                    )?;
                bincode::serialize(&richest_row)?
            }
            web_common::database::Table::Organisms => {
                let flat_row: crate::database::flat_variants::Organism =
                    serde_json::from_str::<crate::database::flat_variants::Organism>(row)?;
                let richest_row = crate::database::nested_variants::NestedOrganism::from_flat(
                    flat_row, user_id, connection,
                )?;
                bincode::serialize(&richest_row)?
            }
            web_common::database::Table::Organizations => {
                let flat_row: crate::database::flat_variants::Organization =
                    serde_json::from_str::<crate::database::flat_variants::Organization>(row)?;
                let richest_row = crate::database::nested_variants::NestedOrganization::from_flat(
                    flat_row, connection,
                )?;
                bincode::serialize(&richest_row)?
            }
            web_common::database::Table::PermanenceCategories => {
                let flat_row: crate::database::flat_variants::PermanenceCategory =
                    serde_json::from_str::<crate::database::flat_variants::PermanenceCategory>(
                        row,
                    )?;
                let richest_row =
                    crate::database::nested_variants::NestedPermanenceCategory::from_flat(
                        flat_row, connection,
                    )?;
                bincode::serialize(&richest_row)?
            }
            web_common::database::Table::ProjectStates => {
                let flat_row: crate::database::flat_variants::ProjectState =
                    serde_json::from_str::<crate::database::flat_variants::ProjectState>(row)?;
                let richest_row = crate::database::nested_variants::NestedProjectState::from_flat(
                    flat_row, connection,
                )?;
                bincode::serialize(&richest_row)?
            }
            web_common::database::Table::Projects => {
                let flat_row: crate::database::flat_variants::Project =
                    serde_json::from_str::<crate::database::flat_variants::Project>(row)?;
                let richest_row = crate::database::nested_variants::NestedProject::from_flat(
                    flat_row, user_id, connection,
                )?;
                bincode::serialize(&richest_row)?
            }
            web_common::database::Table::ProjectsTeamsRoleInvitations => {
                let flat_row: crate::database::flat_variants::ProjectsTeamsRoleInvitation =
                    serde_json::from_str::<
                        crate::database::flat_variants::ProjectsTeamsRoleInvitation,
                    >(row)?;
                let richest_row =
                    crate::database::nested_variants::NestedProjectsTeamsRoleInvitation::from_flat(
                        flat_row, user_id, connection,
                    )?;
                bincode::serialize(&richest_row)?
            }
            web_common::database::Table::ProjectsTeamsRoleRequests => {
                let flat_row: crate::database::flat_variants::ProjectsTeamsRoleRequest =
                    serde_json::from_str::<crate::database::flat_variants::ProjectsTeamsRoleRequest>(
                        row,
                    )?;
                let richest_row =
                    crate::database::nested_variants::NestedProjectsTeamsRoleRequest::from_flat(
                        flat_row, user_id, connection,
                    )?;
                bincode::serialize(&richest_row)?
            }
            web_common::database::Table::ProjectsTeamsRoles => {
                let flat_row: crate::database::flat_variants::ProjectsTeamsRole =
                    serde_json::from_str::<crate::database::flat_variants::ProjectsTeamsRole>(row)?;
                let richest_row =
                    crate::database::nested_variants::NestedProjectsTeamsRole::from_flat(
                        flat_row, user_id, connection,
                    )?;
                bincode::serialize(&richest_row)?
            }
            web_common::database::Table::ProjectsUsersRoleInvitations => {
                let flat_row: crate::database::flat_variants::ProjectsUsersRoleInvitation =
                    serde_json::from_str::<
                        crate::database::flat_variants::ProjectsUsersRoleInvitation,
                    >(row)?;
                let richest_row =
                    crate::database::nested_variants::NestedProjectsUsersRoleInvitation::from_flat(
                        flat_row, user_id, connection,
                    )?;
                bincode::serialize(&richest_row)?
            }
            web_common::database::Table::ProjectsUsersRoleRequests => {
                let flat_row: crate::database::flat_variants::ProjectsUsersRoleRequest =
                    serde_json::from_str::<crate::database::flat_variants::ProjectsUsersRoleRequest>(
                        row,
                    )?;
                let richest_row =
                    crate::database::nested_variants::NestedProjectsUsersRoleRequest::from_flat(
                        flat_row, user_id, connection,
                    )?;
                bincode::serialize(&richest_row)?
            }
            web_common::database::Table::ProjectsUsersRoles => {
                let flat_row: crate::database::flat_variants::ProjectsUsersRole =
                    serde_json::from_str::<crate::database::flat_variants::ProjectsUsersRole>(row)?;
                let richest_row =
                    crate::database::nested_variants::NestedProjectsUsersRole::from_flat(
                        flat_row, user_id, connection,
                    )?;
                bincode::serialize(&richest_row)?
            }
            web_common::database::Table::Roles => {
                let flat_row: crate::database::flat_variants::Role =
                    serde_json::from_str::<crate::database::flat_variants::Role>(row)?;
                let richest_row =
                    crate::database::nested_variants::NestedRole::from_flat(flat_row, connection)?;
                bincode::serialize(&richest_row)?
            }
            web_common::database::Table::SampleBioOttTaxonItems => {
                let flat_row: crate::database::flat_variants::SampleBioOttTaxonItem =
                    serde_json::from_str::<crate::database::flat_variants::SampleBioOttTaxonItem>(
                        row,
                    )?;
                let richest_row =
                    crate::database::nested_variants::NestedSampleBioOttTaxonItem::from_flat(
                        flat_row, user_id, connection,
                    )?;
                bincode::serialize(&richest_row)?
            }
            web_common::database::Table::SampleContainerCategories => {
                let flat_row: crate::database::flat_variants::SampleContainerCategory =
                    serde_json::from_str::<crate::database::flat_variants::SampleContainerCategory>(
                        row,
                    )?;
                let richest_row =
                    crate::database::nested_variants::NestedSampleContainerCategory::from_flat(
                        flat_row, connection,
                    )?;
                bincode::serialize(&richest_row)?
            }
            web_common::database::Table::SampleContainers => {
                let flat_row: crate::database::flat_variants::SampleContainer =
                    serde_json::from_str::<crate::database::flat_variants::SampleContainer>(row)?;
                let richest_row =
                    crate::database::nested_variants::NestedSampleContainer::from_flat(
                        flat_row, user_id, connection,
                    )?;
                bincode::serialize(&richest_row)?
            }
            web_common::database::Table::SampleStates => {
                let flat_row: crate::database::flat_variants::SampleState =
                    serde_json::from_str::<crate::database::flat_variants::SampleState>(row)?;
                let richest_row = crate::database::nested_variants::NestedSampleState::from_flat(
                    flat_row, connection,
                )?;
                bincode::serialize(&richest_row)?
            }
            web_common::database::Table::Samples => {
                let flat_row: crate::database::flat_variants::Sample =
                    serde_json::from_str::<crate::database::flat_variants::Sample>(row)?;
                let richest_row = crate::database::nested_variants::NestedSample::from_flat(
                    flat_row, user_id, connection,
                )?;
                bincode::serialize(&richest_row)?
            }
            web_common::database::Table::Spectra => {
                let flat_row: crate::database::flat_variants::Spectra =
                    serde_json::from_str::<crate::database::flat_variants::Spectra>(row)?;
                let richest_row = crate::database::nested_variants::NestedSpectra::from_flat(
                    flat_row, user_id, connection,
                )?;
                bincode::serialize(&richest_row)?
            }
            web_common::database::Table::SpectraCollections => {
                let flat_row: crate::database::flat_variants::SpectraCollection =
                    serde_json::from_str::<crate::database::flat_variants::SpectraCollection>(row)?;
                let richest_row =
                    crate::database::nested_variants::NestedSpectraCollection::from_flat(
                        flat_row, user_id, connection,
                    )?;
                bincode::serialize(&richest_row)?
            }
            web_common::database::Table::TeamStates => {
                let flat_row: crate::database::flat_variants::TeamState =
                    serde_json::from_str::<crate::database::flat_variants::TeamState>(row)?;
                let richest_row = crate::database::nested_variants::NestedTeamState::from_flat(
                    flat_row, connection,
                )?;
                bincode::serialize(&richest_row)?
            }
            web_common::database::Table::Teams => {
                let flat_row: crate::database::flat_variants::Team =
                    serde_json::from_str::<crate::database::flat_variants::Team>(row)?;
                let richest_row =
                    crate::database::nested_variants::NestedTeam::from_flat(flat_row, connection)?;
                bincode::serialize(&richest_row)?
            }
            web_common::database::Table::TeamsTeamsRoleInvitations => {
                let flat_row: crate::database::flat_variants::TeamsTeamsRoleInvitation =
                    serde_json::from_str::<crate::database::flat_variants::TeamsTeamsRoleInvitation>(
                        row,
                    )?;
                let richest_row =
                    crate::database::nested_variants::NestedTeamsTeamsRoleInvitation::from_flat(
                        flat_row, connection,
                    )?;
                bincode::serialize(&richest_row)?
            }
            web_common::database::Table::TeamsUsersRoleInvitations => {
                let flat_row: crate::database::flat_variants::TeamsUsersRoleInvitation =
                    serde_json::from_str::<crate::database::flat_variants::TeamsUsersRoleInvitation>(
                        row,
                    )?;
                let richest_row =
                    crate::database::nested_variants::NestedTeamsUsersRoleInvitation::from_flat(
                        flat_row, connection,
                    )?;
                bincode::serialize(&richest_row)?
            }
            web_common::database::Table::TeamsUsersRoleRequests => {
                let flat_row: crate::database::flat_variants::TeamsUsersRoleRequest =
                    serde_json::from_str::<crate::database::flat_variants::TeamsUsersRoleRequest>(
                        row,
                    )?;
                let richest_row =
                    crate::database::nested_variants::NestedTeamsUsersRoleRequest::from_flat(
                        flat_row, connection,
                    )?;
                bincode::serialize(&richest_row)?
            }
            web_common::database::Table::TeamsUsersRoles => {
                let flat_row: crate::database::flat_variants::TeamsUsersRole =
                    serde_json::from_str::<crate::database::flat_variants::TeamsUsersRole>(row)?;
                let richest_row =
                    crate::database::nested_variants::NestedTeamsUsersRole::from_flat(
                        flat_row, connection,
                    )?;
                bincode::serialize(&richest_row)?
            }
            web_common::database::Table::Units => {
                let flat_row: crate::database::flat_variants::Unit =
                    serde_json::from_str::<crate::database::flat_variants::Unit>(row)?;
                let richest_row =
                    crate::database::nested_variants::NestedUnit::from_flat(flat_row, connection)?;
                bincode::serialize(&richest_row)?
            }
            web_common::database::Table::UserEmails => {
                let flat_row: crate::database::flat_variants::UserEmail =
                    serde_json::from_str::<crate::database::flat_variants::UserEmail>(row)?;
                let richest_row = crate::database::nested_variants::NestedUserEmail::from_flat(
                    flat_row, connection,
                )?;
                bincode::serialize(&richest_row)?
            }
            web_common::database::Table::Users => {
                let flat_row: crate::database::flat_variants::User =
                    serde_json::from_str::<crate::database::flat_variants::User>(row)?;
                let richest_row =
                    crate::database::nested_variants::NestedUser::from_flat(flat_row, connection)?;
                bincode::serialize(&richest_row)?
            }
            web_common::database::Table::UsersUsersRoleInvitations => {
                let flat_row: crate::database::flat_variants::UsersUsersRoleInvitation =
                    serde_json::from_str::<crate::database::flat_variants::UsersUsersRoleInvitation>(
                        row,
                    )?;
                let richest_row =
                    crate::database::nested_variants::NestedUsersUsersRoleInvitation::from_flat(
                        flat_row, connection,
                    )?;
                bincode::serialize(&richest_row)?
            }
            web_common::database::Table::UsersUsersRoleRequests => {
                let flat_row: crate::database::flat_variants::UsersUsersRoleRequest =
                    serde_json::from_str::<crate::database::flat_variants::UsersUsersRoleRequest>(
                        row,
                    )?;
                let richest_row =
                    crate::database::nested_variants::NestedUsersUsersRoleRequest::from_flat(
                        flat_row, connection,
                    )?;
                bincode::serialize(&richest_row)?
            }
            web_common::database::Table::UsersUsersRoles => {
                let flat_row: crate::database::flat_variants::UsersUsersRole =
                    serde_json::from_str::<crate::database::flat_variants::UsersUsersRole>(row)?;
                let richest_row =
                    crate::database::nested_variants::NestedUsersUsersRole::from_flat(
                        flat_row, connection,
                    )?;
                bincode::serialize(&richest_row)?
            }
        })
    }
}
