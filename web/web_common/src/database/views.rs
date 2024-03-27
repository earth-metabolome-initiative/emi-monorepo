use chrono::DateTime;
use chrono::NaiveDateTime;
use serde::Deserialize;
use serde::Serialize;
use uuid::Uuid;
#[derive(Deserialize, Serialize, Clone, Debug, PartialEq)]
pub struct DocumentView {
    pub id: Uuid,
    pub format_id: Uuid,
    pub document_path: String,
    pub bytes: i32,
    pub document_name: String,
    pub description: Option<String>,
    pub creator_id: Uuid,
    pub creator_first_name: Option<String>,
    pub creator_middle_name: Option<String>,
    pub creator_last_name: Option<String>,
    pub last_edit_id: Option<Uuid>,
    pub last_edit_at: Option<String>,
    pub last_editor_id: Option<Uuid>,
    pub last_editor_first_name: Option<String>,
    pub last_editor_middle_name: Option<String>,
    pub last_editor_last_name: Option<String>,
    pub last_edit_title: Option<String>,
    pub last_edit_extended_reason: Option<String>,
    pub extension: String,
    pub mime_type: String,
}
#[derive(Deserialize, Serialize, Clone, Debug, PartialEq)]
pub struct PublicUser {
    pub id: Uuid,
    pub first_name: Option<String>,
    pub middle_name: Option<String>,
    pub last_name: Option<String>,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Deserialize, Serialize, Clone, Debug, PartialEq, Copy, Eq)]
pub enum View {
    DocumentView,
    PublicUser,
}

impl View {
    pub fn name(&self) -> &'static str {
        match self {
            View::DocumentView => "documents_view",
            View::PublicUser => "users_view",
        }
    }
}

#[derive(Deserialize, Serialize, Clone, Debug, PartialEq)]
pub enum ViewRow {
    DocumentView(DocumentView),
    PublicUser(PublicUser),
}

impl From<&str> for View {
    fn from(item: &str) -> Self {
        match item {
            "documents_view" => View::DocumentView,
            "users_view" => View::PublicUser,
            _ => panic!("Unknown table name"),
        }
    }
}
impl ViewRow {
    pub fn view(&self) -> &'static View {
        match self {
            ViewRow::DocumentView(_) => &View::DocumentView,
            ViewRow::PublicUser(_) => &View::PublicUser,
        }
    }

    pub fn view_name(&self) -> &'static str {
        self.view().name()
    }
}
