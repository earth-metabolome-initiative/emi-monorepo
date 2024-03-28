use super::views::*;

impl DocumentsView {
    pub fn internal_path(&self) -> String {
        format!(
            "{}/{}.{}",
            std::env::var("DOCUMENTS_DIRECTORY").unwrap(),
            self.id,
            self.extension
        )
    }
}