use crate::codegen::diesel_codegen::tables::{
    directus_access::directus_access, directus_policies::directus_policies,
};
diesel::allow_tables_to_appear_in_same_query!(directus_access, directus_policies);
use crate::codegen::diesel_codegen::tables::directus_users::directus_users;
diesel::allow_tables_to_appear_in_same_query!(directus_access, directus_users);
use crate::codegen::diesel_codegen::tables::directus_roles::directus_roles;
diesel::allow_tables_to_appear_in_same_query!(directus_access, directus_roles);
