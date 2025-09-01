use crate::codegen::diesel_codegen::tables::{
    asset_models::asset_models, commercial_weighing_device_lots::commercial_weighing_device_lots,
};
diesel::allow_tables_to_appear_in_same_query!(commercial_weighing_device_lots, asset_models);
use crate::codegen::diesel_codegen::tables::commercial_product_lots::commercial_product_lots;
diesel::allow_tables_to_appear_in_same_query!(
    commercial_weighing_device_lots,
    commercial_product_lots
);
use crate::codegen::diesel_codegen::tables::commercial_weighing_device_models::commercial_weighing_device_models;
diesel::allow_tables_to_appear_in_same_query!(
    commercial_weighing_device_lots,
    commercial_weighing_device_models
);
use crate::codegen::diesel_codegen::tables::weighing_device_models::weighing_device_models;
diesel::allow_tables_to_appear_in_same_query!(
    commercial_weighing_device_lots,
    weighing_device_models
);
use crate::codegen::diesel_codegen::tables::physical_asset_models::physical_asset_models;
diesel::allow_tables_to_appear_in_same_query!(
    commercial_weighing_device_lots,
    physical_asset_models
);
