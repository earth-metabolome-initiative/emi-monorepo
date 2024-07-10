diesel::table! {
    colors (id) {
        id -> diesel::sql_types::Integer,
        name -> diesel::sql_types::Text,
        hexadecimal_value -> diesel::sql_types::Text,
        description -> diesel::sql_types::Text,
    }
}

diesel::table! {
    countries (id) {
        id -> diesel::sql_types::Integer,
        iso -> diesel::sql_types::Text,
        emoji -> diesel::sql_types::Text,
        unicode -> diesel::sql_types::Text,
        name -> diesel::sql_types::Text,
    }
}

diesel::table! {
    derived_samples (parent_sample_id, child_sample_id) {
        created_by -> diesel::sql_types::Integer,
        created_at -> diesel::sql_types::Timestamp,
        updated_by -> diesel::sql_types::Integer,
        updated_at -> diesel::sql_types::Timestamp,
        parent_sample_id -> diesel::sql_types::Uuid,
        child_sample_id -> diesel::sql_types::Uuid,
        quantity -> diesel::sql_types::Double,
        unit_id -> diesel::sql_types::Integer,
    }
}

diesel::table! {
    document_formats (id) {
        id -> diesel::sql_types::Integer,
        extension -> diesel::sql_types::Text,
        mime_type -> diesel::sql_types::Text,
        description -> diesel::sql_types::Text,
        icon_id -> diesel::sql_types::Integer,
        color_id -> diesel::sql_types::Integer,
    }
}

diesel::table! {
    font_awesome_icons (id) {
        id -> diesel::sql_types::Integer,
        name -> diesel::sql_types::Text,
        description -> diesel::sql_types::Text,
    }
}

diesel::table! {
    login_providers (id) {
        id -> diesel::sql_types::Integer,
        name -> diesel::sql_types::Text,
        icon_id -> diesel::sql_types::Integer,
        color_id -> diesel::sql_types::Integer,
        client_id_var_name -> diesel::sql_types::Text,
        redirect_uri_var_name -> diesel::sql_types::Text,
        oauth_url -> diesel::sql_types::Text,
        scope -> diesel::sql_types::Text,
    }
}

diesel::table! {
    materials (id) {
        id -> diesel::sql_types::Integer,
        name -> diesel::sql_types::Text,
        description -> diesel::sql_types::Text,
        icon_id -> diesel::sql_types::Integer,
        color_id -> diesel::sql_types::Integer,
    }
}

diesel::table! {
    nameplate_categories (id) {
        id -> diesel::sql_types::Integer,
        name -> diesel::sql_types::Text,
        permanence_id -> diesel::sql_types::Integer,
        material_id -> diesel::sql_types::Integer,
        description -> diesel::sql_types::Text,
        icon_id -> diesel::sql_types::Integer,
        color_id -> diesel::sql_types::Integer,
    }
}

diesel::table! {
    nameplates (id) {
        id -> diesel::sql_types::Integer,
        barcode -> diesel::sql_types::Text,
        project_id -> diesel::sql_types::Integer,
        category_id -> diesel::sql_types::Integer,
        created_by -> diesel::sql_types::Integer,
        created_at -> diesel::sql_types::Timestamp,
        updated_by -> diesel::sql_types::Integer,
        updated_at -> diesel::sql_types::Timestamp,
        geolocation -> postgis_diesel::sql_types::Geometry,
    }
}

diesel::table! {
    notifications (id) {
        id -> diesel::sql_types::Integer,
        user_id -> diesel::sql_types::Integer,
        operation -> diesel::sql_types::Text,
        table_name -> diesel::sql_types::Text,
        record -> diesel::sql_types::Text,
        read -> diesel::sql_types::Bool,
    }
}

diesel::table! {
    observation_subjects (id) {
        id -> diesel::sql_types::Integer,
        name -> diesel::sql_types::Text,
        description -> diesel::sql_types::Text,
        icon_id -> diesel::sql_types::Integer,
        color_id -> diesel::sql_types::Integer,
    }
}

