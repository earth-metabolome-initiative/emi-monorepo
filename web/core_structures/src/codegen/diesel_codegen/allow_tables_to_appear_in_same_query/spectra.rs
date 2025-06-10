use crate::codegen::diesel_codegen::tables::spectra::spectra;
use crate::codegen::diesel_codegen::tables::spectra_collections::spectra_collections;
diesel::allow_tables_to_appear_in_same_query!(spectra, spectra_collections);
