use crate::codegen::diesel_codegen::tables::{
    asset_models::asset_models, commercial_pipette_tip_models::commercial_pipette_tip_models,
};
diesel::allow_tables_to_appear_in_same_query!(commercial_pipette_tip_models, asset_models);
use crate::codegen::diesel_codegen::tables::commercial_products::commercial_products;
diesel::allow_tables_to_appear_in_same_query!(commercial_pipette_tip_models, commercial_products);
use crate::codegen::diesel_codegen::tables::pipette_tip_models::pipette_tip_models;
diesel::allow_tables_to_appear_in_same_query!(commercial_pipette_tip_models, pipette_tip_models);
use crate::codegen::diesel_codegen::tables::physical_asset_models::physical_asset_models;
diesel::allow_tables_to_appear_in_same_query!(commercial_pipette_tip_models, physical_asset_models);