diesel::table! {
    observations (id) {
        id -> diesel::sql_types::Uuid,
        parent_observation_id -> diesel::sql_types::Nullable<diesel::sql_types::Uuid>,
        created_by -> diesel::sql_types::Integer,
        created_at -> diesel::sql_types::Nullable<diesel::sql_types::Timestamp>,
        updated_by -> diesel::sql_types::Integer,
        updated_at -> diesel::sql_types::Nullable<diesel::sql_types::Timestamp>,
        project_id -> diesel::sql_types::Integer,
        organism_id -> diesel::sql_types::Nullable<diesel::sql_types::Uuid>,
        sample_id -> diesel::sql_types::Nullable<diesel::sql_types::Uuid>,
        subject_id -> diesel::sql_types::Integer,
        notes -> diesel::sql_types::Nullable<diesel::sql_types::Text>,
        picture -> crate::database::sql_type_bindings::Jpeg,
    }
}

diesel::table! {
    organism_taxa (organism_id, taxon_id) {
        created_by -> diesel::sql_types::Integer,
        created_at -> diesel::sql_types::Timestamp,
        organism_id -> diesel::sql_types::Uuid,
        taxon_id -> diesel::sql_types::Integer,
    }
}

diesel::table! {
    organisms (id) {
        id -> diesel::sql_types::Uuid,
        host_organism_id -> diesel::sql_types::Nullable<diesel::sql_types::Uuid>,
        sample_id -> diesel::sql_types::Nullable<diesel::sql_types::Uuid>,
        notes -> diesel::sql_types::Nullable<diesel::sql_types::Text>,
        wild -> diesel::sql_types::Bool,
        nameplate_id -> diesel::sql_types::Integer,
        project_id -> diesel::sql_types::Integer,
        created_by -> diesel::sql_types::Integer,
        created_at -> diesel::sql_types::Timestamp,
        updated_by -> diesel::sql_types::Integer,
        updated_at -> diesel::sql_types::Timestamp,
    }
}

diesel::table! {
    organizations (id) {
        id -> diesel::sql_types::Integer,
        name -> diesel::sql_types::Text,
        url -> diesel::sql_types::Text,
        country_id -> diesel::sql_types::Integer,
        state_province -> diesel::sql_types::Nullable<diesel::sql_types::Text>,
        domain -> diesel::sql_types::Text,
    }
}

diesel::table! {
    permanence_categories (id) {
        id -> diesel::sql_types::Integer,
        name -> diesel::sql_types::Text,
        description -> diesel::sql_types::Text,
        icon_id -> diesel::sql_types::Integer,
        color_id -> diesel::sql_types::Integer,
    }
}

diesel::table! {
    project_states (id) {
        id -> diesel::sql_types::Integer,
        name -> diesel::sql_types::Text,
        description -> diesel::sql_types::Text,
        icon_id -> diesel::sql_types::Integer,
        color_id -> diesel::sql_types::Integer,
    }
}

diesel::table! {
    projects (id) {
        id -> diesel::sql_types::Integer,
        name -> diesel::sql_types::Text,
        description -> diesel::sql_types::Text,
        public -> diesel::sql_types::Bool,
        state_id -> diesel::sql_types::Integer,
        icon_id -> diesel::sql_types::Integer,
        color_id -> diesel::sql_types::Integer,
        parent_project_id -> diesel::sql_types::Nullable<diesel::sql_types::Integer>,
        budget -> diesel::sql_types::Nullable<diesel::sql_types::Double>,
        expenses -> diesel::sql_types::Nullable<diesel::sql_types::Double>,
        created_by -> diesel::sql_types::Integer,
        created_at -> diesel::sql_types::Timestamp,
        updated_by -> diesel::sql_types::Integer,
        updated_at -> diesel::sql_types::Timestamp,
        expected_end_date -> diesel::sql_types::Nullable<diesel::sql_types::Timestamp>,
        end_date -> diesel::sql_types::Nullable<diesel::sql_types::Timestamp>,
    }
}

