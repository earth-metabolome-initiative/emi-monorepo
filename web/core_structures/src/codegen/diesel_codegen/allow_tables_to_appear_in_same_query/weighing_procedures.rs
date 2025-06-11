use crate::codegen::diesel_codegen::tables::{
    procedure_trackables::procedure_trackables, weighing_procedures::weighing_procedures,
};
diesel::allow_tables_to_appear_in_same_query!(weighing_procedures, procedure_trackables);
use crate::codegen::diesel_codegen::tables::procedures::procedures;
diesel::allow_tables_to_appear_in_same_query!(weighing_procedures, procedures);
use crate::codegen::diesel_codegen::tables::trackables::trackables;
diesel::allow_tables_to_appear_in_same_query!(weighing_procedures, trackables);
use crate::codegen::diesel_codegen::tables::weighing_procedure_models::weighing_procedure_models;
diesel::allow_tables_to_appear_in_same_query!(weighing_procedures, weighing_procedure_models);
