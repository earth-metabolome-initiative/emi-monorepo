// @generated automatically by Diesel CLI.

diesel::table! {
    bio_ott_ranks (id) {
        id -> Int4,
        name -> Text,
        description -> Text,
        icon_id -> Int4,
        color_id -> Int4,
    }
}

diesel::table! {
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
    colors (id) {
        id -> Int4,
        name -> Text,
        hexadecimal_value -> Text,
        description -> Text,
    }
}

diesel::table! {
    countries (id) {
        id -> Int4,
        iso -> Text,
        emoji -> Text,
        unicode -> Text,
        name -> Text,
    }
}

diesel::table! {
    derived_samples (parent_sample_id, child_sample_id) {
        created_by -> Int4,
        created_at -> Timestamp,
        parent_sample_id -> Uuid,
        child_sample_id -> Uuid,
    }
}

diesel::table! {
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
    font_awesome_icons (id) {
        id -> Int4,
        name -> Text,
        description -> Text,
    }
}

diesel::table! {
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
    materials (id) {
        id -> Int4,
        name -> Text,
        description -> Text,
        icon_id -> Nullable<Int4>,
        color_id -> Nullable<Int4>,
    }
}

diesel::table! {
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
    observations (id) {
        id -> Uuid,
        created_by -> Int4,
        created_at -> Nullable<Timestamp>,
        updated_by -> Int4,
        updated_at -> Nullable<Timestamp>,
        project_id -> Int4,
        individual_id -> Nullable<Uuid>,
        notes -> Nullable<Text>,
        picture -> Bytea,
    }
}

diesel::table! {
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
    project_states (id) {
        id -> Int4,
        name -> Text,
        description -> Text,
        icon_id -> Int4,
        color_id -> Int4,
    }
}

diesel::table! {
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
    projects_teams_role_invitations (table_id, team_id) {
        table_id -> Int4,
        team_id -> Int4,
        role_id -> Int4,
        created_by -> Int4,
        created_at -> Timestamp,
    }
}

diesel::table! {
    projects_teams_role_requests (table_id, team_id) {
        table_id -> Int4,
        team_id -> Int4,
        role_id -> Int4,
        created_by -> Int4,
        created_at -> Timestamp,
    }
}

diesel::table! {
    projects_teams_roles (table_id, team_id) {
        table_id -> Int4,
        team_id -> Int4,
        role_id -> Int4,
        created_by -> Int4,
        created_at -> Timestamp,
    }
}

diesel::table! {
    projects_users_role_invitations (table_id, user_id) {
        table_id -> Int4,
        user_id -> Int4,
        role_id -> Int4,
        created_by -> Int4,
        created_at -> Timestamp,
    }
}

diesel::table! {
    projects_users_role_requests (table_id, user_id) {
        table_id -> Int4,
        user_id -> Int4,
        role_id -> Int4,
        created_by -> Int4,
        created_at -> Timestamp,
    }
}

diesel::table! {
    projects_users_roles (table_id, user_id) {
        table_id -> Int4,
        user_id -> Int4,
        role_id -> Int4,
        created_by -> Int4,
        created_at -> Timestamp,
    }
}

diesel::table! {
    roles (id) {
        id -> Int4,
        name -> Text,
        description -> Text,
        icon_id -> Int4,
        color_id -> Int4,
    }
}

diesel::table! {
    sample_bio_ott_taxon_items (sample_id, taxon_id) {
        created_by -> Int4,
        created_at -> Timestamp,
        sample_id -> Uuid,
        taxon_id -> Int4,
    }
}

diesel::table! {
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
    sample_containers (id) {
        id -> Int4,
        barcode -> Text,
        project_id -> Int4,
        category_id -> Int4,
        created_by -> Int4,
        created_at -> Timestamp,
    }
}

diesel::table! {
    sample_states (id) {
        id -> Int4,
        name -> Text,
        description -> Text,
        icon_id -> Int4,
        color_id -> Int4,
    }
}

