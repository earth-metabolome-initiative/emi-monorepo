use crate::codegen::diesel_codegen::tables::{
    directus_users::directus_users, extraction_methods::extraction_methods,
};
diesel::allow_tables_to_appear_in_same_query!(extraction_methods, directus_users);
