#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(
    diesel::Selectable,
    diesel::Insertable,
    diesel::AsChangeset,
    diesel::Queryable,
    diesel::Identifiable,
    diesel::Associations
)]
#[diesel(
    belongs_to(
        crate::codegen::structs_codegen::tables::directus_folders::DirectusFolder,
        foreign_key = folder
    )
)]
#[diesel(primary_key(id))]
#[diesel(
    table_name = crate::codegen::diesel_codegen::tables::directus_files::directus_files
)]
pub struct DirectusFile {
    pub id: ::rosetta_uuid::Uuid,
    pub storage: String,
    pub filename_disk: Option<String>,
    pub filename_download: String,
    pub title: Option<String>,
    pub r#type: Option<String>,
    pub folder: Option<::rosetta_uuid::Uuid>,
    pub uploaded_by: Option<::rosetta_uuid::Uuid>,
    pub created_on: ::rosetta_timestamp::TimestampUTC,
    pub modified_by: Option<::rosetta_uuid::Uuid>,
    pub modified_on: ::rosetta_timestamp::TimestampUTC,
    pub charset: Option<String>,
    pub filesize: Option<i64>,
    pub width: Option<i32>,
    pub height: Option<i32>,
    pub duration: Option<i32>,
    pub embed: Option<String>,
    pub description: Option<String>,
    pub location: Option<String>,
    pub tags: Option<String>,
    pub metadata: Option<::serde_json::Value>,
    pub focal_point_x: Option<i32>,
    pub focal_point_y: Option<i32>,
    pub tus_id: Option<String>,
    pub tus_data: Option<::serde_json::Value>,
    pub uploaded_on: Option<::rosetta_timestamp::TimestampUTC>,
}
impl web_common_traits::prelude::TableName for DirectusFile {
    const TABLE_NAME: &'static str = "directus_files";
}
impl web_common_traits::prelude::ExtensionTable<
    crate::codegen::structs_codegen::tables::directus_files::DirectusFile,
