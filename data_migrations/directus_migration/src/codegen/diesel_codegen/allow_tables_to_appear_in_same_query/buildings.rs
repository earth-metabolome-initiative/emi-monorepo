use crate::codegen::diesel_codegen::tables::{
    buildings::buildings, directus_users::directus_users,
};
diesel::allow_tables_to_appear_in_same_query!(buildings, directus_users);
use crate::codegen::diesel_codegen::tables::universities::universities;
diesel::allow_tables_to_appear_in_same_query!(buildings, universities);
