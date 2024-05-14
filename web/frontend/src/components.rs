pub mod app;
pub mod database;
pub mod error_page;
pub mod footer;
pub mod forms;
pub mod hamburger;
pub mod input_with_icon;
pub mod login_provider;
mod logout;
pub mod navigator;
mod search_bar;
pub mod sidebar;

pub mod basic_page;
pub use basic_page::{BasicPage, PageLike};

pub mod basic_pages;
pub use basic_pages::BasicPages;

pub use app::App;
pub use error_page::ErrorPage;
pub use footer::Footer;
pub use input_with_icon::InputWithIcon;
pub use navigator::*;
