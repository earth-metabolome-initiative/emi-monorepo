use crate::codegen::diesel_codegen::tables::processables::processables;
use crate::codegen::diesel_codegen::tables::volumetric_processables::volumetric_processables;
diesel::allow_tables_to_appear_in_same_query!(volumetric_processables, processables);
use crate::codegen::diesel_codegen::tables::trackables::trackables;
diesel::allow_tables_to_appear_in_same_query!(volumetric_processables, trackables);
