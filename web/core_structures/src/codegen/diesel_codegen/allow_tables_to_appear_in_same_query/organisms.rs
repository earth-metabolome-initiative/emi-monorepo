use crate::codegen::diesel_codegen::tables::organisms::organisms;
use crate::codegen::diesel_codegen::tables::trackables::trackables;
diesel::allow_tables_to_appear_in_same_query!(organisms, trackables);
