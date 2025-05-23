use crate::codegen::diesel_codegen::tables::{
    aliquoting_steps::aliquoting_steps, instruments::instruments,
};
diesel::allow_tables_to_appear_in_same_query!(aliquoting_steps, instruments);
use crate::codegen::diesel_codegen::tables::steps::steps;
diesel::allow_tables_to_appear_in_same_query!(aliquoting_steps, steps);
use crate::codegen::diesel_codegen::tables::users::users;
diesel::allow_tables_to_appear_in_same_query!(aliquoting_steps, users);
use crate::codegen::diesel_codegen::tables::volumetric_processables::volumetric_processables;
diesel::allow_tables_to_appear_in_same_query!(aliquoting_steps, volumetric_processables);
