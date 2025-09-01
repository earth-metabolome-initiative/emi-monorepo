use crate::codegen::diesel_codegen::tables::{
    fractioning_procedure_templates::fractioning_procedure_templates,
    fractioning_procedures::fractioning_procedures,
};
diesel::allow_tables_to_appear_in_same_query!(
    fractioning_procedures,
    fractioning_procedure_templates
);
use crate::codegen::diesel_codegen::tables::procedure_templates::procedure_templates;
diesel::allow_tables_to_appear_in_same_query!(fractioning_procedures, procedure_templates);
use crate::codegen::diesel_codegen::tables::procedures::procedures;
diesel::allow_tables_to_appear_in_same_query!(fractioning_procedures, procedures);
use crate::codegen::diesel_codegen::tables::assets::assets;
diesel::allow_tables_to_appear_in_same_query!(fractioning_procedures, assets);
use crate::codegen::diesel_codegen::tables::procedure_assets::procedure_assets;
diesel::allow_tables_to_appear_in_same_query!(fractioning_procedures, procedure_assets);
use crate::codegen::diesel_codegen::tables::volumetric_containers::volumetric_containers;
diesel::allow_tables_to_appear_in_same_query!(fractioning_procedures, volumetric_containers);
use crate::codegen::diesel_codegen::tables::weighing_device_models::weighing_device_models;
diesel::allow_tables_to_appear_in_same_query!(fractioning_procedures, weighing_device_models);
use crate::codegen::diesel_codegen::tables::weighing_devices::weighing_devices;
diesel::allow_tables_to_appear_in_same_query!(fractioning_procedures, weighing_devices);
