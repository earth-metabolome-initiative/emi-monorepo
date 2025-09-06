use crate::codegen::diesel_codegen::tables::{
    geolocation_procedure_templates::geolocation_procedure_templates,
    geolocation_procedures::geolocation_procedures,
};
diesel::allow_tables_to_appear_in_same_query!(
    geolocation_procedures,
    geolocation_procedure_templates
);
use crate::codegen::diesel_codegen::tables::physical_assets::physical_assets;
diesel::allow_tables_to_appear_in_same_query!(geolocation_procedures, physical_assets);
use crate::codegen::diesel_codegen::tables::positioning_devices::positioning_devices;
diesel::allow_tables_to_appear_in_same_query!(geolocation_procedures, positioning_devices);
use crate::codegen::diesel_codegen::tables::procedure_assets::procedure_assets;
diesel::allow_tables_to_appear_in_same_query!(geolocation_procedures, procedure_assets);
use crate::codegen::diesel_codegen::tables::procedure_template_asset_models::procedure_template_asset_models;
diesel::allow_tables_to_appear_in_same_query!(
    geolocation_procedures,
    procedure_template_asset_models
);
use crate::codegen::diesel_codegen::tables::procedures::procedures;
diesel::allow_tables_to_appear_in_same_query!(geolocation_procedures, procedures);
