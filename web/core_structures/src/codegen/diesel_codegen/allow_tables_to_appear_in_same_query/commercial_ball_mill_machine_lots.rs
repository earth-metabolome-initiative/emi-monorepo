use crate::codegen::diesel_codegen::tables::{
    asset_models::asset_models,
    commercial_ball_mill_machine_lots::commercial_ball_mill_machine_lots,
};
diesel::allow_tables_to_appear_in_same_query!(commercial_ball_mill_machine_lots, asset_models);
use crate::codegen::diesel_codegen::tables::ball_mill_machine_models::ball_mill_machine_models;
diesel::allow_tables_to_appear_in_same_query!(
    commercial_ball_mill_machine_lots,
    ball_mill_machine_models
);
use crate::codegen::diesel_codegen::tables::commercial_ball_mill_machine_models::commercial_ball_mill_machine_models;
diesel::allow_tables_to_appear_in_same_query!(
    commercial_ball_mill_machine_lots,
    commercial_ball_mill_machine_models
);
use crate::codegen::diesel_codegen::tables::commercial_product_lots::commercial_product_lots;
diesel::allow_tables_to_appear_in_same_query!(
    commercial_ball_mill_machine_lots,
    commercial_product_lots
);
use crate::codegen::diesel_codegen::tables::physical_asset_models::physical_asset_models;
diesel::allow_tables_to_appear_in_same_query!(
    commercial_ball_mill_machine_lots,
    physical_asset_models
);
