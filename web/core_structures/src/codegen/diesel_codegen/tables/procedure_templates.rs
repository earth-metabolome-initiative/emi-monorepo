diesel::table! {
    procedure_templates(procedure_template_id) { procedure_template ->
    diesel::sql_types::Integer, most_concrete_table -> diesel::sql_types::Text, name ->
    diesel::sql_types::Text, description -> diesel::sql_types::Text, created_by ->
    diesel::sql_types::Integer, created_at ->
    rosetta_timestamp::diesel_impls::TimestampUTC, updated_by ->
    diesel::sql_types::Integer, updated_at ->
    rosetta_timestamp::diesel_impls::TimestampUTC, deprecated -> diesel::sql_types::Bool
    }
}
