use crate::codegen::diesel_codegen::tables::spectra_collections::spectra_collections;
use crate::codegen::diesel_codegen::tables::trackables::trackables;
diesel::allow_tables_to_appear_in_same_query!(spectra_collections, trackables);
use crate::codegen::diesel_codegen::tables::users::users;
diesel::allow_tables_to_appear_in_same_query!(spectra_collections, users);
