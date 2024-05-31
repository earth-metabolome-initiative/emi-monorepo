// @generated automatically by Diesel CLI.

diesel::table! {
    use postgis_diesel::sql_types::*;
    use diesel::sql_types::*;

    bio_ott_ranks (id) {
        id -> Int4,
        name -> Text,
        description -> Text,
        icon_id -> Int4,
        color_id -> Int4,
    }
}

diesel::table! {
    use postgis_diesel::sql_types::*;
    use diesel::sql_types::*;

    bio_ott_taxon_items (id) {
        id -> Int4,
        name -> Text,
        ott_id -> Int4,
        ott_rank_id -> Int4,
        wikidata_id -> Nullable<Int4>,
        ncbi_id -> Nullable<Int4>,
        gbif_id -> Nullable<Int4>,
        irmng_id -> Nullable<Int4>,
        worms_id -> Nullable<Int4>,
        domain_id -> Nullable<Int4>,
        kingdom_id -> Nullable<Int4>,
        phylum_id -> Nullable<Int4>,
        class_id -> Nullable<Int4>,
        order_id -> Nullable<Int4>,
        family_id -> Nullable<Int4>,
        genus_id -> Nullable<Int4>,
        parent_id -> Int4,
        icon_id -> Int4,
        color_id -> Int4,
    }
}

diesel::table! {
    use postgis_diesel::sql_types::*;
    use diesel::sql_types::*;

    colors (id) {
        id -> Int4,
        name -> Text,
        hexadecimal_value -> Text,
        description -> Text,
    }
}

diesel::table! {
    use postgis_diesel::sql_types::*;
    use diesel::sql_types::*;

    countries (id) {
        id -> Int4,
        iso -> Text,
        emoji -> Text,
        unicode -> Text,
        name -> Text,
    }
}

diesel::table! {
    use postgis_diesel::sql_types::*;
    use diesel::sql_types::*;

    derived_samples (parent_sample_id, child_sample_id) {
        created_by -> Int4,
        created_at -> Timestamp,
        updated_by -> Int4,
        updated_at -> Timestamp,
        parent_sample_id -> Uuid,
        child_sample_id -> Uuid,
    }
}

diesel::table! {
    use postgis_diesel::sql_types::*;
    use diesel::sql_types::*;

    document_formats (id) {
        id -> Int4,
        #[max_length = 255]
        extension -> Varchar,
        #[max_length = 255]
        mime_type -> Varchar,
        description -> Text,
        icon_id -> Int4,
        color_id -> Int4,
    }
}

diesel::table! {
    use postgis_diesel::sql_types::*;
    use diesel::sql_types::*;

    font_awesome_icons (id) {
        id -> Int4,
        name -> Text,
        description -> Text,
    }
}

diesel::table! {
    use postgis_diesel::sql_types::*;
    use diesel::sql_types::*;

    login_providers (id) {
        id -> Int4,
        #[max_length = 255]
        name -> Varchar,
        icon_id -> Int4,
        color_id -> Int4,
        #[max_length = 255]
        client_id_var_name -> Varchar,
        #[max_length = 255]
        redirect_uri_var_name -> Varchar,
        #[max_length = 255]
        oauth_url -> Varchar,
        #[max_length = 255]
        scope -> Varchar,
    }
}

diesel::table! {
    use postgis_diesel::sql_types::*;
    use diesel::sql_types::*;

    materials (id) {
        id -> Int4,
        name -> Text,
        description -> Text,
        icon_id -> Nullable<Int4>,
        color_id -> Nullable<Int4>,
    }
}

diesel::table! {
    use postgis_diesel::sql_types::*;
    use diesel::sql_types::*;

    nameplate_categories (id) {
        id -> Int4,
        name -> Text,
        permanence_id -> Int4,
        material_id -> Int4,
        description -> Text,
        icon_id -> Int4,
        color_id -> Int4,
    }
}

