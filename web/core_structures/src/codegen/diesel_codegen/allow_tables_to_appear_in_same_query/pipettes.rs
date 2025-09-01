use crate::codegen::diesel_codegen::tables::{assets::assets, pipettes::pipettes};
diesel::allow_tables_to_appear_in_same_query!(pipettes, assets);
use crate::codegen::diesel_codegen::tables::commercial_pipette_lots::commercial_pipette_lots;
diesel::allow_tables_to_appear_in_same_query!(pipettes, commercial_pipette_lots);
use crate::codegen::diesel_codegen::tables::physical_assets::physical_assets;
diesel::allow_tables_to_appear_in_same_query!(pipettes, physical_assets);
