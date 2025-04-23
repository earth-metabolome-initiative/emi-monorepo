use crate::codegen::diesel_codegen::tables::{
    icons::icons, permanence_categories::permanence_categories,
};
diesel::allow_tables_to_appear_in_same_query!(permanence_categories, icons);
use crate::codegen::diesel_codegen::tables::colors::colors;
diesel::allow_tables_to_appear_in_same_query!(permanence_categories, colors);
