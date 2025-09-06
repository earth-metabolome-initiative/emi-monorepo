diesel::table! {
    supernatant_procedures(procedure) { procedure -> ::rosetta_uuid::diesel_impls::Uuid,
    procedure_template -> diesel::sql_types::Integer, stratified_source ->
    ::rosetta_uuid::diesel_impls::Uuid, procedure_template_stratified_source_model ->
    diesel::sql_types::Integer, procedure_stratified_source ->
    ::rosetta_uuid::diesel_impls::Uuid, supernatant_destination ->
    ::rosetta_uuid::diesel_impls::Uuid, procedure_template_supernatant_destination_model
    -> diesel::sql_types::Integer, procedure_supernatant_destination ->
    ::rosetta_uuid::diesel_impls::Uuid, transferred_with ->
    ::rosetta_uuid::diesel_impls::Uuid, transferred_with_model ->
    diesel::sql_types::Integer, procedure_template_transferred_with_model ->
    diesel::sql_types::Integer, procedure_transferred_with ->
    ::rosetta_uuid::diesel_impls::Uuid, pipette_tip_model -> diesel::sql_types::Integer,
    procedure_template_pipette_tip_model -> diesel::sql_types::Integer,
    procedure_pipette_tip -> ::rosetta_uuid::diesel_impls::Uuid }
}
