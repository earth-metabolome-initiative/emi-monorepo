use crate::codegen::diesel_codegen::tables::{
    assets::assets, pouring_procedures::pouring_procedures,
};
diesel::allow_tables_to_appear_in_same_query!(pouring_procedures, assets);
use crate::codegen::diesel_codegen::tables::pouring_procedure_templates::pouring_procedure_templates;
diesel::allow_tables_to_appear_in_same_query!(pouring_procedures, pouring_procedure_templates);
use crate::codegen::diesel_codegen::tables::procedure_assets::procedure_assets;
diesel::allow_tables_to_appear_in_same_query!(pouring_procedures, procedure_assets);
use crate::codegen::diesel_codegen::tables::procedure_templates::procedure_templates;
diesel::allow_tables_to_appear_in_same_query!(pouring_procedures, procedure_templates);
use crate::codegen::diesel_codegen::tables::procedures::procedures;
diesel::allow_tables_to_appear_in_same_query!(pouring_procedures, procedures);
use crate::codegen::diesel_codegen::tables::volume_measuring_device_models::volume_measuring_device_models;
diesel::allow_tables_to_appear_in_same_query!(pouring_procedures, volume_measuring_device_models);
use crate::codegen::diesel_codegen::tables::volume_measuring_devices::volume_measuring_devices;
diesel::allow_tables_to_appear_in_same_query!(pouring_procedures, volume_measuring_devices);
use crate::codegen::diesel_codegen::tables::volumetric_containers::volumetric_containers;
diesel::allow_tables_to_appear_in_same_query!(pouring_procedures, volumetric_containers);
