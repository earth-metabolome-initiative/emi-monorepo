diesel::table! {
    ppe_reminder_procedures(procedure) { procedure -> ::rosetta_uuid::diesel_impls::Uuid,
    procedure_template -> diesel::sql_types::Integer, procedure_template_ppe_asset_model
    -> diesel::sql_types::Integer, procedure_ppe_asset ->
    ::rosetta_uuid::diesel_impls::Uuid }
}
