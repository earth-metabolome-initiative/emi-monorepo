use crate::codegen::diesel_codegen::tables::instrument_categories::instrument_categories;
use crate::codegen::diesel_codegen::tables::icons::icons;
diesel::allow_tables_to_appear_in_same_query!(instrument_categories, icons);