diesel::table! {
    sampled_individual_bio_ott_taxon_items (sampled_individual_id, taxon_id) {
        created_by -> Int4,
        created_at -> Timestamp,
        sampled_individual_id -> Uuid,
        taxon_id -> Int4,
    }
}

diesel::table! {
    sampled_individuals (id) {
        id -> Uuid,
        notes -> Nullable<Text>,
        barcode -> Nullable<Text>,
        project_id -> Int4,
        created_by -> Int4,
        created_at -> Timestamp,
        updated_by -> Int4,
        updated_at -> Timestamp,
        picture -> Bytea,
    }
}

diesel::table! {
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
        state -> Int4,
    }
}

diesel::table! {
    spectra (id) {
        id -> Int4,
        notes -> Nullable<Text>,
        spectra_collection_id -> Int4,
    }
}

diesel::table! {
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
    team_states (id) {
        id -> Int4,
        name -> Text,
        description -> Text,
        icon_id -> Int4,
        color_id -> Int4,
    }
}

diesel::table! {
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
    teams_teams_role_invitations (table_id, team_id) {
        table_id -> Int4,
        team_id -> Int4,
        role_id -> Int4,
        created_by -> Int4,
        created_at -> Timestamp,
    }
}

diesel::table! {
    teams_users_role_invitations (table_id, user_id) {
        table_id -> Int4,
        user_id -> Int4,
        role_id -> Int4,
        created_by -> Int4,
        created_at -> Timestamp,
    }
}

diesel::table! {
    teams_users_role_requests (table_id, user_id) {
        table_id -> Int4,
        user_id -> Int4,
        role_id -> Int4,
        created_by -> Int4,
        created_at -> Timestamp,
    }
}

diesel::table! {
    teams_users_roles (table_id, user_id) {
        table_id -> Int4,
        user_id -> Int4,
        role_id -> Int4,
        created_by -> Int4,
        created_at -> Timestamp,
    }
}

diesel::table! {
    units (id) {
        id -> Int4,
        name -> Text,
        description -> Text,
        symbol -> Text,
    }
}

diesel::table! {
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
    users_users_role_invitations (table_id, user_id) {
        table_id -> Int4,
        user_id -> Int4,
        role_id -> Int4,
        created_by -> Int4,
        created_at -> Timestamp,
    }
}

diesel::table! {
    users_users_role_requests (table_id, user_id) {
        table_id -> Int4,
        user_id -> Int4,
        role_id -> Int4,
        created_by -> Int4,
        created_at -> Timestamp,
    }
}

diesel::table! {
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
diesel::joinable!(derived_samples -> users (created_by));
diesel::joinable!(document_formats -> colors (color_id));
diesel::joinable!(document_formats -> font_awesome_icons (icon_id));
diesel::joinable!(login_providers -> colors (color_id));
diesel::joinable!(login_providers -> font_awesome_icons (icon_id));
diesel::joinable!(materials -> colors (color_id));
diesel::joinable!(materials -> font_awesome_icons (icon_id));
diesel::joinable!(notifications -> users (user_id));
diesel::joinable!(observations -> projects (project_id));
diesel::joinable!(observations -> sampled_individuals (individual_id));
diesel::joinable!(organizations -> countries (country_id));
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
diesel::joinable!(sample_containers -> users (created_by));
diesel::joinable!(sample_states -> colors (color_id));
diesel::joinable!(sample_states -> font_awesome_icons (icon_id));
diesel::joinable!(sampled_individual_bio_ott_taxon_items -> bio_ott_taxon_items (taxon_id));
diesel::joinable!(sampled_individual_bio_ott_taxon_items -> sampled_individuals (sampled_individual_id));
diesel::joinable!(sampled_individual_bio_ott_taxon_items -> users (created_by));
diesel::joinable!(sampled_individuals -> projects (project_id));
diesel::joinable!(samples -> projects (project_id));
diesel::joinable!(samples -> sample_containers (container_id));
diesel::joinable!(samples -> sample_states (state));
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
    notifications,
    observations,
    organizations,
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
    sampled_individual_bio_ott_taxon_items,
    sampled_individuals,
    samples,
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
