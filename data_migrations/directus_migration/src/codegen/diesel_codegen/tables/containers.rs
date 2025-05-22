diesel::table! {
    #[sql_name = "Containers"] public.containers(id) { id -> diesel::sql_types::Integer,
    status -> diesel::sql_types::Nullable < diesel::sql_types::Text >, user_created ->
    diesel::sql_types::Nullable < rosetta_uuid::diesel_impls::Uuid >, date_created ->
    diesel::sql_types::Nullable < rosetta_timestamp::diesel_impls::TimestampUTC >,
    user_updated -> diesel::sql_types::Nullable < rosetta_uuid::diesel_impls::Uuid >,
    date_updated -> diesel::sql_types::Nullable <
    rosetta_timestamp::diesel_impls::TimestampUTC >, used -> diesel::sql_types::Bool,
    reserved -> diesel::sql_types::Bool, uuid_container -> diesel::sql_types::Nullable <
    rosetta_uuid::diesel_impls::Uuid >, container_id -> diesel::sql_types::Text,
    container_model -> diesel::sql_types::Nullable < diesel::sql_types::Integer >,
    is_finite -> diesel::sql_types::Nullable < diesel::sql_types::Bool >, #[sql_name =
    "columns"] __columns -> diesel::sql_types::Nullable < diesel::sql_types::Integer >,
    rows -> diesel::sql_types::Nullable < diesel::sql_types::Integer >, rows_numeric ->
    diesel::sql_types::Nullable < diesel::sql_types::Bool >, columns_numeric ->
    diesel::sql_types::Nullable < diesel::sql_types::Bool >, location ->
    diesel::sql_types::Nullable < diesel::sql_types::Integer >, old_id ->
    diesel::sql_types::Nullable < diesel::sql_types::Text >, parent_container ->
    diesel::sql_types::Nullable < diesel::sql_types::Integer > }
}
