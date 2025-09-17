use crate::codegen::diesel_codegen::tables::{digital_assets::digital_assets, spectra::spectra};
diesel::allow_tables_to_appear_in_same_query!(spectra, digital_assets);
use crate::codegen::diesel_codegen::tables::spectra_collections::spectra_collections;
diesel::allow_tables_to_appear_in_same_query!(spectra, spectra_collections);
use crate::codegen::diesel_codegen::tables::assets::assets;
diesel::allow_tables_to_appear_in_same_query!(spectra, assets);
