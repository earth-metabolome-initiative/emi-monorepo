use crate::codegen::diesel_codegen::tables::{processables::processables, trackables::trackables};
diesel::allow_tables_to_appear_in_same_query!(processables, trackables);
