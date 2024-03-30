// @generated automatically by Diesel CLI.

diesel::table! {
    archivables (id) {
        id -> Uuid,
        archived_at -> Timestamp,
        archived_by -> Uuid,
    }
}

diesel::table! {
    container_horizontal_rules (id) {
        id -> Uuid,
        item_type_id -> Nullable<Uuid>,
        other_item_type_id -> Nullable<Uuid>,
        minimum_temperature -> Nullable<Float8>,
        maximum_temperature -> Nullable<Float8>,
        minimum_humidity -> Nullable<Float8>,
        maximum_humidity -> Nullable<Float8>,
        minimum_pressure -> Nullable<Float8>,
        maximum_pressure -> Nullable<Float8>,
    }
}

diesel::table! {
    container_vertical_rules (id) {
        id -> Uuid,
        container_item_type_id -> Nullable<Uuid>,
        contained_item_type_id -> Nullable<Uuid>,
        minimum_temperature -> Nullable<Float8>,
        maximum_temperature -> Nullable<Float8>,
        minimum_humidity -> Nullable<Float8>,
        maximum_humidity -> Nullable<Float8>,
        minimum_pressure -> Nullable<Float8>,
        maximum_pressure -> Nullable<Float8>,
    }
}

diesel::table! {
    continuous_units (id) {
        id -> Uuid,
    }
}

diesel::table! {
    describables (id) {
        id -> Uuid,
        name -> Text,
        description -> Nullable<Text>,
    }
}

diesel::table! {
    discrete_units (id) {
        id -> Uuid,
    }
}

diesel::table! {
    document_formats (id) {
        id -> Uuid,
        #[max_length = 255]
        mime_type -> Varchar,
    }
}

diesel::table! {
    documents (id) {
        id -> Uuid,
        #[max_length = 255]
        path -> Varchar,
        format_id -> Uuid,
        bytes -> Int4,
    }
}

diesel::table! {
    editables (id) {
        id -> Uuid,
        created_at -> Timestamp,
        created_by -> Uuid,
    }
}

diesel::table! {
    edits (id) {
        id -> Uuid,
        editable_id -> Uuid,
    }
}

diesel::table! {
    item_categories (id) {
        id -> Uuid,
    }
}

diesel::table! {
    item_category_relationships (id) {
        id -> Uuid,
        parent_id -> Uuid,
        child_id -> Uuid,
    }
}

diesel::table! {
    item_category_units (id) {
        id -> Uuid,
        item_category_id -> Uuid,
        unit_id -> Uuid,
    }
}

diesel::table! {
    item_continuous_quantities (id) {
        id -> Uuid,
        item_id -> Nullable<Uuid>,
        weight -> Float8,
        unit_id -> Nullable<Uuid>,
        sensor_id -> Nullable<Uuid>,
        measured_at -> Timestamp,
        measured_by -> Nullable<Uuid>,
    }
}

diesel::table! {
    item_discrete_quantities (id) {
        id -> Uuid,
        item_id -> Nullable<Uuid>,
        quantity -> Int4,
        unit_id -> Nullable<Uuid>,
        measured_at -> Timestamp,
        measured_by -> Nullable<Uuid>,
    }
}

diesel::table! {
    item_locations (id) {
        id -> Uuid,
        item_id -> Nullable<Uuid>,
        location_id -> Nullable<Uuid>,
        previous_location_id -> Nullable<Uuid>,
    }
}

diesel::table! {
    item_units (id) {
        id -> Uuid,
        item_id -> Uuid,
        unit_id -> Uuid,
    }
}

diesel::table! {
    items (id) {
        id -> Uuid,
        parent_id -> Nullable<Uuid>,
    }
}

diesel::table! {
    location_states (id) {
        id -> Uuid,
        #[max_length = 255]
        font_awesome_icon -> Nullable<Varchar>,
    }
}

diesel::table! {
    locations (id) {
        id -> Uuid,
        latitude -> Nullable<Float8>,
        longitude -> Nullable<Float8>,
        altitude -> Nullable<Float8>,
        address -> Nullable<Text>,
        geolocalization_device_id -> Nullable<Uuid>,
        altitude_device_id -> Nullable<Uuid>,
        parent_location_id -> Nullable<Uuid>,
        state_id -> Uuid,
    }
}

diesel::table! {
    login_providers (id) {
        id -> Uuid,
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
        id -> Uuid,
        cost -> Float8,
        cost_per_day -> Float8,
        #[max_length = 3]
        currency -> Varchar,
        manifacturer_id -> Uuid,
    }
}

