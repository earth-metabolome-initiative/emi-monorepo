use crate::codegen::diesel_codegen::tables::{
    physical_asset_models::physical_asset_models, weighing_device_models::weighing_device_models,
};
diesel::allow_tables_to_appear_in_same_query!(weighing_device_models, physical_asset_models);
use crate::codegen::diesel_codegen::tables::asset_models::asset_models;
diesel::allow_tables_to_appear_in_same_query!(weighing_device_models, asset_models);
