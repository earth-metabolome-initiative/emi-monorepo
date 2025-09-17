use crate::codegen::diesel_codegen::tables::{directus_users::directus_users, si_units::si_units};
diesel::allow_tables_to_appear_in_same_query!(si_units, directus_users);
