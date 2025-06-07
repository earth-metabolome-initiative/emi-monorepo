use crate::codegen::diesel_codegen::tables::{buildings::buildings, universities::universities};
diesel::allow_tables_to_appear_in_same_query!(buildings, universities);
use crate::codegen::diesel_codegen::tables::directus_users::directus_users;
diesel::allow_tables_to_appear_in_same_query!(buildings, directus_users);
