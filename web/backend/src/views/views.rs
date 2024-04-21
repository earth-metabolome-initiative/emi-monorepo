use serde::Deserialize;
use serde::Serialize;
use diesel::Queryable;
use diesel::QueryableByName;
use uuid::Uuid;
use chrono::NaiveDateTime;
use diesel::r2d2::PooledConnection;
use diesel::r2d2::ConnectionManager;
use diesel::prelude::*;
use crate::views::schema::*;
#[derive(Deserialize, Serialize, Clone, Debug, PartialEq, Queryable, QueryableByName, )]
#[diesel(table_name = formats_view)]
pub struct FormatsView {
    pub id: Uuid,
    pub mime_type: String,
    pub extension: String,
    pub format_description: String,
}

impl From<FormatsView> for web_common::database::views::FormatsView {
    fn from(item: FormatsView) -> Self {
        Self {
            id: item.id,
            mime_type: item.mime_type,
            extension: item.extension,
            format_description: item.format_description,
        }
    }
}

impl From<web_common::database::views::FormatsView> for FormatsView {
    fn from(item: web_common::database::views::FormatsView) -> Self {
        Self {
            id: item.id,
            mime_type: item.mime_type,
            extension: item.extension,
            format_description: item.format_description,
        }
    }
}

impl FormatsView {
    /// Get the struct from the database by its ID.
    ///
    /// # Arguments
    /// * `id` - The ID of the struct to get.
    /// * `connection` - The connection to the database.
    ///
    pub fn get(
        id: Uuid,
        connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>
    ) -> Result<Self, diesel::result::Error> {
        formats_view::dsl::formats_view
            .filter(formats_view::dsl::id.eq(id))
            .first::<Self>(connection)
    }
}

#[derive(Deserialize, Serialize, Clone, Debug, PartialEq, Queryable, QueryableByName, )]
#[diesel(table_name = public_user)]
pub struct PublicUser {
    pub id: Uuid,
    pub first_name: String,
    pub middle_name: String,
    pub last_name: String,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

impl From<PublicUser> for web_common::database::views::PublicUser {
    fn from(item: PublicUser) -> Self {
        Self {
            id: item.id,
            first_name: item.first_name,
            middle_name: item.middle_name,
            last_name: item.last_name,
            created_at: item.created_at,
            updated_at: item.updated_at,
        }
    }
}

impl From<web_common::database::views::PublicUser> for PublicUser {
    fn from(item: web_common::database::views::PublicUser) -> Self {
        Self {
            id: item.id,
            first_name: item.first_name,
            middle_name: item.middle_name,
            last_name: item.last_name,
            created_at: item.created_at,
            updated_at: item.updated_at,
        }
    }
}

impl PublicUser {
    /// Get the struct from the database by its ID.
    ///
    /// # Arguments
    /// * `id` - The ID of the struct to get.
    /// * `connection` - The connection to the database.
    ///
    pub fn get(
        id: Uuid,
        connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>
    ) -> Result<Self, diesel::result::Error> {
        public_user::dsl::public_user
            .filter(public_user::dsl::id.eq(id))
            .first::<Self>(connection)
    }
}


#[derive(Deserialize, Serialize, Clone, Debug, PartialEq)]
pub enum ViewRow {
    FormatsView(FormatsView),
    PublicUser(PublicUser),
}

impl From<web_common::database::views::ViewRow> for ViewRow {
    fn from(item: web_common::database::views::ViewRow) -> Self {
        match item {
            web_common::database::views::ViewRow::FormatsView(item) => ViewRow::FormatsView(item.into()),
            web_common::database::views::ViewRow::PublicUser(item) => ViewRow::PublicUser(item.into()),
        }
    }
}
impl From<ViewRow> for web_common::database::views::ViewRow {
    fn from(item: ViewRow) -> Self {
        match item {
            ViewRow::FormatsView(item) => web_common::database::views::ViewRow::FormatsView(item.into()),
            ViewRow::PublicUser(item) => web_common::database::views::ViewRow::PublicUser(item.into()),
        }
    }
}
impl From<FormatsView> for ViewRow {
    fn from(item: FormatsView) -> Self {
        ViewRow::FormatsView(item)
    }
}
impl From<PublicUser> for ViewRow {
    fn from(item: PublicUser) -> Self {
        ViewRow::PublicUser(item)
    }
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
    /// Get the struct from the database by its ID.
    ///
    /// # Arguments
    /// * `id` - The ID of the struct to get.
    /// * `connection` - The connection to the database.
    ///
    pub fn get(
        &self,
        id: Uuid,
        connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>
    ) -> Result<ViewRow, diesel::result::Error> {
        Ok(match self {
            View::FormatsView => ViewRow::FormatsView(FormatsView::get(id, connection)?),
            View::PublicUser => ViewRow::PublicUser(PublicUser::get(id, connection)?),
        })
    }
}
impl std::fmt::Display for View {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.name())
    }
}
impl From<&str> for View {
    fn from(item: &str) -> Self {
        match item {
            "formats_view" => View::FormatsView,
            "public_user" => View::PublicUser,
            _ => panic!("Unknown views name"),
        }
    }
}
impl From<web_common::database::views::View> for View {
    fn from(item: web_common::database::views::View) -> Self {
        match item {
            web_common::database::views::View::FormatsView => View::FormatsView,
            web_common::database::views::View::PublicUser => View::PublicUser,
        }
    }
}
impl From<View> for web_common::database::views::View {
    fn from(item: View) -> Self {
        match item {
            View::FormatsView => web_common::database::views::View::FormatsView,
            View::PublicUser => web_common::database::views::View::PublicUser,
        }
    }
}
