// @generated automatically by Diesel CLI.

diesel::table! {
    archivables (id) {
        id -> Int8,
        archived_at -> Timestamp,
        archived_by -> Int4,
    }
}

diesel::table! {
    container_horizontal_rules (id) {
        id -> Int8,
        item_type_id -> Nullable<Int4>,
        other_item_type_id -> Nullable<Int4>,
        temperature -> Nullable<Numrange>,
        humidity -> Nullable<Numrange>,
        pressure -> Nullable<Numrange>,
    }
}

diesel::table! {
    container_vertical_rules (id) {
        id -> Int8,
        container_item_type_id -> Nullable<Int4>,
        contained_item_type_id -> Nullable<Int4>,
        temperature -> Nullable<Numrange>,
        humidity -> Nullable<Numrange>,
        pressure -> Nullable<Numrange>,
    }
}

diesel::table! {
    continuous_units (id) {
        id -> Int8,
    }
}

diesel::table! {
    describables (id) {
        id -> Int8,
        name -> Text,
        description -> Nullable<Text>,
    }
}

diesel::table! {
    discrete_units (id) {
        id -> Int8,
    }
}

diesel::table! {
    document_formats (id) {
        id -> Int8,
    }
}

diesel::table! {
    documents (id) {
        id -> Int8,
        #[max_length = 255]
        path -> Varchar,
        format_id -> Int8,
        bytes -> Int4,
    }
}

diesel::table! {
    editables (id) {
        id -> Int8,
        created_at -> Timestamp,
        created_by -> Int4,
    }
}

diesel::table! {
    edits (id) {
        id -> Int8,
        edited_by -> Int4,
        edited_at -> Timestamp,
    }
}

diesel::table! {
    expirable_item_categories (id) {
        id -> Int8,
        item_type_id -> Nullable<Int8>,
        expiration_interval -> Interval,
    }
}

diesel::table! {
    item_categories (id) {
        id -> Int8,
    }
}

diesel::table! {
    item_category_relationships (id) {
        id -> Int8,
        parent_id -> Int4,
        child_id -> Int4,
    }
}

diesel::table! {
    item_category_units (id) {
        id -> Int8,
        item_category_id -> Int8,
        unit_id -> Int8,
    }
}

diesel::table! {
    item_continuous_quantities (id) {
        id -> Int8,
        item_id -> Nullable<Int8>,
        weight -> Numeric,
        unit_id -> Nullable<Int8>,
        sensor_id -> Nullable<Int8>,
        measured_at -> Timestamptz,
        measured_by -> Nullable<Int4>,
    }
}

diesel::table! {
    item_discrete_quantities (id) {
        id -> Int8,
        item_id -> Nullable<Int8>,
        quantity -> Int4,
        unit_id -> Nullable<Int8>,
        measured_at -> Timestamptz,
        measured_by -> Nullable<Int4>,
    }
}

diesel::table! {
    item_locations (id) {
        id -> Int8,
        item_id -> Nullable<Int8>,
        location_id -> Nullable<Int8>,
        previous_location_id -> Nullable<Int8>,
    }
}

diesel::table! {
    item_units (id) {
        id -> Int8,
        item_id -> Int8,
        unit_id -> Int8,
    }
}

diesel::table! {
    items (id) {
        id -> Int8,
        parent_id -> Nullable<Int4>,
    }
}

diesel::table! {
    location_states (id) {
        id -> Int8,
        #[max_length = 255]
        font_awesome_icon -> Nullable<Varchar>,
    }
}

diesel::table! {
    locations (id) {
        id -> Int8,
        latitude -> Nullable<Numeric>,
        longitude -> Nullable<Numeric>,
        altitude -> Nullable<Numeric>,
        address -> Nullable<Text>,
        geolocalization_device_id -> Nullable<Int8>,
        altitude_device_id -> Nullable<Int8>,
        parent_location_id -> Nullable<Int8>,
        state_id -> Int8,
    }
}

