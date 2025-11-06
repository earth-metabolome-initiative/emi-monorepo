#![doc = include_str!("../README.md")]

#[cfg(feature = "pgrx")]
::pgrx::pg_module_magic!();

mod font_awesome_icons;
mod str;
pub use font_awesome_icons::must_be_font_awesome_class;
pub use str::{
    must_be_email, must_be_paragraph, must_not_be_padded, must_not_contain_consecutive_whitespace,
    must_not_contain_control_characters,
};
