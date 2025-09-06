use crate::codegen::diesel_codegen::tables::{
    asset_compatibility_rules::asset_compatibility_rules,
    centrifuge_procedures::centrifuge_procedures,
};
diesel::allow_tables_to_appear_in_same_query!(centrifuge_procedures, asset_compatibility_rules);
use crate::codegen::diesel_codegen::tables::assets::assets;
diesel::allow_tables_to_appear_in_same_query!(centrifuge_procedures, assets);
use crate::codegen::diesel_codegen::tables::centrifuge_models::centrifuge_models;
diesel::allow_tables_to_appear_in_same_query!(centrifuge_procedures, centrifuge_models);
use crate::codegen::diesel_codegen::tables::centrifuge_procedure_templates::centrifuge_procedure_templates;
diesel::allow_tables_to_appear_in_same_query!(
    centrifuge_procedures,
    centrifuge_procedure_templates
);
use crate::codegen::diesel_codegen::tables::centrifuges::centrifuges;
diesel::allow_tables_to_appear_in_same_query!(centrifuge_procedures, centrifuges);
use crate::codegen::diesel_codegen::tables::procedure_assets::procedure_assets;
diesel::allow_tables_to_appear_in_same_query!(centrifuge_procedures, procedure_assets);
use crate::codegen::diesel_codegen::tables::procedure_template_asset_models::procedure_template_asset_models;
diesel::allow_tables_to_appear_in_same_query!(
    centrifuge_procedures,
    procedure_template_asset_models
);
use crate::codegen::diesel_codegen::tables::procedures::procedures;
diesel::allow_tables_to_appear_in_same_query!(centrifuge_procedures, procedures);
use crate::codegen::diesel_codegen::tables::volumetric_container_models::volumetric_container_models;
diesel::allow_tables_to_appear_in_same_query!(centrifuge_procedures, volumetric_container_models);
use crate::codegen::diesel_codegen::tables::volumetric_containers::volumetric_containers;
diesel::allow_tables_to_appear_in_same_query!(centrifuge_procedures, volumetric_containers);
