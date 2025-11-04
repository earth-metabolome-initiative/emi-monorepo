//! Traits for SynQL table models.

pub mod table_model_like;
pub use table_model_like::{MODEL_MODULE_NAME, TableModelLike};
pub mod column_model_like;
pub use column_model_like::ColumnModelLike;
