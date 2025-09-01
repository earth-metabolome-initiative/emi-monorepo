use crate::codegen::diesel_codegen::tables::{
    container_compatibility_rules::container_compatibility_rules,
    container_models::container_models,
};
diesel::allow_tables_to_appear_in_same_query!(container_compatibility_rules, container_models);
use crate::codegen::diesel_codegen::tables::physical_asset_models::physical_asset_models;
diesel::allow_tables_to_appear_in_same_query!(container_compatibility_rules, physical_asset_models);
use crate::codegen::diesel_codegen::tables::users::users;
diesel::allow_tables_to_appear_in_same_query!(container_compatibility_rules, users);
