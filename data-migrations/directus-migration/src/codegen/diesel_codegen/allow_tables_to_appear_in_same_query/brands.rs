use crate::codegen::diesel_codegen::tables::{brands::brands, directus_users::directus_users};
diesel::allow_tables_to_appear_in_same_query!(brands, directus_users);
