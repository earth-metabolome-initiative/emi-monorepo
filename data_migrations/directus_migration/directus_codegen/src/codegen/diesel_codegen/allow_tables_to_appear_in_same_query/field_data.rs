use crate::codegen::diesel_codegen::tables::field_data::field_data;
use crate::codegen::diesel_codegen::tables::directus_users::directus_users;
diesel::allow_tables_to_appear_in_same_query!(field_data, directus_users);
