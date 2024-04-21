use serde::Deserialize;
use serde::Serialize;
use uuid::Uuid;
use chrono::NaiveDateTime;
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
    FormatsView,
    PublicUser,
}

impl View {
    pub fn name(&self) -> &'static str {
        match self {
            View::FormatsView => "formats_view",
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
    FormatsView(FormatsView),
    PublicUser(PublicUser),
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
            "formats_view" => View::FormatsView,
            "public_user" => View::PublicUser,
            _ => panic!("Unknown table name"),
        }
    }
}
impl ViewRow {
    pub fn view(&self) -> &'static View {
        match self {
            ViewRow::FormatsView(_) => &View::FormatsView,
            ViewRow::PublicUser(_) => &View::PublicUser,
        }
    }

    pub fn view_name(&self) -> &'static str {
        self.view().name()
    }
}
