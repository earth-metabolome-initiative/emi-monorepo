use crate::codegen::diesel_codegen::tables::tool_categories::tool_categories;
use crate::codegen::diesel_codegen::tables::icons::icons;
diesel::allow_tables_to_appear_in_same_query!(tool_categories, icons);
