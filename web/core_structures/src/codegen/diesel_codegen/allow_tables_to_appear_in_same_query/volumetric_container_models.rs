use crate::codegen::diesel_codegen::tables::{
    container_models::container_models, volumetric_container_models::volumetric_container_models,
};
diesel::allow_tables_to_appear_in_same_query!(volumetric_container_models, container_models);
use crate::codegen::diesel_codegen::tables::asset_models::asset_models;
diesel::allow_tables_to_appear_in_same_query!(volumetric_container_models, asset_models);
use crate::codegen::diesel_codegen::tables::physical_asset_models::physical_asset_models;
diesel::allow_tables_to_appear_in_same_query!(volumetric_container_models, physical_asset_models);
