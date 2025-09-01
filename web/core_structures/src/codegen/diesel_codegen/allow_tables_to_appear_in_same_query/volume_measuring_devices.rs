use crate::codegen::diesel_codegen::tables::{
    assets::assets, volume_measuring_devices::volume_measuring_devices,
};
diesel::allow_tables_to_appear_in_same_query!(volume_measuring_devices, assets);
use crate::codegen::diesel_codegen::tables::commercial_volume_measuring_device_lots::commercial_volume_measuring_device_lots;
diesel::allow_tables_to_appear_in_same_query!(
    volume_measuring_devices,
    commercial_volume_measuring_device_lots
);
use crate::codegen::diesel_codegen::tables::physical_assets::physical_assets;
diesel::allow_tables_to_appear_in_same_query!(volume_measuring_devices, physical_assets);
