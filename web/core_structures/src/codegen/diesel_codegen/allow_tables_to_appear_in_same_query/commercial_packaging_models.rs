use crate::codegen::diesel_codegen::tables::{
    commercial_packaging_models::commercial_packaging_models,
    commercial_products::commercial_products,
};
diesel::allow_tables_to_appear_in_same_query!(commercial_packaging_models, commercial_products);
use crate::codegen::diesel_codegen::tables::packaging_models::packaging_models;
diesel::allow_tables_to_appear_in_same_query!(commercial_packaging_models, packaging_models);
use crate::codegen::diesel_codegen::tables::asset_models::asset_models;
diesel::allow_tables_to_appear_in_same_query!(commercial_packaging_models, asset_models);
use crate::codegen::diesel_codegen::tables::physical_asset_models::physical_asset_models;
diesel::allow_tables_to_appear_in_same_query!(commercial_packaging_models, physical_asset_models);
