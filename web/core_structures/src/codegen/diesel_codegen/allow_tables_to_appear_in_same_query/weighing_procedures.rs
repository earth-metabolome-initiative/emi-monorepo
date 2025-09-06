use crate::codegen::diesel_codegen::tables::{
    procedure_assets::procedure_assets, weighing_procedures::weighing_procedures,
};
diesel::allow_tables_to_appear_in_same_query!(weighing_procedures, procedure_assets);
use crate::codegen::diesel_codegen::tables::procedure_template_asset_models::procedure_template_asset_models;
diesel::allow_tables_to_appear_in_same_query!(weighing_procedures, procedure_template_asset_models);
use crate::codegen::diesel_codegen::tables::procedures::procedures;
diesel::allow_tables_to_appear_in_same_query!(weighing_procedures, procedures);
use crate::codegen::diesel_codegen::tables::volumetric_containers::volumetric_containers;
diesel::allow_tables_to_appear_in_same_query!(weighing_procedures, volumetric_containers);
use crate::codegen::diesel_codegen::tables::weighing_devices::weighing_devices;
diesel::allow_tables_to_appear_in_same_query!(weighing_procedures, weighing_devices);
use crate::codegen::diesel_codegen::tables::weighing_procedure_templates::weighing_procedure_templates;
diesel::allow_tables_to_appear_in_same_query!(weighing_procedures, weighing_procedure_templates);
