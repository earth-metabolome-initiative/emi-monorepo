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
#[diesel(table_name = edits_view)]
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

impl From<EditsView> for web_common::database::views::EditsView {
    fn from(item: EditsView) -> Self {
        Self {
            id: item.id,
            editable_id: item.editable_id,
            edited_at: item.edited_at,
            editor_id: item.editor_id,
            editor_first_name: item.editor_first_name,
            editor_middle_name: item.editor_middle_name,
            editor_last_name: item.editor_last_name,
            edit_title: item.edit_title,
            edit_extended_reason: item.edit_extended_reason,
        }
    }
}

impl From<web_common::database::views::EditsView> for EditsView {
    fn from(item: web_common::database::views::EditsView) -> Self {
        Self {
            id: item.id,
            editable_id: item.editable_id,
            edited_at: item.edited_at,
            editor_id: item.editor_id,
            editor_first_name: item.editor_first_name,
            editor_middle_name: item.editor_middle_name,
            editor_last_name: item.editor_last_name,
            edit_title: item.edit_title,
            edit_extended_reason: item.edit_extended_reason,
        }
    }
}

impl EditsView {
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
        edits_view::dsl::edits_view
            .filter(edits_view::dsl::id.eq(id))
            .first::<Self>(connection)
    }
}

#[derive(Deserialize, Serialize, Clone, Debug, PartialEq, Queryable, QueryableByName, )]
#[diesel(table_name = last_edits_view)]
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

impl From<LastEditsView> for web_common::database::views::LastEditsView {
    fn from(item: LastEditsView) -> Self {
        Self {
            id: item.id,
            editable_id: item.editable_id,
            edited_at: item.edited_at,
            editor_id: item.editor_id,
            editor_first_name: item.editor_first_name,
            editor_middle_name: item.editor_middle_name,
            editor_last_name: item.editor_last_name,
            edit_title: item.edit_title,
            edit_extended_reason: item.edit_extended_reason,
        }
    }
}

impl From<web_common::database::views::LastEditsView> for LastEditsView {
    fn from(item: web_common::database::views::LastEditsView) -> Self {
        Self {
            id: item.id,
            editable_id: item.editable_id,
            edited_at: item.edited_at,
            editor_id: item.editor_id,
            editor_first_name: item.editor_first_name,
            editor_middle_name: item.editor_middle_name,
            editor_last_name: item.editor_last_name,
            edit_title: item.edit_title,
            edit_extended_reason: item.edit_extended_reason,
        }
    }
}

impl LastEditsView {
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
        last_edits_view::dsl::last_edits_view
            .filter(last_edits_view::dsl::id.eq(id))
            .first::<Self>(connection)
    }
}

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
#[diesel(table_name = documents_view)]
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

impl From<DocumentsView> for web_common::database::views::DocumentsView {
    fn from(item: DocumentsView) -> Self {
        Self {
            id: item.id,
            format_id: item.format_id,
            document_path: item.document_path,
            bytes: item.bytes,
            document_name: item.document_name,
            description: item.description,
            creator_id: item.creator_id,
            creator_first_name: item.creator_first_name,
            creator_middle_name: item.creator_middle_name,
            creator_last_name: item.creator_last_name,
            last_edit_id: item.last_edit_id,
            last_edit_at: item.last_edit_at,
            last_editor_id: item.last_editor_id,
            last_editor_first_name: item.last_editor_first_name,
            last_editor_middle_name: item.last_editor_middle_name,
            last_editor_last_name: item.last_editor_last_name,
            last_edit_title: item.last_edit_title,
            last_edit_extended_reason: item.last_edit_extended_reason,
            extension: item.extension,
            mime_type: item.mime_type,
        }
    }
}

