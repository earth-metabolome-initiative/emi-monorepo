use crate::codegen::diesel_codegen::tables::{
    asset_models::asset_models, commercial_camera_lots::commercial_camera_lots,
};
diesel::allow_tables_to_appear_in_same_query!(commercial_camera_lots, asset_models);
use crate::codegen::diesel_codegen::tables::camera_models::camera_models;
diesel::allow_tables_to_appear_in_same_query!(commercial_camera_lots, camera_models);
use crate::codegen::diesel_codegen::tables::commercial_camera_models::commercial_camera_models;
diesel::allow_tables_to_appear_in_same_query!(commercial_camera_lots, commercial_camera_models);
use crate::codegen::diesel_codegen::tables::commercial_product_lots::commercial_product_lots;
diesel::allow_tables_to_appear_in_same_query!(commercial_camera_lots, commercial_product_lots);
use crate::codegen::diesel_codegen::tables::physical_asset_models::physical_asset_models;
diesel::allow_tables_to_appear_in_same_query!(commercial_camera_lots, physical_asset_models);
