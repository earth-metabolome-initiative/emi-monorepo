use crate::codegen::diesel_codegen::tables::container_categories::container_categories;
use crate::codegen::diesel_codegen::tables::icons::icons;
diesel::allow_tables_to_appear_in_same_query!(container_categories, icons);