diesel::table! {
    notifications (id) {
        id -> Uuid,
        user_id -> Uuid,
        #[max_length = 6]
        operation -> Varchar,
        #[max_length = 255]
        table_name -> Varchar,
        row_id -> Nullable<Uuid>,
        read -> Bool,
    }
}

diesel::table! {
    organization_authorizations (id) {
        id -> Uuid,
        organization_id -> Uuid,
        editable_id -> Uuid,
        role_id -> Uuid,
    }
}

diesel::table! {
    organization_locations (id) {
        id -> Uuid,
        organization_id -> Nullable<Uuid>,
        location_id -> Nullable<Uuid>,
        previous_location_id -> Nullable<Uuid>,
    }
}

diesel::table! {
    organization_states (id) {
        id -> Uuid,
        #[max_length = 255]
        font_awesome_icon -> Nullable<Varchar>,
    }
}

diesel::table! {
    organizations (id) {
        id -> Uuid,
        state_id -> Nullable<Uuid>,
        parent_organization_id -> Nullable<Uuid>,
        logo_id -> Nullable<Uuid>,
        #[max_length = 255]
        website_url -> Nullable<Varchar>,
    }
}

diesel::table! {
    primary_user_emails (id) {
        id -> Uuid,
    }
}

diesel::table! {
    procedure_continuous_requirements (id) {
        id -> Uuid,
        procedure_id -> Uuid,
        item_category_id -> Uuid,
        quantity -> Float8,
        unit_id -> Nullable<Uuid>,
    }
}

diesel::table! {
    procedure_discrete_requirements (id) {
        id -> Uuid,
        procedure_id -> Uuid,
        item_category_id -> Uuid,
        quantity -> Int4,
        unit_id -> Nullable<Uuid>,
    }
}

diesel::table! {
    procedures (id) {
        id -> Uuid,
    }
}

diesel::table! {
    project_continuous_requirements (id) {
        id -> Uuid,
        project_id -> Uuid,
        item_id -> Uuid,
        quantity -> Float8,
        unit_id -> Nullable<Uuid>,
    }
}

diesel::table! {
    project_discrete_requirements (id) {
        id -> Uuid,
        project_id -> Uuid,
        item_id -> Uuid,
        quantity -> Float8,
        unit_id -> Nullable<Uuid>,
    }
}

diesel::table! {
    project_milestones (id) {
        id -> Uuid,
        project_id -> Uuid,
        due_date -> Timestamp,
        completed_at -> Nullable<Timestamp>,
    }
}

diesel::table! {
    project_states (id) {
        id -> Uuid,
        name -> Varchar,
        description -> Text,
        font_awesome_icon -> Varchar,
        icon_color -> Varchar,
    }
}

diesel::table! {
    projects (id) {
        id -> Uuid,
        #[max_length = 255]
        name -> Varchar,
        description -> Text,
        public -> Bool,
        state_id -> Uuid,
        parent_project_id -> Nullable<Uuid>,
        budget -> Nullable<Float8>,
        expenses -> Nullable<Float8>,
        #[max_length = 3]
        currency -> Nullable<Varchar>,
        created_by -> Uuid,
        created_at -> Timestamp,
        expected_end_date -> Nullable<Timestamp>,
        end_date -> Nullable<Timestamp>,
    }
}

diesel::table! {
    roles (id) {
        id -> Uuid,
    }
}

diesel::table! {
    sample_taxa (id) {
        id -> Uuid,
        sample_id -> Uuid,
        taxon_id -> Uuid,
    }
}

diesel::table! {
    sampled_individual_taxa (id) {
        id -> Uuid,
        sampled_individual_id -> Uuid,
        taxon_id -> Uuid,
    }
}

diesel::table! {
    sampled_individuals (id) {
        id -> Uuid,
    }
}

diesel::table! {
    samples (id) {
        id -> Uuid,
        derived_from -> Nullable<Uuid>,
    }
}

diesel::table! {
    spectra (id) {
        id -> Uuid,
        spectra_collection_id -> Uuid,
    }
}

diesel::table! {
    spectra_collection (id) {
        id -> Uuid,
        sample_id -> Uuid,
    }
}

diesel::table! {
    taxa (id) {
        id -> Uuid,
        name -> Text,
        ncbi_taxon_id -> Nullable<Int4>,
    }
}

diesel::table! {
    team_authorizations (id) {
        id -> Uuid,
        team_id -> Uuid,
        editable_id -> Uuid,
        role_id -> Uuid,
    }
}

