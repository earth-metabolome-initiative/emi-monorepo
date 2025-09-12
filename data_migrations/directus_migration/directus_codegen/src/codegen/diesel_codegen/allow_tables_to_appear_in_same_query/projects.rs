use crate::codegen::diesel_codegen::tables::projects::projects;
use crate::codegen::diesel_codegen::tables::batches::batches;
diesel::allow_tables_to_appear_in_same_query!(projects, batches);
use crate::codegen::diesel_codegen::tables::directus_users::directus_users;
diesel::allow_tables_to_appear_in_same_query!(projects, directus_users);
