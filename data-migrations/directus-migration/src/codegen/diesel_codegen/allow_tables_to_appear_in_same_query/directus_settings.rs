use crate::codegen::diesel_codegen::tables::directus_settings::directus_settings;
use crate::codegen::diesel_codegen::tables::directus_roles::directus_roles;
diesel::allow_tables_to_appear_in_same_query!(directus_settings, directus_roles);
use crate::codegen::diesel_codegen::tables::directus_files::directus_files;
diesel::allow_tables_to_appear_in_same_query!(directus_settings, directus_files);
use crate::codegen::diesel_codegen::tables::directus_folders::directus_folders;
diesel::allow_tables_to_appear_in_same_query!(directus_settings, directus_folders);
