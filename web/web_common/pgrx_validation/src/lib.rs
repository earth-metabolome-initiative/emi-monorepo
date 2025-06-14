#![doc = include_str!("../README.md")]

#[cfg(feature = "pgrx")]
::pgrx::pg_module_magic!();

mod chrono;
mod float;
mod font_awesome_icons;
mod int;
mod str;
mod uuid;
pub use chrono::{must_be_smaller_than_utc, must_be_strictly_smaller_than_utc};
pub use float::{
    must_be_greater_than_f32, must_be_smaller_than_f32, must_be_strictly_greater_than_f32,
    must_be_strictly_greater_than_f64, must_be_strictly_positive_f32,
    must_be_strictly_positive_f64, must_be_strictly_smaller_than_f32,
    must_be_strictly_smaller_than_f64,
};
pub use font_awesome_icons::must_be_font_awesome_class;
pub use int::{
    must_be_distinct_i16, must_be_distinct_i32, must_be_strictly_positive_i16,
    must_be_strictly_positive_i32,
};
pub use str::{
    must_be_distinct, must_be_email, must_be_paragraph, must_not_be_empty, must_not_be_padded,
    must_not_contain_consecutive_whitespace, must_not_contain_control_characters,
};
pub use uuid::must_be_distinct_uuid;
