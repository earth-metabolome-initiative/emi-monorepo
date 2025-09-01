use crate::codegen::diesel_codegen::tables::{
    freezing_procedure_templates::freezing_procedure_templates,
    freezing_procedures::freezing_procedures,
};
diesel::allow_tables_to_appear_in_same_query!(freezing_procedures, freezing_procedure_templates);
use crate::codegen::diesel_codegen::tables::procedure_templates::procedure_templates;
diesel::allow_tables_to_appear_in_same_query!(freezing_procedures, procedure_templates);
use crate::codegen::diesel_codegen::tables::procedures::procedures;
diesel::allow_tables_to_appear_in_same_query!(freezing_procedures, procedures);
use crate::codegen::diesel_codegen::tables::assets::assets;
diesel::allow_tables_to_appear_in_same_query!(freezing_procedures, assets);
use crate::codegen::diesel_codegen::tables::freezer_models::freezer_models;
diesel::allow_tables_to_appear_in_same_query!(freezing_procedures, freezer_models);
use crate::codegen::diesel_codegen::tables::freezers::freezers;
diesel::allow_tables_to_appear_in_same_query!(freezing_procedures, freezers);
use crate::codegen::diesel_codegen::tables::procedure_assets::procedure_assets;
diesel::allow_tables_to_appear_in_same_query!(freezing_procedures, procedure_assets);
use crate::codegen::diesel_codegen::tables::volumetric_containers::volumetric_containers;
diesel::allow_tables_to_appear_in_same_query!(freezing_procedures, volumetric_containers);
