//! Submodule defining font-awesom icons for tables.

use core_structures::AssetModel;

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
        "bead_models" => "fa:fa-drum",
        "pipette_models" => "fa:fa-eye-dropper",
        _ => return None,
    })
}
