use crate::codegen::diesel_codegen::tables::{batches::batches, directus_users::directus_users};
diesel::allow_tables_to_appear_in_same_query!(batches, directus_users);
use crate::codegen::diesel_codegen::tables::batch_types::batch_types;
diesel::allow_tables_to_appear_in_same_query!(batches, batch_types);
