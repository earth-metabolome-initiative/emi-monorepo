use crate::codegen::diesel_codegen::tables::{assets::assets, weighing_devices::weighing_devices};
diesel::allow_tables_to_appear_in_same_query!(weighing_devices, assets);
use crate::codegen::diesel_codegen::tables::commercial_weighing_device_lots::commercial_weighing_device_lots;
diesel::allow_tables_to_appear_in_same_query!(weighing_devices, commercial_weighing_device_lots);
use crate::codegen::diesel_codegen::tables::physical_assets::physical_assets;
diesel::allow_tables_to_appear_in_same_query!(weighing_devices, physical_assets);