diesel::table! {
    use postgis_diesel::sql_types::*;
    use diesel::sql_types::*;

    nameplates (id) {
        id -> Int4,
        barcode -> Text,
        project_id -> Int4,
        category_id -> Int4,
        created_by -> Int4,
        created_at -> Timestamp,
        updated_by -> Int4,
        updated_at -> Timestamp,
    }
}

diesel::table! {
    use postgis_diesel::sql_types::*;
    use diesel::sql_types::*;

    notifications (id) {
        id -> Int4,
        user_id -> Int4,
        #[max_length = 6]
        operation -> Varchar,
        #[max_length = 255]
        table_name -> Varchar,
        record -> Text,
        read -> Bool,
    }
}

diesel::table! {
    use postgis_diesel::sql_types::*;
    use diesel::sql_types::*;

    observation_subjects (id) {
        id -> Int4,
        name -> Text,
        description -> Text,
        icon_id -> Int4,
        color_id -> Int4,
    }
}

diesel::table! {
    use postgis_diesel::sql_types::*;
    use diesel::sql_types::*;

    observations (id) {
        id -> Uuid,
        parent_observation_id -> Nullable<Uuid>,
        created_by -> Int4,
        created_at -> Nullable<Timestamp>,
        updated_by -> Int4,
        updated_at -> Nullable<Timestamp>,
        project_id -> Int4,
        organism_id -> Nullable<Uuid>,
        sample_id -> Nullable<Uuid>,
        notes -> Nullable<Text>,
        picture -> Bytea,
    }
}

diesel::table! {
    use postgis_diesel::sql_types::*;
    use diesel::sql_types::*;

    organism_bio_ott_taxon_items (organism_id, taxon_id) {
        created_by -> Int4,
        created_at -> Timestamp,
        organism_id -> Uuid,
        taxon_id -> Int4,
    }
}

diesel::table! {
    use postgis_diesel::sql_types::*;
    use diesel::sql_types::*;

    organisms (id) {
        id -> Uuid,
        host_organism_id -> Nullable<Uuid>,
        sample_id -> Nullable<Uuid>,
        notes -> Nullable<Text>,
        nameplate_id -> Int4,
        project_id -> Int4,
        created_by -> Int4,
        created_at -> Timestamp,
        updated_by -> Int4,
        updated_at -> Timestamp,
        picture -> Bytea,
    }
}

diesel::table! {
    use postgis_diesel::sql_types::*;
    use diesel::sql_types::*;

    organizations (id) {
        id -> Int4,
        name -> Text,
        url -> Text,
        country_id -> Int4,
        state_province -> Nullable<Text>,
        domain -> Text,
    }
}

diesel::table! {
    use postgis_diesel::sql_types::*;
    use diesel::sql_types::*;

    permanence_categories (id) {
        id -> Int4,
        name -> Text,
        description -> Text,
        icon_id -> Nullable<Int4>,
        color_id -> Nullable<Int4>,
    }
}

diesel::table! {
    use postgis_diesel::sql_types::*;
    use diesel::sql_types::*;

    project_states (id) {
        id -> Int4,
        name -> Text,
        description -> Text,
        icon_id -> Int4,
        color_id -> Int4,
    }
}

diesel::table! {
    use postgis_diesel::sql_types::*;
    use diesel::sql_types::*;

    projects (id) {
        id -> Int4,
        name -> Text,
        description -> Text,
        public -> Bool,
        state_id -> Int4,
        icon_id -> Int4,
        color_id -> Int4,
        parent_project_id -> Nullable<Int4>,
        budget -> Nullable<Float8>,
        expenses -> Nullable<Float8>,
        created_by -> Int4,
        created_at -> Timestamp,
        updated_by -> Int4,
        updated_at -> Timestamp,
        expected_end_date -> Nullable<Timestamp>,
        end_date -> Nullable<Timestamp>,
    }
}

diesel::table! {
    use postgis_diesel::sql_types::*;
    use diesel::sql_types::*;

    projects_teams_role_invitations (table_id, team_id) {
        table_id -> Int4,
        team_id -> Int4,
        role_id -> Int4,
        created_by -> Int4,
        created_at -> Timestamp,
    }
}

