use crate::codegen::diesel_codegen::tables::universities::universities;
use crate::codegen::diesel_codegen::tables::directus_users::directus_users;
diesel::allow_tables_to_appear_in_same_query!(universities, directus_users);
