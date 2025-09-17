use crate::codegen::diesel_codegen::tables::{
    asset_models::asset_models, commercial_bead_lots::commercial_bead_lots,
};
diesel::allow_tables_to_appear_in_same_query!(commercial_bead_lots, asset_models);
use crate::codegen::diesel_codegen::tables::bead_models::bead_models;
diesel::allow_tables_to_appear_in_same_query!(commercial_bead_lots, bead_models);
use crate::codegen::diesel_codegen::tables::commercial_bead_models::commercial_bead_models;
diesel::allow_tables_to_appear_in_same_query!(commercial_bead_lots, commercial_bead_models);
use crate::codegen::diesel_codegen::tables::commercial_product_lots::commercial_product_lots;
diesel::allow_tables_to_appear_in_same_query!(commercial_bead_lots, commercial_product_lots);
use crate::codegen::diesel_codegen::tables::physical_asset_models::physical_asset_models;
diesel::allow_tables_to_appear_in_same_query!(commercial_bead_lots, physical_asset_models);