diesel::table! {
    login_providers (id) {
        id -> Int2,
        #[max_length = 255]
        name -> Varchar,
        #[max_length = 255]
        font_awesome_icon -> Varchar,
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
    manufactured_item_categories (id) {
        id -> Int8,
        cost -> Numeric,
        cost_per_day -> Numeric,
        #[max_length = 3]
        currency -> Varchar,
        manifacturer_id -> Nullable<Int8>,
    }
}

diesel::table! {
    organization_locations (id) {
        id -> Int8,
        organization_id -> Nullable<Int8>,
        location_id -> Nullable<Int8>,
        previous_location_id -> Nullable<Int8>,
    }
}

diesel::table! {
    organization_project_roles (id) {
        id -> Int8,
    }
}

diesel::table! {
    organization_projects (id) {
        id -> Int8,
        organization_id -> Int8,
        project_id -> Int8,
        role_id -> Int8,
    }
}

diesel::table! {
    organization_states (id) {
        id -> Int8,
        #[max_length = 255]
        font_awesome_icon -> Nullable<Varchar>,
    }
}

diesel::table! {
    organization_user_roles (id) {
        id -> Int8,
    }
}

diesel::table! {
    organization_users (id) {
        id -> Int8,
        user_id -> Int4,
        organization_id -> Int8,
        role_id -> Int8,
    }
}

diesel::table! {
    organizations (id) {
        id -> Int8,
        state_id -> Nullable<Int8>,
        parent_organization_id -> Nullable<Int8>,
        logo_id -> Nullable<Int8>,
        #[max_length = 255]
        website_url -> Nullable<Varchar>,
    }
}

diesel::table! {
    primary_user_emails (id) {
        id -> Int4,
    }
}

diesel::table! {
    procedure_continuous_requirements (id) {
        id -> Int8,
        procedure_id -> Int8,
        item_category_id -> Int8,
        quantity -> Float8,
        unit_id -> Nullable<Int8>,
    }
}

diesel::table! {
    procedure_discrete_requirements (id) {
        id -> Int8,
        procedure_id -> Int8,
        item_category_id -> Int8,
        quantity -> Int4,
        unit_id -> Nullable<Int8>,
    }
}

diesel::table! {
    procedures (id) {
        id -> Int8,
    }
}

diesel::table! {
    project_continuous_requirements (id) {
        id -> Int8,
        project_id -> Int8,
        item_id -> Int8,
        quantity -> Float8,
        unit_id -> Nullable<Int8>,
    }
}

diesel::table! {
    project_discrete_requirements (id) {
        id -> Int8,
        project_id -> Int8,
        item_id -> Int8,
        quantity -> Float8,
        unit_id -> Nullable<Int8>,
    }
}

diesel::table! {
    project_milestones (id) {
        id -> Int8,
        project_id -> Int8,
        due_date -> Timestamptz,
        completed_at -> Nullable<Timestamptz>,
    }
}

diesel::table! {
    project_states (id) {
        id -> Int8,
    }
}

diesel::table! {
    project_user_roles (id) {
        id -> Int8,
    }
}

diesel::table! {
    project_users (id) {
        id -> Int8,
        user_id -> Int4,
        project_id -> Int8,
        role_id -> Int8,
    }
}

diesel::table! {
    projects (id) {
        id -> Int8,
        public -> Nullable<Bool>,
        state_id -> Int8,
        parent_project_id -> Nullable<Int8>,
        budget -> Nullable<Money>,
        expenses -> Nullable<Money>,
        #[max_length = 3]
        currency -> Nullable<Varchar>,
        expected_end_date -> Nullable<Timestamptz>,
        end_date -> Nullable<Timestamptz>,
        #[max_length = 255]
        website_url -> Nullable<Varchar>,
        logo_id -> Nullable<Int8>,
    }
}

diesel::table! {
    sample_taxa (sample_id, taxon_id) {
        sample_id -> Int8,
        taxon_id -> Int8,
    }
}

diesel::table! {
    sampled_individual_taxa (sampled_individual_id, taxon_id) {
        sampled_individual_id -> Int8,
        taxon_id -> Int8,
    }
}

diesel::table! {
    sampled_individuals (id) {
        id -> Int8,
    }
}

diesel::table! {
    samples (id) {
        id -> Int8,
        derived_from -> Nullable<Int4>,
    }
}

diesel::table! {
    spectra (id) {
        id -> Int4,
        spectra_collection_id -> Int8,
    }
}

diesel::table! {
    spectra_collection (id) {
        id -> Int8,
        sample_id -> Int8,
    }
}

diesel::table! {
    taxa (id) {
        id -> Int8,
    }
}

diesel::table! {
    team_states (id) {
        id -> Int8,
        #[max_length = 255]
        font_awesome_icon -> Nullable<Varchar>,
    }
}

diesel::table! {
    team_user_roles (id) {
        id -> Int8,
    }
}

diesel::table! {
    team_users (id) {
        id -> Int8,
        user_id -> Int4,
        team_id -> Int8,
        role_id -> Int8,
    }
}

diesel::table! {
    teams (id) {
        id -> Int8,
        parent_team_id -> Nullable<Int4>,
        team_state_id -> Int8,
    }
}

diesel::table! {
    units (id) {
        id -> Int8,
        #[max_length = 255]
        symbol -> Varchar,
    }
}

diesel::table! {
    user_emails (id) {
        id -> Int4,
        #[max_length = 255]
        email -> Varchar,
        user_id -> Int4,
        login_provider_id -> Int2,
    }
}

diesel::table! {
    user_pictures (user_id, document_id) {
        user_id -> Int4,
        document_id -> Int8,
    }
}

diesel::table! {
    users (id) {
        id -> Int4,
        first_name -> Nullable<Varchar>,
        middle_name -> Nullable<Varchar>,
        last_name -> Nullable<Varchar>,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::joinable!(archivables -> editables (id));
diesel::joinable!(archivables -> users (archived_by));
diesel::joinable!(container_horizontal_rules -> editables (id));
diesel::joinable!(container_vertical_rules -> editables (id));
diesel::joinable!(continuous_units -> units (id));
diesel::joinable!(describables -> editables (id));
diesel::joinable!(discrete_units -> units (id));
diesel::joinable!(document_formats -> describables (id));
diesel::joinable!(document_formats -> editables (id));
diesel::joinable!(documents -> describables (id));
diesel::joinable!(documents -> document_formats (format_id));
diesel::joinable!(documents -> editables (id));
diesel::joinable!(editables -> users (created_by));
diesel::joinable!(edits -> describables (id));
diesel::joinable!(edits -> editables (id));
diesel::joinable!(edits -> users (edited_by));
diesel::joinable!(expirable_item_categories -> editables (id));
diesel::joinable!(expirable_item_categories -> item_categories (item_type_id));
diesel::joinable!(item_categories -> describables (id));
diesel::joinable!(item_categories -> editables (id));
diesel::joinable!(item_category_relationships -> editables (id));
diesel::joinable!(item_category_units -> editables (id));
diesel::joinable!(item_category_units -> item_categories (item_category_id));
diesel::joinable!(item_category_units -> units (unit_id));
diesel::joinable!(item_continuous_quantities -> continuous_units (unit_id));
diesel::joinable!(item_continuous_quantities -> editables (id));
diesel::joinable!(item_continuous_quantities -> units (unit_id));
diesel::joinable!(item_continuous_quantities -> users (measured_by));
diesel::joinable!(item_discrete_quantities -> discrete_units (unit_id));
diesel::joinable!(item_discrete_quantities -> editables (id));
diesel::joinable!(item_discrete_quantities -> items (item_id));
diesel::joinable!(item_discrete_quantities -> units (unit_id));
diesel::joinable!(item_discrete_quantities -> users (measured_by));
diesel::joinable!(item_locations -> editables (id));
diesel::joinable!(item_locations -> items (item_id));
diesel::joinable!(item_units -> editables (id));
diesel::joinable!(item_units -> items (item_id));
diesel::joinable!(item_units -> units (unit_id));
diesel::joinable!(items -> describables (id));
diesel::joinable!(items -> editables (id));
diesel::joinable!(location_states -> describables (id));
diesel::joinable!(location_states -> editables (id));
diesel::joinable!(locations -> describables (id));
diesel::joinable!(locations -> editables (id));
diesel::joinable!(locations -> location_states (state_id));
diesel::joinable!(manufactured_item_categories -> item_categories (id));
diesel::joinable!(manufactured_item_categories -> organizations (manifacturer_id));
diesel::joinable!(organization_locations -> editables (id));
diesel::joinable!(organization_locations -> locations (location_id));
diesel::joinable!(organization_locations -> organizations (organization_id));
diesel::joinable!(organization_project_roles -> describables (id));
diesel::joinable!(organization_project_roles -> editables (id));
diesel::joinable!(organization_projects -> editables (id));
diesel::joinable!(organization_projects -> organizations (organization_id));
diesel::joinable!(organization_projects -> projects (project_id));
diesel::joinable!(organization_projects -> team_user_roles (role_id));
diesel::joinable!(organization_states -> describables (id));
diesel::joinable!(organization_states -> editables (id));
diesel::joinable!(organization_user_roles -> describables (id));
diesel::joinable!(organization_user_roles -> editables (id));
diesel::joinable!(organization_users -> editables (id));
diesel::joinable!(organization_users -> organization_user_roles (role_id));
diesel::joinable!(organization_users -> organizations (organization_id));
diesel::joinable!(organization_users -> users (user_id));
diesel::joinable!(organizations -> describables (id));
diesel::joinable!(organizations -> documents (logo_id));
diesel::joinable!(organizations -> editables (id));
diesel::joinable!(organizations -> organization_states (state_id));
diesel::joinable!(primary_user_emails -> user_emails (id));
diesel::joinable!(procedure_continuous_requirements -> continuous_units (unit_id));
diesel::joinable!(procedure_continuous_requirements -> editables (id));
diesel::joinable!(procedure_continuous_requirements -> item_categories (item_category_id));
diesel::joinable!(procedure_continuous_requirements -> procedures (procedure_id));
diesel::joinable!(procedure_continuous_requirements -> units (unit_id));
diesel::joinable!(procedure_discrete_requirements -> discrete_units (unit_id));
diesel::joinable!(procedure_discrete_requirements -> editables (id));
diesel::joinable!(procedure_discrete_requirements -> item_categories (item_category_id));
diesel::joinable!(procedure_discrete_requirements -> procedures (procedure_id));
diesel::joinable!(procedure_discrete_requirements -> units (unit_id));
diesel::joinable!(procedures -> describables (id));
diesel::joinable!(procedures -> editables (id));
diesel::joinable!(project_continuous_requirements -> continuous_units (unit_id));
diesel::joinable!(project_continuous_requirements -> editables (id));
diesel::joinable!(project_continuous_requirements -> item_categories (item_id));
diesel::joinable!(project_continuous_requirements -> projects (project_id));
diesel::joinable!(project_continuous_requirements -> units (unit_id));
diesel::joinable!(project_discrete_requirements -> discrete_units (unit_id));
diesel::joinable!(project_discrete_requirements -> editables (id));
diesel::joinable!(project_discrete_requirements -> item_categories (item_id));
diesel::joinable!(project_discrete_requirements -> projects (project_id));
diesel::joinable!(project_discrete_requirements -> units (unit_id));
diesel::joinable!(project_milestones -> describables (id));
diesel::joinable!(project_milestones -> editables (id));
diesel::joinable!(project_milestones -> projects (project_id));
diesel::joinable!(project_states -> describables (id));
diesel::joinable!(project_states -> editables (id));
diesel::joinable!(project_user_roles -> describables (id));
diesel::joinable!(project_user_roles -> editables (id));
diesel::joinable!(project_users -> editables (id));
diesel::joinable!(project_users -> project_user_roles (role_id));
diesel::joinable!(project_users -> projects (project_id));
diesel::joinable!(project_users -> users (user_id));
diesel::joinable!(projects -> describables (id));
diesel::joinable!(projects -> documents (logo_id));
diesel::joinable!(projects -> editables (id));
diesel::joinable!(projects -> project_states (state_id));
diesel::joinable!(sample_taxa -> samples (sample_id));
diesel::joinable!(sample_taxa -> taxa (taxon_id));
diesel::joinable!(sampled_individual_taxa -> sampled_individuals (sampled_individual_id));
diesel::joinable!(sampled_individual_taxa -> taxa (taxon_id));
diesel::joinable!(sampled_individuals -> items (id));
diesel::joinable!(samples -> editables (id));
diesel::joinable!(spectra -> spectra_collection (spectra_collection_id));
diesel::joinable!(spectra_collection -> editables (id));
diesel::joinable!(spectra_collection -> samples (sample_id));
diesel::joinable!(taxa -> describables (id));
diesel::joinable!(taxa -> editables (id));
diesel::joinable!(team_states -> describables (id));
diesel::joinable!(team_states -> editables (id));
diesel::joinable!(team_user_roles -> describables (id));
diesel::joinable!(team_user_roles -> editables (id));
diesel::joinable!(team_users -> editables (id));
diesel::joinable!(team_users -> team_user_roles (role_id));
diesel::joinable!(team_users -> teams (team_id));
diesel::joinable!(team_users -> users (user_id));
diesel::joinable!(teams -> describables (id));
diesel::joinable!(teams -> editables (id));
diesel::joinable!(teams -> team_states (team_state_id));
diesel::joinable!(units -> describables (id));
diesel::joinable!(units -> editables (id));
diesel::joinable!(user_emails -> login_providers (login_provider_id));
diesel::joinable!(user_emails -> users (user_id));
diesel::joinable!(user_pictures -> documents (document_id));
diesel::joinable!(user_pictures -> users (user_id));

diesel::allow_tables_to_appear_in_same_query!(
    archivables,
    container_horizontal_rules,
    container_vertical_rules,
    continuous_units,
    describables,
    discrete_units,
    document_formats,
    documents,
    editables,
    edits,
    expirable_item_categories,
    item_categories,
    item_category_relationships,
    item_category_units,
    item_continuous_quantities,
    item_discrete_quantities,
    item_locations,
    item_units,
    items,
    location_states,
    locations,
    login_providers,
    manufactured_item_categories,
    organization_locations,
    organization_project_roles,
    organization_projects,
    organization_states,
    organization_user_roles,
    organization_users,
    organizations,
    primary_user_emails,
    procedure_continuous_requirements,
    procedure_discrete_requirements,
    procedures,
    project_continuous_requirements,
    project_discrete_requirements,
    project_milestones,
    project_states,
    project_user_roles,
    project_users,
    projects,
    sample_taxa,
    sampled_individual_taxa,
    sampled_individuals,
    samples,
    spectra,
    spectra_collection,
    taxa,
    team_states,
    team_user_roles,
    team_users,
    teams,
    units,
    user_emails,
    user_pictures,
    users,
);
