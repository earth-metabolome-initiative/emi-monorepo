use crate::codegen::diesel_codegen::tables::{
    brands::brands, commercial_products::commercial_products,
};
diesel::allow_tables_to_appear_in_same_query!(commercial_products, brands);
use crate::codegen::diesel_codegen::tables::physical_asset_models::physical_asset_models;
diesel::allow_tables_to_appear_in_same_query!(commercial_products, physical_asset_models);
use crate::codegen::diesel_codegen::tables::asset_models::asset_models;
diesel::allow_tables_to_appear_in_same_query!(commercial_products, asset_models);
