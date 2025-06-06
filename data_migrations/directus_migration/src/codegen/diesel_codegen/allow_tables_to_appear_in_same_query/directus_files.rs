use crate::codegen::diesel_codegen::tables::directus_files::directus_files;
use crate::codegen::diesel_codegen::tables::directus_folders::directus_folders;
diesel::allow_tables_to_appear_in_same_query!(directus_files, directus_folders);
use crate::codegen::diesel_codegen::tables::directus_users::directus_users;
diesel::allow_tables_to_appear_in_same_query!(directus_files, directus_users);
