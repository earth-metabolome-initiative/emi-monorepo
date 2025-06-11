use crate::codegen::diesel_codegen::tables::{reagents::reagents, trackables::trackables};
diesel::allow_tables_to_appear_in_same_query!(reagents, trackables);
