//! Submodule defining font-awesom icons for tables.

use core_structures::{AssetModel, ProcedureTemplate};

pub(crate) fn asset_model_icon(asset_model: &AssetModel) -> Option<&'static str> {
    Some(match asset_model.most_concrete_table.as_str() {
        "digital_asset_models" => "fa:fa-file",
        "organism_models" => "fa:fa-bacterium",
        "phone_models" => "fa:fa-mobile-screen-button",
        "sample_models" => "fa:fa-vial",
        "container_models" => "fa:fa-box",
        "volumetric_container_models" => "fa:fa-flask-vial",
        "packaging_models" => "fa:fa-sheet-plastic",
        "freezer_models" => "fa:fa-snowflake",
        "freeze_dryer_models" => "fa:fa-icicles",
        "weighing_device_models" => "fa:fa-scale-unbalanced",
        "pipette_models" => "fa:fa-eye-dropper",
        _ => return None,
    })
}

pub(crate) fn procedure_template_icon(
    procedure_template: &ProcedureTemplate,
) -> Option<&'static str> {
    Some(match procedure_template.most_concrete_table.as_str() {
        "disposal_procedure_templates" => "fa:fa-trash",
        "photograph_procedure_templates" => "fa:fa-camera",
        "packaging_procedure_templates" => "fa:fa-gifts",
        "freezing_procedure_templates" => "fa:fa-snowflake",
        "freeze_drying_procedure_templates" => "fa:fa-icicles",
        "weighing_procedure_templates" => "fa:fa-scale-unbalanced",
        "supernatant_procedure_templates" => "fa:fa-eye-dropper",
        "geolocation_procedure_templates" => "fa:fa-map-location-dot",
        "storage_procedure_templates" => "fa:fa-warehouse",
        _ => return None,
    })
}
