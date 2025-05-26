use crate::codegen::diesel_codegen::tables::{
    trackable_categories::trackable_categories, users::users,
};
diesel::allow_tables_to_appear_in_same_query!(trackable_categories, users);
