diesel::table! {
    #[sql_name = "SI_Units"] public.si_units(id) { id -> diesel::sql_types::Integer,
    status -> diesel::sql_types::Nullable < diesel::sql_types::Text >, user_created ->
    diesel::sql_types::Nullable < diesel::sql_types::Uuid >, date_created ->
    diesel::sql_types::Nullable < diesel::sql_types::Timestamptz >, user_updated ->
    diesel::sql_types::Nullable < diesel::sql_types::Uuid >, date_updated ->
    diesel::sql_types::Nullable < diesel::sql_types::Timestamptz >, unit_name ->
    diesel::sql_types::Text, symbol -> diesel::sql_types::Text, base_unit ->
    diesel::sql_types::Text, multiplication_factor -> diesel::sql_types::Float }
}
