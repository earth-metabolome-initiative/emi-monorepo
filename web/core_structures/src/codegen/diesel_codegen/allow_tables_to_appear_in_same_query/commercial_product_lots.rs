use crate::codegen::diesel_codegen::tables::{
    asset_models::asset_models, commercial_product_lots::commercial_product_lots,
};
diesel::allow_tables_to_appear_in_same_query!(commercial_product_lots, asset_models);
use crate::codegen::diesel_codegen::tables::commercial_products::commercial_products;
diesel::allow_tables_to_appear_in_same_query!(commercial_product_lots, commercial_products);
use crate::codegen::diesel_codegen::tables::physical_asset_models::physical_asset_models;
diesel::allow_tables_to_appear_in_same_query!(commercial_product_lots, physical_asset_models);
