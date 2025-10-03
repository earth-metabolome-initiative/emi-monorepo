use crate::codegen::diesel_codegen::tables::{
    physical_assets::physical_assets, tagging_procedures::tagging_procedures,
};
diesel::allow_tables_to_appear_in_same_query!(tagging_procedures, physical_assets);
use crate::codegen::diesel_codegen::tables::positioning_devices::positioning_devices;
diesel::allow_tables_to_appear_in_same_query!(tagging_procedures, positioning_devices);
use crate::codegen::diesel_codegen::tables::procedure_assets::procedure_assets;
diesel::allow_tables_to_appear_in_same_query!(tagging_procedures, procedure_assets);
use crate::codegen::diesel_codegen::tables::procedure_template_asset_models::procedure_template_asset_models;
diesel::allow_tables_to_appear_in_same_query!(tagging_procedures, procedure_template_asset_models);
use crate::codegen::diesel_codegen::tables::procedures::procedures;
diesel::allow_tables_to_appear_in_same_query!(tagging_procedures, procedures);
use crate::codegen::diesel_codegen::tables::tagging_procedure_templates::tagging_procedure_templates;
diesel::allow_tables_to_appear_in_same_query!(tagging_procedures, tagging_procedure_templates);
