use crate::codegen::diesel_codegen::tables::{
    asset_models::asset_models, commercial_packaging_lots::commercial_packaging_lots,
};
diesel::allow_tables_to_appear_in_same_query!(commercial_packaging_lots, asset_models);
use crate::codegen::diesel_codegen::tables::commercial_packaging_models::commercial_packaging_models;
diesel::allow_tables_to_appear_in_same_query!(
    commercial_packaging_lots,
    commercial_packaging_models
);
use crate::codegen::diesel_codegen::tables::commercial_product_lots::commercial_product_lots;
diesel::allow_tables_to_appear_in_same_query!(commercial_packaging_lots, commercial_product_lots);
use crate::codegen::diesel_codegen::tables::packaging_models::packaging_models;
diesel::allow_tables_to_appear_in_same_query!(commercial_packaging_lots, packaging_models);
use crate::codegen::diesel_codegen::tables::physical_asset_models::physical_asset_models;
diesel::allow_tables_to_appear_in_same_query!(commercial_packaging_lots, physical_asset_models);
