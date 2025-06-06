use crate::codegen::diesel_codegen::tables::{
    procedure_trackables::procedure_trackables, sampling_procedures::sampling_procedures,
};
diesel::allow_tables_to_appear_in_same_query!(sampling_procedures, procedure_trackables);
use crate::codegen::diesel_codegen::tables::procedures::procedures;
diesel::allow_tables_to_appear_in_same_query!(sampling_procedures, procedures);
use crate::codegen::diesel_codegen::tables::processables::processables;
diesel::allow_tables_to_appear_in_same_query!(sampling_procedures, processables);
use crate::codegen::diesel_codegen::tables::trackables::trackables;
diesel::allow_tables_to_appear_in_same_query!(sampling_procedures, trackables);
