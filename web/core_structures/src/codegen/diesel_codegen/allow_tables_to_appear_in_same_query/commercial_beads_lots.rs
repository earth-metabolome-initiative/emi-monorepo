use crate::codegen::diesel_codegen::tables::{
    asset_models::asset_models, commercial_beads_lots::commercial_beads_lots,
};
diesel::allow_tables_to_appear_in_same_query!(commercial_beads_lots, asset_models);
use crate::codegen::diesel_codegen::tables::beads_models::beads_models;
diesel::allow_tables_to_appear_in_same_query!(commercial_beads_lots, beads_models);
use crate::codegen::diesel_codegen::tables::commercial_beads_models::commercial_beads_models;
diesel::allow_tables_to_appear_in_same_query!(commercial_beads_lots, commercial_beads_models);
use crate::codegen::diesel_codegen::tables::commercial_product_lots::commercial_product_lots;
diesel::allow_tables_to_appear_in_same_query!(commercial_beads_lots, commercial_product_lots);
use crate::codegen::diesel_codegen::tables::physical_asset_models::physical_asset_models;
diesel::allow_tables_to_appear_in_same_query!(commercial_beads_lots, physical_asset_models);
