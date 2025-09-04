use crate::codegen::diesel_codegen::tables::{
    asset_models::asset_models, commercial_cap_lots::commercial_cap_lots,
};
diesel::allow_tables_to_appear_in_same_query!(commercial_cap_lots, asset_models);
use crate::codegen::diesel_codegen::tables::cap_models::cap_models;
diesel::allow_tables_to_appear_in_same_query!(commercial_cap_lots, cap_models);
use crate::codegen::diesel_codegen::tables::commercial_cap_models::commercial_cap_models;
diesel::allow_tables_to_appear_in_same_query!(commercial_cap_lots, commercial_cap_models);
use crate::codegen::diesel_codegen::tables::commercial_product_lots::commercial_product_lots;
diesel::allow_tables_to_appear_in_same_query!(commercial_cap_lots, commercial_product_lots);
use crate::codegen::diesel_codegen::tables::physical_asset_models::physical_asset_models;
diesel::allow_tables_to_appear_in_same_query!(commercial_cap_lots, physical_asset_models);
