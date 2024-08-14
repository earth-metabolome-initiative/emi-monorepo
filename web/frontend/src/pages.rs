pub mod automatic_pages;
pub mod collect;
pub mod home;
pub mod login;
pub mod not_found;
pub mod project_selection;
pub use automatic_pages::*;
// pub mod server_error;

pub use collect::Collect;
pub use home::Home;
pub use login::Login;
pub use not_found::NotFound;
pub use project_selection::ProjectSelection;
// pub use server_error::ServerErrorPage;