> for DirectusFile
where
    for<'a> &'a Self: diesel::Identifiable<Id = &'a ::rosetta_uuid::Uuid>,
{}
impl diesel::Identifiable for DirectusFile {
    type Id = ::rosetta_uuid::Uuid;
    fn id(self) -> Self::Id {
        self.id
    }
}
impl web_common_traits::database::PrimaryKeyLike for DirectusFile {
    type PrimaryKey = ::rosetta_uuid::Uuid;
    fn primary_key(&self) -> Self::PrimaryKey {
        self.id
    }
}
impl DirectusFile {
    pub fn folder<C: diesel::connection::LoadConnection>(
        &self,
        conn: &mut C,
    ) -> Result<
        Option<
            crate::codegen::structs_codegen::tables::directus_folders::DirectusFolder,
        >,
        diesel::result::Error,
    >
    where
        crate::codegen::structs_codegen::tables::directus_folders::DirectusFolder: web_common_traits::database::Read<
            C,
        >,
    {
        use web_common_traits::database::Read;
        use diesel::OptionalExtension;
        let Some(folder) = self.folder else {
            return Ok(None);
        };
        crate::codegen::structs_codegen::tables::directus_folders::DirectusFolder::read(
                folder,
                conn,
            )
            .optional()
    }
    pub fn modified_by<C: diesel::connection::LoadConnection>(
        &self,
        conn: &mut C,
    ) -> Result<
        Option<crate::codegen::structs_codegen::tables::directus_users::DirectusUser>,
        diesel::result::Error,
    >
    where
        crate::codegen::structs_codegen::tables::directus_users::DirectusUser: web_common_traits::database::Read<
            C,
        >,
    {
        use web_common_traits::database::Read;
        use diesel::OptionalExtension;
        let Some(modified_by) = self.modified_by else {
            return Ok(None);
        };
        crate::codegen::structs_codegen::tables::directus_users::DirectusUser::read(
                modified_by,
                conn,
            )
            .optional()
    }
    pub fn uploaded_by<C: diesel::connection::LoadConnection>(
        &self,
        conn: &mut C,
    ) -> Result<
        Option<crate::codegen::structs_codegen::tables::directus_users::DirectusUser>,
        diesel::result::Error,
    >
    where
        crate::codegen::structs_codegen::tables::directus_users::DirectusUser: web_common_traits::database::Read<
            C,
        >,
    {
        use web_common_traits::database::Read;
        use diesel::OptionalExtension;
        let Some(uploaded_by) = self.uploaded_by else {
            return Ok(None);
        };
        crate::codegen::structs_codegen::tables::directus_users::DirectusUser::read(
                uploaded_by,
                conn,
            )
            .optional()
    }
    #[cfg(feature = "postgres")]
    pub fn from_modified_by(
        modified_by: ::rosetta_uuid::Uuid,
        conn: &mut diesel::PgConnection,
    ) -> Result<Vec<Self>, diesel::result::Error> {
        use diesel::RunQueryDsl;
        use diesel::associations::HasTable;
        use diesel::{QueryDsl, ExpressionMethods};
        use crate::codegen::diesel_codegen::tables::directus_files::directus_files;
        Self::table()
            .filter(directus_files::modified_by.eq(modified_by))
            .order_by(directus_files::id.asc())
            .load::<Self>(conn)
    }
    #[cfg(feature = "postgres")]
    pub fn from_uploaded_by(
        uploaded_by: ::rosetta_uuid::Uuid,
        conn: &mut diesel::PgConnection,
    ) -> Result<Vec<Self>, diesel::result::Error> {
        use diesel::RunQueryDsl;
        use diesel::associations::HasTable;
        use diesel::{QueryDsl, ExpressionMethods};
        use crate::codegen::diesel_codegen::tables::directus_files::directus_files;
        Self::table()
            .filter(directus_files::uploaded_by.eq(uploaded_by))
            .order_by(directus_files::id.asc())
            .load::<Self>(conn)
    }
    #[cfg(feature = "postgres")]
    pub fn from_storage(
        storage: &str,
        conn: &mut diesel::PgConnection,
    ) -> Result<Vec<Self>, diesel::result::Error> {
        use diesel::RunQueryDsl;
        use diesel::associations::HasTable;
        use diesel::{QueryDsl, ExpressionMethods};
        use crate::codegen::diesel_codegen::tables::directus_files::directus_files;
        Self::table()
            .filter(directus_files::storage.eq(storage))
            .order_by(directus_files::id.asc())
            .load::<Self>(conn)
    }
    #[cfg(feature = "postgres")]
    pub fn from_filename_disk(
        filename_disk: &str,
        conn: &mut diesel::PgConnection,
    ) -> Result<Vec<Self>, diesel::result::Error> {
        use diesel::RunQueryDsl;
        use diesel::associations::HasTable;
        use diesel::{QueryDsl, ExpressionMethods};
        use crate::codegen::diesel_codegen::tables::directus_files::directus_files;
        Self::table()
            .filter(directus_files::filename_disk.eq(filename_disk))
            .order_by(directus_files::id.asc())
            .load::<Self>(conn)
    }
    #[cfg(feature = "postgres")]
    pub fn from_filename_download(
        filename_download: &str,
        conn: &mut diesel::PgConnection,
    ) -> Result<Vec<Self>, diesel::result::Error> {
        use diesel::RunQueryDsl;
        use diesel::associations::HasTable;
        use diesel::{QueryDsl, ExpressionMethods};
        use crate::codegen::diesel_codegen::tables::directus_files::directus_files;
        Self::table()
            .filter(directus_files::filename_download.eq(filename_download))
            .order_by(directus_files::id.asc())
            .load::<Self>(conn)
    }
    #[cfg(feature = "postgres")]
    pub fn from_title(
        title: &str,
        conn: &mut diesel::PgConnection,
    ) -> Result<Vec<Self>, diesel::result::Error> {
        use diesel::RunQueryDsl;
        use diesel::associations::HasTable;
        use diesel::{QueryDsl, ExpressionMethods};
        use crate::codegen::diesel_codegen::tables::directus_files::directus_files;
        Self::table()
            .filter(directus_files::title.eq(title))
            .order_by(directus_files::id.asc())
            .load::<Self>(conn)
    }
    #[cfg(feature = "postgres")]
    pub fn from_type(
        r#type: &str,
        conn: &mut diesel::PgConnection,
    ) -> Result<Vec<Self>, diesel::result::Error> {
        use diesel::RunQueryDsl;
        use diesel::associations::HasTable;
        use diesel::{QueryDsl, ExpressionMethods};
        use crate::codegen::diesel_codegen::tables::directus_files::directus_files;
        Self::table()
            .filter(directus_files::r#type.eq(r#type))
            .order_by(directus_files::id.asc())
            .load::<Self>(conn)
    }
    pub fn from_created_on<C>(
        created_on: ::rosetta_timestamp::TimestampUTC,
        conn: &mut C,
    ) -> Result<Vec<Self>, diesel::result::Error>
    where
        C: diesel::connection::LoadConnection,
        <Self as diesel::associations::HasTable>::Table: diesel::query_dsl::methods::FilterDsl<
            <crate::codegen::diesel_codegen::tables::directus_files::directus_files::created_on as diesel::expression_methods::EqAll<
                ::rosetta_timestamp::TimestampUTC,
            >>::Output,
        >,
        <<Self as diesel::associations::HasTable>::Table as diesel::query_dsl::methods::FilterDsl<
            <crate::codegen::diesel_codegen::tables::directus_files::directus_files::created_on as diesel::expression_methods::EqAll<
                ::rosetta_timestamp::TimestampUTC,
            >>::Output,
        >>::Output: diesel::query_dsl::methods::OrderDsl<
            diesel::helper_types::Asc<
                crate::codegen::diesel_codegen::tables::directus_files::directus_files::id,
            >,
        >,
        <<<Self as diesel::associations::HasTable>::Table as diesel::query_dsl::methods::FilterDsl<
            <crate::codegen::diesel_codegen::tables::directus_files::directus_files::created_on as diesel::expression_methods::EqAll<
                ::rosetta_timestamp::TimestampUTC,
            >>::Output,
        >>::Output as diesel::query_dsl::methods::OrderDsl<
            diesel::helper_types::Asc<
                crate::codegen::diesel_codegen::tables::directus_files::directus_files::id,
            >,
        >>::Output: diesel::RunQueryDsl<C>
            + for<'a> diesel::query_dsl::LoadQuery<'a, C, Self>,
    {
        use diesel::RunQueryDsl;
        use diesel::associations::HasTable;
        use diesel::{QueryDsl, ExpressionMethods};
        use crate::codegen::diesel_codegen::tables::directus_files::directus_files;
        Self::table()
            .filter(directus_files::created_on.eq(created_on))
            .order_by(directus_files::id.asc())
            .load::<Self>(conn)
    }
    pub fn from_modified_on<C>(
        modified_on: ::rosetta_timestamp::TimestampUTC,
        conn: &mut C,
    ) -> Result<Vec<Self>, diesel::result::Error>
    where
        C: diesel::connection::LoadConnection,
        <Self as diesel::associations::HasTable>::Table: diesel::query_dsl::methods::FilterDsl<
            <crate::codegen::diesel_codegen::tables::directus_files::directus_files::modified_on as diesel::expression_methods::EqAll<
                ::rosetta_timestamp::TimestampUTC,
            >>::Output,
        >,
        <<Self as diesel::associations::HasTable>::Table as diesel::query_dsl::methods::FilterDsl<
            <crate::codegen::diesel_codegen::tables::directus_files::directus_files::modified_on as diesel::expression_methods::EqAll<
                ::rosetta_timestamp::TimestampUTC,
            >>::Output,
        >>::Output: diesel::query_dsl::methods::OrderDsl<
            diesel::helper_types::Asc<
                crate::codegen::diesel_codegen::tables::directus_files::directus_files::id,
            >,
        >,
        <<<Self as diesel::associations::HasTable>::Table as diesel::query_dsl::methods::FilterDsl<
            <crate::codegen::diesel_codegen::tables::directus_files::directus_files::modified_on as diesel::expression_methods::EqAll<
                ::rosetta_timestamp::TimestampUTC,
            >>::Output,
        >>::Output as diesel::query_dsl::methods::OrderDsl<
            diesel::helper_types::Asc<
                crate::codegen::diesel_codegen::tables::directus_files::directus_files::id,
            >,
        >>::Output: diesel::RunQueryDsl<C>
            + for<'a> diesel::query_dsl::LoadQuery<'a, C, Self>,
    {
        use diesel::RunQueryDsl;
        use diesel::associations::HasTable;
        use diesel::{QueryDsl, ExpressionMethods};
        use crate::codegen::diesel_codegen::tables::directus_files::directus_files;
        Self::table()
            .filter(directus_files::modified_on.eq(modified_on))
            .order_by(directus_files::id.asc())
            .load::<Self>(conn)
    }
    #[cfg(feature = "postgres")]
    pub fn from_charset(
        charset: &str,
        conn: &mut diesel::PgConnection,
    ) -> Result<Vec<Self>, diesel::result::Error> {
        use diesel::RunQueryDsl;
        use diesel::associations::HasTable;
        use diesel::{QueryDsl, ExpressionMethods};
        use crate::codegen::diesel_codegen::tables::directus_files::directus_files;
        Self::table()
            .filter(directus_files::charset.eq(charset))
            .order_by(directus_files::id.asc())
            .load::<Self>(conn)
    }
    #[cfg(feature = "postgres")]
    pub fn from_filesize(
        filesize: i64,
        conn: &mut diesel::PgConnection,
    ) -> Result<Vec<Self>, diesel::result::Error> {
        use diesel::RunQueryDsl;
        use diesel::associations::HasTable;
        use diesel::{QueryDsl, ExpressionMethods};
        use crate::codegen::diesel_codegen::tables::directus_files::directus_files;
        Self::table()
            .filter(directus_files::filesize.eq(filesize))
            .order_by(directus_files::id.asc())
            .load::<Self>(conn)
    }
    #[cfg(feature = "postgres")]
    pub fn from_width(
        width: i32,
        conn: &mut diesel::PgConnection,
    ) -> Result<Vec<Self>, diesel::result::Error> {
        use diesel::RunQueryDsl;
        use diesel::associations::HasTable;
        use diesel::{QueryDsl, ExpressionMethods};
        use crate::codegen::diesel_codegen::tables::directus_files::directus_files;
        Self::table()
            .filter(directus_files::width.eq(width))
            .order_by(directus_files::id.asc())
            .load::<Self>(conn)
    }
    #[cfg(feature = "postgres")]
    pub fn from_height(
        height: i32,
        conn: &mut diesel::PgConnection,
    ) -> Result<Vec<Self>, diesel::result::Error> {
        use diesel::RunQueryDsl;
        use diesel::associations::HasTable;
        use diesel::{QueryDsl, ExpressionMethods};
        use crate::codegen::diesel_codegen::tables::directus_files::directus_files;
        Self::table()
            .filter(directus_files::height.eq(height))
            .order_by(directus_files::id.asc())
            .load::<Self>(conn)
    }
    #[cfg(feature = "postgres")]
    pub fn from_duration(
        duration: i32,
        conn: &mut diesel::PgConnection,
    ) -> Result<Vec<Self>, diesel::result::Error> {
        use diesel::RunQueryDsl;
        use diesel::associations::HasTable;
        use diesel::{QueryDsl, ExpressionMethods};
        use crate::codegen::diesel_codegen::tables::directus_files::directus_files;
        Self::table()
            .filter(directus_files::duration.eq(duration))
            .order_by(directus_files::id.asc())
            .load::<Self>(conn)
    }
    #[cfg(feature = "postgres")]
    pub fn from_embed(
        embed: &str,
        conn: &mut diesel::PgConnection,
    ) -> Result<Vec<Self>, diesel::result::Error> {
        use diesel::RunQueryDsl;
        use diesel::associations::HasTable;
        use diesel::{QueryDsl, ExpressionMethods};
        use crate::codegen::diesel_codegen::tables::directus_files::directus_files;
        Self::table()
            .filter(directus_files::embed.eq(embed))
            .order_by(directus_files::id.asc())
            .load::<Self>(conn)
    }
    #[cfg(feature = "postgres")]
    pub fn from_description(
        description: &str,
        conn: &mut diesel::PgConnection,
    ) -> Result<Vec<Self>, diesel::result::Error> {
        use diesel::RunQueryDsl;
        use diesel::associations::HasTable;
        use diesel::{QueryDsl, ExpressionMethods};
        use crate::codegen::diesel_codegen::tables::directus_files::directus_files;
        Self::table()
            .filter(directus_files::description.eq(description))
            .order_by(directus_files::id.asc())
            .load::<Self>(conn)
    }
    #[cfg(feature = "postgres")]
    pub fn from_location(
        location: &str,
        conn: &mut diesel::PgConnection,
    ) -> Result<Vec<Self>, diesel::result::Error> {
        use diesel::RunQueryDsl;
        use diesel::associations::HasTable;
        use diesel::{QueryDsl, ExpressionMethods};
        use crate::codegen::diesel_codegen::tables::directus_files::directus_files;
        Self::table()
            .filter(directus_files::location.eq(location))
            .order_by(directus_files::id.asc())
            .load::<Self>(conn)
    }
    #[cfg(feature = "postgres")]
    pub fn from_tags(
        tags: &str,
        conn: &mut diesel::PgConnection,
    ) -> Result<Vec<Self>, diesel::result::Error> {
        use diesel::RunQueryDsl;
        use diesel::associations::HasTable;
        use diesel::{QueryDsl, ExpressionMethods};
        use crate::codegen::diesel_codegen::tables::directus_files::directus_files;
        Self::table()
            .filter(directus_files::tags.eq(tags))
            .order_by(directus_files::id.asc())
            .load::<Self>(conn)
    }
    #[cfg(feature = "postgres")]
    pub fn from_focal_point_x(
        focal_point_x: i32,
        conn: &mut diesel::PgConnection,
    ) -> Result<Vec<Self>, diesel::result::Error> {
        use diesel::RunQueryDsl;
        use diesel::associations::HasTable;
        use diesel::{QueryDsl, ExpressionMethods};
        use crate::codegen::diesel_codegen::tables::directus_files::directus_files;
        Self::table()
            .filter(directus_files::focal_point_x.eq(focal_point_x))
            .order_by(directus_files::id.asc())
            .load::<Self>(conn)
    }
    #[cfg(feature = "postgres")]
    pub fn from_focal_point_y(
        focal_point_y: i32,
        conn: &mut diesel::PgConnection,
    ) -> Result<Vec<Self>, diesel::result::Error> {
        use diesel::RunQueryDsl;
        use diesel::associations::HasTable;
        use diesel::{QueryDsl, ExpressionMethods};
        use crate::codegen::diesel_codegen::tables::directus_files::directus_files;
        Self::table()
            .filter(directus_files::focal_point_y.eq(focal_point_y))
            .order_by(directus_files::id.asc())
            .load::<Self>(conn)
    }
    #[cfg(feature = "postgres")]
    pub fn from_tus_id(
        tus_id: &str,
        conn: &mut diesel::PgConnection,
    ) -> Result<Vec<Self>, diesel::result::Error> {
        use diesel::RunQueryDsl;
        use diesel::associations::HasTable;
        use diesel::{QueryDsl, ExpressionMethods};
        use crate::codegen::diesel_codegen::tables::directus_files::directus_files;
        Self::table()
            .filter(directus_files::tus_id.eq(tus_id))
            .order_by(directus_files::id.asc())
            .load::<Self>(conn)
    }
    #[cfg(feature = "postgres")]
    pub fn from_uploaded_on(
        uploaded_on: ::rosetta_timestamp::TimestampUTC,
        conn: &mut diesel::PgConnection,
    ) -> Result<Vec<Self>, diesel::result::Error> {
        use diesel::RunQueryDsl;
        use diesel::associations::HasTable;
        use diesel::{QueryDsl, ExpressionMethods};
        use crate::codegen::diesel_codegen::tables::directus_files::directus_files;
        Self::table()
            .filter(directus_files::uploaded_on.eq(uploaded_on))
            .order_by(directus_files::id.asc())
            .load::<Self>(conn)
    }
}
impl AsRef<DirectusFile> for DirectusFile {
    fn as_ref(&self) -> &DirectusFile {
        self
    }
}
