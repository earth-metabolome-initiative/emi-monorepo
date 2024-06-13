//! This module contains the database schema for the frontend.
//!
//! This module is automatically generated. Do not write anything here.

use gluesql::prelude::Glue;
use gluesql::prelude::IdbStorage;
use sql_minifier::macros::load_sql;

pub(super) async fn create_schema(database: &mut Glue<IdbStorage>) {
    if let Err(error) = database
        .execute(load_sql!(
            "../backend/migrations/00000000000005_create_colors_table/up.sql"
        ))
        .await
    {
        log::error!(
            "Failed to create table migrations/00000000000005_create_colors_table/up.sql: {:?}",
            error
        );
        unreachable!("Failed to create table");
    }
    if let Err(error) = database
        .execute(load_sql!(
            "../backend/migrations/00000000000009_create_countries_table/up.sql"
        ))
        .await
    {
        log::error!(
            "Failed to create table migrations/00000000000009_create_countries_table/up.sql: {:?}",
            error
        );
        unreachable!("Failed to create table");
    }
    if let Err(error) = database
        .execute(load_sql!(
            "../backend/migrations/00000000000013_create_font_awesome_icons_table/up.sql"
        ))
        .await
    {
        log::error!("Failed to create table migrations/00000000000013_create_font_awesome_icons_table/up.sql: {:?}", error);
        unreachable!("Failed to create table");
    }
    if let Err(error) = database
        .execute(load_sql!(
            "../backend/migrations/00000000000017_create_bio_ott_ranks_table/up.sql"
        ))
        .await
    {
        log::error!("Failed to create table migrations/00000000000017_create_bio_ott_ranks_table/up.sql: {:?}", error);
        unreachable!("Failed to create table");
    }
    if let Err(error) = database
        .execute(load_sql!(
            "../backend/migrations/00000000000021_create_bio_ott_taxon_items_table/up.sql"
        ))
        .await
    {
        log::error!("Failed to create table migrations/00000000000021_create_bio_ott_taxon_items_table/up.sql: {:?}", error);
        unreachable!("Failed to create table");
    }
    if let Err(error) = database
        .execute(load_sql!(
            "../backend/migrations/00000000000026_create_document_formats_table/up.sql"
        ))
        .await
    {
        log::error!("Failed to create table migrations/00000000000026_create_document_formats_table/up.sql: {:?}", error);
        unreachable!("Failed to create table");
    }
    if let Err(error) = database
        .execute(load_sql!(
            "../backend/migrations/00000000000030_create_login_providers_table/up.sql"
        ))
        .await
    {
        log::error!("Failed to create table migrations/00000000000030_create_login_providers_table/up.sql: {:?}", error);
        unreachable!("Failed to create table");
    }
    if let Err(error) = database
        .execute(load_sql!(
            "../backend/migrations/00000000000033_create_materials_table/up.sql"
        ))
        .await
    {
        log::error!(
            "Failed to create table migrations/00000000000033_create_materials_table/up.sql: {:?}",
            error
        );
        unreachable!("Failed to create table");
    }
    if let Err(error) = database
        .execute(load_sql!(
            "../backend/migrations/00000000000036_create_observation_subjects_table/up.sql"
        ))
        .await
    {
        log::error!("Failed to create table migrations/00000000000036_create_observation_subjects_table/up.sql: {:?}", error);
        unreachable!("Failed to create table");
    }
    if let Err(error) = database
        .execute(load_sql!(
            "../backend/migrations/00000000000040_create_organizations_table/up.sql"
        ))
        .await
    {
        log::error!("Failed to create table migrations/00000000000040_create_organizations_table/up.sql: {:?}", error);
        unreachable!("Failed to create table");
    }
    if let Err(error) = database
        .execute(load_sql!(
            "../backend/migrations/00000000000044_create_permanence_categories_table/up.sql"
        ))
        .await
    {
        log::error!("Failed to create table migrations/00000000000044_create_permanence_categories_table/up.sql: {:?}", error);
        unreachable!("Failed to create table");
    }
    if let Err(error) = database
        .execute(load_sql!(
            "../backend/migrations/00000000000047_create_nameplate_categories_table/up.sql"
        ))
        .await
    {
        log::error!("Failed to create table migrations/00000000000047_create_nameplate_categories_table/up.sql: {:?}", error);
        unreachable!("Failed to create table");
    }
    if let Err(error) = database
        .execute(load_sql!(
            "../backend/migrations/00000000000051_create_project_states_table/up.sql"
        ))
        .await
    {
        log::error!("Failed to create table migrations/00000000000051_create_project_states_table/up.sql: {:?}", error);
        unreachable!("Failed to create table");
    }
    if let Err(error) = database
        .execute(load_sql!(
            "../backend/migrations/00000000000055_create_roles_table/up.sql"
        ))
        .await
    {
        log::error!(
            "Failed to create table migrations/00000000000055_create_roles_table/up.sql: {:?}",
            error
        );
        unreachable!("Failed to create table");
    }
    if let Err(error) = database
        .execute(load_sql!(
            "../backend/migrations/00000000000059_create_sample_container_categories_table/up.sql"
        ))
        .await
    {
        log::error!("Failed to create table migrations/00000000000059_create_sample_container_categories_table/up.sql: {:?}", error);
        unreachable!("Failed to create table");
    }
    if let Err(error) = database
        .execute(load_sql!(
            "../backend/migrations/00000000000063_create_sample_states_table/up.sql"
        ))
        .await
    {
        log::error!("Failed to create table migrations/00000000000063_create_sample_states_table/up.sql: {:?}", error);
        unreachable!("Failed to create table");
    }
    if let Err(error) = database
        .execute(load_sql!(
            "../backend/migrations/00000000000067_create_team_states_table/up.sql"
        ))
        .await
    {
        log::error!("Failed to create table migrations/00000000000067_create_team_states_table/up.sql: {:?}", error);
        unreachable!("Failed to create table");
    }
    if let Err(error) = database
        .execute(load_sql!(
            "../backend/migrations/00000000000071_create_units_table/up.sql"
        ))
        .await
    {
        log::error!(
            "Failed to create table migrations/00000000000071_create_units_table/up.sql: {:?}",
            error
        );
        unreachable!("Failed to create table");
    }
    if let Err(error) = database
        .execute(load_sql!(
            "../backend/migrations/00000000000075_create_users_table/up.sql"
        ))
        .await
    {
        log::error!(
            "Failed to create table migrations/00000000000075_create_users_table/up.sql: {:?}",
            error
        );
        unreachable!("Failed to create table");
    }
    if let Err(error) = database
        .execute(load_sql!(
            "../backend/migrations/00000000000081_create_notifications_table/up.sql"
        ))
        .await
    {
        log::error!("Failed to create table migrations/00000000000081_create_notifications_table/up.sql: {:?}", error);
        unreachable!("Failed to create table");
    }
    if let Err(error) = database
        .execute(load_sql!(
            "../backend/migrations/00000000000083_create_projects_table/up.sql"
        ))
        .await
    {
        log::error!(
            "Failed to create table migrations/00000000000083_create_projects_table/up.sql: {:?}",
            error
        );
        unreachable!("Failed to create table");
    }
    if let Err(error) = database
        .execute(load_sql!(
            "../backend/migrations/00000000000089_create_nameplates_table/up.sql"
        ))
        .await
    {
        log::error!(
            "Failed to create table migrations/00000000000089_create_nameplates_table/up.sql: {:?}",
            error
        );
        unreachable!("Failed to create table");
    }
    if let Err(error) = database.execute(load_sql!("../backend/migrations/00000000000095_create_projects_users_role_invitations_table/up.sql")).await {
    log::error!("Failed to create table migrations/00000000000095_create_projects_users_role_invitations_table/up.sql: {:?}", error);
     unreachable!("Failed to create table");
}
    if let Err(error) = database
        .execute(load_sql!(
            "../backend/migrations/00000000000097_create_projects_users_role_requests_table/up.sql"
        ))
        .await
    {
        log::error!("Failed to create table migrations/00000000000097_create_projects_users_role_requests_table/up.sql: {:?}", error);
        unreachable!("Failed to create table");
    }
    if let Err(error) = database
        .execute(load_sql!(
            "../backend/migrations/00000000000099_create_projects_users_roles_table/up.sql"
        ))
        .await
    {
        log::error!("Failed to create table migrations/00000000000099_create_projects_users_roles_table/up.sql: {:?}", error);
        unreachable!("Failed to create table");
    }
    if let Err(error) = database
        .execute(load_sql!(
            "../backend/migrations/00000000000101_create_sample_containers_table/up.sql"
        ))
        .await
    {
        log::error!("Failed to create table migrations/00000000000101_create_sample_containers_table/up.sql: {:?}", error);
        unreachable!("Failed to create table");
    }
    if let Err(error) = database
        .execute(load_sql!(
            "../backend/migrations/00000000000106_create_samples_table/up.sql"
        ))
        .await
    {
        log::error!(
            "Failed to create table migrations/00000000000106_create_samples_table/up.sql: {:?}",
            error
        );
        unreachable!("Failed to create table");
    }
    if let Err(error) = database
        .execute(load_sql!(
            "../backend/migrations/00000000000109_create_derived_samples_table/up.sql"
        ))
        .await
    {
        log::error!("Failed to create table migrations/00000000000109_create_derived_samples_table/up.sql: {:?}", error);
        unreachable!("Failed to create table");
    }
    if let Err(error) = database
        .execute(load_sql!(
            "../backend/migrations/00000000000112_create_organisms_table/up.sql"
        ))
        .await
    {
        log::error!(
            "Failed to create table migrations/00000000000112_create_organisms_table/up.sql: {:?}",
            error
        );
        unreachable!("Failed to create table");
    }
    if let Err(error) = database
        .execute(load_sql!(
            "../backend/migrations/00000000000116_create_observations_table/up.sql"
        ))
        .await
    {
        log::error!("Failed to create table migrations/00000000000116_create_observations_table/up.sql: {:?}", error);
        unreachable!("Failed to create table");
    }
    if let Err(error) = database
        .execute(load_sql!(
            "../backend/migrations/00000000000121_create_organism_bio_ott_taxon_items_table/up.sql"
        ))
        .await
    {
        log::error!("Failed to create table migrations/00000000000121_create_organism_bio_ott_taxon_items_table/up.sql: {:?}", error);
        unreachable!("Failed to create table");
    }
    if let Err(error) = database
        .execute(load_sql!(
            "../backend/migrations/00000000000123_create_sample_bio_ott_taxon_items_table/up.sql"
        ))
        .await
    {
        log::error!("Failed to create table migrations/00000000000123_create_sample_bio_ott_taxon_items_table/up.sql: {:?}", error);
        unreachable!("Failed to create table");
    }
    if let Err(error) = database
        .execute(load_sql!(
            "../backend/migrations/00000000000125_create_spectra_collections_table/up.sql"
        ))
        .await
    {
        log::error!("Failed to create table migrations/00000000000125_create_spectra_collections_table/up.sql: {:?}", error);
        unreachable!("Failed to create table");
    }
    if let Err(error) = database
        .execute(load_sql!(
            "../backend/migrations/00000000000129_create_spectra_table/up.sql"
        ))
        .await
    {
        log::error!(
            "Failed to create table migrations/00000000000129_create_spectra_table/up.sql: {:?}",
            error
        );
        unreachable!("Failed to create table");
    }
    if let Err(error) = database
        .execute(load_sql!(
            "../backend/migrations/00000000000132_create_teams_table/up.sql"
        ))
        .await
    {
        log::error!(
            "Failed to create table migrations/00000000000132_create_teams_table/up.sql: {:?}",
            error
        );
        unreachable!("Failed to create table");
    }
    if let Err(error) = database.execute(load_sql!("../backend/migrations/00000000000138_create_projects_teams_role_invitations_table/up.sql")).await {
    log::error!("Failed to create table migrations/00000000000138_create_projects_teams_role_invitations_table/up.sql: {:?}", error);
     unreachable!("Failed to create table");
}
    if let Err(error) = database
        .execute(load_sql!(
            "../backend/migrations/00000000000140_create_projects_teams_role_requests_table/up.sql"
        ))
        .await
    {
        log::error!("Failed to create table migrations/00000000000140_create_projects_teams_role_requests_table/up.sql: {:?}", error);
        unreachable!("Failed to create table");
    }
    if let Err(error) = database
        .execute(load_sql!(
            "../backend/migrations/00000000000142_create_projects_teams_roles_table/up.sql"
        ))
        .await
    {
        log::error!("Failed to create table migrations/00000000000142_create_projects_teams_roles_table/up.sql: {:?}", error);
        unreachable!("Failed to create table");
    }
    if let Err(error) = database
        .execute(load_sql!(
            "../backend/migrations/00000000000144_create_teams_teams_role_invitations_table/up.sql"
        ))
        .await
    {
        log::error!("Failed to create table migrations/00000000000144_create_teams_teams_role_invitations_table/up.sql: {:?}", error);
        unreachable!("Failed to create table");
    }
    if let Err(error) = database
        .execute(load_sql!(
            "../backend/migrations/00000000000146_create_teams_users_role_invitations_table/up.sql"
        ))
        .await
    {
        log::error!("Failed to create table migrations/00000000000146_create_teams_users_role_invitations_table/up.sql: {:?}", error);
        unreachable!("Failed to create table");
    }
    if let Err(error) = database
        .execute(load_sql!(
            "../backend/migrations/00000000000148_create_teams_users_role_requests_table/up.sql"
        ))
        .await
    {
        log::error!("Failed to create table migrations/00000000000148_create_teams_users_role_requests_table/up.sql: {:?}", error);
        unreachable!("Failed to create table");
    }
    if let Err(error) = database
        .execute(load_sql!(
            "../backend/migrations/00000000000150_create_teams_users_roles_table/up.sql"
        ))
        .await
    {
        log::error!("Failed to create table migrations/00000000000150_create_teams_users_roles_table/up.sql: {:?}", error);
        unreachable!("Failed to create table");
    }
    if let Err(error) = database
        .execute(load_sql!(
            "../backend/migrations/00000000000155_create_users_users_role_invitations_table/up.sql"
        ))
        .await
    {
        log::error!("Failed to create table migrations/00000000000155_create_users_users_role_invitations_table/up.sql: {:?}", error);
        unreachable!("Failed to create table");
    }
    if let Err(error) = database
        .execute(load_sql!(
            "../backend/migrations/00000000000157_create_users_users_role_requests_table/up.sql"
        ))
        .await
    {
        log::error!("Failed to create table migrations/00000000000157_create_users_users_role_requests_table/up.sql: {:?}", error);
        unreachable!("Failed to create table");
    }
    if let Err(error) = database
        .execute(load_sql!(
            "../backend/migrations/00000000000159_create_users_users_roles_table/up.sql"
        ))
        .await
    {
        log::error!("Failed to create table migrations/00000000000159_create_users_users_roles_table/up.sql: {:?}", error);
        unreachable!("Failed to create table");
    }
}
