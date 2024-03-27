use super::views;
use super::schema::documents_view;

impl DocumentView {
    pub fn get(conn: &mut DieselConn, id: Uuid) -> Result<DocumentView, diesel::result::Error> {
        documents_view::table
            .filter(documents_view::id.eq(id))
            .first(conn)
    }

    pub fn internal_path(&self) -> String {
        format!(
            "{}/{}.{}",
            std::env::var("DOCUMENTS_DIRECTORY").unwrap(),
            self.id,
            self.extension
        )
    }
}