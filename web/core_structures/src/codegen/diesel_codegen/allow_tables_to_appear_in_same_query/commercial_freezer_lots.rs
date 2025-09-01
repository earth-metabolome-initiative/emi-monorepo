use crate::codegen::diesel_codegen::tables::{
    asset_models::asset_models, commercial_freezer_lots::commercial_freezer_lots,
};
diesel::allow_tables_to_appear_in_same_query!(commercial_freezer_lots, asset_models);
use crate::codegen::diesel_codegen::tables::commercial_freezer_models::commercial_freezer_models;
diesel::allow_tables_to_appear_in_same_query!(commercial_freezer_lots, commercial_freezer_models);
use crate::codegen::diesel_codegen::tables::commercial_product_lots::commercial_product_lots;
diesel::allow_tables_to_appear_in_same_query!(commercial_freezer_lots, commercial_product_lots);
use crate::codegen::diesel_codegen::tables::freezer_models::freezer_models;
diesel::allow_tables_to_appear_in_same_query!(commercial_freezer_lots, freezer_models);
use crate::codegen::diesel_codegen::tables::physical_asset_models::physical_asset_models;
diesel::allow_tables_to_appear_in_same_query!(commercial_freezer_lots, physical_asset_models);
