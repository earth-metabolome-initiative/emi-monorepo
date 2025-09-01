use crate::codegen::diesel_codegen::tables::{assets::assets, freeze_dryers::freeze_dryers};
diesel::allow_tables_to_appear_in_same_query!(freeze_dryers, assets);
use crate::codegen::diesel_codegen::tables::commercial_freeze_dryer_lots::commercial_freeze_dryer_lots;
diesel::allow_tables_to_appear_in_same_query!(freeze_dryers, commercial_freeze_dryer_lots);
use crate::codegen::diesel_codegen::tables::physical_assets::physical_assets;
diesel::allow_tables_to_appear_in_same_query!(freeze_dryers, physical_assets);
