use crate::codegen::diesel_codegen::tables::{
    aliquoting_data::aliquoting_data, si_units::si_units,
};
diesel::allow_tables_to_appear_in_same_query!(aliquoting_data, si_units);
use crate::codegen::diesel_codegen::tables::directus_users::directus_users;
diesel::allow_tables_to_appear_in_same_query!(aliquoting_data, directus_users);
use crate::codegen::diesel_codegen::tables::containers::containers;
diesel::allow_tables_to_appear_in_same_query!(aliquoting_data, containers);
