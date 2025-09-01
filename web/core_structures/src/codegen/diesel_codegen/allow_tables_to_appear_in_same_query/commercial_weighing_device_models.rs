use crate::codegen::diesel_codegen::tables::{
    commercial_products::commercial_products,
    commercial_weighing_device_models::commercial_weighing_device_models,
};
diesel::allow_tables_to_appear_in_same_query!(
    commercial_weighing_device_models,
    commercial_products
);
use crate::codegen::diesel_codegen::tables::weighing_device_models::weighing_device_models;
diesel::allow_tables_to_appear_in_same_query!(
    commercial_weighing_device_models,
    weighing_device_models
);
use crate::codegen::diesel_codegen::tables::asset_models::asset_models;
diesel::allow_tables_to_appear_in_same_query!(commercial_weighing_device_models, asset_models);
use crate::codegen::diesel_codegen::tables::physical_asset_models::physical_asset_models;
diesel::allow_tables_to_appear_in_same_query!(
    commercial_weighing_device_models,
    physical_asset_models
);
