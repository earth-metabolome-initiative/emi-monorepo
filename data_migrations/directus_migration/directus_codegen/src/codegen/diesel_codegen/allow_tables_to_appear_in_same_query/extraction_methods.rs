use crate::codegen::diesel_codegen::tables::extraction_methods::extraction_methods;
use crate::codegen::diesel_codegen::tables::directus_users::directus_users;
diesel::allow_tables_to_appear_in_same_query!(extraction_methods, directus_users);
