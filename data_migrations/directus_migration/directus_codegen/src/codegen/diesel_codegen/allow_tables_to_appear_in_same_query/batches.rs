use crate::codegen::diesel_codegen::tables::{batch_types::batch_types, batches::batches};
diesel::allow_tables_to_appear_in_same_query!(batches, batch_types);
use crate::codegen::diesel_codegen::tables::directus_users::directus_users;
diesel::allow_tables_to_appear_in_same_query!(batches, directus_users);
