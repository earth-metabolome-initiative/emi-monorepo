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
impl web_common_traits::prelude::TableName for SpatialRefSy {
    const TABLE_NAME: &'static str = "spatial_ref_sys";
}
impl
    web_common_traits::prelude::ExtensionTable<
        crate::codegen::structs_codegen::tables::spatial_ref_sys::SpatialRefSy,
    > for SpatialRefSy
where
    for<'a> &'a Self: diesel::Identifiable<Id = &'a i32>,
{
}
impl diesel::Identifiable for SpatialRefSy {
    type Id = i32;
    fn id(self) -> Self::Id {
        self.srid
    }
}
impl SpatialRefSy {
    #[cfg(feature = "postgres")]
    pub fn from_auth_name(
        auth_name: &str,
        conn: &mut diesel::PgConnection,
    ) -> Result<Vec<Self>, diesel::result::Error> {
        use diesel::{ExpressionMethods, QueryDsl, RunQueryDsl, associations::HasTable};

        use crate::codegen::diesel_codegen::tables::spatial_ref_sys::spatial_ref_sys;
        Self::table()
            .filter(spatial_ref_sys::auth_name.eq(auth_name))
            .order_by(spatial_ref_sys::srid.asc())
            .load::<Self>(conn)
    }
    #[cfg(feature = "postgres")]
    pub fn from_auth_srid(
        auth_srid: &i32,
        conn: &mut diesel::PgConnection,
    ) -> Result<Vec<Self>, diesel::result::Error> {
        use diesel::{ExpressionMethods, QueryDsl, RunQueryDsl, associations::HasTable};

        use crate::codegen::diesel_codegen::tables::spatial_ref_sys::spatial_ref_sys;
        Self::table()
            .filter(spatial_ref_sys::auth_srid.eq(auth_srid))
            .order_by(spatial_ref_sys::srid.asc())
            .load::<Self>(conn)
    }
    #[cfg(feature = "postgres")]
    pub fn from_srtext(
        srtext: &str,
        conn: &mut diesel::PgConnection,
    ) -> Result<Vec<Self>, diesel::result::Error> {
        use diesel::{ExpressionMethods, QueryDsl, RunQueryDsl, associations::HasTable};

        use crate::codegen::diesel_codegen::tables::spatial_ref_sys::spatial_ref_sys;
        Self::table()
            .filter(spatial_ref_sys::srtext.eq(srtext))
            .order_by(spatial_ref_sys::srid.asc())
            .load::<Self>(conn)
    }
    #[cfg(feature = "postgres")]
    pub fn from_proj4text(
        proj4text: &str,
        conn: &mut diesel::PgConnection,
    ) -> Result<Vec<Self>, diesel::result::Error> {
        use diesel::{ExpressionMethods, QueryDsl, RunQueryDsl, associations::HasTable};

        use crate::codegen::diesel_codegen::tables::spatial_ref_sys::spatial_ref_sys;
        Self::table()
            .filter(spatial_ref_sys::proj4text.eq(proj4text))
            .order_by(spatial_ref_sys::srid.asc())
            .load::<Self>(conn)
    }
}
impl AsRef<SpatialRefSy> for SpatialRefSy {
    fn as_ref(&self) -> &SpatialRefSy {
        self
    }
}
