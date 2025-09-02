use crate::codegen::diesel_codegen::tables::{
    asset_models::asset_models, commercial_freezer_models::commercial_freezer_models,
};
diesel::allow_tables_to_appear_in_same_query!(commercial_freezer_models, asset_models);
use crate::codegen::diesel_codegen::tables::commercial_products::commercial_products;
diesel::allow_tables_to_appear_in_same_query!(commercial_freezer_models, commercial_products);
use crate::codegen::diesel_codegen::tables::freezer_models::freezer_models;
diesel::allow_tables_to_appear_in_same_query!(commercial_freezer_models, freezer_models);
use crate::codegen::diesel_codegen::tables::physical_asset_models::physical_asset_models;
diesel::allow_tables_to_appear_in_same_query!(commercial_freezer_models, physical_asset_models);