impl From<web_common::database::views::DocumentsView> for DocumentsView {
    fn from(item: web_common::database::views::DocumentsView) -> Self {
        Self {
            id: item.id,
            format_id: item.format_id,
            document_path: item.document_path,
            bytes: item.bytes,
            document_name: item.document_name,
            description: item.description,
            creator_id: item.creator_id,
            creator_first_name: item.creator_first_name,
            creator_middle_name: item.creator_middle_name,
            creator_last_name: item.creator_last_name,
            last_edit_id: item.last_edit_id,
            last_edit_at: item.last_edit_at,
            last_editor_id: item.last_editor_id,
            last_editor_first_name: item.last_editor_first_name,
            last_editor_middle_name: item.last_editor_middle_name,
            last_editor_last_name: item.last_editor_last_name,
            last_edit_title: item.last_edit_title,
            last_edit_extended_reason: item.last_edit_extended_reason,
            extension: item.extension,
            mime_type: item.mime_type,
        }
    }
}

impl DocumentsView {
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
        documents_view::dsl::documents_view
            .filter(documents_view::dsl::id.eq(id))
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
    EditsView(EditsView),
    LastEditsView(LastEditsView),
    FormatsView(FormatsView),
    DocumentsView(DocumentsView),
    PublicUser(PublicUser),
}

impl ViewRow {
    /// Get the row from the database by its ID.
    ///
    /// # Arguments
    /// * `id` - The ID of the row to get.
    /// * `connection` - The connection to the database.
    /// * `views` - The variant of the row to get.
    ///
    pub fn get(
        id: Uuid,
        connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>,
        views: &web_common::database::View
    ) -> Result<Self, diesel::result::Error> {
        match views {
            web_common::database::View::EditsView => Ok(Self::EditsView(EditsView::get(id, connection)?)),
            web_common::database::View::LastEditsView => Ok(Self::LastEditsView(LastEditsView::get(id, connection)?)),
            web_common::database::View::FormatsView => Ok(Self::FormatsView(FormatsView::get(id, connection)?)),
            web_common::database::View::DocumentsView => Ok(Self::DocumentsView(DocumentsView::get(id, connection)?)),
            web_common::database::View::PublicUser => Ok(Self::PublicUser(PublicUser::get(id, connection)?)),
        }
    }
    /// Search for the row by a given string.
    ///
    /// # Arguments
    /// * `query` - The string to search for.
    /// * `limit` - The maximum number of results, by default `10`.
    /// * `threshold` - The similarity threshold, by default `0.6`.
    /// * `views` - The variant of the row to search.
    /// * `connection` - The connection to the database.
    ///
    pub fn search(
        query: &str,
        limit: Option<i32>,
        threshold: Option<f64>,
        views: &web_common::database::View,
        connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>
    ) -> Result<Vec<Self>, diesel::result::Error> {
        match views {
            web_common::database::View::EditsView => unimplemented!(),
            web_common::database::View::LastEditsView => unimplemented!(),
            web_common::database::View::FormatsView => unimplemented!(),
            web_common::database::View::DocumentsView => unimplemented!(),
            web_common::database::View::PublicUser => unimplemented!(),
        }
    }
}
impl From<web_common::database::views::ViewRow> for ViewRow {
    fn from(item: web_common::database::views::ViewRow) -> Self {
        match item {
            web_common::database::views::ViewRow::EditsView(item) => ViewRow::EditsView(item.into()),
            web_common::database::views::ViewRow::LastEditsView(item) => ViewRow::LastEditsView(item.into()),
            web_common::database::views::ViewRow::FormatsView(item) => ViewRow::FormatsView(item.into()),
            web_common::database::views::ViewRow::DocumentsView(item) => ViewRow::DocumentsView(item.into()),
            web_common::database::views::ViewRow::PublicUser(item) => ViewRow::PublicUser(item.into()),
        }
    }
}
impl From<ViewRow> for web_common::database::views::ViewRow {
    fn from(item: ViewRow) -> Self {
        match item {
            ViewRow::EditsView(item) => web_common::database::views::ViewRow::EditsView(item.into()),
            ViewRow::LastEditsView(item) => web_common::database::views::ViewRow::LastEditsView(item.into()),
            ViewRow::FormatsView(item) => web_common::database::views::ViewRow::FormatsView(item.into()),
            ViewRow::DocumentsView(item) => web_common::database::views::ViewRow::DocumentsView(item.into()),
            ViewRow::PublicUser(item) => web_common::database::views::ViewRow::PublicUser(item.into()),
        }
    }
}
