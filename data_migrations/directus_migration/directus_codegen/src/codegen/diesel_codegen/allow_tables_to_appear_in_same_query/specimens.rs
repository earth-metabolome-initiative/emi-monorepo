use crate::codegen::diesel_codegen::tables::specimens::specimens;
use crate::codegen::diesel_codegen::tables::directus_users::directus_users;
diesel::allow_tables_to_appear_in_same_query!(specimens, directus_users);
