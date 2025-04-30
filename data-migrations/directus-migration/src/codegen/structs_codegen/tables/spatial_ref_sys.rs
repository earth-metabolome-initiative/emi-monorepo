#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "yew", derive(yew::prelude::Properties))]
#[derive(
    diesel::Selectable,
    diesel::Insertable,
    diesel::AsChangeset,
    diesel::Queryable,
    diesel::Identifiable,
)]
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
impl SpatialRefSy {}
