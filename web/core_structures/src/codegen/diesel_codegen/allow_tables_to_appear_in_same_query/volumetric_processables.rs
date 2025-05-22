use crate::codegen::diesel_codegen::tables::{
    processables::processables, volumetric_processables::volumetric_processables,
};
diesel::allow_tables_to_appear_in_same_query!(volumetric_processables, processables);
