"""Constants for mapping Rust types to GlueSQL types."""

GLUESQL_TYPES_MAPPING = {
    "i8": "gluesql::core::ast_builder::num({})",
    "i16": "gluesql::core::ast_builder::num({})",
    "i32": "gluesql::core::ast_builder::num({})",
    "i64": "gluesql::core::ast_builder::num({})",
    "i128": "gluesql::core::ast_builder::num({})",
    "u8": "gluesql::core::ast_builder::num({})",
    "u16": "gluesql::core::ast_builder::num({})",
    "u32": "gluesql::core::ast_builder::num({})",
    "u64": "gluesql::core::ast_builder::num({})",
    "u128": "gluesql::core::ast_builder::num({})",
    "f32": "gluesql::core::ast_builder::num({})",
    "f64": "gluesql::core::ast_builder::num({})",
    "String": "gluesql::core::ast_builder::text({})",
    "uuid::Uuid": "gluesql::core::ast_builder::uuid({}.to_string())",
    "bool": "({}.into())",
    "chrono::NaiveDateTime": "gluesql::core::ast_builder::timestamp({}.to_string())",
    "DateTime<Utc>": "gluesql::core::ast_builder::timestamp({}.to_string())",
    "Vec<u8>": "gluesql::core::ast_builder::bytea({})",
    "JPEG": "gluesql::core::ast_builder::bytea({})",
}
