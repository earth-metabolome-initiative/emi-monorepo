// @generated automatically by Diesel CLI.

diesel::table! {
    bio_ott_ranks (id) {
        id -> Int4,
        name -> Text,
        font_awesome_icon_id -> Int4,
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
        font_awesome_icon_id -> Int4,
        color_id -> Int4,
    }
}

diesel::table! {
    colors (id) {
        id -> Int4,
        name -> Text,
        hexadecimal_value -> Text,
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
        updated_by -> Int4,
        updated_at -> Timestamp,
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
    }
}

diesel::table! {
    font_awesome_icons (id) {
        id -> Int4,
        name -> Text,
    }
}

diesel::table! {
    login_providers (id) {
        id -> Int4,
        #[max_length = 255]
        name -> Varchar,
        font_awesome_icon_id -> Int4,
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
        font_awesome_icon_id -> Int4,
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
    projects_teams_roles (table_id, team_id, role_id) {
        table_id -> Int4,
        team_id -> Int4,
        role_id -> Int4,
        created_by -> Int4,
        created_at -> Timestamp,
        updated_by -> Int4,
        updated_at -> Timestamp,
    }
}

diesel::table! {
    projects_users_roles (table_id, user_id, role_id) {
        table_id -> Int4,
        user_id -> Int4,
        role_id -> Int4,
        created_by -> Int4,
        created_at -> Nullable<Timestamp>,
        updated_by -> Int4,
        updated_at -> Nullable<Timestamp>,
    }
}

diesel::table! {
    roles (id) {
        id -> Int4,
        name -> Text,
        description -> Text,
        font_awesome_icon_id -> Int4,
        color_id -> Int4,
    }
}

diesel::table! {
    sample_bio_ott_taxon_items (sample_id, taxon_id) {
        created_by -> Int4,
        created_at -> Timestamp,
        updated_by -> Int4,
        updated_at -> Timestamp,
        sample_id -> Uuid,
        taxon_id -> Int4,
    }
}

diesel::table! {
    sample_states (id) {
        id -> Int4,
        name -> Text,
        description -> Text,
        font_awesome_icon_id -> Int4,
        color_id -> Int4,
    }
}

diesel::table! {
    sampled_individual_bio_ott_taxon_items (sampled_individual_id, taxon_id) {
        created_by -> Int4,
        created_at -> Timestamp,
        updated_by -> Int4,
        updated_at -> Timestamp,
        sampled_individual_id -> Uuid,
        taxon_id -> Int4,
    }
}

diesel::table! {
    sampled_individuals (id) {
        id -> Uuid,
        created_by -> Int4,
        created_at -> Timestamp,
        updated_by -> Int4,
        updated_at -> Timestamp,
        tagged -> Bool,
    }
}

diesel::table! {
    sampled_individuals_teams_roles (table_id, team_id, role_id) {
        table_id -> Uuid,
        team_id -> Int4,
        role_id -> Int4,
        created_by -> Int4,
        created_at -> Timestamp,
        updated_by -> Int4,
        updated_at -> Timestamp,
    }
}

diesel::table! {
    sampled_individuals_users_roles (table_id, user_id, role_id) {
        table_id -> Uuid,
        user_id -> Int4,
        role_id -> Int4,
        created_by -> Int4,
        created_at -> Timestamp,
        updated_by -> Int4,
        updated_at -> Timestamp,
    }
}

diesel::table! {
    samples (id) {
        id -> Uuid,
        created_by -> Int4,
        sampled_by -> Int4,
        created_at -> Timestamp,
        updated_by -> Int4,
        updated_at -> Timestamp,
        state -> Int4,
    }
}

diesel::table! {
    samples_teams_roles (table_id, team_id, role_id) {
        table_id -> Uuid,
        team_id -> Int4,
        role_id -> Int4,
        created_by -> Int4,
        created_at -> Timestamp,
        updated_by -> Int4,
        updated_at -> Timestamp,
    }
}

diesel::table! {
    samples_users_roles (table_id, user_id, role_id) {
        table_id -> Uuid,
        user_id -> Int4,
        role_id -> Int4,
        created_by -> Int4,
        created_at -> Timestamp,
        updated_by -> Int4,
        updated_at -> Timestamp,
    }
}

diesel::table! {
    spectra (id) {
        id -> Int4,
        spectra_collection_id -> Int4,
    }
}

diesel::table! {
    spectra_collections (id) {
        id -> Int4,
        sample_id -> Uuid,
        created_by -> Int4,
        created_at -> Timestamp,
        updated_by -> Int4,
        updated_at -> Timestamp,
    }
}

diesel::table! {
    spectra_collections_teams_roles (table_id, team_id, role_id) {
        table_id -> Int4,
        team_id -> Int4,
        role_id -> Int4,
        created_by -> Int4,
        created_at -> Timestamp,
        updated_by -> Int4,
        updated_at -> Timestamp,
    }
}

diesel::table! {
    spectra_collections_users_roles (table_id, user_id, role_id) {
        table_id -> Int4,
        user_id -> Int4,
        role_id -> Int4,
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
        font_awesome_icon_id -> Int4,
        color_id -> Int4,
    }
}

diesel::table! {
    teams (id) {
        id -> Int4,
        name -> Text,
        description -> Text,
        parent_team_id -> Nullable<Int4>,
        created_by -> Int4,
        created_at -> Timestamp,
        updated_by -> Int4,
        updated_at -> Timestamp,
    }
}

diesel::table! {
    teams_users_roles (table_id, user_id, role_id) {
        table_id -> Int4,
        user_id -> Int4,
        role_id -> Int4,
        created_by -> Int4,
        created_at -> Timestamp,
        updated_by -> Int4,
        updated_at -> Timestamp,
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
        first_name -> Varchar,
        middle_name -> Nullable<Varchar>,
        last_name -> Varchar,
        profile_picture -> Bytea,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::joinable!(bio_ott_ranks -> font_awesome_icons (font_awesome_icon_id));
diesel::joinable!(bio_ott_taxon_items -> bio_ott_ranks (ott_rank_id));
diesel::joinable!(bio_ott_taxon_items -> colors (color_id));
diesel::joinable!(bio_ott_taxon_items -> font_awesome_icons (font_awesome_icon_id));
diesel::joinable!(login_providers -> colors (color_id));
diesel::joinable!(login_providers -> font_awesome_icons (font_awesome_icon_id));
diesel::joinable!(notifications -> users (user_id));
diesel::joinable!(organizations -> countries (country_id));
diesel::joinable!(project_states -> colors (color_id));
diesel::joinable!(project_states -> font_awesome_icons (font_awesome_icon_id));
diesel::joinable!(projects -> project_states (state_id));
diesel::joinable!(projects_teams_roles -> projects (table_id));
diesel::joinable!(projects_teams_roles -> roles (role_id));
diesel::joinable!(projects_teams_roles -> teams (team_id));
diesel::joinable!(projects_users_roles -> projects (table_id));
diesel::joinable!(projects_users_roles -> roles (role_id));
diesel::joinable!(roles -> colors (color_id));
diesel::joinable!(roles -> font_awesome_icons (font_awesome_icon_id));
diesel::joinable!(sample_bio_ott_taxon_items -> bio_ott_taxon_items (taxon_id));
diesel::joinable!(sample_bio_ott_taxon_items -> samples (sample_id));
diesel::joinable!(sample_states -> colors (color_id));
diesel::joinable!(sample_states -> font_awesome_icons (font_awesome_icon_id));
diesel::joinable!(sampled_individual_bio_ott_taxon_items -> bio_ott_taxon_items (taxon_id));
diesel::joinable!(sampled_individual_bio_ott_taxon_items -> sampled_individuals (sampled_individual_id));
diesel::joinable!(sampled_individuals_teams_roles -> roles (role_id));
diesel::joinable!(sampled_individuals_teams_roles -> sampled_individuals (table_id));
diesel::joinable!(sampled_individuals_teams_roles -> teams (team_id));
diesel::joinable!(sampled_individuals_users_roles -> roles (role_id));
diesel::joinable!(sampled_individuals_users_roles -> sampled_individuals (table_id));
diesel::joinable!(samples -> sample_states (state));
diesel::joinable!(samples_teams_roles -> roles (role_id));
diesel::joinable!(samples_teams_roles -> samples (table_id));
diesel::joinable!(samples_teams_roles -> teams (team_id));
diesel::joinable!(samples_users_roles -> roles (role_id));
diesel::joinable!(samples_users_roles -> samples (table_id));
diesel::joinable!(spectra -> spectra_collections (spectra_collection_id));
diesel::joinable!(spectra_collections -> samples (sample_id));
diesel::joinable!(spectra_collections_teams_roles -> roles (role_id));
diesel::joinable!(spectra_collections_teams_roles -> spectra_collections (table_id));
diesel::joinable!(spectra_collections_teams_roles -> teams (team_id));
diesel::joinable!(spectra_collections_users_roles -> roles (role_id));
diesel::joinable!(spectra_collections_users_roles -> spectra_collections (table_id));
diesel::joinable!(team_states -> colors (color_id));
diesel::joinable!(team_states -> font_awesome_icons (font_awesome_icon_id));
diesel::joinable!(teams_users_roles -> roles (role_id));
diesel::joinable!(teams_users_roles -> teams (table_id));
diesel::joinable!(user_emails -> login_providers (login_provider_id));
diesel::joinable!(user_emails -> users (created_by));

diesel::allow_tables_to_appear_in_same_query!(
    bio_ott_ranks,
    bio_ott_taxon_items,
    colors,
    countries,
    derived_samples,
    document_formats,
    font_awesome_icons,
    login_providers,
    notifications,
    organizations,
    project_states,
    projects,
    projects_teams_roles,
    projects_users_roles,
    roles,
    sample_bio_ott_taxon_items,
    sample_states,
    sampled_individual_bio_ott_taxon_items,
    sampled_individuals,
    sampled_individuals_teams_roles,
    sampled_individuals_users_roles,
    samples,
    samples_teams_roles,
    samples_users_roles,
    spectra,
    spectra_collections,
    spectra_collections_teams_roles,
    spectra_collections_users_roles,
    team_states,
    teams,
    teams_users_roles,
    units,
    user_emails,
    users,
);
