#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "diesel", derive(diesel::Selectable, diesel::Queryable, diesel::Identifiable))]
#[cfg_attr(feature = "diesel", diesel(primary_key(id)))]
# [cfg_attr (feature = "diesel" , diesel (table_name = crate :: codegen :: diesel_codegen :: tables :: sample_container_categories :: sample_container_categories))]
pub struct SampleContainerCategory {
    pub name: String,
    pub volume: String,
    pub unit_id: i16,
    pub material_id: i16,
    pub description: String,
    pub icon_id: i16,
    pub color_id: i16,
    pub id: i16,
}
impl SampleContainerCategory {
    #[cfg(feature = "postgres")]
    pub async fn unit(
        &self,
        conn: &mut diesel_async::AsyncPgConnection,
    ) -> Result<crate::codegen::structs_codegen::tables::units::Unit, diesel::result::Error> {
        use diesel::{associations::HasTable, QueryDsl};
        use diesel_async::RunQueryDsl;
        crate::codegen::structs_codegen::tables::units::Unit::table()
            .find(&self.unit_id)
            .first::<crate::codegen::structs_codegen::tables::units::Unit>(conn)
            .await
    }
    #[cfg(feature = "postgres")]
    pub async fn material(
        &self,
        conn: &mut diesel_async::AsyncPgConnection,
    ) -> Result<crate::codegen::structs_codegen::tables::materials::Material, diesel::result::Error>
    {
        use diesel::{associations::HasTable, QueryDsl};
        use diesel_async::RunQueryDsl;
        crate::codegen::structs_codegen::tables::materials::Material::table()
            .find(&self.material_id)
            .first::<crate::codegen::structs_codegen::tables::materials::Material>(conn)
            .await
    }
    #[cfg(feature = "postgres")]
    pub async fn icon(
        &self,
        conn: &mut diesel_async::AsyncPgConnection,
    ) -> Result<crate::codegen::structs_codegen::tables::icons::Icon, diesel::result::Error> {
        use diesel::{associations::HasTable, QueryDsl};
        use diesel_async::RunQueryDsl;
        crate::codegen::structs_codegen::tables::icons::Icon::table()
            .find(&self.icon_id)
            .first::<crate::codegen::structs_codegen::tables::icons::Icon>(conn)
            .await
    }
    #[cfg(feature = "postgres")]
    pub async fn color(
        &self,
        conn: &mut diesel_async::AsyncPgConnection,
    ) -> Result<crate::codegen::structs_codegen::tables::colors::Color, diesel::result::Error> {
        use diesel::{associations::HasTable, QueryDsl};
        use diesel_async::RunQueryDsl;
        crate::codegen::structs_codegen::tables::colors::Color::table()
            .find(&self.color_id)
            .first::<crate::codegen::structs_codegen::tables::colors::Color>(conn)
            .await
    }
}
