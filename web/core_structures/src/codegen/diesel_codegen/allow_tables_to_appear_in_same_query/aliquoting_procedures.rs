use crate::codegen::diesel_codegen::tables::{
    aliquoting_procedures::aliquoting_procedures, procedure_trackables::procedure_trackables,
};
diesel::allow_tables_to_appear_in_same_query!(aliquoting_procedures, procedure_trackables);
use crate::codegen::diesel_codegen::tables::procedures::procedures;
diesel::allow_tables_to_appear_in_same_query!(aliquoting_procedures, procedures);
use crate::codegen::diesel_codegen::tables::trackables::trackables;
diesel::allow_tables_to_appear_in_same_query!(aliquoting_procedures, trackables);
use crate::codegen::diesel_codegen::tables::volumetric_processables::volumetric_processables;
diesel::allow_tables_to_appear_in_same_query!(aliquoting_procedures, volumetric_processables);
