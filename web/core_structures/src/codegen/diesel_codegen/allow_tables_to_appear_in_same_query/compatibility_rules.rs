use crate::codegen::diesel_codegen::tables::{
    compatibility_rules::compatibility_rules, trackables::trackables,
};
diesel::allow_tables_to_appear_in_same_query!(compatibility_rules, trackables);
use crate::codegen::diesel_codegen::tables::users::users;
diesel::allow_tables_to_appear_in_same_query!(compatibility_rules, users);