diesel::table! {
    projects_teams_role_invitations (table_id, team_id) {
        table_id -> diesel::sql_types::Integer,
        team_id -> diesel::sql_types::Integer,
        role_id -> diesel::sql_types::Integer,
        created_by -> diesel::sql_types::Integer,
        created_at -> diesel::sql_types::Timestamp,
    }
}

diesel::table! {
    projects_teams_role_requests (table_id, team_id) {
        table_id -> diesel::sql_types::Integer,
        team_id -> diesel::sql_types::Integer,
        role_id -> diesel::sql_types::Integer,
        created_by -> diesel::sql_types::Integer,
        created_at -> diesel::sql_types::Timestamp,
    }
}

diesel::table! {
    projects_teams_roles (table_id, team_id) {
        table_id -> diesel::sql_types::Integer,
        team_id -> diesel::sql_types::Integer,
        role_id -> diesel::sql_types::Integer,
        created_by -> diesel::sql_types::Integer,
        created_at -> diesel::sql_types::Timestamp,
    }
}

diesel::table! {
    projects_users_role_invitations (table_id, user_id) {
        table_id -> diesel::sql_types::Integer,
        user_id -> diesel::sql_types::Integer,
        role_id -> diesel::sql_types::Integer,
        created_by -> diesel::sql_types::Integer,
        created_at -> diesel::sql_types::Timestamp,
    }
}

diesel::table! {
    projects_users_role_requests (table_id, user_id) {
        table_id -> diesel::sql_types::Integer,
        user_id -> diesel::sql_types::Integer,
        role_id -> diesel::sql_types::Integer,
        created_by -> diesel::sql_types::Integer,
        created_at -> diesel::sql_types::Timestamp,
    }
}

diesel::table! {
    projects_users_roles (table_id, user_id) {
        table_id -> diesel::sql_types::Integer,
        user_id -> diesel::sql_types::Integer,
        role_id -> diesel::sql_types::Integer,
        created_by -> diesel::sql_types::Integer,
        created_at -> diesel::sql_types::Timestamp,
    }
}

diesel::table! {
    ranks (id) {
        id -> diesel::sql_types::Integer,
        name -> diesel::sql_types::Text,
        description -> diesel::sql_types::Text,
        icon_id -> diesel::sql_types::Integer,
        color_id -> diesel::sql_types::Integer,
    }
}

diesel::table! {
    roles (id) {
        id -> diesel::sql_types::Integer,
        name -> diesel::sql_types::Text,
        description -> diesel::sql_types::Text,
        icon_id -> diesel::sql_types::Integer,
        color_id -> diesel::sql_types::Integer,
    }
}

diesel::table! {
    sample_container_categories (id) {
        id -> diesel::sql_types::Integer,
        name -> diesel::sql_types::Text,
        volume -> diesel::sql_types::Double,
        unit -> diesel::sql_types::Text,
        material_id -> diesel::sql_types::Integer,
        description -> diesel::sql_types::Text,
        icon_id -> diesel::sql_types::Integer,
        color_id -> diesel::sql_types::Integer,
    }
}

diesel::table! {
    sample_containers (id) {
        id -> diesel::sql_types::Integer,
        barcode -> diesel::sql_types::Text,
        project_id -> diesel::sql_types::Integer,
        category_id -> diesel::sql_types::Integer,
        created_by -> diesel::sql_types::Integer,
        created_at -> diesel::sql_types::Timestamp,
        updated_by -> diesel::sql_types::Integer,
        updated_at -> diesel::sql_types::Timestamp,
    }
}

diesel::table! {
    sample_states (id) {
        id -> diesel::sql_types::Integer,
        name -> diesel::sql_types::Text,
        description -> diesel::sql_types::Text,
        icon_id -> diesel::sql_types::Integer,
        color_id -> diesel::sql_types::Integer,
    }
}