diesel::table! {
    team_states (id) {
        id -> Uuid,
        #[max_length = 255]
        font_awesome_icon -> Varchar,
    }
}

diesel::table! {
    teams (id) {
        id -> Uuid,
        parent_team_id -> Nullable<Uuid>,
        team_state_id -> Uuid,
    }
}

diesel::table! {
    units (id) {
        id -> Uuid,
        #[max_length = 255]
        symbol -> Varchar,
    }
}

diesel::table! {
    user_authorizations (id) {
        id -> Uuid,
        user_id -> Uuid,
        editable_id -> Uuid,
        role_id -> Uuid,
    }
}

diesel::table! {
    user_emails (id) {
        id -> Uuid,
        #[max_length = 255]
        email -> Varchar,
        user_id -> Uuid,
        login_provider_id -> Uuid,
    }
}

diesel::table! {
    users (id) {
        id -> Uuid,
        first_name -> Varchar,
        middle_name -> Nullable<Varchar>,
        last_name -> Varchar,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::joinable!(archivables -> editables (id));
diesel::joinable!(archivables -> users (archived_by));
diesel::joinable!(container_horizontal_rules -> describables (id));
diesel::joinable!(container_vertical_rules -> describables (id));
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
diesel::joinable!(edits -> editables (editable_id));
diesel::joinable!(item_categories -> describables (id));
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
diesel::joinable!(location_states -> describables (id));
diesel::joinable!(locations -> describables (id));
diesel::joinable!(locations -> location_states (state_id));
diesel::joinable!(manufactured_item_categories -> item_categories (id));
diesel::joinable!(manufactured_item_categories -> organizations (manifacturer_id));
diesel::joinable!(notifications -> users (user_id));
diesel::joinable!(organization_authorizations -> organizations (organization_id));
diesel::joinable!(organization_authorizations -> roles (role_id));
diesel::joinable!(organization_locations -> editables (id));
diesel::joinable!(organization_locations -> locations (location_id));
diesel::joinable!(organization_locations -> organizations (organization_id));
diesel::joinable!(organization_states -> describables (id));
diesel::joinable!(organizations -> describables (id));
diesel::joinable!(organizations -> documents (logo_id));
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
diesel::joinable!(project_milestones -> projects (project_id));
diesel::joinable!(projects -> project_states (state_id));
diesel::joinable!(projects -> users (created_by));
diesel::joinable!(roles -> describables (id));
diesel::joinable!(roles -> editables (id));
diesel::joinable!(sample_taxa -> editables (id));
diesel::joinable!(sample_taxa -> samples (sample_id));
diesel::joinable!(sample_taxa -> taxa (taxon_id));
diesel::joinable!(sampled_individual_taxa -> editables (id));
diesel::joinable!(sampled_individual_taxa -> sampled_individuals (sampled_individual_id));
diesel::joinable!(sampled_individual_taxa -> taxa (taxon_id));
diesel::joinable!(sampled_individuals -> items (id));
diesel::joinable!(samples -> editables (id));
diesel::joinable!(spectra -> editables (id));
diesel::joinable!(spectra -> spectra_collection (spectra_collection_id));
diesel::joinable!(spectra_collection -> editables (id));
diesel::joinable!(spectra_collection -> samples (sample_id));
diesel::joinable!(team_authorizations -> roles (role_id));
diesel::joinable!(team_authorizations -> teams (team_id));
diesel::joinable!(team_states -> describables (id));
diesel::joinable!(team_states -> editables (id));
diesel::joinable!(teams -> describables (id));
diesel::joinable!(teams -> editables (id));
diesel::joinable!(teams -> team_states (team_state_id));
diesel::joinable!(units -> describables (id));
diesel::joinable!(user_authorizations -> roles (role_id));
diesel::joinable!(user_authorizations -> users (user_id));
diesel::joinable!(user_emails -> login_providers (login_provider_id));
diesel::joinable!(user_emails -> users (user_id));

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
    notifications,
    organization_authorizations,
    organization_locations,
    organization_states,
    organizations,
    primary_user_emails,
    procedure_continuous_requirements,
    procedure_discrete_requirements,
    procedures,
    project_continuous_requirements,
    project_discrete_requirements,
    project_milestones,
    project_states,
    projects,
    roles,
    sample_taxa,
    sampled_individual_taxa,
    sampled_individuals,
    samples,
    spectra,
    spectra_collection,
    taxa,
    team_authorizations,
    team_states,
    teams,
    units,
    user_authorizations,
    user_emails,
    users,
);
