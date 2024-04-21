use serde::Deserialize;
use serde::Serialize;
use uuid::Uuid;
use chrono::NaiveDateTime;
#[derive(Deserialize, Serialize, Clone, Debug, PartialEq, Eq, )]
#[cfg_attr(feature = "frontend", derive(yew::html::Properties))]
pub struct PublicUser {
    pub id: i32,
    pub first_name: String,
    pub middle_name: String,
    pub last_name: String,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Deserialize, Serialize, Clone, Debug, PartialEq, Copy, Eq, )]
pub enum View {
    PublicUser,
}

impl View {
    pub fn name(&self) -> &'static str {
        match self {
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
    PublicUser(PublicUser),
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
            "public_user" => View::PublicUser,
            _ => panic!("Unknown table name"),
        }
    }
}
impl ViewRow {
    pub fn view(&self) -> &'static View {
        match self {
            ViewRow::PublicUser(_) => &View::PublicUser,
        }
    }

    pub fn view_name(&self) -> &'static str {
        self.view().name()
    }
}
