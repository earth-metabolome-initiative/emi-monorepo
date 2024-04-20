use serde::Deserialize;
use serde::Serialize;
use uuid::Uuid;
use chrono::NaiveDateTime;
#[derive(Deserialize, Serialize, Clone, Debug, PartialEq, Eq, )]
#[cfg_attr(feature = "frontend", derive(yew::html::Properties))]
pub struct EditsView {
    pub id: Uuid,
    pub editable_id: Uuid,
    pub edited_at: NaiveDateTime,
    pub editor_id: Uuid,
    pub editor_first_name: String,
    pub editor_middle_name: String,
    pub editor_last_name: String,
    pub edit_title: String,
    pub edit_extended_reason: String,
}
#[derive(Deserialize, Serialize, Clone, Debug, PartialEq, Eq, )]
#[cfg_attr(feature = "frontend", derive(yew::html::Properties))]
pub struct LastEditsView {
    pub id: Uuid,
    pub editable_id: Uuid,
    pub edited_at: NaiveDateTime,
    pub editor_id: Uuid,
    pub editor_first_name: String,
    pub editor_middle_name: String,
    pub editor_last_name: String,
    pub edit_title: String,
    pub edit_extended_reason: String,
}
#[derive(Deserialize, Serialize, Clone, Debug, PartialEq, Eq, )]
#[cfg_attr(feature = "frontend", derive(yew::html::Properties))]
pub struct FormatsView {
    pub id: Uuid,
    pub mime_type: String,
    pub extension: String,
    pub format_description: String,
}
#[derive(Deserialize, Serialize, Clone, Debug, PartialEq, Eq, )]
#[cfg_attr(feature = "frontend", derive(yew::html::Properties))]
pub struct DocumentsView {
    pub id: Uuid,
    pub format_id: Uuid,
    pub document_path: String,
    pub bytes: i32,
    pub document_name: String,
    pub description: String,
    pub creator_id: Uuid,
    pub creator_first_name: String,
    pub creator_middle_name: String,
    pub creator_last_name: String,
    pub last_edit_id: Uuid,
    pub last_edit_at: NaiveDateTime,
    pub last_editor_id: Uuid,
    pub last_editor_first_name: String,
    pub last_editor_middle_name: String,
    pub last_editor_last_name: String,
    pub last_edit_title: String,
    pub last_edit_extended_reason: String,
    pub extension: String,
    pub mime_type: String,
}
#[derive(Deserialize, Serialize, Clone, Debug, PartialEq, Eq, )]
#[cfg_attr(feature = "frontend", derive(yew::html::Properties))]
pub struct PublicUser {
    pub id: Uuid,
    pub first_name: String,
    pub middle_name: String,
    pub last_name: String,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Deserialize, Serialize, Clone, Debug, PartialEq, Copy, Eq, )]
pub enum View {
    EditsView,
    LastEditsView,
    FormatsView,
    DocumentsView,
    PublicUser,
}

impl View {
    pub fn name(&self) -> &'static str {
        match self {
            View::EditsView => "edits_view",
            View::LastEditsView => "last_edits_view",
            View::FormatsView => "formats_view",
            View::DocumentsView => "documents_view",
            View::PublicUser => "public_user",
        }
    }
}
impl std::fmt::Display for View {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.name())
    }
}

#[derive(Deserialize, Serialize, Clone, Debug, PartialEq, )]
pub enum ViewRow {
    EditsView(EditsView),
    LastEditsView(LastEditsView),
    FormatsView(FormatsView),
    DocumentsView(DocumentsView),
    PublicUser(PublicUser),
}

impl From<EditsView> for ViewRow {
    fn from(item: EditsView) -> Self {
        ViewRow::EditsView(item)
    }
}
impl std::convert::TryFrom<ViewRow> for EditsView {
    type Error = &'static str;
    fn try_from(item: ViewRow) -> Result<Self, Self::Error> {
        match item {
            ViewRow::EditsView(item) => Ok(item),
            _ => Err("Invalid conversion"),
        }
    }
}
impl From<LastEditsView> for ViewRow {
    fn from(item: LastEditsView) -> Self {
        ViewRow::LastEditsView(item)
    }
}
impl std::convert::TryFrom<ViewRow> for LastEditsView {
    type Error = &'static str;
    fn try_from(item: ViewRow) -> Result<Self, Self::Error> {
        match item {
            ViewRow::LastEditsView(item) => Ok(item),
            _ => Err("Invalid conversion"),
        }
    }
}
impl From<FormatsView> for ViewRow {
    fn from(item: FormatsView) -> Self {
        ViewRow::FormatsView(item)
    }
}
impl std::convert::TryFrom<ViewRow> for FormatsView {
    type Error = &'static str;
    fn try_from(item: ViewRow) -> Result<Self, Self::Error> {
        match item {
            ViewRow::FormatsView(item) => Ok(item),
            _ => Err("Invalid conversion"),
        }
    }
}
impl From<DocumentsView> for ViewRow {
    fn from(item: DocumentsView) -> Self {
        ViewRow::DocumentsView(item)
    }
}
impl std::convert::TryFrom<ViewRow> for DocumentsView {
    type Error = &'static str;
    fn try_from(item: ViewRow) -> Result<Self, Self::Error> {
        match item {
            ViewRow::DocumentsView(item) => Ok(item),
            _ => Err("Invalid conversion"),
        }
    }
}
impl From<PublicUser> for ViewRow {
    fn from(item: PublicUser) -> Self {
        ViewRow::PublicUser(item)
    }
}
impl std::convert::TryFrom<ViewRow> for PublicUser {
    type Error = &'static str;
    fn try_from(item: ViewRow) -> Result<Self, Self::Error> {
        match item {
            ViewRow::PublicUser(item) => Ok(item),
            _ => Err("Invalid conversion"),
        }
    }
}
impl From<&str> for View {
    fn from(item: &str) -> Self {
        match item {
            "edits_view" => View::EditsView,
            "last_edits_view" => View::LastEditsView,
            "formats_view" => View::FormatsView,
            "documents_view" => View::DocumentsView,
            "public_user" => View::PublicUser,
            _ => panic!("Unknown table name"),
        }
    }
}
impl ViewRow {
    pub fn view(&self) -> &'static View {
        match self {
            ViewRow::EditsView(_) => &View::EditsView,
            ViewRow::LastEditsView(_) => &View::LastEditsView,
            ViewRow::FormatsView(_) => &View::FormatsView,
            ViewRow::DocumentsView(_) => &View::DocumentsView,
            ViewRow::PublicUser(_) => &View::PublicUser,
        }
    }

    pub fn view_name(&self) -> &'static str {
        self.view().name()
    }
}
