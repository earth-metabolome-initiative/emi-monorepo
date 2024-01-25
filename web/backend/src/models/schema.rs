// @generated automatically by Diesel CLI.

diesel::table! {
    users (id) {
        id -> Int4,
        first_name -> Varchar,
        middle_name -> Nullable<Varchar>,
        last_name -> Varchar,
        email -> Varchar,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::table! {
    samples (id) {
        id -> Int4,
        taxon_id -> Int4,
        name -> Varchar,
        description -> Text,
        latitude -> Float8,
        longitude -> Float8,
        altitude -> Float8,
        derived_from -> Nullable<Int4>,
        created_at -> Timestamp,
        updated_at -> Timestamp,
        user_id -> Int4,
    }
}

diesel::table! {
    taxons (id) {
        id -> Int4,
        name -> Varchar,
        description -> Text,
        user_id -> Int4,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::table! {
    spectra (id) {
        id -> Int4,
        name -> Varchar,
        description -> Text,
        spectra_collection_id -> Int4,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::table! {
    spectra_collection (id) {
        id -> Int4,
        name -> Varchar,
        description -> Text,
        sample_id -> Int4,
        user_id -> Int4,
        updated_by_id -> Int4,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::table! {
    task_types (id) {
        id -> Int4,
        name -> Varchar,
        description -> Text,
    }
}

diesel::table! {
    use diesel::sql_types::*;
    tasks (id) {
        id -> Int4,
        user_id -> Int4,
        status -> SmallInt,
        task_type_id -> Int4,
        created_at -> Timestamp,
        started_at -> Nullable<Timestamp>,
        updated_at -> Timestamp,
    }
}

diesel::table! {
    documents (id) {
        id -> Int4,
        name -> Varchar,
        description -> Text,
        path -> Varchar,
        user_id -> Int4,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

