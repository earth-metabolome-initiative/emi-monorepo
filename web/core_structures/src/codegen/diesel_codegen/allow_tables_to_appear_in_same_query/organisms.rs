use crate::codegen::diesel_codegen::tables::{organisms::organisms, trackables::trackables};
diesel::allow_tables_to_appear_in_same_query!(organisms, trackables);