diesel::table! {
    use postgis_diesel::sql_types::*;
    use diesel::sql_types::*;

    projects_teams_role_requests (table_id, team_id) {
        table_id -> Int4,
        team_id -> Int4,
        role_id -> Int4,
        created_by -> Int4,
        created_at -> Timestamp,
    }
}

diesel::table! {
    use postgis_diesel::sql_types::*;
    use diesel::sql_types::*;

    projects_teams_roles (table_id, team_id) {
        table_id -> Int4,
        team_id -> Int4,
        role_id -> Int4,
        created_by -> Int4,
        created_at -> Timestamp,
    }
}

diesel::table! {
    use postgis_diesel::sql_types::*;
    use diesel::sql_types::*;

    projects_users_role_invitations (table_id, user_id) {
        table_id -> Int4,
        user_id -> Int4,
        role_id -> Int4,
        created_by -> Int4,
        created_at -> Timestamp,
    }
}

diesel::table! {
    use postgis_diesel::sql_types::*;
    use diesel::sql_types::*;

    projects_users_role_requests (table_id, user_id) {
        table_id -> Int4,
        user_id -> Int4,
        role_id -> Int4,
        created_by -> Int4,
        created_at -> Timestamp,
    }
}

diesel::table! {
    use postgis_diesel::sql_types::*;
    use diesel::sql_types::*;

    projects_users_roles (table_id, user_id) {
        table_id -> Int4,
        user_id -> Int4,
        role_id -> Int4,
        created_by -> Int4,
        created_at -> Timestamp,
    }
}

diesel::table! {
    use postgis_diesel::sql_types::*;
    use diesel::sql_types::*;

    roles (id) {
        id -> Int4,
        name -> Text,
        description -> Text,
        icon_id -> Int4,
        color_id -> Int4,
    }
}

diesel::table! {
    use postgis_diesel::sql_types::*;
    use diesel::sql_types::*;

    sample_bio_ott_taxon_items (sample_id, taxon_id) {
        created_by -> Int4,
        created_at -> Timestamp,
        sample_id -> Uuid,
        taxon_id -> Int4,
    }
}

diesel::table! {
    use postgis_diesel::sql_types::*;
    use diesel::sql_types::*;

    sample_container_categories (id) {
        id -> Int4,
        name -> Text,
        volume -> Float8,
        unit -> Text,
        material_id -> Int4,
        description -> Text,
        icon_id -> Int4,
        color_id -> Int4,
    }
}

diesel::table! {
    use postgis_diesel::sql_types::*;
    use diesel::sql_types::*;

    sample_containers (id) {
        id -> Int4,
        barcode -> Text,
        project_id -> Int4,
        category_id -> Int4,
        created_by -> Int4,
        created_at -> Timestamp,
        updated_by -> Int4,
        updated_at -> Timestamp,
    }
}

diesel::table! {
    use postgis_diesel::sql_types::*;
    use diesel::sql_types::*;

    sample_states (id) {
        id -> Int4,
        name -> Text,
        description -> Text,
        icon_id -> Int4,
        color_id -> Int4,
    }
}

diesel::table! {
    use postgis_diesel::sql_types::*;
    use diesel::sql_types::*;

    samples (id) {
        id -> Uuid,
        container_id -> Int4,
        notes -> Nullable<Text>,
        project_id -> Int4,
        created_by -> Int4,
        sampled_by -> Int4,
        created_at -> Timestamp,
        updated_by -> Int4,
        updated_at -> Timestamp,
        state_id -> Int4,
    }
}

diesel::table! {
    use postgis_diesel::sql_types::*;
    use diesel::sql_types::*;

    spatial_ref_sys (srid) {
        srid -> Int4,
        #[max_length = 256]
        auth_name -> Nullable<Varchar>,
        auth_srid -> Nullable<Int4>,
        #[max_length = 2048]
        srtext -> Nullable<Varchar>,
        #[max_length = 2048]
        proj4text -> Nullable<Varchar>,
    }
}

