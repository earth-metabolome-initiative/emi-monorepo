use crate::codegen::diesel_codegen::tables::directus_revisions::directus_revisions;
use crate::codegen::diesel_codegen::tables::directus_versions::directus_versions;
diesel::allow_tables_to_appear_in_same_query!(directus_revisions, directus_versions);
use crate::codegen::diesel_codegen::tables::directus_activity::directus_activity;
diesel::allow_tables_to_appear_in_same_query!(directus_revisions, directus_activity);
