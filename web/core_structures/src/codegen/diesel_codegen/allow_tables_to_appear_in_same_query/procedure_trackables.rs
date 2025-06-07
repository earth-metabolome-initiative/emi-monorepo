use crate::codegen::diesel_codegen::tables::{
    procedure_trackables::procedure_trackables, procedures::procedures,
};
diesel::allow_tables_to_appear_in_same_query!(procedure_trackables, procedures);
use crate::codegen::diesel_codegen::tables::trackables::trackables;
diesel::allow_tables_to_appear_in_same_query!(procedure_trackables, trackables);
use crate::codegen::diesel_codegen::tables::users::users;
diesel::allow_tables_to_appear_in_same_query!(procedure_trackables, users);
