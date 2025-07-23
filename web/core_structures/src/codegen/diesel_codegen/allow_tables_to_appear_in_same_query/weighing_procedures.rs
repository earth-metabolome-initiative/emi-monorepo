use crate::codegen::diesel_codegen::tables::{
    procedure_trackables::procedure_trackables, weighing_procedures::weighing_procedures,
};
diesel::allow_tables_to_appear_in_same_query!(weighing_procedures, procedure_trackables);
use crate::codegen::diesel_codegen::tables::procedures::procedures;
diesel::allow_tables_to_appear_in_same_query!(weighing_procedures, procedures);
use crate::codegen::diesel_codegen::tables::volumetric_container_models::volumetric_container_models;
diesel::allow_tables_to_appear_in_same_query!(weighing_procedures, volumetric_container_models);
use crate::codegen::diesel_codegen::tables::weighing_instrument_models::weighing_instrument_models;
diesel::allow_tables_to_appear_in_same_query!(weighing_procedures, weighing_instrument_models);
use crate::codegen::diesel_codegen::tables::weighing_procedure_models::weighing_procedure_models;
diesel::allow_tables_to_appear_in_same_query!(weighing_procedures, weighing_procedure_models);
