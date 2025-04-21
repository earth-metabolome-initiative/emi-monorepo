use crate::codegen::diesel_codegen::tables::{
    nameplate_categories::nameplate_categories, organisms::organisms,
};
diesel::allow_tables_to_appear_in_same_query!(organisms, nameplate_categories);
use crate::codegen::diesel_codegen::tables::trackables::trackables;
diesel::allow_tables_to_appear_in_same_query!(organisms, trackables);
