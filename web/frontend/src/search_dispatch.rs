//! This module contains the search method for the Table enum.
//!
//! This module is automatically generated. Do not write anything here.

use crate::search::*;
use web_common::database::*;
pub(crate) trait SearchableTable {
    fn search<C>(
        &self,
        lowercase_query: &str,
        filter: Option<Vec<u8>>,
        limit: Option<i64>,
        offset: Option<i64>,
        connection: &mut gluesql::prelude::Glue<C>,
    ) -> impl std::future::Future<Output = Result<Vec<u8>, web_common::api::ApiError>>
    where
        C: gluesql::core::store::GStore + gluesql::core::store::GStoreMut;
}

impl SearchableTable for Table {
    async fn search<C>(
        &self,
        lowercase_query: &str,
        filter: Option<Vec<u8>>,
        limit: Option<i64>,
        offset: Option<i64>,
        connection: &mut gluesql::prelude::Glue<C>,
    ) -> Result<Vec<u8>, web_common::api::ApiError>
    where
        C: gluesql::core::store::GStore + gluesql::core::store::GStoreMut,
    {
        Ok(match self {
            Table::BioOttRanks => {
                let filter = filter
                    .map(|filter| bincode::deserialize(&filter))
                    .transpose()?;
                bincode::serialize(
                    &search::<C, web_common::database::nested_variants::NestedBioOttRank>(
                        lowercase_query,
                        filter.as_ref(),
                        limit,
                        offset,
                        connection,
                    )
                    .await?,
                )?
            }
            Table::BioOttTaxonItems => {
                let filter = filter
                    .map(|filter| bincode::deserialize(&filter))
                    .transpose()?;
                bincode::serialize(
                    &search::<C, web_common::database::nested_variants::NestedBioOttTaxonItem>(
                        lowercase_query,
                        filter.as_ref(),
                        limit,
                        offset,
                        connection,
                    )
                    .await?,
                )?
            }
            Table::Colors => {
                let filter = filter
                    .map(|filter| bincode::deserialize(&filter))
                    .transpose()?;
                bincode::serialize(
                    &search::<C, web_common::database::flat_variants::Color>(
                        lowercase_query,
                        filter.as_ref(),
                        limit,
                        offset,
                        connection,
                    )
                    .await?,
                )?
            }
            Table::Countries => {
                let filter = filter
                    .map(|filter| bincode::deserialize(&filter))
                    .transpose()?;
                bincode::serialize(
                    &search::<C, web_common::database::flat_variants::Country>(
                        lowercase_query,
                        filter.as_ref(),
                        limit,
                        offset,
                        connection,
                    )
                    .await?,
                )?
            }
            Table::DerivedSamples => {
                let filter = filter
                    .map(|filter| bincode::deserialize(&filter))
                    .transpose()?;
                bincode::serialize(
                    &search::<C, web_common::database::nested_variants::NestedDerivedSample>(
                        lowercase_query,
                        filter.as_ref(),
                        limit,
                        offset,
                        connection,
                    )
                    .await?,
                )?
            }
            Table::DocumentFormats => {
                let filter = filter
                    .map(|filter| bincode::deserialize(&filter))
                    .transpose()?;
                bincode::serialize(
                    &search::<C, web_common::database::nested_variants::NestedDocumentFormat>(
                        lowercase_query,
                        filter.as_ref(),
                        limit,
                        offset,
                        connection,
                    )
                    .await?,
                )?
            }
            Table::FontAwesomeIcons => {
                let filter = filter
                    .map(|filter| bincode::deserialize(&filter))
                    .transpose()?;
                bincode::serialize(
                    &search::<C, web_common::database::flat_variants::FontAwesomeIcon>(
                        lowercase_query,
                        filter.as_ref(),
                        limit,
                        offset,
                        connection,
                    )
                    .await?,
                )?
            }
            Table::LoginProviders => {
                let filter = filter
                    .map(|filter| bincode::deserialize(&filter))
                    .transpose()?;
                bincode::serialize(
                    &search::<C, web_common::database::nested_variants::NestedLoginProvider>(
                        lowercase_query,
                        filter.as_ref(),
                        limit,
                        offset,
                        connection,
                    )
                    .await?,
                )?
            }
            Table::Materials => {
                let filter = filter
                    .map(|filter| bincode::deserialize(&filter))
                    .transpose()?;
                bincode::serialize(
                    &search::<C, web_common::database::nested_variants::NestedMaterial>(
                        lowercase_query,
                        filter.as_ref(),
                        limit,
                        offset,
                        connection,
                    )
                    .await?,
                )?
            }
            Table::NameplateCategories => {
                let filter = filter
                    .map(|filter| bincode::deserialize(&filter))
                    .transpose()?;
                bincode::serialize(
                    &search::<C, web_common::database::nested_variants::NestedNameplateCategory>(
                        lowercase_query,
                        filter.as_ref(),
                        limit,
                        offset,
                        connection,
                    )
                    .await?,
                )?
            }
            Table::Nameplates => {
                let filter = filter
                    .map(|filter| bincode::deserialize(&filter))
                    .transpose()?;
                bincode::serialize(
                    &search::<C, web_common::database::nested_variants::NestedNameplate>(
                        lowercase_query,
                        filter.as_ref(),
                        limit,
                        offset,
                        connection,
                    )
                    .await?,
                )?
            }
            Table::Notifications => {
                let filter = filter
                    .map(|filter| bincode::deserialize(&filter))
                    .transpose()?;
                bincode::serialize(
                    &search::<C, web_common::database::nested_variants::NestedNotification>(
                        lowercase_query,
                        filter.as_ref(),
                        limit,
                        offset,
                        connection,
                    )
                    .await?,
                )?
            }
            Table::ObservationSubjects => {
                let filter = filter
                    .map(|filter| bincode::deserialize(&filter))
                    .transpose()?;
                bincode::serialize(
                    &search::<C, web_common::database::nested_variants::NestedObservationSubject>(
                        lowercase_query,
                        filter.as_ref(),
                        limit,
                        offset,
                        connection,
                    )
                    .await?,
                )?
            }
            Table::Observations => {
                let filter = filter
                    .map(|filter| bincode::deserialize(&filter))
                    .transpose()?;
                bincode::serialize(
                    &search::<C, web_common::database::nested_variants::NestedObservation>(
                        lowercase_query,
                        filter.as_ref(),
                        limit,
                        offset,
                        connection,
                    )
                    .await?,
                )?
            }
            Table::OrganismBioOttTaxonItems => {
                let filter = filter
                    .map(|filter| bincode::deserialize(&filter))
                    .transpose()?;
                bincode::serialize(
                    &search::<
                        C,
                        web_common::database::nested_variants::NestedOrganismBioOttTaxonItem,
                    >(
                        lowercase_query, filter.as_ref(), limit, offset, connection
                    )
                    .await?,
                )?
            }
            Table::Organisms => {
                let filter = filter
                    .map(|filter| bincode::deserialize(&filter))
                    .transpose()?;
                bincode::serialize(
                    &search::<C, web_common::database::nested_variants::NestedOrganism>(
                        lowercase_query,
                        filter.as_ref(),
                        limit,
                        offset,
                        connection,
                    )
                    .await?,
                )?
            }
            Table::Organizations => {
                let filter = filter
                    .map(|filter| bincode::deserialize(&filter))
                    .transpose()?;
                bincode::serialize(
                    &search::<C, web_common::database::nested_variants::NestedOrganization>(
                        lowercase_query,
                        filter.as_ref(),
                        limit,
                        offset,
                        connection,
                    )
                    .await?,
                )?
            }
            Table::PermanenceCategories => {
                let filter = filter
                    .map(|filter| bincode::deserialize(&filter))
                    .transpose()?;
                bincode::serialize(
                    &search::<C, web_common::database::nested_variants::NestedPermanenceCategory>(
                        lowercase_query,
                        filter.as_ref(),
                        limit,
                        offset,
                        connection,
                    )
                    .await?,
                )?
            }
            Table::ProjectStates => {
                let filter = filter
                    .map(|filter| bincode::deserialize(&filter))
                    .transpose()?;
                bincode::serialize(
                    &search::<C, web_common::database::nested_variants::NestedProjectState>(
                        lowercase_query,
                        filter.as_ref(),
                        limit,
                        offset,
                        connection,
                    )
                    .await?,
                )?
            }
            Table::Projects => {
                let filter = filter
                    .map(|filter| bincode::deserialize(&filter))
                    .transpose()?;
                bincode::serialize(
                    &search::<C, web_common::database::nested_variants::NestedProject>(
                        lowercase_query,
                        filter.as_ref(),
                        limit,
                        offset,
                        connection,
                    )
                    .await?,
                )?
            }
            Table::ProjectsTeamsRoleInvitations => {
                let filter = filter
                    .map(|filter| bincode::deserialize(&filter))
                    .transpose()?;
                bincode::serialize(
                    &search::<
                        C,
                        web_common::database::nested_variants::NestedProjectsTeamsRoleInvitation,
                    >(
                        lowercase_query, filter.as_ref(), limit, offset, connection
                    )
                    .await?,
                )?
            }
            Table::ProjectsTeamsRoleRequests => {
                let filter = filter
                    .map(|filter| bincode::deserialize(&filter))
                    .transpose()?;
                bincode::serialize(
                    &search::<
                        C,
                        web_common::database::nested_variants::NestedProjectsTeamsRoleRequest,
                    >(
                        lowercase_query, filter.as_ref(), limit, offset, connection
                    )
                    .await?,
                )?
            }
            Table::ProjectsTeamsRoles => {
                let filter = filter
                    .map(|filter| bincode::deserialize(&filter))
                    .transpose()?;
                bincode::serialize(
                    &search::<C, web_common::database::nested_variants::NestedProjectsTeamsRole>(
                        lowercase_query,
                        filter.as_ref(),
                        limit,
                        offset,
                        connection,
                    )
                    .await?,
                )?
            }
            Table::ProjectsUsersRoleInvitations => {
                let filter = filter
                    .map(|filter| bincode::deserialize(&filter))
                    .transpose()?;
                bincode::serialize(
                    &search::<
                        C,
                        web_common::database::nested_variants::NestedProjectsUsersRoleInvitation,
                    >(
                        lowercase_query, filter.as_ref(), limit, offset, connection
                    )
                    .await?,
                )?
            }
            Table::ProjectsUsersRoleRequests => {
                let filter = filter
                    .map(|filter| bincode::deserialize(&filter))
                    .transpose()?;
                bincode::serialize(
                    &search::<
                        C,
                        web_common::database::nested_variants::NestedProjectsUsersRoleRequest,
                    >(
                        lowercase_query, filter.as_ref(), limit, offset, connection
                    )
                    .await?,
                )?
            }
            Table::ProjectsUsersRoles => {
                let filter = filter
                    .map(|filter| bincode::deserialize(&filter))
                    .transpose()?;
                bincode::serialize(
                    &search::<C, web_common::database::nested_variants::NestedProjectsUsersRole>(
                        lowercase_query,
                        filter.as_ref(),
                        limit,
                        offset,
                        connection,
                    )
                    .await?,
                )?
            }
            Table::Roles => {
                let filter = filter
                    .map(|filter| bincode::deserialize(&filter))
                    .transpose()?;
                bincode::serialize(
                    &search::<C, web_common::database::nested_variants::NestedRole>(
                        lowercase_query,
                        filter.as_ref(),
                        limit,
                        offset,
                        connection,
                    )
                    .await?,
                )?
            }
            Table::SampleBioOttTaxonItems => {
                let filter = filter
                    .map(|filter| bincode::deserialize(&filter))
                    .transpose()?;
                bincode::serialize(&search::<C, web_common::database::nested_variants::NestedSampleBioOttTaxonItem>(
         lowercase_query,
         filter.as_ref(),
         limit,
         offset,
         connection
     ).await?)?
            }
            Table::SampleContainerCategories => {
                let filter = filter
                    .map(|filter| bincode::deserialize(&filter))
                    .transpose()?;
                bincode::serialize(
                    &search::<
                        C,
                        web_common::database::nested_variants::NestedSampleContainerCategory,
                    >(
                        lowercase_query, filter.as_ref(), limit, offset, connection
                    )
                    .await?,
                )?
            }
            Table::SampleContainers => {
                let filter = filter
                    .map(|filter| bincode::deserialize(&filter))
                    .transpose()?;
                bincode::serialize(
                    &search::<C, web_common::database::nested_variants::NestedSampleContainer>(
                        lowercase_query,
                        filter.as_ref(),
                        limit,
                        offset,
                        connection,
                    )
                    .await?,
                )?
            }
            Table::SampleStates => {
                let filter = filter
                    .map(|filter| bincode::deserialize(&filter))
                    .transpose()?;
                bincode::serialize(
                    &search::<C, web_common::database::nested_variants::NestedSampleState>(
                        lowercase_query,
                        filter.as_ref(),
                        limit,
                        offset,
                        connection,
                    )
                    .await?,
                )?
            }
            Table::Samples => {
                let filter = filter
                    .map(|filter| bincode::deserialize(&filter))
                    .transpose()?;
                bincode::serialize(
                    &search::<C, web_common::database::nested_variants::NestedSample>(
                        lowercase_query,
                        filter.as_ref(),
                        limit,
                        offset,
                        connection,
                    )
                    .await?,
                )?
            }
            Table::Spectra => {
                let filter = filter
                    .map(|filter| bincode::deserialize(&filter))
                    .transpose()?;
                bincode::serialize(
                    &search::<C, web_common::database::nested_variants::NestedSpectra>(
                        lowercase_query,
                        filter.as_ref(),
                        limit,
                        offset,
                        connection,
                    )
                    .await?,
                )?
            }
            Table::SpectraCollections => {
                let filter = filter
                    .map(|filter| bincode::deserialize(&filter))
                    .transpose()?;
                bincode::serialize(
                    &search::<C, web_common::database::nested_variants::NestedSpectraCollection>(
                        lowercase_query,
                        filter.as_ref(),
                        limit,
                        offset,
                        connection,
                    )
                    .await?,
                )?
            }
            Table::TeamStates => {
                let filter = filter
                    .map(|filter| bincode::deserialize(&filter))
                    .transpose()?;
                bincode::serialize(
                    &search::<C, web_common::database::nested_variants::NestedTeamState>(
                        lowercase_query,
                        filter.as_ref(),
                        limit,
                        offset,
                        connection,
                    )
                    .await?,
                )?
            }
            Table::Teams => {
                let filter = filter
                    .map(|filter| bincode::deserialize(&filter))
                    .transpose()?;
                bincode::serialize(
                    &search::<C, web_common::database::nested_variants::NestedTeam>(
                        lowercase_query,
                        filter.as_ref(),
                        limit,
                        offset,
                        connection,
                    )
                    .await?,
                )?
            }
            Table::TeamsTeamsRoleInvitations => {
                let filter = filter
                    .map(|filter| bincode::deserialize(&filter))
                    .transpose()?;
                bincode::serialize(
                    &search::<
                        C,
                        web_common::database::nested_variants::NestedTeamsTeamsRoleInvitation,
                    >(
                        lowercase_query, filter.as_ref(), limit, offset, connection
                    )
                    .await?,
                )?
            }
            Table::TeamsUsersRoleInvitations => {
                let filter = filter
                    .map(|filter| bincode::deserialize(&filter))
                    .transpose()?;
                bincode::serialize(
                    &search::<
                        C,
                        web_common::database::nested_variants::NestedTeamsUsersRoleInvitation,
                    >(
                        lowercase_query, filter.as_ref(), limit, offset, connection
                    )
                    .await?,
                )?
            }
            Table::TeamsUsersRoleRequests => {
                let filter = filter
                    .map(|filter| bincode::deserialize(&filter))
                    .transpose()?;
                bincode::serialize(&search::<C, web_common::database::nested_variants::NestedTeamsUsersRoleRequest>(
         lowercase_query,
         filter.as_ref(),
         limit,
         offset,
         connection
     ).await?)?
            }
            Table::TeamsUsersRoles => {
                let filter = filter
                    .map(|filter| bincode::deserialize(&filter))
                    .transpose()?;
                bincode::serialize(
                    &search::<C, web_common::database::nested_variants::NestedTeamsUsersRole>(
                        lowercase_query,
                        filter.as_ref(),
                        limit,
                        offset,
                        connection,
                    )
                    .await?,
                )?
            }
            Table::Units => {
                let filter = filter
                    .map(|filter| bincode::deserialize(&filter))
                    .transpose()?;
                bincode::serialize(
                    &search::<C, web_common::database::nested_variants::NestedUnit>(
                        lowercase_query,
                        filter.as_ref(),
                        limit,
                        offset,
                        connection,
                    )
                    .await?,
                )?
            }
            Table::UserEmails => {
                let filter = filter
                    .map(|filter| bincode::deserialize(&filter))
                    .transpose()?;
                bincode::serialize(
                    &search::<C, web_common::database::nested_variants::NestedUserEmail>(
                        lowercase_query,
                        filter.as_ref(),
                        limit,
                        offset,
                        connection,
                    )
                    .await?,
                )?
            }
            Table::Users => {
                let filter = filter
                    .map(|filter| bincode::deserialize(&filter))
                    .transpose()?;
                bincode::serialize(
                    &search::<C, web_common::database::nested_variants::NestedUser>(
                        lowercase_query,
                        filter.as_ref(),
                        limit,
                        offset,
                        connection,
                    )
                    .await?,
                )?
            }
            Table::UsersUsersRoleInvitations => {
                let filter = filter
                    .map(|filter| bincode::deserialize(&filter))
                    .transpose()?;
                bincode::serialize(
                    &search::<
                        C,
                        web_common::database::nested_variants::NestedUsersUsersRoleInvitation,
                    >(
                        lowercase_query, filter.as_ref(), limit, offset, connection
                    )
                    .await?,
                )?
            }
            Table::UsersUsersRoleRequests => {
                let filter = filter
                    .map(|filter| bincode::deserialize(&filter))
                    .transpose()?;
                bincode::serialize(&search::<C, web_common::database::nested_variants::NestedUsersUsersRoleRequest>(
         lowercase_query,
         filter.as_ref(),
         limit,
         offset,
         connection
     ).await?)?
            }
            Table::UsersUsersRoles => {
                let filter = filter
                    .map(|filter| bincode::deserialize(&filter))
                    .transpose()?;
                bincode::serialize(
                    &search::<C, web_common::database::nested_variants::NestedUsersUsersRole>(
                        lowercase_query,
                        filter.as_ref(),
                        limit,
                        offset,
                        connection,
                    )
                    .await?,
                )?
            }
        })
    }
}
