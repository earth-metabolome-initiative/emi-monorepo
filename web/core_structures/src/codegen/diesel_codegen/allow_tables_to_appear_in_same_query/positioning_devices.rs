use crate::codegen::diesel_codegen::tables::{
    assets::assets, positioning_devices::positioning_devices,
};
diesel::allow_tables_to_appear_in_same_query!(positioning_devices, assets);
use crate::codegen::diesel_codegen::tables::commercial_positioning_device_lots::commercial_positioning_device_lots;
diesel::allow_tables_to_appear_in_same_query!(
    positioning_devices,
    commercial_positioning_device_lots
);
use crate::codegen::diesel_codegen::tables::physical_assets::physical_assets;
diesel::allow_tables_to_appear_in_same_query!(positioning_devices, physical_assets);
