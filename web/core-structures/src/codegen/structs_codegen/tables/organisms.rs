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
#[diesel(primary_key(id))]
#[diesel(table_name = crate::codegen::diesel_codegen::tables::organisms::organisms)]
pub struct Organism {
    pub id: rosetta_uuid::Uuid,
    pub name: Option<String>,
    pub nameplate_category_id: i16,
}
impl diesel::Identifiable for Organism {
    type Id = rosetta_uuid::Uuid;
    fn id(self) -> Self::Id {
        self.id
    }
}
impl Organism {
    #[cfg(feature = "postgres")]
    pub async fn id(
        &self,
        conn: &mut diesel_async::AsyncPgConnection,
    ) -> Result<crate::codegen::structs_codegen::tables::trackables::Trackable, diesel::result::Error>
    {
        use diesel::{ExpressionMethods, QueryDsl, associations::HasTable};
        use diesel_async::RunQueryDsl;
        crate::codegen::structs_codegen::tables::trackables::Trackable::table()
            .filter(
                crate::codegen::diesel_codegen::tables::trackables::trackables::dsl::id
                    .eq(&self.id),
            )
            .first::<crate::codegen::structs_codegen::tables::trackables::Trackable>(conn)
            .await
    }
    #[cfg(feature = "postgres")]
    pub async fn nameplate_category(
        &self,
        conn: &mut diesel_async::AsyncPgConnection,
    ) -> Result<
        crate::codegen::structs_codegen::tables::nameplate_categories::NameplateCategory,
        diesel::result::Error,
    > {
        use diesel::{ExpressionMethods, QueryDsl, associations::HasTable};
        use diesel_async::RunQueryDsl;
        crate::codegen::structs_codegen::tables::nameplate_categories::NameplateCategory::table()
            .filter(
                crate::codegen::diesel_codegen::tables::nameplate_categories::nameplate_categories::dsl::id
                    .eq(&self.nameplate_category_id),
            )
            .first::<
                crate::codegen::structs_codegen::tables::nameplate_categories::NameplateCategory,
            >(conn)
            .await
    }
    #[cfg(feature = "postgres")]
    pub async fn from_id(
        conn: &mut diesel_async::AsyncPgConnection,
        id: &crate::codegen::structs_codegen::tables::trackables::Trackable,
    ) -> Result<Self, diesel::result::Error> {
        use diesel::{ExpressionMethods, QueryDsl, associations::HasTable};
        use diesel_async::RunQueryDsl;
        Self::table()
            .filter(crate::codegen::diesel_codegen::tables::organisms::organisms::dsl::id.eq(id.id))
            .first::<Self>(conn)
            .await
    }
    #[cfg(feature = "postgres")]
    pub async fn from_nameplate_category_id(
        conn: &mut diesel_async::AsyncPgConnection,
        nameplate_category_id: &crate::codegen::structs_codegen::tables::nameplate_categories::NameplateCategory,
    ) -> Result<Vec<Self>, diesel::result::Error> {
        use diesel::{ExpressionMethods, QueryDsl, associations::HasTable};
        use diesel_async::RunQueryDsl;
        Self::table()
            .filter(
                crate::codegen::diesel_codegen::tables::organisms::organisms::dsl::nameplate_category_id
                    .eq(nameplate_category_id.id),
            )
            .load::<Self>(conn)
            .await
    }
}
