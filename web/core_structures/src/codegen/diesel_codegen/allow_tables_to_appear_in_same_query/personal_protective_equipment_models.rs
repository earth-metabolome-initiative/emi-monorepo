use crate::codegen::diesel_codegen::tables::{
    personal_protective_equipment_models::personal_protective_equipment_models,
    physical_asset_models::physical_asset_models,
};
diesel::allow_tables_to_appear_in_same_query!(
    personal_protective_equipment_models,
    physical_asset_models
);
use crate::codegen::diesel_codegen::tables::asset_models::asset_models;
diesel::allow_tables_to_appear_in_same_query!(personal_protective_equipment_models, asset_models);
