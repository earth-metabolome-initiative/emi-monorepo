// @generated automatically by Diesel CLI.

diesel::table! {
    colors (id) {
        id -> Int4,
        name -> Text,
        hexadecimal_value -> Text,
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
    primary_user_emails (id) {
        id -> Int4,
    }
}

diesel::table! {
    sampling_procedures (id) {
        id -> Uuid,
        name -> Text,
        description -> Nullable<Text>,
        created_by -> Int4,
        created_at -> Timestamp,
        updated_by -> Int4,
        updated_at -> Timestamp,
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
        profile_picture -> Nullable<Bytea>,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::joinable!(login_providers -> colors (color_id));
diesel::joinable!(login_providers -> font_awesome_icons (font_awesome_icon_id));
diesel::joinable!(notifications -> users (user_id));
diesel::joinable!(primary_user_emails -> user_emails (id));
diesel::joinable!(user_emails -> login_providers (login_provider_id));
diesel::joinable!(user_emails -> users (user_id));

diesel::allow_tables_to_appear_in_same_query!(
    colors,
    font_awesome_icons,
    login_providers,
    notifications,
    primary_user_emails,
    sampling_procedures,
    user_emails,
    users,
);
