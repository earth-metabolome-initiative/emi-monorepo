use crate::codegen::diesel_codegen::tables::{
    trackable_ancestors::trackable_ancestors, trackables::trackables,
};
diesel::allow_tables_to_appear_in_same_query!(trackable_ancestors, trackables);
