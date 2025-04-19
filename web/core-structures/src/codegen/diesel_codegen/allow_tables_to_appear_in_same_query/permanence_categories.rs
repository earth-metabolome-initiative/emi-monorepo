use crate::codegen::diesel_codegen::tables::{
    colors::colors, permanence_categories::permanence_categories,
};
diesel::allow_tables_to_appear_in_same_query!(permanence_categories, colors);
use crate::codegen::diesel_codegen::tables::icons::icons;
diesel::allow_tables_to_appear_in_same_query!(permanence_categories, icons);
