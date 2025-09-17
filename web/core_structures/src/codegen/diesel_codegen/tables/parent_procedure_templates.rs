diesel::table! {
    parent_procedure_templates(parent, child) { parent -> diesel::sql_types::Integer,
    child -> diesel::sql_types::Integer, created_by -> diesel::sql_types::Integer,
    created_at -> rosetta_timestamp::diesel_impls::TimestampUTC }
}
