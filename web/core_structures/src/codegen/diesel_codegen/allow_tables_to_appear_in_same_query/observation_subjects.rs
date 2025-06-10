use crate::codegen::diesel_codegen::tables::colors::colors;
use crate::codegen::diesel_codegen::tables::observation_subjects::observation_subjects;
diesel::allow_tables_to_appear_in_same_query!(observation_subjects, colors);
