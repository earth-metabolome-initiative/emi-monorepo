#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(
    diesel::Selectable,
    diesel::Insertable,
    diesel::AsChangeset,
    diesel::Queryable,
    diesel::Identifiable,
)]
#[diesel(primary_key(id))]
#[diesel(
    table_name = crate::codegen::diesel_codegen::tables::directus_fields::directus_fields
)]
pub struct DirectusField {
    pub id: i32,
    pub collection: String,
    pub field: String,
    pub special: Option<String>,
    pub interface: Option<String>,
    pub options: Option<::serde_json::Value>,
    pub display: Option<String>,
    pub display_options: Option<::serde_json::Value>,
    pub readonly: bool,
    pub hidden: bool,
    pub sort: Option<i32>,
    pub width: Option<String>,
    pub translations: Option<::serde_json::Value>,
    pub note: Option<String>,
    pub conditions: Option<::serde_json::Value>,
    pub required: Option<bool>,
    pub group: Option<String>,
    pub validation: Option<::serde_json::Value>,
    pub validation_message: Option<String>,
}
impl web_common_traits::prelude::TableName for DirectusField {
    const TABLE_NAME: &'static str = "directus_fields";
}
impl diesel::Identifiable for DirectusField {
    type Id = i32;
    fn id(self) -> Self::Id {
        self.id
    }
}
impl DirectusField {
    #[cfg(feature = "postgres")]
    pub fn from_collection(
        collection: &str,
        conn: &mut diesel::PgConnection,
    ) -> Result<Vec<Self>, diesel::result::Error> {
        use diesel::{ExpressionMethods, QueryDsl, RunQueryDsl, associations::HasTable};

        use crate::codegen::diesel_codegen::tables::directus_fields::directus_fields;
        Self::table()
            .filter(directus_fields::collection.eq(collection))
            .order_by(directus_fields::id.asc())
            .load::<Self>(conn)
    }
    #[cfg(feature = "postgres")]
    pub fn from_field(
        field: &str,
        conn: &mut diesel::PgConnection,
    ) -> Result<Vec<Self>, diesel::result::Error> {
        use diesel::{ExpressionMethods, QueryDsl, RunQueryDsl, associations::HasTable};

        use crate::codegen::diesel_codegen::tables::directus_fields::directus_fields;
        Self::table()
            .filter(directus_fields::field.eq(field))
            .order_by(directus_fields::id.asc())
            .load::<Self>(conn)
    }
    #[cfg(feature = "postgres")]
    pub fn from_special(
        special: &str,
        conn: &mut diesel::PgConnection,
    ) -> Result<Vec<Self>, diesel::result::Error> {
        use diesel::{ExpressionMethods, QueryDsl, RunQueryDsl, associations::HasTable};

        use crate::codegen::diesel_codegen::tables::directus_fields::directus_fields;
        Self::table()
            .filter(directus_fields::special.eq(special))
            .order_by(directus_fields::id.asc())
            .load::<Self>(conn)
    }
    #[cfg(feature = "postgres")]
    pub fn from_interface(
        interface: &str,
        conn: &mut diesel::PgConnection,
    ) -> Result<Vec<Self>, diesel::result::Error> {
        use diesel::{ExpressionMethods, QueryDsl, RunQueryDsl, associations::HasTable};

        use crate::codegen::diesel_codegen::tables::directus_fields::directus_fields;
        Self::table()
            .filter(directus_fields::interface.eq(interface))
            .order_by(directus_fields::id.asc())
            .load::<Self>(conn)
    }
    #[cfg(feature = "postgres")]
    pub fn from_display(
        display: &str,
        conn: &mut diesel::PgConnection,
    ) -> Result<Vec<Self>, diesel::result::Error> {
        use diesel::{ExpressionMethods, QueryDsl, RunQueryDsl, associations::HasTable};

        use crate::codegen::diesel_codegen::tables::directus_fields::directus_fields;
        Self::table()
            .filter(directus_fields::display.eq(display))
            .order_by(directus_fields::id.asc())
            .load::<Self>(conn)
    }
    #[cfg(feature = "postgres")]
    pub fn from_readonly(
        readonly: &bool,
        conn: &mut diesel::PgConnection,
    ) -> Result<Vec<Self>, diesel::result::Error> {
        use diesel::{ExpressionMethods, QueryDsl, RunQueryDsl, associations::HasTable};

        use crate::codegen::diesel_codegen::tables::directus_fields::directus_fields;
        Self::table()
            .filter(directus_fields::readonly.eq(readonly))
            .order_by(directus_fields::id.asc())
            .load::<Self>(conn)
    }
    #[cfg(feature = "postgres")]
    pub fn from_hidden(
        hidden: &bool,
        conn: &mut diesel::PgConnection,
    ) -> Result<Vec<Self>, diesel::result::Error> {
        use diesel::{ExpressionMethods, QueryDsl, RunQueryDsl, associations::HasTable};

        use crate::codegen::diesel_codegen::tables::directus_fields::directus_fields;
        Self::table()
            .filter(directus_fields::hidden.eq(hidden))
            .order_by(directus_fields::id.asc())
            .load::<Self>(conn)
    }
    #[cfg(feature = "postgres")]
    pub fn from_sort(
        sort: &i32,
        conn: &mut diesel::PgConnection,
    ) -> Result<Vec<Self>, diesel::result::Error> {
        use diesel::{ExpressionMethods, QueryDsl, RunQueryDsl, associations::HasTable};

        use crate::codegen::diesel_codegen::tables::directus_fields::directus_fields;
        Self::table()
            .filter(directus_fields::sort.eq(sort))
            .order_by(directus_fields::id.asc())
            .load::<Self>(conn)
    }
    #[cfg(feature = "postgres")]
    pub fn from_width(
        width: &str,
        conn: &mut diesel::PgConnection,
    ) -> Result<Vec<Self>, diesel::result::Error> {
        use diesel::{ExpressionMethods, QueryDsl, RunQueryDsl, associations::HasTable};

        use crate::codegen::diesel_codegen::tables::directus_fields::directus_fields;
        Self::table()
            .filter(directus_fields::width.eq(width))
            .order_by(directus_fields::id.asc())
            .load::<Self>(conn)
    }
    #[cfg(feature = "postgres")]
    pub fn from_note(
        note: &str,
        conn: &mut diesel::PgConnection,
    ) -> Result<Vec<Self>, diesel::result::Error> {
        use diesel::{ExpressionMethods, QueryDsl, RunQueryDsl, associations::HasTable};

        use crate::codegen::diesel_codegen::tables::directus_fields::directus_fields;
        Self::table()
            .filter(directus_fields::note.eq(note))
            .order_by(directus_fields::id.asc())
            .load::<Self>(conn)
    }
    #[cfg(feature = "postgres")]
    pub fn from_required(
        required: &bool,
        conn: &mut diesel::PgConnection,
    ) -> Result<Vec<Self>, diesel::result::Error> {
        use diesel::{ExpressionMethods, QueryDsl, RunQueryDsl, associations::HasTable};

        use crate::codegen::diesel_codegen::tables::directus_fields::directus_fields;
        Self::table()
            .filter(directus_fields::required.eq(required))
            .order_by(directus_fields::id.asc())
            .load::<Self>(conn)
    }
    #[cfg(feature = "postgres")]
    pub fn from_group(
        group: &str,
        conn: &mut diesel::PgConnection,
    ) -> Result<Vec<Self>, diesel::result::Error> {
        use diesel::{ExpressionMethods, QueryDsl, RunQueryDsl, associations::HasTable};

        use crate::codegen::diesel_codegen::tables::directus_fields::directus_fields;
        Self::table()
            .filter(directus_fields::group.eq(group))
            .order_by(directus_fields::id.asc())
            .load::<Self>(conn)
    }
    #[cfg(feature = "postgres")]
    pub fn from_validation_message(
        validation_message: &str,
        conn: &mut diesel::PgConnection,
    ) -> Result<Vec<Self>, diesel::result::Error> {
        use diesel::{ExpressionMethods, QueryDsl, RunQueryDsl, associations::HasTable};

        use crate::codegen::diesel_codegen::tables::directus_fields::directus_fields;
        Self::table()
            .filter(directus_fields::validation_message.eq(validation_message))
            .order_by(directus_fields::id.asc())
            .load::<Self>(conn)
    }
}
impl AsRef<DirectusField> for DirectusField {
    fn as_ref(&self) -> &DirectusField {
        self
    }
}