diesel::table! {
    use postgis_diesel::sql_types::*;
    use diesel::sql_types::*;

    spectra (id) {
        id -> Int4,
        notes -> Nullable<Text>,
        spectra_collection_id -> Int4,
        created_by -> Int4,
        created_at -> Timestamp,
        updated_by -> Int4,
        updated_at -> Timestamp,
    }
}

diesel::table! {
    use postgis_diesel::sql_types::*;
    use diesel::sql_types::*;

    spectra_collections (id) {
        id -> Int4,
        notes -> Nullable<Text>,
        sample_id -> Uuid,
        created_by -> Int4,
        created_at -> Timestamp,
        updated_by -> Int4,
        updated_at -> Timestamp,
    }
}

diesel::table! {
    use postgis_diesel::sql_types::*;
    use diesel::sql_types::*;

    team_states (id) {
        id -> Int4,
        name -> Text,
        description -> Text,
        icon_id -> Int4,
        color_id -> Int4,
    }
}

diesel::table! {
    use postgis_diesel::sql_types::*;
    use diesel::sql_types::*;

    teams (id) {
        id -> Int4,
        name -> Text,
        description -> Text,
        icon_id -> Int4,
        color_id -> Int4,
        state_id -> Int4,
        parent_team_id -> Nullable<Int4>,
        created_by -> Int4,
        created_at -> Timestamp,
        updated_by -> Int4,
        updated_at -> Timestamp,
    }
}

diesel::table! {
    use postgis_diesel::sql_types::*;
    use diesel::sql_types::*;

    teams_teams_role_invitations (table_id, team_id) {
        table_id -> Int4,
        team_id -> Int4,
        role_id -> Int4,
        created_by -> Int4,
        created_at -> Timestamp,
    }
}

diesel::table! {
    use postgis_diesel::sql_types::*;
    use diesel::sql_types::*;

    teams_users_role_invitations (table_id, user_id) {
        table_id -> Int4,
        user_id -> Int4,
        role_id -> Int4,
        created_by -> Int4,
        created_at -> Timestamp,
    }
}

diesel::table! {
    use postgis_diesel::sql_types::*;
    use diesel::sql_types::*;

    teams_users_role_requests (table_id, user_id) {
        table_id -> Int4,
        user_id -> Int4,
        role_id -> Int4,
        created_by -> Int4,
        created_at -> Timestamp,
    }
}

diesel::table! {
    use postgis_diesel::sql_types::*;
    use diesel::sql_types::*;

    teams_users_roles (table_id, user_id) {
        table_id -> Int4,
        user_id -> Int4,
        role_id -> Int4,
        created_by -> Int4,
        created_at -> Timestamp,
    }
}

diesel::table! {
    use postgis_diesel::sql_types::*;
    use diesel::sql_types::*;

    units (id) {
        id -> Int4,
        name -> Text,
        description -> Text,
        symbol -> Text,
    }
}

diesel::table! {
    use postgis_diesel::sql_types::*;
    use diesel::sql_types::*;

    user_emails (id) {
        id -> Int4,
        #[max_length = 255]
        email -> Varchar,
        created_by -> Int4,
        created_at -> Timestamp,
        login_provider_id -> Int4,
        primary_email -> Bool,
    }
}

