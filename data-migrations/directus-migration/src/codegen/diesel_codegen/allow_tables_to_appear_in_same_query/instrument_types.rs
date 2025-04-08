use crate::codegen::diesel_codegen::tables::{
    directus_users::directus_users, instrument_types::instrument_types,
};
diesel::allow_tables_to_appear_in_same_query!(instrument_types, directus_users);
