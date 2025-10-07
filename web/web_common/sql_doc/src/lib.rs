#![doc = include_str!("../README.md")]

pub mod column_documentation;
pub use column_documentation::ColumnDocumentation;
pub mod table_documentation;
pub use table_documentation::TableDocumentation;
pub mod sql_doc_error;
pub use sql_doc_error::SqlDocError;
