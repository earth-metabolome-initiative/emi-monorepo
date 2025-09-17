use crate::codegen::diesel_codegen::tables::{
    physical_asset_models::physical_asset_models,
    positioning_device_models::positioning_device_models,
};
diesel::allow_tables_to_appear_in_same_query!(positioning_device_models, physical_asset_models);
use crate::codegen::diesel_codegen::tables::asset_models::asset_models;
diesel::allow_tables_to_appear_in_same_query!(positioning_device_models, asset_models);
