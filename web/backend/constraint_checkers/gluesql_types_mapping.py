"""Constants for mapping Rust types to GlueSQL types."""

GLUESQL_TYPES_MAPPING = {
    "i8": "gluesql::core::ast_builder::num({value})",
    "i16": "gluesql::core::ast_builder::num({value})",
    "i32": "gluesql::core::ast_builder::num({value})",
    "i64": "gluesql::core::ast_builder::num({value})",
    "i128": "gluesql::core::ast_builder::num({value})",
    "u8": "gluesql::core::ast_builder::num({value})",
    "u16": "gluesql::core::ast_builder::num({value})",
    "u32": "gluesql::core::ast_builder::num({value})",
    "u64": "gluesql::core::ast_builder::num({value})",
    "u128": "gluesql::core::ast_builder::num({value})",
    "f32": "gluesql::core::ast_builder::num({value})",
    "f64": "gluesql::core::ast_builder::num({value})",
    "String": "gluesql::core::ast_builder::text({value})",
    "uuid::Uuid": "gluesql::core::ast_builder::uuid({value}.to_string())",
    "bool": "({value}.into())",
    "chrono::NaiveDateTime": "gluesql::core::ast_builder::timestamp({value}.to_string())",
    "DateTime<Utc>": "gluesql::core::ast_builder::timestamp({value}.to_string())",
    "Vec<u8>": "gluesql::core::ast_builder::bytea({value})",
    "JPEG": "gluesql::core::ast_builder::bytea({value})",
    "Point": "gluesql::core::ast_builder::function::point(gluesql::core::ast_builder::num({value}.x), gluesql::core::ast_builder::num({value}.y))",
}
