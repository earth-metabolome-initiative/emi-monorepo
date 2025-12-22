diesel::table! {
    storage_procedures(procedure) { procedure -> ::rosetta_uuid::diesel_impls::Uuid,
    procedure_template -> diesel::sql_types::Integer, stored_asset ->
    ::rosetta_uuid::diesel_impls::Uuid, stored_asset_model_id -> diesel::sql_types::Integer,
    procedure_template_stored_asset_model_id -> diesel::sql_types::Integer,
    procedure_stored_asset -> ::rosetta_uuid::diesel_impls::Uuid, stored_into ->
    ::rosetta_uuid::diesel_impls::Uuid, stored_into_model -> diesel::sql_types::Integer,
    procedure_template_stored_into_model -> diesel::sql_types::Integer,
    procedure_stored_into -> ::rosetta_uuid::diesel_impls::Uuid }
}
