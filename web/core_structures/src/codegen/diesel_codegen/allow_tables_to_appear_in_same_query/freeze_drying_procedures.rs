use crate::codegen::diesel_codegen::tables::{
    assets::assets, freeze_drying_procedures::freeze_drying_procedures,
};
diesel::allow_tables_to_appear_in_same_query!(freeze_drying_procedures, assets);
use crate::codegen::diesel_codegen::tables::freeze_dryer_models::freeze_dryer_models;
diesel::allow_tables_to_appear_in_same_query!(freeze_drying_procedures, freeze_dryer_models);
use crate::codegen::diesel_codegen::tables::freeze_dryers::freeze_dryers;
diesel::allow_tables_to_appear_in_same_query!(freeze_drying_procedures, freeze_dryers);
use crate::codegen::diesel_codegen::tables::freeze_drying_procedure_templates::freeze_drying_procedure_templates;
diesel::allow_tables_to_appear_in_same_query!(
    freeze_drying_procedures,
    freeze_drying_procedure_templates
);
use crate::codegen::diesel_codegen::tables::procedure_assets::procedure_assets;
diesel::allow_tables_to_appear_in_same_query!(freeze_drying_procedures, procedure_assets);
use crate::codegen::diesel_codegen::tables::procedure_templates::procedure_templates;
diesel::allow_tables_to_appear_in_same_query!(freeze_drying_procedures, procedure_templates);
use crate::codegen::diesel_codegen::tables::procedures::procedures;
diesel::allow_tables_to_appear_in_same_query!(freeze_drying_procedures, procedures);
use crate::codegen::diesel_codegen::tables::volumetric_containers::volumetric_containers;
diesel::allow_tables_to_appear_in_same_query!(freeze_drying_procedures, volumetric_containers);
