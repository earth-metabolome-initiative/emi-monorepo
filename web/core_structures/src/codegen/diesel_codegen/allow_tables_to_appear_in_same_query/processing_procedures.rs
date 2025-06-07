use crate::codegen::diesel_codegen::tables::{
    instruments::instruments, processing_procedures::processing_procedures,
};
diesel::allow_tables_to_appear_in_same_query!(processing_procedures, instruments);
use crate::codegen::diesel_codegen::tables::procedure_trackables::procedure_trackables;
diesel::allow_tables_to_appear_in_same_query!(processing_procedures, procedure_trackables);
use crate::codegen::diesel_codegen::tables::procedures::procedures;
diesel::allow_tables_to_appear_in_same_query!(processing_procedures, procedures);
use crate::codegen::diesel_codegen::tables::processables::processables;
diesel::allow_tables_to_appear_in_same_query!(processing_procedures, processables);
