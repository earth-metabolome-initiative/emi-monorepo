use crate::codegen::diesel_codegen::tables::{
    instrument_models::instrument_models, instruments::instruments,
};
diesel::allow_tables_to_appear_in_same_query!(instruments, instrument_models);
use crate::codegen::diesel_codegen::tables::directus_users::directus_users;
diesel::allow_tables_to_appear_in_same_query!(instruments, directus_users);
use crate::codegen::diesel_codegen::tables::rooms::rooms;
diesel::allow_tables_to_appear_in_same_query!(instruments, rooms);