diesel::table! {
    sample_taxa (sample_id, taxon_id) {
        created_by -> diesel::sql_types::Integer,
        created_at -> diesel::sql_types::Timestamp,
        sample_id -> diesel::sql_types::Uuid,
        taxon_id -> diesel::sql_types::Integer,
    }
}

diesel::table! {
    samples (id) {
        id -> diesel::sql_types::Uuid,
        container_id -> diesel::sql_types::Integer,
        notes -> diesel::sql_types::Nullable<diesel::sql_types::Text>,
        project_id -> diesel::sql_types::Integer,
        created_by -> diesel::sql_types::Integer,
        sampled_by -> diesel::sql_types::Integer,
        created_at -> diesel::sql_types::Timestamp,
        updated_by -> diesel::sql_types::Integer,
        updated_at -> diesel::sql_types::Timestamp,
        state_id -> diesel::sql_types::Integer,
    }
}

diesel::table! {
    spectra (id) {
        id -> diesel::sql_types::Integer,
        spectra_collection_id -> diesel::sql_types::Integer,
    }
}

diesel::table! {
    spectra_collections (id) {
        id -> diesel::sql_types::Integer,
        notes -> diesel::sql_types::Nullable<diesel::sql_types::Text>,
        sample_id -> diesel::sql_types::Uuid,
        created_by -> diesel::sql_types::Integer,
        created_at -> diesel::sql_types::Timestamp,
        updated_by -> diesel::sql_types::Integer,
        updated_at -> diesel::sql_types::Timestamp,
    }
}

diesel::table! {
    taxa (id) {
        id -> diesel::sql_types::Integer,
        name -> diesel::sql_types::Text,
        ott_id -> diesel::sql_types::Integer,
        ott_rank_id -> diesel::sql_types::Integer,
        wikidata_id -> diesel::sql_types::Nullable<diesel::sql_types::Integer>,
        ncbi_id -> diesel::sql_types::Nullable<diesel::sql_types::Integer>,
        gbif_id -> diesel::sql_types::Nullable<diesel::sql_types::Integer>,
        irmng_id -> diesel::sql_types::Nullable<diesel::sql_types::Integer>,
        worms_id -> diesel::sql_types::Nullable<diesel::sql_types::Integer>,
        domain_id -> diesel::sql_types::Nullable<diesel::sql_types::Integer>,
        kingdom_id -> diesel::sql_types::Nullable<diesel::sql_types::Integer>,
        phylum_id -> diesel::sql_types::Nullable<diesel::sql_types::Integer>,
        class_id -> diesel::sql_types::Nullable<diesel::sql_types::Integer>,
        order_id -> diesel::sql_types::Nullable<diesel::sql_types::Integer>,
        family_id -> diesel::sql_types::Nullable<diesel::sql_types::Integer>,
        genus_id -> diesel::sql_types::Nullable<diesel::sql_types::Integer>,
        parent_id -> diesel::sql_types::Integer,
        icon_id -> diesel::sql_types::Integer,
        color_id -> diesel::sql_types::Integer,
    }
}

diesel::table! {
    team_states (id) {
        id -> diesel::sql_types::Integer,
        name -> diesel::sql_types::Text,
        description -> diesel::sql_types::Text,
        icon_id -> diesel::sql_types::Integer,
        color_id -> diesel::sql_types::Integer,
    }
}

diesel::table! {
    teams (id) {
        id -> diesel::sql_types::Integer,
        name -> diesel::sql_types::Text,
        description -> diesel::sql_types::Text,
        icon_id -> diesel::sql_types::Integer,
        color_id -> diesel::sql_types::Integer,
        state_id -> diesel::sql_types::Integer,
        parent_team_id -> diesel::sql_types::Nullable<diesel::sql_types::Integer>,
        created_by -> diesel::sql_types::Integer,
        created_at -> diesel::sql_types::Timestamp,
        updated_by -> diesel::sql_types::Integer,
        updated_at -> diesel::sql_types::Timestamp,
    }
}

