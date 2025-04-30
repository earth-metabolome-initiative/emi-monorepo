use crate::codegen::diesel_codegen::tables::{
    colors::colors, nameplate_categories::nameplate_categories,
};
diesel::allow_tables_to_appear_in_same_query!(nameplate_categories, colors);
use crate::codegen::diesel_codegen::tables::permanence_categories::permanence_categories;
diesel::allow_tables_to_appear_in_same_query!(nameplate_categories, permanence_categories);
