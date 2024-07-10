pub mod app;
pub mod badge;
pub mod basic_page;
pub mod error_page;
pub mod footer;
pub mod forms;
pub mod hamburger;
pub mod login_provider;
mod logout;
pub mod navigator;
mod search_bar;
pub mod sidebar;

pub mod basic_list;
pub use basic_list::BasicList;
pub(crate) use basic_page::{BasicPage, PageLike};

pub use app::App;
pub use badge::{Badge, RowToBadge};
pub use error_page::ErrorPage;
pub use footer::Footer;
pub use navigator::*;

pub mod link_button;
pub use link_button::LinkButton;
pub mod action_button;
pub use action_button::ActionButton;
