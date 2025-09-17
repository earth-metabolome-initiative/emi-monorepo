use crate::codegen::diesel_codegen::tables::{
    asset_compatibility_rules::asset_compatibility_rules, asset_models::asset_models,
};
diesel::allow_tables_to_appear_in_same_query!(asset_compatibility_rules, asset_models);
use crate::codegen::diesel_codegen::tables::users::users;
diesel::allow_tables_to_appear_in_same_query!(asset_compatibility_rules, users);
