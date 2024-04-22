// @generated automatically by Diesel CLI.

diesel::table! {
    container_horizontal_rules (id) {
        id -> Int4,
        created_by -> Int4,
        name -> Text,
        item_type_id -> Int4,
        other_item_type_id -> Int4,
        minimum_temperature -> Nullable<Int4>,
        maximum_temperature -> Nullable<Int4>,
        minimum_humidity -> Nullable<Int4>,
        maximum_humidity -> Nullable<Int4>,
        minimum_pressure -> Nullable<Int4>,
        maximum_pressure -> Nullable<Int4>,
    }
}

diesel::table! {
    container_vertical_rules (id) {
        id -> Int4,
        created_by -> Int4,
        name -> Text,
        container_item_type_id -> Int4,
        contained_item_type_id -> Int4,
        minimum_temperature -> Nullable<Int4>,
        maximum_temperature -> Nullable<Int4>,
        minimum_humidity -> Nullable<Int4>,
        maximum_humidity -> Nullable<Int4>,
        minimum_pressure -> Nullable<Int4>,
        maximum_pressure -> Nullable<Int4>,
    }
}

diesel::table! {
    continuous_units (id) {
        id -> Int4,
    }
}

diesel::table! {
    discrete_units (id) {
        id -> Int4,
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
    documents (id) {
        id -> Uuid,
        author_id -> Int4,
        #[max_length = 255]
        path -> Varchar,
        format_id -> Int4,
        bytes -> Int4,
    }
}

diesel::table! {
    item_categories (id) {
        id -> Int4,
        name -> Text,
        description -> Text,
        created_by -> Int4,
    }
}

diesel::table! {
    item_category_relationships (id) {
        id -> Int4,
        parent_id -> Int4,
        child_id -> Int4,
        added_by -> Int4,
    }
}

diesel::table! {
    item_category_units (id) {
        id -> Int4,
        item_category_id -> Int4,
        unit_id -> Int4,
    }
}

diesel::table! {
    item_continuous_quantities (id) {
        id -> Uuid,
        item_id -> Uuid,
        amount -> Int4,
        unit_id -> Int4,
        sensor_id -> Nullable<Uuid>,
        measured_at -> Timestamp,
        measured_by -> Nullable<Int4>,
    }
}

diesel::table! {
    item_discrete_quantities (id) {
        id -> Uuid,
        item_id -> Uuid,
        quantity -> Int4,
        unit_id -> Int4,
        measured_at -> Timestamp,
        measured_by -> Nullable<Int4>,
    }
}

diesel::table! {
    item_locations (id) {
        id -> Uuid,
        item_id -> Nullable<Uuid>,
        located_by -> Nullable<Int4>,
        located_at -> Timestamp,
        location_id -> Nullable<Uuid>,
    }
}

diesel::table! {
    item_units (id) {
        id -> Uuid,
        item_id -> Uuid,
        unit_id -> Int4,
    }
}

diesel::table! {
    items (id) {
        id -> Uuid,
        parent_id -> Nullable<Uuid>,
    }
}

diesel::table! {
    locations (id) {
        id -> Uuid,
        latitude_degrees -> Nullable<Int4>,
        latitude_minutes -> Nullable<Int4>,
        latitude_seconds -> Nullable<Int4>,
        longitude_degrees -> Nullable<Int4>,
        longitude_minutes -> Nullable<Int4>,
        longitude_seconds -> Nullable<Int4>,
        altitude -> Nullable<Int4>,
        address -> Nullable<Text>,
        geolocalization_device_id -> Nullable<Uuid>,
        altitude_device_id -> Nullable<Uuid>,
        parent_location_id -> Nullable<Uuid>,
    }
}

diesel::table! {
    login_providers (id) {
        id -> Int4,
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
        id -> Int4,
        cost -> Int4,
        cost_per_day -> Int4,
        #[max_length = 3]
        currency -> Varchar,
        manifacturer_id -> Int4,
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
        read -> Bool,
    }
}

diesel::table! {
    organizations (id) {
        id -> Int4,
        parent_organization_id -> Nullable<Int4>,
        name -> Text,
    }
}

diesel::table! {
    primary_user_emails (id) {
        id -> Int4,
    }
}

diesel::table! {
    procedure_continuous_requirements (id) {
        id -> Int4,
        created_by -> Int4,
        procedure_id -> Int4,
        item_category_id -> Int4,
        quantity -> Int4,
        unit_id -> Nullable<Int4>,
    }
}

diesel::table! {
    procedure_discrete_requirements (id) {
        id -> Int4,
        created_by -> Int4,
        procedure_id -> Int4,
        item_category_id -> Int4,
        quantity -> Int4,
        unit_id -> Nullable<Int4>,
    }
}

diesel::table! {
    procedures (id) {
        id -> Int4,
        name -> Text,
        description -> Nullable<Text>,
        created_by -> Nullable<Int4>,
    }
}

diesel::table! {
    project_requirements (id) {
        id -> Int4,
        created_by -> Int4,
        project_id -> Int4,
        item_category_id -> Int4,
        quantity -> Int4,
        unit_id -> Nullable<Int4>,
    }
}

diesel::table! {
    project_states (id) {
        id -> Int4,
        name -> Text,
        description -> Text,
        font_awesome_icon -> Text,
        icon_color -> Text,
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
        budget -> Nullable<Int8>,
        expenses -> Nullable<Int8>,
        created_by -> Int4,
        created_at -> Timestamp,
        expected_end_date -> Nullable<Timestamp>,
        end_date -> Nullable<Timestamp>,
    }
}

diesel::table! {
    roles (id) {
        id -> Int4,
        name -> Text,
    }
}

diesel::table! {
    sample_taxa (id) {
        id -> Uuid,
        created_by -> Int4,
        sample_id -> Uuid,
        taxon_id -> Int4,
    }
}

diesel::table! {
    sampled_individual_taxa (id) {
        id -> Uuid,
        created_by -> Int4,
        sampled_individual_id -> Uuid,
        taxon_id -> Int4,
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
        created_by -> Nullable<Int4>,
        derived_from -> Nullable<Uuid>,
    }
}

diesel::table! {
    spectra (id) {
        id -> Int4,
        spectra_collection_id -> Int4,
    }
}

diesel::table! {
    spectra_collection (id) {
        id -> Int4,
        sample_id -> Uuid,
        created_by -> Int4,
    }
}

diesel::table! {
    taxa (id) {
        id -> Int4,
        name -> Text,
        ncbi_taxon_id -> Nullable<Int4>,
    }
}

diesel::table! {
    teams (id) {
        id -> Int4,
        parent_team_id -> Nullable<Int4>,
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
        user_id -> Int4,
        login_provider_id -> Int4,
    }
}

diesel::table! {
    users (id) {
        id -> Int4,
        first_name -> Varchar,
        middle_name -> Nullable<Varchar>,
        last_name -> Varchar,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::joinable!(container_horizontal_rules -> users (created_by));
diesel::joinable!(container_vertical_rules -> users (created_by));
diesel::joinable!(continuous_units -> units (id));
diesel::joinable!(discrete_units -> units (id));
diesel::joinable!(documents -> document_formats (format_id));
diesel::joinable!(documents -> users (author_id));
diesel::joinable!(item_categories -> users (created_by));
diesel::joinable!(item_category_relationships -> users (added_by));
diesel::joinable!(item_category_units -> item_categories (item_category_id));
diesel::joinable!(item_category_units -> units (unit_id));
diesel::joinable!(item_continuous_quantities -> continuous_units (unit_id));
diesel::joinable!(item_continuous_quantities -> units (unit_id));
diesel::joinable!(item_continuous_quantities -> users (measured_by));
diesel::joinable!(item_discrete_quantities -> discrete_units (unit_id));
diesel::joinable!(item_discrete_quantities -> items (item_id));
diesel::joinable!(item_discrete_quantities -> units (unit_id));
diesel::joinable!(item_discrete_quantities -> users (measured_by));
diesel::joinable!(item_locations -> items (item_id));
diesel::joinable!(item_locations -> locations (location_id));
diesel::joinable!(item_locations -> users (located_by));
diesel::joinable!(item_units -> items (item_id));
diesel::joinable!(item_units -> units (unit_id));
diesel::joinable!(manufactured_item_categories -> organizations (manifacturer_id));
diesel::joinable!(notifications -> users (user_id));
diesel::joinable!(primary_user_emails -> user_emails (id));
diesel::joinable!(procedure_continuous_requirements -> continuous_units (unit_id));
diesel::joinable!(procedure_continuous_requirements -> item_categories (item_category_id));
diesel::joinable!(procedure_continuous_requirements -> procedures (procedure_id));
diesel::joinable!(procedure_continuous_requirements -> units (unit_id));
diesel::joinable!(procedure_continuous_requirements -> users (created_by));
diesel::joinable!(procedure_discrete_requirements -> discrete_units (unit_id));
diesel::joinable!(procedure_discrete_requirements -> item_categories (item_category_id));
diesel::joinable!(procedure_discrete_requirements -> procedures (procedure_id));
diesel::joinable!(procedure_discrete_requirements -> units (unit_id));
diesel::joinable!(procedure_discrete_requirements -> users (created_by));
diesel::joinable!(procedures -> users (created_by));
diesel::joinable!(project_requirements -> item_categories (item_category_id));
diesel::joinable!(project_requirements -> projects (project_id));
diesel::joinable!(project_requirements -> units (unit_id));
diesel::joinable!(project_requirements -> users (created_by));
diesel::joinable!(projects -> project_states (state_id));
diesel::joinable!(projects -> users (created_by));
diesel::joinable!(sample_taxa -> samples (sample_id));
diesel::joinable!(sample_taxa -> taxa (taxon_id));
diesel::joinable!(sample_taxa -> users (created_by));
diesel::joinable!(sampled_individual_taxa -> sampled_individuals (sampled_individual_id));
diesel::joinable!(sampled_individual_taxa -> taxa (taxon_id));
diesel::joinable!(sampled_individual_taxa -> users (created_by));
diesel::joinable!(sampled_individuals -> items (id));
diesel::joinable!(samples -> users (created_by));
diesel::joinable!(spectra -> spectra_collection (spectra_collection_id));
diesel::joinable!(spectra_collection -> samples (sample_id));
diesel::joinable!(spectra_collection -> users (created_by));
diesel::joinable!(user_emails -> login_providers (login_provider_id));
diesel::joinable!(user_emails -> users (user_id));

diesel::allow_tables_to_appear_in_same_query!(
    container_horizontal_rules,
    container_vertical_rules,
    continuous_units,
    discrete_units,
    document_formats,
    documents,
    item_categories,
    item_category_relationships,
    item_category_units,
    item_continuous_quantities,
    item_discrete_quantities,
    item_locations,
    item_units,
    items,
    locations,
    login_providers,
    manufactured_item_categories,
    notifications,
    organizations,
    primary_user_emails,
    procedure_continuous_requirements,
    procedure_discrete_requirements,
    procedures,
    project_requirements,
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
    teams,
    units,
    user_emails,
    users,
);
