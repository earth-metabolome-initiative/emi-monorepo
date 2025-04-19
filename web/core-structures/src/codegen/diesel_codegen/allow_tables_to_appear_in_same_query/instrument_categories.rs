use crate::codegen::diesel_codegen::tables::{
    icons::icons, instrument_categories::instrument_categories,
};
diesel::allow_tables_to_appear_in_same_query!(instrument_categories, icons);
