use crate::codegen::diesel_codegen::tables::{
    capping_rules::capping_rules, container_models::container_models,
};
diesel::allow_tables_to_appear_in_same_query!(capping_rules, container_models);
use crate::codegen::diesel_codegen::tables::trackables::trackables;
diesel::allow_tables_to_appear_in_same_query!(capping_rules, trackables);