diesel::table! {
    use postgis_diesel::sql_types::*;
    use diesel::sql_types::*;

    users (id) {
        id -> Int4,
        first_name -> Text,
        middle_name -> Nullable<Text>,
        last_name -> Text,
        description -> Nullable<Text>,
        profile_picture -> Bytea,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::table! {
    use postgis_diesel::sql_types::*;
    use diesel::sql_types::*;

    users_users_role_invitations (table_id, user_id) {
        table_id -> Int4,
        user_id -> Int4,
        role_id -> Int4,
        created_by -> Int4,
        created_at -> Timestamp,
    }
}

diesel::table! {
    use postgis_diesel::sql_types::*;
    use diesel::sql_types::*;

    users_users_role_requests (table_id, user_id) {
        table_id -> Int4,
        user_id -> Int4,
        role_id -> Int4,
        created_by -> Int4,
        created_at -> Timestamp,
    }
}

diesel::table! {
    use postgis_diesel::sql_types::*;
    use diesel::sql_types::*;

    users_users_roles (table_id, user_id) {
        table_id -> Int4,
        user_id -> Int4,
        role_id -> Int4,
        created_by -> Int4,
        created_at -> Timestamp,
    }
}

diesel::joinable!(bio_ott_ranks -> colors (color_id));
diesel::joinable!(bio_ott_ranks -> font_awesome_icons (icon_id));
diesel::joinable!(bio_ott_taxon_items -> bio_ott_ranks (ott_rank_id));
diesel::joinable!(bio_ott_taxon_items -> colors (color_id));
diesel::joinable!(bio_ott_taxon_items -> font_awesome_icons (icon_id));
diesel::joinable!(document_formats -> colors (color_id));
diesel::joinable!(document_formats -> font_awesome_icons (icon_id));
diesel::joinable!(login_providers -> colors (color_id));
diesel::joinable!(login_providers -> font_awesome_icons (icon_id));
diesel::joinable!(materials -> colors (color_id));
diesel::joinable!(materials -> font_awesome_icons (icon_id));
diesel::joinable!(nameplate_categories -> colors (color_id));
diesel::joinable!(nameplate_categories -> font_awesome_icons (icon_id));
diesel::joinable!(nameplate_categories -> materials (material_id));
diesel::joinable!(nameplate_categories -> permanence_categories (permanence_id));
diesel::joinable!(nameplates -> nameplate_categories (category_id));
diesel::joinable!(nameplates -> projects (project_id));
diesel::joinable!(notifications -> users (user_id));
diesel::joinable!(observation_subjects -> colors (color_id));
diesel::joinable!(observation_subjects -> font_awesome_icons (icon_id));
diesel::joinable!(observations -> organisms (organism_id));
diesel::joinable!(observations -> projects (project_id));
diesel::joinable!(observations -> samples (sample_id));
diesel::joinable!(organism_bio_ott_taxon_items -> bio_ott_taxon_items (taxon_id));
diesel::joinable!(organism_bio_ott_taxon_items -> organisms (organism_id));
diesel::joinable!(organism_bio_ott_taxon_items -> users (created_by));
diesel::joinable!(organisms -> nameplates (nameplate_id));
diesel::joinable!(organisms -> projects (project_id));
diesel::joinable!(organisms -> samples (sample_id));
diesel::joinable!(organizations -> countries (country_id));
diesel::joinable!(permanence_categories -> colors (color_id));
diesel::joinable!(permanence_categories -> font_awesome_icons (icon_id));
diesel::joinable!(project_states -> colors (color_id));
diesel::joinable!(project_states -> font_awesome_icons (icon_id));
diesel::joinable!(projects -> colors (color_id));
diesel::joinable!(projects -> font_awesome_icons (icon_id));
diesel::joinable!(projects -> project_states (state_id));
diesel::joinable!(projects_teams_role_invitations -> projects (table_id));
diesel::joinable!(projects_teams_role_invitations -> roles (role_id));
diesel::joinable!(projects_teams_role_invitations -> teams (team_id));
diesel::joinable!(projects_teams_role_invitations -> users (created_by));
diesel::joinable!(projects_teams_role_requests -> projects (table_id));
diesel::joinable!(projects_teams_role_requests -> roles (role_id));
diesel::joinable!(projects_teams_role_requests -> teams (team_id));
diesel::joinable!(projects_teams_role_requests -> users (created_by));
diesel::joinable!(projects_teams_roles -> projects (table_id));
diesel::joinable!(projects_teams_roles -> roles (role_id));
diesel::joinable!(projects_teams_roles -> teams (team_id));
diesel::joinable!(projects_teams_roles -> users (created_by));
diesel::joinable!(projects_users_role_invitations -> projects (table_id));
diesel::joinable!(projects_users_role_invitations -> roles (role_id));
diesel::joinable!(projects_users_role_requests -> projects (table_id));
diesel::joinable!(projects_users_role_requests -> roles (role_id));
diesel::joinable!(projects_users_roles -> projects (table_id));
diesel::joinable!(projects_users_roles -> roles (role_id));
diesel::joinable!(roles -> colors (color_id));
diesel::joinable!(roles -> font_awesome_icons (icon_id));
diesel::joinable!(sample_bio_ott_taxon_items -> bio_ott_taxon_items (taxon_id));
diesel::joinable!(sample_bio_ott_taxon_items -> samples (sample_id));
diesel::joinable!(sample_bio_ott_taxon_items -> users (created_by));
diesel::joinable!(sample_container_categories -> colors (color_id));
diesel::joinable!(sample_container_categories -> font_awesome_icons (icon_id));
diesel::joinable!(sample_container_categories -> materials (material_id));
diesel::joinable!(sample_containers -> projects (project_id));
diesel::joinable!(sample_containers -> sample_container_categories (category_id));
diesel::joinable!(sample_states -> colors (color_id));
diesel::joinable!(sample_states -> font_awesome_icons (icon_id));
diesel::joinable!(samples -> projects (project_id));
diesel::joinable!(samples -> sample_containers (container_id));
diesel::joinable!(samples -> sample_states (state_id));
diesel::joinable!(spectra -> spectra_collections (spectra_collection_id));
diesel::joinable!(spectra_collections -> samples (sample_id));
diesel::joinable!(team_states -> colors (color_id));
diesel::joinable!(team_states -> font_awesome_icons (icon_id));
diesel::joinable!(teams -> colors (color_id));
diesel::joinable!(teams -> font_awesome_icons (icon_id));
diesel::joinable!(teams -> team_states (state_id));
diesel::joinable!(teams_teams_role_invitations -> roles (role_id));
diesel::joinable!(teams_teams_role_invitations -> users (created_by));
diesel::joinable!(teams_users_role_invitations -> roles (role_id));
diesel::joinable!(teams_users_role_invitations -> teams (table_id));
diesel::joinable!(teams_users_role_requests -> roles (role_id));
diesel::joinable!(teams_users_role_requests -> teams (table_id));
diesel::joinable!(teams_users_roles -> roles (role_id));
diesel::joinable!(teams_users_roles -> teams (table_id));
diesel::joinable!(user_emails -> login_providers (login_provider_id));
diesel::joinable!(user_emails -> users (created_by));
diesel::joinable!(users_users_role_invitations -> roles (role_id));
diesel::joinable!(users_users_role_requests -> roles (role_id));
diesel::joinable!(users_users_roles -> roles (role_id));

diesel::allow_tables_to_appear_in_same_query!(
    bio_ott_ranks,
    bio_ott_taxon_items,
    colors,
    countries,
    derived_samples,
    document_formats,
    font_awesome_icons,
    login_providers,
    materials,
    nameplate_categories,
    nameplates,
    notifications,
    observation_subjects,
    observations,
    organism_bio_ott_taxon_items,
    organisms,
    organizations,
    permanence_categories,
    project_states,
    projects,
    projects_teams_role_invitations,
    projects_teams_role_requests,
    projects_teams_roles,
    projects_users_role_invitations,
    projects_users_role_requests,
    projects_users_roles,
    roles,
    sample_bio_ott_taxon_items,
    sample_container_categories,
    sample_containers,
    sample_states,
    samples,
    spatial_ref_sys,
    spectra,
    spectra_collections,
    team_states,
    teams,
    teams_teams_role_invitations,
    teams_users_role_invitations,
    teams_users_role_requests,
    teams_users_roles,
    units,
    user_emails,
    users,
    users_users_role_invitations,
    users_users_role_requests,
    users_users_roles,
);
