use crate::codegen::diesel_codegen::tables::{
    asset_models::asset_models, commercial_freeze_dryer_lots::commercial_freeze_dryer_lots,
};
diesel::allow_tables_to_appear_in_same_query!(commercial_freeze_dryer_lots, asset_models);
use crate::codegen::diesel_codegen::tables::commercial_freeze_dryer_models::commercial_freeze_dryer_models;
diesel::allow_tables_to_appear_in_same_query!(
    commercial_freeze_dryer_lots,
    commercial_freeze_dryer_models
);
use crate::codegen::diesel_codegen::tables::commercial_product_lots::commercial_product_lots;
diesel::allow_tables_to_appear_in_same_query!(
    commercial_freeze_dryer_lots,
    commercial_product_lots
);
use crate::codegen::diesel_codegen::tables::freeze_dryer_models::freeze_dryer_models;
diesel::allow_tables_to_appear_in_same_query!(commercial_freeze_dryer_lots, freeze_dryer_models);
use crate::codegen::diesel_codegen::tables::physical_asset_models::physical_asset_models;
diesel::allow_tables_to_appear_in_same_query!(commercial_freeze_dryer_lots, physical_asset_models);