diesel::table! {
    teams_teams_role_invitations (table_id, team_id) {
        table_id -> diesel::sql_types::Integer,
        team_id -> diesel::sql_types::Integer,
        role_id -> diesel::sql_types::Integer,
        created_by -> diesel::sql_types::Integer,
        created_at -> diesel::sql_types::Timestamp,
    }
}

diesel::table! {
    teams_users_role_invitations (table_id, user_id) {
        table_id -> diesel::sql_types::Integer,
        user_id -> diesel::sql_types::Integer,
        role_id -> diesel::sql_types::Integer,
        created_by -> diesel::sql_types::Integer,
        created_at -> diesel::sql_types::Timestamp,
    }
}

diesel::table! {
    teams_users_role_requests (table_id, user_id) {
        table_id -> diesel::sql_types::Integer,
        user_id -> diesel::sql_types::Integer,
        role_id -> diesel::sql_types::Integer,
        created_by -> diesel::sql_types::Integer,
        created_at -> diesel::sql_types::Timestamp,
    }
}

diesel::table! {
    teams_users_roles (table_id, user_id) {
        table_id -> diesel::sql_types::Integer,
        user_id -> diesel::sql_types::Integer,
        role_id -> diesel::sql_types::Integer,
        created_by -> diesel::sql_types::Integer,
        created_at -> diesel::sql_types::Timestamp,
    }
}

diesel::table! {
    units (id) {
        id -> diesel::sql_types::Integer,
        name -> diesel::sql_types::Text,
        unit -> diesel::sql_types::Text,
        icon_id -> diesel::sql_types::Integer,
        color_id -> diesel::sql_types::Integer,
    }
}

diesel::table! {
    user_emails (id) {
        id -> diesel::sql_types::Integer,
        email -> diesel::sql_types::Text,
        created_by -> diesel::sql_types::Integer,
        created_at -> diesel::sql_types::Timestamp,
        login_provider_id -> diesel::sql_types::Integer,
        primary_email -> diesel::sql_types::Bool,
    }
}

diesel::table! {
    users (id) {
        id -> diesel::sql_types::Integer,
        first_name -> diesel::sql_types::Text,
        middle_name -> diesel::sql_types::Nullable<diesel::sql_types::Text>,
        last_name -> diesel::sql_types::Text,
        description -> diesel::sql_types::Nullable<diesel::sql_types::Text>,
        organization_id -> diesel::sql_types::Nullable<diesel::sql_types::Integer>,
        created_at -> diesel::sql_types::Timestamp,
        updated_at -> diesel::sql_types::Timestamp,
        picture -> crate::database::sql_type_bindings::Jpeg,
    }
}

diesel::table! {
    users_users_role_invitations (table_id, user_id) {
        table_id -> diesel::sql_types::Integer,
        user_id -> diesel::sql_types::Integer,
        role_id -> diesel::sql_types::Integer,
        created_by -> diesel::sql_types::Integer,
        created_at -> diesel::sql_types::Timestamp,
    }
}

diesel::table! {
    users_users_role_requests (table_id, user_id) {
        table_id -> diesel::sql_types::Integer,
        user_id -> diesel::sql_types::Integer,
        role_id -> diesel::sql_types::Integer,
        created_by -> diesel::sql_types::Integer,
        created_at -> diesel::sql_types::Timestamp,
    }
}

diesel::table! {
    users_users_roles (table_id, user_id) {
        table_id -> diesel::sql_types::Integer,
        user_id -> diesel::sql_types::Integer,
        role_id -> diesel::sql_types::Integer,
        created_by -> diesel::sql_types::Integer,
        created_at -> diesel::sql_types::Timestamp,
    }
}

