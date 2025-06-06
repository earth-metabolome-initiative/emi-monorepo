#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(
    diesel::Selectable,
    diesel::Insertable,
    diesel::AsChangeset,
    diesel::Queryable,
    diesel::Identifiable,
)]
#[diesel(primary_key(collection))]
#[diesel(
    table_name = crate::codegen::diesel_codegen::tables::directus_collections::directus_collections
)]
pub struct DirectusCollection {
    pub collection: String,
    pub icon: Option<String>,
    pub note: Option<String>,
    pub display_template: Option<String>,
    pub hidden: bool,
    pub singleton: bool,
    pub translations: Option<::serde_json::Value>,
    pub archive_field: Option<String>,
    pub archive_app_filter: bool,
    pub archive_value: Option<String>,
    pub unarchive_value: Option<String>,
    pub sort_field: Option<String>,
    pub accountability: Option<String>,
    pub color: Option<String>,
    pub item_duplication_fields: Option<::serde_json::Value>,
    pub sort: Option<i32>,
    pub group: Option<String>,
    pub collapse: String,
    pub preview_url: Option<String>,
    pub versioning: bool,
}
impl web_common_traits::prelude::TableName for DirectusCollection {
    const TABLE_NAME: &'static str = "directus_collections";
}
impl<C> web_common_traits::prelude::Ancestor<C> for DirectusCollection
where
    Self: web_common_traits::prelude::TableName + Sized,
    C: diesel::connection::LoadConnection,
    <C as diesel::Connection>::Backend: diesel::backend::DieselReserveSpecialization
        + diesel::sql_types::HasSqlType<diesel::sql_types::Text>
        + 'static,
    web_common_traits::prelude::AncestorExists: diesel::deserialize::FromSqlRow<
            diesel::sql_types::Untyped,
            <C as diesel::Connection>::Backend,
        >,
    for<'a> &'a Self: diesel::Identifiable,
    for<'a> <&'a Self as diesel::Identifiable>::Id:
        diesel::serialize::ToSql<diesel::sql_types::Text, C::Backend>,
{
    const PARENT_ID: &'static str = "group";
    const ID: &'static str = "collection";
    type SqlType = diesel::sql_types::Text;
}
impl web_common_traits::prelude::Descendant<DirectusCollection> for DirectusCollection {
    fn parent_id(&self) -> Option<<&Self as diesel::Identifiable>::Id> {
        self.group.as_ref()
    }
}
impl diesel::Identifiable for DirectusCollection {
    type Id = String;
    fn id(self) -> Self::Id {
        self.collection
    }
}
impl DirectusCollection {
    pub fn group<C: diesel::connection::LoadConnection>(
        &self,
        conn: &mut C,
    ) -> Result<
        Option<
            crate::codegen::structs_codegen::tables::directus_collections::DirectusCollection,
        >,
        diesel::result::Error,
    >
    where
        crate::codegen::structs_codegen::tables::directus_collections::DirectusCollection: diesel::Identifiable,
        <crate::codegen::structs_codegen::tables::directus_collections::DirectusCollection as diesel::associations::HasTable>::Table: diesel::query_dsl::methods::FindDsl<
            <crate::codegen::structs_codegen::tables::directus_collections::DirectusCollection as diesel::Identifiable>::Id,
        >,
        <<crate::codegen::structs_codegen::tables::directus_collections::DirectusCollection as diesel::associations::HasTable>::Table as diesel::query_dsl::methods::FindDsl<
            <crate::codegen::structs_codegen::tables::directus_collections::DirectusCollection as diesel::Identifiable>::Id,
        >>::Output: diesel::query_dsl::methods::LimitDsl + diesel::RunQueryDsl<C>,
        <<<crate::codegen::structs_codegen::tables::directus_collections::DirectusCollection as diesel::associations::HasTable>::Table as diesel::query_dsl::methods::FindDsl<
            <crate::codegen::structs_codegen::tables::directus_collections::DirectusCollection as diesel::Identifiable>::Id,
        >>::Output as diesel::query_dsl::methods::LimitDsl>::Output: for<'a> diesel::query_dsl::LoadQuery<
            'a,
            C,
            crate::codegen::structs_codegen::tables::directus_collections::DirectusCollection,
        >,
    {
        use diesel::{QueryDsl, RunQueryDsl, associations::HasTable};
        let Some(group) = self.group else {
            return Ok(None);
        };
        RunQueryDsl::first(
                QueryDsl::find(
                    crate::codegen::structs_codegen::tables::directus_collections::DirectusCollection::table(),
                    group,
                ),
                conn,
            )
            .map(Some)
    }
    #[cfg(feature = "postgres")]
    pub fn from_icon(
        icon: &str,
        conn: &mut diesel::PgConnection,
    ) -> Result<Vec<Self>, diesel::result::Error> {
        use diesel::{ExpressionMethods, QueryDsl, RunQueryDsl, associations::HasTable};

        use crate::codegen::diesel_codegen::tables::directus_collections::directus_collections;
        Self::table()
            .filter(directus_collections::icon.eq(icon))
            .order_by(directus_collections::collection.asc())
            .load::<Self>(conn)
    }
    #[cfg(feature = "postgres")]
    pub fn from_note(
        note: &str,
        conn: &mut diesel::PgConnection,
    ) -> Result<Vec<Self>, diesel::result::Error> {
        use diesel::{ExpressionMethods, QueryDsl, RunQueryDsl, associations::HasTable};

        use crate::codegen::diesel_codegen::tables::directus_collections::directus_collections;
        Self::table()
            .filter(directus_collections::note.eq(note))
            .order_by(directus_collections::collection.asc())
            .load::<Self>(conn)
    }
    #[cfg(feature = "postgres")]
    pub fn from_display_template(
        display_template: &str,
        conn: &mut diesel::PgConnection,
    ) -> Result<Vec<Self>, diesel::result::Error> {
        use diesel::{ExpressionMethods, QueryDsl, RunQueryDsl, associations::HasTable};

        use crate::codegen::diesel_codegen::tables::directus_collections::directus_collections;
        Self::table()
            .filter(directus_collections::display_template.eq(display_template))
            .order_by(directus_collections::collection.asc())
            .load::<Self>(conn)
    }
    #[cfg(feature = "postgres")]
    pub fn from_hidden(
        hidden: &bool,
        conn: &mut diesel::PgConnection,
    ) -> Result<Vec<Self>, diesel::result::Error> {
        use diesel::{ExpressionMethods, QueryDsl, RunQueryDsl, associations::HasTable};

        use crate::codegen::diesel_codegen::tables::directus_collections::directus_collections;
        Self::table()
            .filter(directus_collections::hidden.eq(hidden))
            .order_by(directus_collections::collection.asc())
            .load::<Self>(conn)
    }
    #[cfg(feature = "postgres")]
    pub fn from_singleton(
        singleton: &bool,
        conn: &mut diesel::PgConnection,
    ) -> Result<Vec<Self>, diesel::result::Error> {
        use diesel::{ExpressionMethods, QueryDsl, RunQueryDsl, associations::HasTable};

        use crate::codegen::diesel_codegen::tables::directus_collections::directus_collections;
        Self::table()
            .filter(directus_collections::singleton.eq(singleton))
            .order_by(directus_collections::collection.asc())
            .load::<Self>(conn)
    }
    #[cfg(feature = "postgres")]
    pub fn from_archive_field(
        archive_field: &str,
        conn: &mut diesel::PgConnection,
    ) -> Result<Vec<Self>, diesel::result::Error> {
        use diesel::{ExpressionMethods, QueryDsl, RunQueryDsl, associations::HasTable};

        use crate::codegen::diesel_codegen::tables::directus_collections::directus_collections;
        Self::table()
            .filter(directus_collections::archive_field.eq(archive_field))
            .order_by(directus_collections::collection.asc())
            .load::<Self>(conn)
    }
    #[cfg(feature = "postgres")]
    pub fn from_archive_app_filter(
        archive_app_filter: &bool,
        conn: &mut diesel::PgConnection,
    ) -> Result<Vec<Self>, diesel::result::Error> {
        use diesel::{ExpressionMethods, QueryDsl, RunQueryDsl, associations::HasTable};

        use crate::codegen::diesel_codegen::tables::directus_collections::directus_collections;
        Self::table()
            .filter(directus_collections::archive_app_filter.eq(archive_app_filter))
            .order_by(directus_collections::collection.asc())
            .load::<Self>(conn)
    }
    #[cfg(feature = "postgres")]
    pub fn from_archive_value(
        archive_value: &str,
        conn: &mut diesel::PgConnection,
    ) -> Result<Vec<Self>, diesel::result::Error> {
        use diesel::{ExpressionMethods, QueryDsl, RunQueryDsl, associations::HasTable};

        use crate::codegen::diesel_codegen::tables::directus_collections::directus_collections;
        Self::table()
            .filter(directus_collections::archive_value.eq(archive_value))
            .order_by(directus_collections::collection.asc())
            .load::<Self>(conn)
    }
    #[cfg(feature = "postgres")]
    pub fn from_unarchive_value(
        unarchive_value: &str,
        conn: &mut diesel::PgConnection,
    ) -> Result<Vec<Self>, diesel::result::Error> {
        use diesel::{ExpressionMethods, QueryDsl, RunQueryDsl, associations::HasTable};

        use crate::codegen::diesel_codegen::tables::directus_collections::directus_collections;
        Self::table()
            .filter(directus_collections::unarchive_value.eq(unarchive_value))
            .order_by(directus_collections::collection.asc())
            .load::<Self>(conn)
    }
    #[cfg(feature = "postgres")]
    pub fn from_sort_field(
        sort_field: &str,
        conn: &mut diesel::PgConnection,
    ) -> Result<Vec<Self>, diesel::result::Error> {
        use diesel::{ExpressionMethods, QueryDsl, RunQueryDsl, associations::HasTable};

        use crate::codegen::diesel_codegen::tables::directus_collections::directus_collections;
        Self::table()
            .filter(directus_collections::sort_field.eq(sort_field))
            .order_by(directus_collections::collection.asc())
            .load::<Self>(conn)
    }
    #[cfg(feature = "postgres")]
    pub fn from_accountability(
        accountability: &str,
        conn: &mut diesel::PgConnection,
    ) -> Result<Vec<Self>, diesel::result::Error> {
        use diesel::{ExpressionMethods, QueryDsl, RunQueryDsl, associations::HasTable};

        use crate::codegen::diesel_codegen::tables::directus_collections::directus_collections;
        Self::table()
            .filter(directus_collections::accountability.eq(accountability))
            .order_by(directus_collections::collection.asc())
            .load::<Self>(conn)
    }
    #[cfg(feature = "postgres")]
    pub fn from_color(
        color: &str,
        conn: &mut diesel::PgConnection,
    ) -> Result<Vec<Self>, diesel::result::Error> {
        use diesel::{ExpressionMethods, QueryDsl, RunQueryDsl, associations::HasTable};

        use crate::codegen::diesel_codegen::tables::directus_collections::directus_collections;
        Self::table()
            .filter(directus_collections::color.eq(color))
            .order_by(directus_collections::collection.asc())
            .load::<Self>(conn)
    }
    #[cfg(feature = "postgres")]
    pub fn from_sort(
        sort: &i32,
        conn: &mut diesel::PgConnection,
    ) -> Result<Vec<Self>, diesel::result::Error> {
        use diesel::{ExpressionMethods, QueryDsl, RunQueryDsl, associations::HasTable};

        use crate::codegen::diesel_codegen::tables::directus_collections::directus_collections;
        Self::table()
            .filter(directus_collections::sort.eq(sort))
            .order_by(directus_collections::collection.asc())
            .load::<Self>(conn)
    }
    #[cfg(feature = "postgres")]
    pub fn from_group(
        group: &str,
        conn: &mut diesel::PgConnection,
    ) -> Result<Vec<Self>, diesel::result::Error> {
        use diesel::{ExpressionMethods, QueryDsl, RunQueryDsl, associations::HasTable};

        use crate::codegen::diesel_codegen::tables::directus_collections::directus_collections;
        Self::table()
            .filter(directus_collections::group.eq(group))
            .order_by(directus_collections::collection.asc())
            .load::<Self>(conn)
    }
    #[cfg(feature = "postgres")]
    pub fn from_collapse(
        collapse: &str,
        conn: &mut diesel::PgConnection,
    ) -> Result<Vec<Self>, diesel::result::Error> {
        use diesel::{ExpressionMethods, QueryDsl, RunQueryDsl, associations::HasTable};

        use crate::codegen::diesel_codegen::tables::directus_collections::directus_collections;
        Self::table()
            .filter(directus_collections::collapse.eq(collapse))
            .order_by(directus_collections::collection.asc())
            .load::<Self>(conn)
    }
    #[cfg(feature = "postgres")]
    pub fn from_preview_url(
        preview_url: &str,
        conn: &mut diesel::PgConnection,
    ) -> Result<Vec<Self>, diesel::result::Error> {
        use diesel::{ExpressionMethods, QueryDsl, RunQueryDsl, associations::HasTable};

        use crate::codegen::diesel_codegen::tables::directus_collections::directus_collections;
        Self::table()
            .filter(directus_collections::preview_url.eq(preview_url))
            .order_by(directus_collections::collection.asc())
            .load::<Self>(conn)
    }
    #[cfg(feature = "postgres")]
    pub fn from_versioning(
        versioning: &bool,
        conn: &mut diesel::PgConnection,
    ) -> Result<Vec<Self>, diesel::result::Error> {
        use diesel::{ExpressionMethods, QueryDsl, RunQueryDsl, associations::HasTable};

        use crate::codegen::diesel_codegen::tables::directus_collections::directus_collections;
        Self::table()
            .filter(directus_collections::versioning.eq(versioning))
            .order_by(directus_collections::collection.asc())
            .load::<Self>(conn)
    }
}
impl AsRef<DirectusCollection> for DirectusCollection {
    fn as_ref(&self) -> &DirectusCollection {
        self
    }
}
