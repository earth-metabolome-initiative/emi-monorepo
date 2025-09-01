use crate::codegen::diesel_codegen::tables::{assets::assets, centrifuges::centrifuges};
diesel::allow_tables_to_appear_in_same_query!(centrifuges, assets);
use crate::codegen::diesel_codegen::tables::commercial_centrifuge_lots::commercial_centrifuge_lots;
diesel::allow_tables_to_appear_in_same_query!(centrifuges, commercial_centrifuge_lots);
use crate::codegen::diesel_codegen::tables::physical_assets::physical_assets;
diesel::allow_tables_to_appear_in_same_query!(centrifuges, physical_assets);
