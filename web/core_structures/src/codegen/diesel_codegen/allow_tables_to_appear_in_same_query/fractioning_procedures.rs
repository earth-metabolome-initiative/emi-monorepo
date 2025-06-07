use crate::codegen::diesel_codegen::tables::{
    fractioning_procedures::fractioning_procedures, procedure_trackables::procedure_trackables,
};
diesel::allow_tables_to_appear_in_same_query!(fractioning_procedures, procedure_trackables);
use crate::codegen::diesel_codegen::tables::procedures::procedures;
diesel::allow_tables_to_appear_in_same_query!(fractioning_procedures, procedures);
use crate::codegen::diesel_codegen::tables::processables::processables;
diesel::allow_tables_to_appear_in_same_query!(fractioning_procedures, processables);
use crate::codegen::diesel_codegen::tables::trackables::trackables;
diesel::allow_tables_to_appear_in_same_query!(fractioning_procedures, trackables);
