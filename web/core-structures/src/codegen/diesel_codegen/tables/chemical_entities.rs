diesel::table! {
    chemical_entities(id) { id -> diesel::sql_types::Integer, name ->
    diesel::sql_types::Text, description -> diesel::sql_types::Text, purity ->
    diesel::sql_types::Float, gram_per_mole -> diesel::sql_types::Float, created_by ->
    diesel::sql_types::Integer, created_at ->
    rosetta_timestamp::diesel_impls::TimestampUTC, updated_by ->
    diesel::sql_types::Integer, updated_at ->
    rosetta_timestamp::diesel_impls::TimestampUTC }
}
