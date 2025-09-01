use crate::codegen::diesel_codegen::tables::{
    asset_models::asset_models,
    commercial_volume_measuring_device_lots::commercial_volume_measuring_device_lots,
};
diesel::allow_tables_to_appear_in_same_query!(
    commercial_volume_measuring_device_lots,
    asset_models
);
use crate::codegen::diesel_codegen::tables::commercial_product_lots::commercial_product_lots;
diesel::allow_tables_to_appear_in_same_query!(
    commercial_volume_measuring_device_lots,
    commercial_product_lots
);
use crate::codegen::diesel_codegen::tables::commercial_volume_measuring_device_models::commercial_volume_measuring_device_models;
diesel::allow_tables_to_appear_in_same_query!(
    commercial_volume_measuring_device_lots,
    commercial_volume_measuring_device_models
);
use crate::codegen::diesel_codegen::tables::volume_measuring_device_models::volume_measuring_device_models;
diesel::allow_tables_to_appear_in_same_query!(
    commercial_volume_measuring_device_lots,
    volume_measuring_device_models
);
use crate::codegen::diesel_codegen::tables::physical_asset_models::physical_asset_models;
diesel::allow_tables_to_appear_in_same_query!(
    commercial_volume_measuring_device_lots,
    physical_asset_models
);
