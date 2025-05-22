#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(
    diesel::Selectable,
    diesel::Insertable,
    diesel::AsChangeset,
    diesel::Queryable,
    diesel::Identifiable,
)]
#[cfg_attr(feature = "yew", derive(yew::prelude::Properties))]
#[diesel(primary_key(srid))]
#[diesel(
    table_name = crate::codegen::diesel_codegen::tables::spatial_ref_sys::spatial_ref_sys
)]
pub struct SpatialRefSy {
    pub srid: i32,
    pub auth_name: Option<String>,
    pub auth_srid: Option<i32>,
    pub srtext: Option<String>,
    pub proj4text: Option<String>,
}
impl diesel::Identifiable for SpatialRefSy {
    type Id = i32;
    fn id(self) -> Self::Id {
        self.srid
    }
}
impl SpatialRefSy {
    #[cfg(feature = "postgres")]
    pub async fn from_auth_name(
        auth_name: &Option<String>,
        conn: &mut diesel_async::AsyncPgConnection,
    ) -> Result<Vec<Self>, diesel::result::Error> {
        use diesel::{ExpressionMethods, QueryDsl, associations::HasTable};
        use diesel_async::RunQueryDsl;

        use crate::codegen::diesel_codegen::tables::spatial_ref_sys::spatial_ref_sys;
        Self::table()
            .filter(spatial_ref_sys::auth_name.eq(auth_name))
            .order_by(spatial_ref_sys::srid.asc())
            .load::<Self>(conn)
            .await
    }
    #[cfg(feature = "postgres")]
    pub async fn from_auth_srid(
        auth_srid: &Option<i32>,
        conn: &mut diesel_async::AsyncPgConnection,
    ) -> Result<Vec<Self>, diesel::result::Error> {
        use diesel::{ExpressionMethods, QueryDsl, associations::HasTable};
        use diesel_async::RunQueryDsl;

        use crate::codegen::diesel_codegen::tables::spatial_ref_sys::spatial_ref_sys;
        Self::table()
            .filter(spatial_ref_sys::auth_srid.eq(auth_srid))
            .order_by(spatial_ref_sys::srid.asc())
            .load::<Self>(conn)
            .await
    }
    #[cfg(feature = "postgres")]
    pub async fn from_srtext(
        srtext: &Option<String>,
        conn: &mut diesel_async::AsyncPgConnection,
    ) -> Result<Vec<Self>, diesel::result::Error> {
        use diesel::{ExpressionMethods, QueryDsl, associations::HasTable};
        use diesel_async::RunQueryDsl;

        use crate::codegen::diesel_codegen::tables::spatial_ref_sys::spatial_ref_sys;
        Self::table()
            .filter(spatial_ref_sys::srtext.eq(srtext))
            .order_by(spatial_ref_sys::srid.asc())
            .load::<Self>(conn)
            .await
    }
    #[cfg(feature = "postgres")]
    pub async fn from_proj4text(
        proj4text: &Option<String>,
        conn: &mut diesel_async::AsyncPgConnection,
    ) -> Result<Vec<Self>, diesel::result::Error> {
        use diesel::{ExpressionMethods, QueryDsl, associations::HasTable};
        use diesel_async::RunQueryDsl;

        use crate::codegen::diesel_codegen::tables::spatial_ref_sys::spatial_ref_sys;
        Self::table()
            .filter(spatial_ref_sys::proj4text.eq(proj4text))
            .order_by(spatial_ref_sys::srid.asc())
            .load::<Self>(conn)
            .await
    }
}
