#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(
    diesel::Selectable,
    diesel::Insertable,
    diesel::AsChangeset,
    diesel::Queryable,
    diesel::Identifiable
)]
#[diesel(primary_key(id))]
#[diesel(
    table_name = crate::codegen::diesel_codegen::tables::directus_relations::directus_relations
)]
pub struct DirectusRelation {
    pub id: i32,
    pub many_collection: String,
    pub many_field: String,
    pub one_collection: Option<String>,
    pub one_field: Option<String>,
    pub one_collection_field: Option<String>,
    pub one_allowed_collections: Option<String>,
    pub junction_field: Option<String>,
    pub sort_field: Option<String>,
    pub one_deselect_action: String,
}
impl web_common_traits::prelude::TableName for DirectusRelation {
    const TABLE_NAME: &'static str = "directus_relations";
}
impl diesel::Identifiable for DirectusRelation {
    type Id = i32;
    fn id(self) -> Self::Id {
        self.id
    }
}
impl DirectusRelation {
    #[cfg(feature = "postgres")]
    pub fn from_many_collection(
        many_collection: &str,
        conn: &mut diesel::PgConnection,
    ) -> Result<Vec<Self>, diesel::result::Error> {
        use diesel::RunQueryDsl;
        use diesel::associations::HasTable;
        use diesel::{QueryDsl, ExpressionMethods};
        use crate::codegen::diesel_codegen::tables::directus_relations::directus_relations;
        Self::table()
            .filter(directus_relations::many_collection.eq(many_collection))
            .order_by(directus_relations::id.asc())
            .load::<Self>(conn)
    }
    #[cfg(feature = "postgres")]
    pub fn from_many_field(
        many_field: &str,
        conn: &mut diesel::PgConnection,
    ) -> Result<Vec<Self>, diesel::result::Error> {
        use diesel::RunQueryDsl;
        use diesel::associations::HasTable;
        use diesel::{QueryDsl, ExpressionMethods};
        use crate::codegen::diesel_codegen::tables::directus_relations::directus_relations;
        Self::table()
            .filter(directus_relations::many_field.eq(many_field))
            .order_by(directus_relations::id.asc())
            .load::<Self>(conn)
    }
    #[cfg(feature = "postgres")]
    pub fn from_one_collection(
        one_collection: &str,
        conn: &mut diesel::PgConnection,
    ) -> Result<Vec<Self>, diesel::result::Error> {
        use diesel::RunQueryDsl;
        use diesel::associations::HasTable;
        use diesel::{QueryDsl, ExpressionMethods};
        use crate::codegen::diesel_codegen::tables::directus_relations::directus_relations;
        Self::table()
            .filter(directus_relations::one_collection.eq(one_collection))
            .order_by(directus_relations::id.asc())
            .load::<Self>(conn)
    }
    #[cfg(feature = "postgres")]
    pub fn from_one_field(
        one_field: &str,
        conn: &mut diesel::PgConnection,
    ) -> Result<Vec<Self>, diesel::result::Error> {
        use diesel::RunQueryDsl;
        use diesel::associations::HasTable;
        use diesel::{QueryDsl, ExpressionMethods};
        use crate::codegen::diesel_codegen::tables::directus_relations::directus_relations;
        Self::table()
            .filter(directus_relations::one_field.eq(one_field))
            .order_by(directus_relations::id.asc())
            .load::<Self>(conn)
    }
    #[cfg(feature = "postgres")]
    pub fn from_one_collection_field(
        one_collection_field: &str,
        conn: &mut diesel::PgConnection,
    ) -> Result<Vec<Self>, diesel::result::Error> {
        use diesel::RunQueryDsl;
        use diesel::associations::HasTable;
        use diesel::{QueryDsl, ExpressionMethods};
        use crate::codegen::diesel_codegen::tables::directus_relations::directus_relations;
        Self::table()
            .filter(directus_relations::one_collection_field.eq(one_collection_field))
            .order_by(directus_relations::id.asc())
            .load::<Self>(conn)
    }
    #[cfg(feature = "postgres")]
    pub fn from_one_allowed_collections(
        one_allowed_collections: &str,
        conn: &mut diesel::PgConnection,
    ) -> Result<Vec<Self>, diesel::result::Error> {
        use diesel::RunQueryDsl;
        use diesel::associations::HasTable;
        use diesel::{QueryDsl, ExpressionMethods};
        use crate::codegen::diesel_codegen::tables::directus_relations::directus_relations;
        Self::table()
            .filter(
                directus_relations::one_allowed_collections.eq(one_allowed_collections),
            )
            .order_by(directus_relations::id.asc())
            .load::<Self>(conn)
    }
    #[cfg(feature = "postgres")]
    pub fn from_junction_field(
        junction_field: &str,
        conn: &mut diesel::PgConnection,
    ) -> Result<Vec<Self>, diesel::result::Error> {
        use diesel::RunQueryDsl;
        use diesel::associations::HasTable;
        use diesel::{QueryDsl, ExpressionMethods};
        use crate::codegen::diesel_codegen::tables::directus_relations::directus_relations;
        Self::table()
            .filter(directus_relations::junction_field.eq(junction_field))
            .order_by(directus_relations::id.asc())
            .load::<Self>(conn)
    }
    #[cfg(feature = "postgres")]
    pub fn from_sort_field(
        sort_field: &str,
        conn: &mut diesel::PgConnection,
    ) -> Result<Vec<Self>, diesel::result::Error> {
        use diesel::RunQueryDsl;
        use diesel::associations::HasTable;
        use diesel::{QueryDsl, ExpressionMethods};
        use crate::codegen::diesel_codegen::tables::directus_relations::directus_relations;
        Self::table()
            .filter(directus_relations::sort_field.eq(sort_field))
            .order_by(directus_relations::id.asc())
            .load::<Self>(conn)
    }
    #[cfg(feature = "postgres")]
    pub fn from_one_deselect_action(
        one_deselect_action: &str,
        conn: &mut diesel::PgConnection,
    ) -> Result<Vec<Self>, diesel::result::Error> {
        use diesel::RunQueryDsl;
        use diesel::associations::HasTable;
        use diesel::{QueryDsl, ExpressionMethods};
        use crate::codegen::diesel_codegen::tables::directus_relations::directus_relations;
        Self::table()
            .filter(directus_relations::one_deselect_action.eq(one_deselect_action))
            .order_by(directus_relations::id.asc())
            .load::<Self>(conn)
    }
}
impl AsRef<DirectusRelation> for DirectusRelation {
    fn as_ref(&self) -> &DirectusRelation {
        self
    }
}
