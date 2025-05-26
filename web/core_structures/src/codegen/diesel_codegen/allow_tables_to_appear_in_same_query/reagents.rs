use crate::codegen::diesel_codegen::tables::{
    reagents::reagents, trackable_categories::trackable_categories,
};
diesel::allow_tables_to_appear_in_same_query!(reagents, trackable_categories);
use crate::codegen::diesel_codegen::tables::users::users;
diesel::allow_tables_to_appear_in_same_query!(reagents, users);
