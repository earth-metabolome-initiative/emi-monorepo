use crate::codegen::diesel_codegen::tables::{assets::assets, freezers::freezers};
diesel::allow_tables_to_appear_in_same_query!(freezers, assets);
use crate::codegen::diesel_codegen::tables::commercial_freezer_lots::commercial_freezer_lots;
diesel::allow_tables_to_appear_in_same_query!(freezers, commercial_freezer_lots);
use crate::codegen::diesel_codegen::tables::physical_assets::physical_assets;
diesel::allow_tables_to_appear_in_same_query!(freezers, physical_assets);
