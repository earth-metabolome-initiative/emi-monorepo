use crate::codegen::diesel_codegen::tables::organizations::organizations;
use crate::codegen::diesel_codegen::tables::user_organizations::user_organizations;
diesel::allow_tables_to_appear_in_same_query!(user_organizations, organizations);
use crate::codegen::diesel_codegen::tables::users::users;
diesel::allow_tables_to_appear_in_same_query!(user_organizations, users);
