use crate::codegen::diesel_codegen::tables::{
    asset_models::asset_models,
    commercial_ball_mill_machine_models::commercial_ball_mill_machine_models,
};
diesel::allow_tables_to_appear_in_same_query!(commercial_ball_mill_machine_models, asset_models);
use crate::codegen::diesel_codegen::tables::ball_mill_machine_models::ball_mill_machine_models;
diesel::allow_tables_to_appear_in_same_query!(
    commercial_ball_mill_machine_models,
    ball_mill_machine_models
);
use crate::codegen::diesel_codegen::tables::commercial_products::commercial_products;
diesel::allow_tables_to_appear_in_same_query!(
    commercial_ball_mill_machine_models,
    commercial_products
);
use crate::codegen::diesel_codegen::tables::physical_asset_models::physical_asset_models;
diesel::allow_tables_to_appear_in_same_query!(
    commercial_ball_mill_machine_models,
    physical_asset_models
);
