use crate::codegen::diesel_codegen::tables::processables::processables;
use crate::codegen::diesel_codegen::tables::trackables::trackables;
diesel::allow_tables_to_appear_in_same_query!(processables, trackables);
