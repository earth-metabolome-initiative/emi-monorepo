use crate::codegen::diesel_codegen::tables::si_units::si_units;
use crate::codegen::diesel_codegen::tables::directus_users::directus_users;
diesel::allow_tables_to_appear_in_same_query!(si_units, directus_users);