diesel::joinable!(derived_samples -> users (created_by));
diesel::joinable!(derived_samples -> samples (parent_sample_id));
diesel::joinable!(derived_samples -> units (unit_id));
diesel::joinable!(document_formats -> font_awesome_icons (icon_id));
diesel::joinable!(document_formats -> colors (color_id));
diesel::joinable!(login_providers -> font_awesome_icons (icon_id));
diesel::joinable!(login_providers -> colors (color_id));
diesel::joinable!(materials -> font_awesome_icons (icon_id));
diesel::joinable!(materials -> colors (color_id));
diesel::joinable!(nameplate_categories -> permanence_categories (permanence_id));
diesel::joinable!(nameplate_categories -> materials (material_id));
diesel::joinable!(nameplate_categories -> font_awesome_icons (icon_id));
diesel::joinable!(nameplate_categories -> colors (color_id));
diesel::joinable!(nameplates -> projects (project_id));
diesel::joinable!(nameplates -> nameplate_categories (category_id));
diesel::joinable!(nameplates -> users (created_by));
diesel::joinable!(notifications -> users (user_id));
diesel::joinable!(observation_subjects -> font_awesome_icons (icon_id));
diesel::joinable!(observation_subjects -> colors (color_id));
diesel::joinable!(observations -> users (created_by));
diesel::joinable!(observations -> projects (project_id));
diesel::joinable!(observations -> organisms (organism_id));
diesel::joinable!(observations -> samples (sample_id));
diesel::joinable!(observations -> observation_subjects (subject_id));
diesel::joinable!(organism_taxa -> users (created_by));
diesel::joinable!(organism_taxa -> organisms (organism_id));
diesel::joinable!(organism_taxa -> taxa (taxon_id));
diesel::joinable!(organisms -> samples (sample_id));
diesel::joinable!(organisms -> nameplates (nameplate_id));
diesel::joinable!(organisms -> projects (project_id));
diesel::joinable!(organisms -> users (created_by));
diesel::joinable!(organizations -> countries (country_id));
diesel::joinable!(permanence_categories -> font_awesome_icons (icon_id));
diesel::joinable!(permanence_categories -> colors (color_id));
diesel::joinable!(project_states -> font_awesome_icons (icon_id));
diesel::joinable!(project_states -> colors (color_id));
diesel::joinable!(projects -> project_states (state_id));
diesel::joinable!(projects -> font_awesome_icons (icon_id));
diesel::joinable!(projects -> colors (color_id));
diesel::joinable!(projects -> users (created_by));
diesel::joinable!(projects_teams_role_invitations -> projects (table_id));
diesel::joinable!(projects_teams_role_invitations -> teams (team_id));
diesel::joinable!(projects_teams_role_invitations -> roles (role_id));
diesel::joinable!(projects_teams_role_invitations -> users (created_by));
diesel::joinable!(projects_teams_role_requests -> projects (table_id));
diesel::joinable!(projects_teams_role_requests -> teams (team_id));
diesel::joinable!(projects_teams_role_requests -> roles (role_id));
diesel::joinable!(projects_teams_role_requests -> users (created_by));
diesel::joinable!(projects_teams_roles -> projects (table_id));
diesel::joinable!(projects_teams_roles -> teams (team_id));
diesel::joinable!(projects_teams_roles -> roles (role_id));
diesel::joinable!(projects_teams_roles -> users (created_by));
diesel::joinable!(projects_users_role_invitations -> projects (table_id));
diesel::joinable!(projects_users_role_invitations -> users (user_id));
diesel::joinable!(projects_users_role_invitations -> roles (role_id));
diesel::joinable!(projects_users_role_requests -> projects (table_id));
diesel::joinable!(projects_users_role_requests -> users (user_id));
diesel::joinable!(projects_users_role_requests -> roles (role_id));
diesel::joinable!(projects_users_roles -> projects (table_id));
diesel::joinable!(projects_users_roles -> users (user_id));
diesel::joinable!(projects_users_roles -> roles (role_id));
diesel::joinable!(ranks -> font_awesome_icons (icon_id));
diesel::joinable!(ranks -> colors (color_id));
diesel::joinable!(roles -> font_awesome_icons (icon_id));
diesel::joinable!(roles -> colors (color_id));
diesel::joinable!(sample_container_categories -> materials (material_id));
diesel::joinable!(sample_container_categories -> font_awesome_icons (icon_id));
diesel::joinable!(sample_container_categories -> colors (color_id));
diesel::joinable!(sample_containers -> projects (project_id));
diesel::joinable!(sample_containers -> sample_container_categories (category_id));
diesel::joinable!(sample_containers -> users (created_by));
diesel::joinable!(sample_states -> font_awesome_icons (icon_id));
diesel::joinable!(sample_states -> colors (color_id));
diesel::joinable!(sample_taxa -> users (created_by));
diesel::joinable!(sample_taxa -> samples (sample_id));
diesel::joinable!(sample_taxa -> taxa (taxon_id));
diesel::joinable!(samples -> sample_containers (container_id));
diesel::joinable!(samples -> projects (project_id));
diesel::joinable!(samples -> users (created_by));
diesel::joinable!(samples -> sample_states (state_id));
diesel::joinable!(spectra -> spectra_collections (spectra_collection_id));
diesel::joinable!(spectra_collections -> samples (sample_id));
diesel::joinable!(spectra_collections -> users (created_by));
diesel::joinable!(taxa -> ranks (ott_rank_id));
diesel::joinable!(taxa -> font_awesome_icons (icon_id));
diesel::joinable!(taxa -> colors (color_id));
diesel::joinable!(team_states -> font_awesome_icons (icon_id));
diesel::joinable!(team_states -> colors (color_id));
diesel::joinable!(teams -> font_awesome_icons (icon_id));
diesel::joinable!(teams -> colors (color_id));
diesel::joinable!(teams -> team_states (state_id));
diesel::joinable!(teams -> users (created_by));
diesel::joinable!(teams_teams_role_invitations -> teams (table_id));
diesel::joinable!(teams_teams_role_invitations -> roles (role_id));
diesel::joinable!(teams_teams_role_invitations -> users (created_by));
diesel::joinable!(teams_users_role_invitations -> teams (table_id));
diesel::joinable!(teams_users_role_invitations -> users (user_id));
diesel::joinable!(teams_users_role_invitations -> roles (role_id));
diesel::joinable!(teams_users_role_requests -> teams (table_id));
diesel::joinable!(teams_users_role_requests -> users (user_id));
diesel::joinable!(teams_users_role_requests -> roles (role_id));
diesel::joinable!(teams_users_roles -> teams (table_id));
diesel::joinable!(teams_users_roles -> users (user_id));
diesel::joinable!(teams_users_roles -> roles (role_id));
diesel::joinable!(units -> font_awesome_icons (icon_id));
diesel::joinable!(units -> colors (color_id));
diesel::joinable!(user_emails -> users (created_by));
diesel::joinable!(user_emails -> login_providers (login_provider_id));
diesel::joinable!(users -> organizations (organization_id));
diesel::joinable!(users_users_role_invitations -> users (table_id));
diesel::joinable!(users_users_role_invitations -> roles (role_id));
diesel::joinable!(users_users_role_requests -> users (table_id));
diesel::joinable!(users_users_role_requests -> roles (role_id));
diesel::joinable!(users_users_roles -> users (table_id));
diesel::joinable!(users_users_roles -> roles (role_id));
diesel::allow_tables_to_appear_in_same_query!(
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
    organism_taxa,
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
    ranks,
    roles,
    sample_container_categories,
    sample_containers,
    sample_states,
    sample_taxa,
    samples,
    spectra,
    spectra_collections,
    taxa,
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
    users_users_roles
);
