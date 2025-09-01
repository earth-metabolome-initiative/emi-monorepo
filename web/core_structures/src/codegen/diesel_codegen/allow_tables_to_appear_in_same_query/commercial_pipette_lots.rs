use crate::codegen::diesel_codegen::tables::{
    asset_models::asset_models, commercial_pipette_lots::commercial_pipette_lots,
};
diesel::allow_tables_to_appear_in_same_query!(commercial_pipette_lots, asset_models);
use crate::codegen::diesel_codegen::tables::commercial_pipette_models::commercial_pipette_models;
diesel::allow_tables_to_appear_in_same_query!(commercial_pipette_lots, commercial_pipette_models);
use crate::codegen::diesel_codegen::tables::commercial_product_lots::commercial_product_lots;
diesel::allow_tables_to_appear_in_same_query!(commercial_pipette_lots, commercial_product_lots);
use crate::codegen::diesel_codegen::tables::pipette_models::pipette_models;
diesel::allow_tables_to_appear_in_same_query!(commercial_pipette_lots, pipette_models);
use crate::codegen::diesel_codegen::tables::physical_asset_models::physical_asset_models;
diesel::allow_tables_to_appear_in_same_query!(commercial_pipette_lots, physical_asset_models);
