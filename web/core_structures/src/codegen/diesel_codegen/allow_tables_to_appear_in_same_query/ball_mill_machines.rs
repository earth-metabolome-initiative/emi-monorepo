use crate::codegen::diesel_codegen::tables::{
    assets::assets, ball_mill_machines::ball_mill_machines,
};
diesel::allow_tables_to_appear_in_same_query!(ball_mill_machines, assets);
use crate::codegen::diesel_codegen::tables::commercial_ball_mill_machine_lots::commercial_ball_mill_machine_lots;
diesel::allow_tables_to_appear_in_same_query!(
    ball_mill_machines,
    commercial_ball_mill_machine_lots
);
use crate::codegen::diesel_codegen::tables::physical_assets::physical_assets;
diesel::allow_tables_to_appear_in_same_query!(ball_mill_machines, physical_assets);
